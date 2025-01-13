use std::{sync::mpsc::Receiver, time::Duration};

use eframe::{App, Frame, NativeOptions};
use egui::{
    load::SizedTexture, CentralPanel, ColorImage, Context, TextureOptions, ViewportBuilder,
    ViewportCommand,
};

use crate::{color::Color, image::Image};

#[derive(Debug)]
pub struct Preview {
    shutdown_rx: Receiver<()>,
    samples_rx: Receiver<((u32, u32), Color)>,
    render_target: Image,
}

#[allow(dead_code)]
impl Preview {
    pub const fn new(
        shutdown_rx: Receiver<()>,
        samples_rx: Receiver<((u32, u32), Color)>,
        render_target: Image,
    ) -> Self {
        Self {
            shutdown_rx,
            samples_rx,
            render_target,
        }
    }

    pub fn run(self) {
        let options = NativeOptions {
            viewport: ViewportBuilder::default().with_title("raytracer"),
            ..Default::default()
        };
        eframe::run_native("raytracer", options, Box::new(|_| Ok(Box::new(self))))
            .expect("Could not start preview");
    }
}

impl App for Preview {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if self.shutdown_rx.try_recv() == Ok(()) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        while let Ok(((x, y), color)) = self.samples_rx.try_recv() {
            self.render_target.set_pixel(x, y, color);
        }
        let texture_handle = ctx.load_texture(
            "preview".to_string(),
            ColorImage::from(&self.render_target),
            TextureOptions::default(),
        );
        #[allow(clippy::cast_precision_loss)]
        let texture = SizedTexture::new(
            texture_handle.id(),
            egui::vec2(
                self.render_target.width() as f32,
                self.render_target.height() as f32,
            ),
        );

        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.image(texture));
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
