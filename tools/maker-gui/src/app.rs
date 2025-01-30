use eframe::{run_native, CreationContext, NativeOptions};
use egui::ViewportBuilder;
use std::time::Duration;
use ui9_app::protocol::UiEvent;
use ui9_app::AppLink;
use ui9_dui::subscriber::PortedState;
use ui9_dui::tracers::peer::Peer;

pub struct AppGui {
    state_changed: bool,
    link: AppLink,
    peers: Option<PortedState<Peer>>,
}

impl AppGui {
    pub fn entrypoint(link: AppLink) {
        let app = link.address.clone();
        let native_options = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_inner_size([400.0, 300.0])
                .with_min_inner_size([300.0, 220.0]),
            ..Default::default()
        };
        let _result = run_native(
            "UI9 Dashboard",
            native_options,
            Box::new(move |cc| Ok(Box::new(AppGui::new(cc, link)))),
        );
        let _result = app.interrupt();
    }

    fn new(_cc: &CreationContext<'_>, link: AppLink) -> Self {
        Self {
            state_changed: false,
            link,
            peers: None,
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(event) = self.link.try_recv() {
            self.apply_event(event);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render(ui);
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

impl AppGui {
    fn apply_event(&mut self, event: UiEvent) {
        match event {
            UiEvent::SetState { peers } => {
                self.peers = Some(peers);
                self.state_changed = true;
            }
            UiEvent::StateChanged => {
                self.state_changed = true;
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
            for (peer_id, _) in peers.peers.iter() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.strong(peer_id.to_string());
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
