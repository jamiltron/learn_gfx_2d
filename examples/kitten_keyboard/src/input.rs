extern crate glutin;

use glutin::{ElementState, Event, VirtualKeyCode};

pub struct Input {
    pub left_is_pressed: bool,
    pub right_is_pressed: bool,
    pub up_is_pressed: bool,
    pub down_is_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            left_is_pressed: false,
            right_is_pressed: false,
            up_is_pressed: false,
            down_is_pressed: false,
        }
    }

    pub fn handle_keyboard_input(&mut self, event: Event) -> () {
        match event {
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => {
                self.left_is_pressed = true;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => {
                self.right_is_pressed = true;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Up)) => {
                self.up_is_pressed = true;
            }
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Down)) => {
                self.down_is_pressed = true;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Left)) => {
                self.left_is_pressed = false;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Right)) => {
                self.right_is_pressed = false;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Up)) => {
                self.up_is_pressed = false;
            }
            Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Down)) => {
                self.down_is_pressed = false;
            }
            _ => {}
        }
    }
}
