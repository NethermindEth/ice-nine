use crate::protocol::UiEvent;
use crate::App;
use crb::agent::ToAddress;
use crb::core::mpsc;
use eframe::{run_native, CreationContext, NativeOptions};
use egui::ViewportBuilder;
use std::time::Duration;
use ui9_dui::subscriber::State;
use ui9_dui::tracers::peer::{Peer, PeerId};

pub struct AppUi {
    state_changed: bool,
    events_rx: mpsc::UnboundedReceiver<UiEvent>,
    peers: Option<State<Peer>>,
}

impl AppUi {
    pub fn entrypoint(app: impl ToAddress<App>, rx: mpsc::UnboundedReceiver<UiEvent>) {
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
            Box::new(move |cc| Ok(Box::new(AppUi::new(cc, rx)))),
        );
        let _result = addr.interrupt();
    }

    fn new(_cc: &CreationContext<'_>, events_rx: mpsc::UnboundedReceiver<UiEvent>) -> Self {
        Self {
            state_changed: false,
            events_rx,
            peers: None,
        }
    }
}

impl eframe::App for AppUi {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.events_rx.try_recv() {}

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

impl AppUi {
    fn apply_event(&mut self, event: UiEvent) {
        match event {
            UiEvent::SetState { peers } => {
                self.peers = Some(peers);
            }
        }
    }

    fn render(&self, ui: &mut egui::Ui) {
        if self.render_dashboard(ui).is_none() {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                let dots = ".".repeat(10);
                ui.heading(format!("Loading{}", dots));
            });
        }
    }

    fn render_dashboard(&self, ui: &mut egui::Ui) -> Option<()> {
        let peers = self.peers.as_ref()?.borrow();
        let peers = peers.loaded()?;
        ui.heading("Connected Peers");
        ui.add_space(20.0);

        // Create a scrollable area for the peers list
        egui::ScrollArea::vertical().show(ui, |ui| {
            for peer in peers.peers.iter() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.strong(peer.to_string());
                        /*
                        ui.label(format!("Status: {}", peer.status));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(&peer.last_seen);
                        });
                        */
                    });
                });
                ui.add_space(4.0);
            }
        });
        Some(())
    }
}
