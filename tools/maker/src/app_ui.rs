use crate::app::App;
use crate::state::AppFrame;
use crb::agent::{Address, ToAddress};
use crb::core::mpsc;
use eframe::{run_native, CreationContext, NativeOptions};
use egui::ViewportBuilder;
use std::time::Duration;

pub struct AppUi {
    counter: usize,
    frame: Option<AppFrame>,
    receiver: mpsc::UnboundedReceiver<AppFrame>,
    state_changed: bool,
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
            state_changed: false,
        }
    }
}

impl AppUi {
    fn receive_updates(&mut self) {
        for _ in 0..10 {
            if let Ok(frame) = self.receiver.try_recv() {
                self.frame = Some(frame);
                self.state_changed = true;
            } else {
                break;
            }
        }
    }
}

impl eframe::App for AppUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.receive_updates();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dashboard");
            if let Some(frame) = &self.frame {
                for (key, value) in &frame.dashboard {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", key));
                        ui.monospace(value);
                    });
                }
            }
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
