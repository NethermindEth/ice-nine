use crate::app::App;
use crate::state::AppFrame;
use crb::agent::{Address, ToAddress};
use crb::core::mpsc;
use eframe::{run_native, CreationContext, NativeOptions};
use egui::ViewportBuilder;

pub struct AppUi {
    counter: usize,
    frame: Option<AppFrame>,
    receiver: mpsc::UnboundedReceiver<AppFrame>,
}

impl AppUi {
    pub fn entrypoint(app: impl ToAddress<App>, receiver: mpsc::UnboundedReceiver<AppFrame>) {
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
            Box::new(move |cc| Ok(Box::new(AppUi::new(cc, receiver)))),
        );
        let _result = addr.interrupt();
    }

    fn new(_cc: &CreationContext<'_>, receiver: mpsc::UnboundedReceiver<AppFrame>) -> Self {
        Self {
            counter: 0,
            frame: None,
            receiver,
        }
    }
}

impl eframe::App for AppUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dashboard");
            /*
            for (key, value) in &self.data {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", key));
                    ui.monospace(value);
                });
            }
            */
        });
    }
}

/*
        while let Ok(message) = self.receiver.try_recv() {
            self.data.push(message);
        }
*/
