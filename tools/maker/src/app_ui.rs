use crate::App;
use crb::agent::ToAddress;
use eframe::{run_native, CreationContext, NativeOptions};
use egui::ViewportBuilder;
use std::time::Duration;

pub struct AppUi {
    state_changed: bool,
}

impl AppUi {
    pub fn entrypoint(app: impl ToAddress<App>) {
        let addr = app.to_address();
        let native_options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
            ..Default::default()
        };
        let _result = run_native(
            "UI9 Dashboard",
            native_options,
            Box::new(move |cc| Ok(Box::new(AppUi::new(cc)))),
        );
        let _result = addr.interrupt();
    }

    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            state_changed: false,
        }
    }
}

impl eframe::App for AppUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dashboard");
        });

        if self.state_changed {
            ctx.request_repaint();
            // TODO: Consider using an adaptive rate here
            self.state_changed = false;
        } else {
            ctx.request_repaint_after(Duration::from_millis(250));
        }
    }
}
