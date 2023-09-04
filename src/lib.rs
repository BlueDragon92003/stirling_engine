use std::time::{Instant, Duration};
use controls::ControlsState;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod controls;

pub type UpdateMethod = Box<dyn FnMut(
    u128, // Ticks
    &ControlsState  // Controls State
) -> StirlingEngineControlFlow>;

pub struct StirlingEngine {
    //  Engine Settings
    time_per_tick: Duration,
    watchdog_time: Duration,
    update: UpdateMethod,
    //  Engine State
    last_cycle: Instant,
    this_cycle: Instant,
    tick_time: Duration,
        //  Allows for the constant running at max tick rate for 4.22x10^28
        //  years, approximately
    tick: u128,
    controls_state: ControlsState,
}

pub struct StirlingEngineBuilder {
    tps: Option<u32>,
    watchdog_time: Option<Duration>,
    update: Option<UpdateMethod>,
}

#[derive(std::fmt::Debug)]
pub enum StirlingEngineBuilderError {
    MissingTPS,
    MissingWatchdogTime,
    MissingUpdateMethod,
}

pub enum StirlingEngineControlFlow {
    Run,
    Pause,
    Exit
}

impl StirlingEngine {

    pub fn new() -> StirlingEngineBuilder {
        StirlingEngineBuilder {
            tps: None,
            watchdog_time: None,
            update: None,
        }
    }

    pub fn run(mut self) {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        self.last_cycle = Instant::now();
        self.this_cycle = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            //  Game Loop ------------------------\/
            
            //  Update Ticks
            self.last_cycle = self.this_cycle;
            self.this_cycle = Instant::now();

            let time_since_last_tick = 
                self.this_cycle.duration_since(self.last_cycle);

                //  Watchdog check
                //  TODO: In the future, somehow put this on a different thread
                //  Infinite loops will still hang the game
            if time_since_last_tick > self.watchdog_time {
                    *control_flow = ControlFlow::Exit;
                    return;
            }

            self.tick_time += time_since_last_tick;
            
            let mut ticks = 0;
            while self.tick_time > self.time_per_tick {
                ticks += 1;
                self.tick_time -= self.time_per_tick;
            }

            match event {
                Event::DeviceEvent { device_id, event } => {
                    self.controls_state.device_event(device_id, event);
                }
                Event::WindowEvent { window_id, event } => { }
                Event::RedrawRequested( window_id ) => { }
                _ => {}
            }

            for _ in 0..ticks {
                self.tick += 1;
                *control_flow = match (self.update)(
                    self.tick, 
                    &self.controls_state,
                ) {
                    StirlingEngineControlFlow::Run => ControlFlow::Poll,
                    StirlingEngineControlFlow::Pause => ControlFlow::Wait,
                    StirlingEngineControlFlow::Exit => ControlFlow::Exit,
                }
            }


        //  Game Loop ----------------------------/\
        });
    }

}

impl StirlingEngineBuilder {
    
    pub fn set_tps(mut self, tps: u8) -> Self {
        self.tps = Some(tps.into());
        self
    }

    pub fn set_watchdog_time(mut self, watchdog_time: Duration) -> Self {
        self.watchdog_time = Some(watchdog_time);
        self
    }

    pub fn set_update_method(mut self, update_method: UpdateMethod) -> Self{
        self.update = Some(update_method);
        self
    }

    pub fn run(self) -> Result<(),StirlingEngineBuilderError> {
        let time_per_tick = match self.tps {
            Some(tps) => Duration::from_secs(1) / tps,
            None => return Err(StirlingEngineBuilderError::MissingTPS)
        };

        let watchdog_time = match self.watchdog_time {
            Some(watchdog_time) => watchdog_time,
            None => return Err(StirlingEngineBuilderError::MissingWatchdogTime)
        };

        let update = match self.update {
            Some(update) => update,
            None => return Err(StirlingEngineBuilderError::MissingUpdateMethod)
        };

        let engine = StirlingEngine {
            //  Engine Settings
            time_per_tick,
            watchdog_time,
            update,
            //  Enginge State
            last_cycle: Instant::now(),
            this_cycle: Instant::now(),
            tick_time: Duration::from_secs(0),
            tick: 0,
            controls_state: ControlsState::new(),
        };
        engine.run();
        Ok(())
    }

}