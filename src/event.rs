use crate::rect::*;
use crate::sdl2;
use std::{collections::HashMap, mem};

const NUM_MICE: usize = 3;

#[repr(u8)]
#[derive(Copy, Clone)]
enum Mouse {
    Left = 0,
    Right,
    Middle,
}

impl Mouse {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Mouse::Left),
            1 => Some(Mouse::Right),
            2 => Some(Mouse::Middle),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Status {
    Down = 0x01,
    Up = 0x02,
    Held = 0x04,
    Pressed = 0x08,
}

#[derive(Copy, Clone)]
struct MouseButton {
    mouse: Mouse,
    click_pos: (i32, i32),
    duration: u32,
    status: u8,
}

impl MouseButton {
    fn down(&self) -> bool {
        self.status & Status::Down as u8 != 0
    }

    fn up(&self) -> bool {
        self.status & Status::Up as u8 != 0
    }

    fn held(&self) -> bool {
        self.status & Status::Held as u8 != 0
    }

    fn clicked(&self) -> bool {
        self.status & Status::Pressed as u8 != 0
    }
}

#[derive(Copy, Clone)]
struct KeyButton {
    key: sdl2::SDL_KeyCode,
    duration: u32,
    status: u8,
}

impl KeyButton {
    fn down(&self) -> bool {
        self.status & Status::Down as u8 != 0
    }

    fn up(&self) -> bool {
        self.status & Status::Up as u8 != 0
    }

    fn held(&self) -> bool {
        self.status & Status::Held as u8 != 0
    }

