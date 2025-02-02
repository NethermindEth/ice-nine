trade:
    RUST_LOG=none cargo run -p trading-assistant

gui:
    RUST_LOG=error,crb=trace,ui9=trace,ice9=trace cargo run -p ice9-maker-gui

tui:
    cargo run -p ice9-maker-tui
