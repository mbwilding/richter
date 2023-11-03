// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{cell::RefCell, rc::Rc};

use crate::common::console::Console;

use failure::Error;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct ConsoleInput {
    console: Rc<RefCell<Console>>,
}

impl ConsoleInput {
    pub fn new(console: Rc<RefCell<Console>>) -> ConsoleInput {
        ConsoleInput { console }
    }

    pub fn handle_event<T>(&self, event: Event<T>) -> Result<(), Error> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { event, .. } => match event {
                    KeyEvent {
                        state,
                        physical_key,
                        ..
                    } => {
                        if state.is_pressed() {
                            use KeyCode::*;
                            let mut console = self.console.borrow_mut();
                            match physical_key {
                                PhysicalKey::Code(k) => match k {
                                    ArrowUp => console.history_up(),
                                    ArrowDown => console.history_down(),
                                    ArrowLeft => console.cursor_left(),
                                    ArrowRight => console.cursor_right(),
                                    Backquote => console.stuff_text("toggleconsole\n"),
                                    _ => {
                                        if let Some(text) = event.text {
                                            if let Some(c) = text.chars().next() {
                                                console.send_char(c);
                                            }
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                },
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}
