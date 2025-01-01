use std::{sync::mpsc::Receiver, time::Duration};

use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context, ViewportBuilder, ViewportCommand};

#[derive(Debug)]
pub struct Preview {
    shutdown_receiver: Receiver<()>,
}

#[allow(dead_code)]
impl Preview {
    pub const fn new(shutdown_receiver: Receiver<()>) -> Self {
        Self { shutdown_receiver }
    }

    pub fn run(self) {
        let options = NativeOptions {
            viewport: ViewportBuilder::default().with_title("raytracer"),
            ..Default::default()
        };
        let _result = eframe::run_native("raytracer", options, Box::new(|_| Ok(Box::new(self))));
    }
}

impl App for Preview {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if self.shutdown_receiver.try_recv() == Ok(()) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| ui.image("https://picsum.photos/480"));
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
