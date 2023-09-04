use std::time::Duration;

use stirling_engine::{StirlingEngine, controls::ControlsState, StirlingEngineControlFlow};

fn main() {
    match StirlingEngine::new()
        .set_tps(20)
        .set_watchdog_time(Duration::from_secs(60))
        .set_update_method(Box::new(update))
        .run() {
            Ok(_) => { }
            Err(error) => println!("{:?}", error),
        }
}

fn update(ticks: u128, _controls_state: &ControlsState)
    -> StirlingEngineControlFlow {
        if ticks < 40 {
            return StirlingEngineControlFlow::Run;
        }
        StirlingEngineControlFlow::Exit
}