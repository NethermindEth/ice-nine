trade:
    RUST_LOG=error,crb=trace,ui9=trace,ice9=trace cargo run -p trading-assistant

maker:
    RUST_LOG=error,crb=trace,ui9=trace,ice9=trace cargo run -p ice9-maker
