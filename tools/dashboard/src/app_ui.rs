use eframe::{run_native, CreationContext, NativeOptions, Result};
use egui::ViewportBuilder;
use std::thread;

impl AppUi {
    pub fn entrypoint() {
        let native_options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
            /*
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
            */
            ..Default::default()
        };
        let result = run_native(
            "UI9 Dashboard",
            native_options,
            Box::new(|cc| Ok(Box::new(AppUi::new(cc)))),
        );
        // TODO: Report an actor/shutdown
    }
}

pub struct AppUi {
    counter: usize,
}

impl AppUi {
    /// Called once before the first frame.
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self { counter: 0 }
    }
}

impl eframe::App for AppUi {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // TODO: Send an event/shutdown to the actor?
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

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
