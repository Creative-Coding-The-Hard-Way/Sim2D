#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct TextureId {
    index: i32,
}

impl Default for TextureId {
    fn default() -> Self {
        Self::no_texture()
    }
}

impl TextureId {
    pub const fn no_texture() -> Self {
        Self { index: -1 }
    }

    pub(crate) fn from_raw(index: i32) -> Self {
        Self { index }
    }

    pub(crate) fn get_index(&self) -> i32 {
        self.index
    }
}
