use {
    crate::{graphics::vulkan::sync::UsedByFrames, trace},
    anyhow::{Context, Result},
    std::{
        sync::mpsc::{
            Receiver, RecvTimeoutError, Sender, SyncSender, TryRecvError,
        },
        time::{Duration, Instant},
    },
};

pub struct AsyncNBufferClient<T: Send + Sync> {
    free_resource_reciever: Receiver<T>,
    publish_resource_sender: SyncSender<T>,
}

impl<T: Send + Sync + 'static> AsyncNBufferClient<T> {
    /// Try to get a free resource without blocking. If a resource is not
    /// instantly available, then gives up and returns None.
    pub fn try_get_free_resource(&self) -> Result<Option<T>> {
        let resource = match self.free_resource_reciever.try_recv() {
            Ok(t) => Some(t),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                anyhow::bail!("Free resource sender hung up!");
            }
        };
        Ok(resource)
    }

    /// Best-effort blocks until a resource is available. Gives up after a short
    /// delay and returns None.
    pub fn wait_for_free_resource(&self) -> Result<Option<T>> {
        let resource = match self
            .free_resource_reciever
            .recv_timeout(Duration::from_millis(100))
        {
            Ok(resource) => Some(resource),
            Err(RecvTimeoutError::Timeout) => {
                log::warn!(
                    "{}",
                    trace!("Timeout while waiting for a resource!")()
                );
                None
            }
            Err(RecvTimeoutError::Disconnected) => {
                anyhow::bail!("Free resource sender hung up!");
            }
        };
        Ok(resource)
    }

    /// Publish the given resource to become the new current. When the old
    /// current is no-longer in use by any frames, it will be freed to be
    /// used again.
    pub fn make_resource_current(&self, resource: T) -> Result<()> {
        self.publish_resource_sender
            .send(resource)
            .with_context(trace!("Unable to publish resource!"))
    }
}

/// A variant of the NBuffer which is designed to work
pub struct AsyncNBuffer<T: Send + Sync> {
    last_update: Instant,
    current: UsedByFrames<T>,
    free: Sender<T>,
    in_use: Vec<UsedByFrames<T>>,
    published_resource_reciever: Receiver<T>,
}

impl<T: Send + Sync + 'static> AsyncNBuffer<T> {
    pub fn new(mut resources: Vec<T>) -> Result<(Self, AsyncNBufferClient<T>)> {
        let (free_resource_sender, free_resource_reciever) =
            std::sync::mpsc::channel::<T>();

        let (publish_resource_sender, published_resource_reciever) =
            std::sync::mpsc::sync_channel::<T>(1);

        let current = UsedByFrames::new(resources.pop().unwrap());
        let in_use: Vec<UsedByFrames<T>> = Vec::with_capacity(resources.len());
        for resource in resources.drain(0..) {
            free_resource_sender
                .send(resource)
                .with_context(trace!("Unable to send free resource!"))?;
        }
        let async_n_buffer = Self {
            last_update: Instant::now(),
            current,
            free: free_resource_sender,
            in_use,
            published_resource_reciever,
        };
        let client = AsyncNBufferClient {
            free_resource_reciever,
            publish_resource_sender,
        };
        Ok((async_n_buffer, client))
    }

    pub fn get_current_with_update_time(
        &mut self,
        frame_index: usize,
    ) -> Result<(&mut T, Instant)> {
        if let Some(new_current) = self.try_receive_published_resource()? {
            self.make_current(new_current);
            self.last_update = Instant::now();
        }
        self.current.frame_mask.add_frame(frame_index as u32);

        {
            let mut in_use_update = Vec::with_capacity(self.in_use.len());
            for mut in_use in self.in_use.drain(0..) {
                in_use.frame_mask.remove_frame(frame_index as u32);
                if in_use.frame_mask.is_empty() {
                    self.free.send(in_use.release()).with_context(trace!(
                        "Unable to send newly freed resource!"
                    ))?;
                } else {
                    in_use_update.push(in_use);
                }
            }
            self.in_use = in_use_update;
        }
        let last_update = self.last_update;
        Ok((&mut self.current.resource, last_update))
    }

    pub fn get_current(&mut self, frame_index: usize) -> Result<&mut T> {
        if let Some(new_current) = self.try_receive_published_resource()? {
            self.make_current(new_current);
            self.last_update = Instant::now();
        }
        self.current.frame_mask.add_frame(frame_index as u32);

        {
            let mut in_use_update = Vec::with_capacity(self.in_use.len());
            for mut in_use in self.in_use.drain(0..) {
                in_use.frame_mask.remove_frame(frame_index as u32);
                if in_use.frame_mask.is_empty() {
                    self.free.send(in_use.release()).with_context(trace!(
                        "Unable to send newly freed resource!"
                    ))?;
                } else {
                    in_use_update.push(in_use);
                }
            }
            self.in_use = in_use_update;
        }

        Ok(&mut self.current.resource)
    }

    /// Release all in-use resources.
    ///
    /// This can be useful to call right after waiting for every frame to
    /// complete.
    pub fn free_all(&mut self) -> Result<()> {
        for in_use in self.in_use.drain(0..) {
            self.free
                .send(in_use.release())
                .with_context(trace!("Unable to send newly freed resource!"))?;
        }
        Ok(())
    }

    // Private API

    fn try_receive_published_resource(&self) -> Result<Option<T>> {
        let result = match self.published_resource_reciever.try_recv() {
            Ok(t) => Some(t),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                anyhow::bail!(trace!("Resource sender hung up!")());
            }
        };
        Ok(result)
    }

    /// Make the given resource current.
    fn make_current(&mut self, resource: T) {
        let old_current = {
            let mut new_value = UsedByFrames::new(resource);
            std::mem::swap(&mut self.current, &mut new_value);
            new_value
        };
        self.in_use.push(old_current);
    }
}
