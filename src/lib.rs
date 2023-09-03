use std::time::{Instant, Duration};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct StirlingEngine {
    //  Engine Settings
    time_per_tick: Duration,
    watchdog_time: Duration,
    //  Engine State
    last_cycle: Instant,
    this_cycle: Instant,
    tick_time: Duration,
        //  Allows for the constant running at max tick rate for 4.22x10^28
        //  years, approximately
    tick: u128,
}

pub struct StirlingEngineBuilder {
    tps: Option<u32>,
    watchdog_time: Option<Duration>,
}

pub enum StirlingEngineResult {
    Closed
}

pub enum StirlingEngineBuilderError {
    MissingTPS,
    MissingWatchdogTime,
}

impl StirlingEngine {

    pub fn new() -> StirlingEngineBuilder {
        StirlingEngineBuilder {
            tps: None,
            watchdog_time: None,
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
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                _ => {}
            }

            for _ in 0..ticks {
                self.tick += 1;
                //  Update Method
            }


        //  Game Loop ----------------------------/\
        });
    }

}

impl StirlingEngineBuilder {
    
    pub fn set_tps(mut self, tps: u8) {
        self.tps = Some(tps.into())
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

        let engine = StirlingEngine {
            //  Engine Settings
            time_per_tick,
            watchdog_time,
            //  Enginge State
            last_cycle: Instant::now(),
            this_cycle: Instant::now(),
            tick_time: Duration::from_secs(0),
            tick: 0,
        };
        engine.run();
        Ok(())
    }

}