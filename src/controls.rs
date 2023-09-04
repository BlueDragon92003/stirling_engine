use std::collections::HashMap;

use winit::event::{DeviceEvent, DeviceId, ButtonId, ScanCode, VirtualKeyCode, ElementState};

pub struct ControlsState {
    button_states: HashMap<Button,ButtonState>,
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
pub enum Button {
    Key(DeviceId,VirtualKeyCode),
    Button(DeviceId,ButtonId),
    KeyButton(DeviceId,ScanCode),
}

#[derive(Clone,Copy)]
pub enum ButtonState {
    /// The button has just been pressed, and was open for u32 ticks prior
    /// 
    /// Note: Pressed(0) means that there was a one-tick gap in the button press
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
    /// last tick, Open(30) has been released for 30 ticks total. Open(0) means
    /// the button has yet to be pressed.
    Open(u32),
}


impl ControlsState {

    pub fn new() -> ControlsState {
        ControlsState {
            button_states: HashMap::new(),
        }
    }

    pub fn handle_device_event(&mut self, device_id: DeviceId, event: DeviceEvent) {
        match event {
            DeviceEvent::Button { button, state } => {
                let button = Button::Button(device_id, button);
                let button_state = match self.button_states.get(&button) {
                    Some(button_state) => *button_state,
                    None => ButtonState::Open(0),
                };
                let new_state = match state {
                    ElementState::Pressed => {
                        match button_state {
                            ButtonState::Pressed(_) => ButtonState::Held(1),
                            ButtonState::Held(i) => ButtonState::Held(i+1),
                            ButtonState::Open(i) => ButtonState::Pressed(i),
                            ButtonState::Released(_) => ButtonState::Pressed(0),
                        }
                    }
                    ElementState::Released => {
                        match button_state {
                            ButtonState::Pressed(_) => ButtonState::Released(0),
                            ButtonState::Held(i) => ButtonState::Released(i),
                            ButtonState::Open(i) => ButtonState::Open(i+1),
                            ButtonState::Released(_) => ButtonState::Open(1),
                        }
                    }
                };
                self.button_states.insert(button, new_state);
            }
            _ => { }
        }
    }

    pub fn get_button_state(&self, button: Button)
    -> ButtonState {
        match self.button_states.get(&button) {
            Some(state) => *state,
            None => ButtonState::Open(0),
        }
    }

    pub fn get_pressed_buttons(&self) -> Vec<Button> {
        let mut buttons = Vec::new();
        for (button, state) in &self.button_states {
            match state {
                ButtonState::Held(_) | ButtonState::Pressed(_) => {
                    buttons.push(*button);
                }
                _ => { }
            }
        }
        buttons
    }

}