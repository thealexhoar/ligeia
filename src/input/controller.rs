use sdl2::{EventPump, GameControllerSubsystem};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Button {
    PAUSE, // ie, controller start
    MENU,  // ie, controller select/back
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Axis {
    MOVE_X,
    MOVE_Y,
    CAMERA_X,
    CAMERA_Y
}

pub trait Controller {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        controller_subsystem: &GameControllerSubsystem
    );
    // Note: axis values are in range [-1.0, 1.0]
    fn axis(&self, axis: Axis) -> f32;
    fn button_down(&self, button: Button) -> bool;
    fn button_up(&self, button: Button) -> bool;
    fn button_pressed(&self, button: Button) -> bool;
    fn button_released(&self, button: Button) -> bool;
}
