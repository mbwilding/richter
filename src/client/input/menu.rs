// Copyright © 2019 Cormac O'Brien
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

use crate::{client::menu::Menu, common::console::Console};

use failure::Error;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct MenuInput {
    menu: Rc<RefCell<Menu>>,
    console: Rc<RefCell<Console>>,
}

impl MenuInput {
    pub fn new(menu: Rc<RefCell<Menu>>, console: Rc<RefCell<Console>>) -> MenuInput {
        MenuInput { menu, console }
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
                            let menu = self.menu.borrow();
                            match physical_key {
                                PhysicalKey::Code(k) => match k {
                                    Escape => {
                                        if menu.at_root() {
                                            self.console.borrow().stuff_text("togglemenu\n");
                                        } else {
                                            menu.back()?;
                                        }
                                    }
                                    Enter => menu.activate()?,
                                    ArrowUp => menu.prev()?,
                                    ArrowDown => menu.next()?,
                                    ArrowLeft => menu.left()?,
                                    ArrowRight => menu.right()?,
                                    _ => {}
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
