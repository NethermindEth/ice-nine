use crate::app::App;
use crb::agent::Address;
use eframe::{run_native, CreationContext, NativeOptions};
use egui::ViewportBuilder;

pub struct AppUi {
    counter: usize,
}

impl AppUi {
    pub fn entrypoint(app: Address<App>) {
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
        let _result = app.interrupt();
    }
}

impl AppUi {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for AppUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("UI9 App");
            if ui.button("Increment").clicked() {
                self.counter += 1;
            }
            ui.label(format!("Counter: {}", self.counter));
        });
    }
}
