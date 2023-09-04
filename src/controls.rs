use std::{collections::HashMap, default};

use winit::event::{DeviceEvent, DeviceId, ButtonId, ScanCode};

pub struct ControlsState {
    input_map: HashMap<InputType,String>,
    action_state: HashMap<String,ActionState>,
}

pub enum InputType {
    Button { device_id: DeviceId, button: ButtonInput },
}

pub enum ButtonInput {
    Button { button_id: ButtonId },
    Key { scan_code: ScanCode },
}

enum ActionState {
    Button(ButtonAction)
}

pub enum ButtonAction {
    /// The button has just been pressed, and was open for u32 ticks prior
    /// 
    /// Noe: Pressed(0) means that there was a one-tick gap in the pressing of
    /// buttons
    Pressed(u32),
    /// The button was pressed u32 ticks ago. i.e. Held(1) was pressed last
    /// tick, Held(30) has been pressed for 30 ticks. Held 0 is not a valid
    /// state
    Held(u32),
    /// The button was just released, and was held for u32 ticks prior
    /// 
    /// Note: Released(0) means that there was a one-tick press of the button 
    Released(u32),
    /// The button was released u32 + 1 ticks ago (i.e. Open(1) was released
    /// last tick, Open(30) has been released for 30 ticks total. Open(0) is not
    /// a valid state
    Open(u32),
}

impl ControlsState {

    pub fn new() -> ControlsState {
        ControlsState {
            input_map: HashMap::new(),
            action_state: HashMap::new(),
        }
    }

    pub fn register_action(
        &self,
        action_name: String,
        default_input: Option<InputType>
    ) -> Result<(),()> {
        todo!() // Proper result generics
    }

    pub fn set_action_key(
        &self,
        action_name: String,
        new_input: Option<InputType>
    ) -> Result<(),()> {
        todo!()
    }

    pub fn device_event(&self, device_id: DeviceId, event: DeviceEvent) {
        
    }

}
