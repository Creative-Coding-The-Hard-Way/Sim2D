use {
    crate::{
        graphics::{AssetLoader, Image},
        math::Vec2,
        Sketch,
    },
    anyhow::Result,
};

#[derive(Default, Clone)]
pub struct LoadingSketch {
    loading: Image,
    angle: f32,
}

impl Sketch for LoadingSketch {
    fn setup(&mut self, sim: &mut crate::Sim2D) {
        sim.g.clear_color = [0.5, 0.5, 0.8, 1.0];
        sim.w.resize(1200.0, 800.0);

        self.angle = 0.0
    }

    fn preload(&mut self, asset_loader: &mut AssetLoader) -> Result<()> {
        self.loading = asset_loader.load_image(
            image::load_from_memory_with_format(
                include_bytes!("./loading.png"),
                image::ImageFormat::Png,
            )
            .unwrap()
            .into_rgba8(),
            true,
        );
        Ok(())
    }

    fn update(&mut self, sim: &mut crate::Sim2D) {
        self.angle += sim.dt() * std::f32::consts::PI * 0.5;

        sim.g.image = self.loading;
        sim.g.rect_centered(
            Vec2::zeros(),
            Vec2::new(self.loading.width(), self.loading.height()),
            0.0,
        );
        sim.g.rect_centered(
            sim.w.mouse_pos(),
            Vec2::new(self.loading.width(), self.loading.height()),
            self.angle,
        );
    }
}
