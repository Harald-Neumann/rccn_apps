use anyhow::Result;
use rccn_usr::{pus::app::PusApp, zenoh::key_expr::OwnedKeyExpr};
use bsdiff_service::service::BsdiffService;

mod bsdiff_service;

fn main() -> Result<()> {
    let mut app = PusApp::new(131);

    app
        .add_tc_tm_channel(
            OwnedKeyExpr::new("vc/bus_realtime/rx").unwrap(),
            OwnedKeyExpr::new("vc/bus_realtime/tx").unwrap(),
        )
        .unwrap();

    let diffing_service = BsdiffService::new();
    app.register_service(diffing_service);

    app.run();
    Ok(())
}