use std::collections::HashMap;

use cgmath::Point2;
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::inputs::winit_input::EngineInput;

pub mod winit_input;

#[derive(Default)]
pub struct Inputs {
    raw_inputs: Vec<EngineInput>,

    mouse_position: Option<Point2<f32>>,
    mouse_scroll_delta: Option<Point2<f32>>,

    key_codes: HashMap<KeyCode, bool>,
    keys_just_pressed: Vec<KeyCode>,
    keys_just_released: Vec<KeyCode>,

    mouse_buttons: HashMap<MouseButton, bool>,
    mouse_buttons_just_pressed: Vec<MouseButton>,
    mouse_buttons_just_released: Vec<MouseButton>
}

impl Inputs {

    /// Returns a new instance of `Inputs`. Just a wrapper of `default()`.
    /// 
    /// This is meant to be called from your engine core and distributed from that core.
    pub fn new() -> Self { Self::default() }

    /// Handles the given input and applies the input to this `Inputs` object.
    /// 
    /// Arguments:
    /// * &mut self - The `Inputs` object to apply too.
    /// * input: EngineInput - The input to apply.
    pub fn handle_input(&mut self, input: EngineInput) {
        match input {
            // update mouse position
            EngineInput::MouseMove(position) => { self.mouse_position = Some(position); },

            // handle mouse buttons
            EngineInput::MouseButton(button, state) => {
                match state {
                    winit::event::ElementState::Pressed => {
                        self.mouse_buttons.insert(button, true);
                        self.mouse_buttons_just_pressed.push(button);
                    },
                    winit::event::ElementState::Released => {
                        self.mouse_buttons.insert(button, false);
                        self.mouse_buttons_just_released.push(button);
                    }
                }
            },

            // handle mouse scroll
            EngineInput::MouseWheel(delta) => {
                let delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => Point2 { x: x * 12.0, y: y * 12.0 },
                    winit::event::MouseScrollDelta::PixelDelta(position) => Point2 { x: position.x as f32, y: position.y as f32 }
                };
                self.mouse_scroll_delta = Some(delta);
            },

            // handle keyboard input
            EngineInput::KeyInput(key, state) => {
                match state {
                    winit::event::ElementState::Pressed => {
                        self.key_codes.insert(key, true);
                        self.keys_just_pressed.push(key);
                    },
                    winit::event::ElementState::Released => {
                        self.key_codes.insert(key, false);
                        self.keys_just_released.push(key);
                    }
                }
            },
        };

        self.raw_inputs.push(input);
    }

    /// Resets this `Inputs` object for the next frame.
    /// 
    /// This is meant to be called by the engine core directly.
    pub fn reset(&mut self) {
        self.mouse_scroll_delta = None;
        self.raw_inputs.clear();
        self.mouse_buttons_just_pressed.clear();
        self.mouse_buttons_just_released.clear();
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
    }

    /// Returns all `EngineInput`s from the last frame
    pub fn raw_inputs(&self) -> &Vec<EngineInput> { &self.raw_inputs }

    /// Returns all pressed mouse buttons
    pub fn mouse_buttons_pressed(&self) -> Vec<&MouseButton> { self.mouse_buttons.iter().filter(|a| *a.1).map(|a| a.0).collect() }

    /// Returns all mouse buttons just pressed
    pub fn mouse_buttons_just_pressed(&self) -> &Vec<MouseButton> { &self.mouse_buttons_just_pressed }

    /// Returns all mouse buttons just released
    pub fn mouse_buttons_just_released(&self) -> &Vec<MouseButton> { &self.mouse_buttons_just_released }

    /// Returns all pressed keyboard inputs
    pub fn keys_pressed(&self) -> Vec<&KeyCode> { self.key_codes.iter().filter(|a| *a.1).map(|a| a.0).collect() }
    
    /// Returns all keys just pressed
    pub fn keys_just_pressed(&self) -> &Vec<KeyCode> { &self.keys_just_pressed }
    
    /// Returns all keys just released
    pub fn keys_just_released(&self) -> &Vec<KeyCode> { &self.keys_just_released }

    /// Returns the current mouse position if we have one.
    pub fn mouse_position(&self) -> Option<&Point2<f32>> { self.mouse_position.as_ref() }
    
    /// Returns the last mouse scroll delta.
    pub fn mouse_scroll_delta(&self) -> &Point2<f32> { self.mouse_scroll_delta.as_ref().unwrap_or(&Point2 { x: 0.0, y: 0.0 }) }

    /// Returns true if the given `KeyCode` is currently pressed.
    pub fn is_key_down(&self, key: &KeyCode) -> bool { *self.key_codes.get(key).unwrap_or(&false) }

    /// Returns true if the given `MouseButton` is currently pressed.
    pub fn is_mouse_button_down(&self, button: &MouseButton) -> bool { *self.mouse_buttons.get(button).unwrap_or(&false) }
    
    /// Returns true if the given `KeyCode` was pressed this frame.
    pub fn key_just_pressed(&self, key: &KeyCode) -> bool { self.keys_just_pressed.contains(key) }

    /// Returns true if the given `KeyCode` was released this frame.
    pub fn key_just_released(&self, key: &KeyCode) -> bool { self.keys_just_released.contains(key) }

    /// Returns true if the given `MouseButton` was pressed this frame.
    pub fn mouse_button_just_pressed(&self, button: &MouseButton) -> bool { self.mouse_buttons_just_pressed.contains(button) }

    /// Returns true if the given `MouseButton` was released this frame.
    pub fn mouse_button_just_released(&self, button: &MouseButton) -> bool { self.mouse_buttons_just_released.contains(button) }
}
