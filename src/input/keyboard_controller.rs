use sdl2::{
    EventPump,
    GameControllerSubsystem,
    keyboard::{
        KeyboardState,
        Scancode
    },
};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use input::{Axis, Button, Controller};

pub struct KeyboardController {
    axis_bindings: HashMap<Axis, (Scancode, Scancode)>,
    axis_states: HashMap<Axis, f32>,
    key_bindings: HashMap<Button, Scancode>,
    keyboard_states: [HashMap<Button, bool>; 2],
    current_state: usize,
    old_state: usize
}

impl KeyboardController {
    pub fn new(
        axis_bindings: HashMap<Axis, (Scancode, Scancode)>,
        key_bindings: HashMap<Button, Scancode>,
    ) -> Self {
        let mut out = Self {
            axis_bindings,
            axis_states: HashMap::new(),
            key_bindings,
            keyboard_states: [
                HashMap::new(),
                HashMap::new()
            ],
            current_state: 0,
            old_state: 1
        };

        for (axis, _) in &out.axis_bindings {
            out.axis_states.insert(*axis, 0.);
        }
        for (key, _) in &out.key_bindings {
            out.keyboard_states[0].insert(*key, false);
            out.keyboard_states[1].insert(*key, false);
        }

        out
    }
}

impl Controller for KeyboardController {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        controller_subsystem: &GameControllerSubsystem
    ) {
        let keyboard_state = event_pump.keyboard_state();

        let temp = self.old_state;
        self.old_state = self.current_state;
        self.current_state = temp;


        for (key, binding) in &self.key_bindings {
            *self.keyboard_states[self.current_state].get_mut(key).unwrap()
                = keyboard_state.is_scancode_pressed(*binding);
        }

        for (axis, (neg_bind, pos_bind)) in &self.axis_bindings {
            let mut val = 0.;
            if keyboard_state.is_scancode_pressed(*neg_bind) {
                val -= 1.;
            }
            if keyboard_state.is_scancode_pressed(*pos_bind) {
                val += 1.;
            }
            *self.axis_states.get_mut(axis).unwrap() = val;
        }
    }

    fn axis(&self, axis: Axis) -> f32 {
        self.axis_states[&axis]
    }

    fn button_down(&self, button: Button) -> bool {
        self.keyboard_states[self.current_state][&button]
    }

    fn button_up(&self, button: Button) -> bool {
        !self.keyboard_states[self.current_state][&button]
    }

    fn button_pressed(&self, button: Button) -> bool {
        self.keyboard_states[self.current_state][&button]
            && !self.keyboard_states[self.old_state][&button]
    }

    fn button_released(&self, button: Button) -> bool {
        !self.keyboard_states[self.current_state][&button]
            && self.keyboard_states[self.old_state][&button]
    }
}