    fn pressed(&self) -> bool {
        self.status & Status::Pressed as u8 != 0
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum InputSeek {
    None = 0,
    Start,
    End,
}

struct Event {
    dt: u32,
    quit: bool,
    resized: bool,
    old_dim: Point,
    new_dim: Point,
    mouse: Point,
    abs_mouse: Point,
    mouse_delta: Point,
    scroll: i32,
    input_text: String,
    input_backspace: i32,
    input_delete: i32,
    input_move: i32,
    input_seek: InputSeek,
    mouse_buttons: [MouseButton; NUM_MICE],
    key_buttons: HashMap<sdl2::SDL_KeyCode, KeyButton>,
}

impl Event {
    pub fn update(&mut self, ts: u32, camera: &Rect, screen: &Dimensions) {
        self.dt = ts;
        // Reset event flags
        self.quit = false;
        self.resized = false;
        // Reset text editing
        self.input_text.clear();
        self.input_backspace = 0;
        self.input_delete = 0;
        self.input_move = 0;
        self.input_seek = InputSeek::None;
        // Update mouse
        let (mut x, mut y) = (0, 0);
        unsafe {
            sdl2::SDL_GetMouseState(&mut x, &mut y);
        }
        self.abs_mouse = Point { x, y };
        self.mouse = Point {
            x: (x * camera.w() / screen.w + camera.x()) as i32,
            y: (y * camera.h() / screen.h + camera.y()) as i32,
        };
        // Reset mouse movement
        self.mouse_delta = Point { x: 0, y: 0 };
        self.scroll = 0;
        // Update mouse buttons
        for b in &mut self.mouse_buttons {
            if b.held() {
                b.duration += ts;
            }
            // Reset pressed/released
            b.status &= Status::Held as u8;
        }
        // Update keys
        for (_, b) in &mut self.key_buttons {
            if b.held() {
                b.duration += ts;
            }
            // Reset pressed/released
            b.status &= Status::Held as u8;
        }
        // Handle events
        let mut event = unsafe { mem::zeroed() };
        while unsafe { sdl2::SDL_PollEvent(&mut event) } != 0 {
            self.updateEvent(&event);
        }
    }

    fn updateEvent(&mut self, event: &sdl2::SDL_Event) {
        match event.type_ {
            SDL_QUIT => {
                self.quit = true;
            }
            SDL_WINDOWEVENT => match event.window.event {
                SDL_WINDOWEVENT_SHOWN => {
                    let window = SDL_GetWindowFromID(event.window.windowID);
                    let mut old_dim = mem::MaybeUninit::uninit();
                    let mut old_dim_ptr = old_dim.as_mut_ptr();
                    SDL_GetWindowSize(window, old_dim_ptr, old_dim_ptr.offset(1));
                    self.old_dim = old_dim.assume_init();
                    self.new_dim = self.old_dim;
                }
                SDL_WINDOWEVENT_RESIZED => {
                    self.resized = true;
                    self.old_dim = self.new_dim;
                    self.new_dim = Dimensions::new(event.window.data1, event.window.data2);
                }
                _ => {}
            },
            SDL_MOUSEBUTTONDOWN => {
                let button = &mut self.mouse_buttons[to_mouse(event.button.button)];
                button.status = Status::DOWN | Status::HELD;
                button.duration = 0;
                button.click_pos = self.mouse;
            }
            SDL_MOUSEBUTTONUP => {
                let button = &mut self.mouse_buttons[to_mouse(event.button.button)];
                let max_click_diff = MAX_CLICK_DIFF;
                button.status = if (button.click_pos - self.mouse).length() < max_click_diff {
                    Status::CLICKED | Status::UP
                } else {
                    Status::UP
                };
                button.duration = 0;
            }
            SDL_MOUSEMOTION => {
                self.mouse_delta = Vector2::new(event.motion.xrel, event.motion.yrel);
            }
            SDL_MOUSEWHEEL => {
                self.scroll = -event.wheel.y;
            }
            SDL_KEYDOWN => {
                let b = self.get(event.key.keysym.sym);
                let held = b.held();
                b.status = Status::PRESSED | Status::HELD;
                if !held {
                    b.status |= Status::DOWN;
                    b.duration = 0;
                }
                if SDL_IsTextInputActive() != SDL_bool::SDL_FALSE {
                    self.process_text_input_key(&b);
                }
            }
            SDL_KEYUP => {
                let b = self.get(event.key.keysym.sym);
                b.status = Status::UP;
            }
            SDL_TEXTEDITING => {}
            SDL_TEXTINPUT => {
                let text = unsafe { CStr::from_ptr(event.text.text.as_ptr() as *const _) }
                    .to_string_lossy()
                    .to_owned()
                    .to_string();
                self.input_text.push_str(&text);
            }
            _ => {}
        }
    }

    fn process_text_input_key(&mut self, b: &KeyButton) {
        match b.key {
            SDLK_BACKSPACE => {
                if self.input_text.is_empty() {
                    self.input_backspace += 1;
                } else {
                    self.input_text.pop();
                }
            }
            SDLK_DELETE => {
                self.input_delete += 1;
            }
            SDLK_LEFT => {
                self.input_move -= 1;
            }
            SDLK_RIGHT => {
                self.input_move += 1;
            }
            SDLK_HOME => {
                self.input_seek = InputSeek::Start;
            }
            SDLK_END => {
                self.input_seek = InputSeek::End;
            }
            _ => {}
        }
    }

    fn mouse_moved(&self) -> bool {
        self.mouse_delta.0 != 0 || self.mouse_delta.1 != 0
    }

    fn get_key(&mut self, key: sdl2::SDL_KeyCode) -> &KeyButton {
        // Implementation goes here
        self.key_buttons.entry(key).or_insert_with(|| KeyButton {
            key,
            duration: 0,
            status: 0,
        })
    }

    fn get_mouse(&self, sdl_button_type: u8) -> &MouseButton {
        // Implementation goes here
        &self.mouse_buttons[sdl_button_type as usize]
    }

    fn get_mouse_button(&self, button: Mouse) -> &MouseButton {
        // Implementation goes here
        &self.mouse_buttons[button as usize]
    }
}
