#![feature(specialization)]
#![feature(trait_alias)]

mod sdl2_bindings;
use sdl2_bindings::sdl2_ as sdl2;

mod sdl2_image_bindings;
use sdl2_image_bindings::sdl2_image_ as sdl2_image;

mod asset_manager;
use asset_manager::RenderSystem;

mod pointers;
use pointers::*;

mod globals;
use globals::Globals;

mod event;
use event::Event;
mod rect;
use rect::{Dimensions, Rect};

const FPS: u32 = 60;
const FRAME_TIME: u32 = 1000 / FPS;

fn main() {
    // Initialize SDL2
    if unsafe { sdl2::SDL_Init(sdl2::SDL_INIT_EVERYTHING) } == 0 {
        println!("SDL Initialized");
    } else {
        eprintln!("SDL Failed to Initialize");
    }
    let img_init_flags = sdl2_image::IMG_InitFlags::IMG_INIT_PNG as i32
        | sdl2_image::IMG_InitFlags::IMG_INIT_JPG as i32;
    if unsafe { sdl2_image::IMG_Init(img_init_flags) } & img_init_flags == img_init_flags {
        println!("SDL_Image Initialized");
    } else {
        eprintln!("SDL_Image Failed to Initialize");
    }

    let w = 960;
    let h = 720;
    let img_w = 100;

    // Create a window
    let mut globals = Globals {
        rs: RenderSystem::new(Window::new().title("Game Engine").dimensions(w, h)),
    };

    let screen = Dimensions { w, h };
    let camera = Rect {
        x: 0.0,
        y: 0.0,
        w: w as f32,
        h: h as f32,
    };

    let tex = globals.rs.get_image("res/bra_vector.png");
    let mut rect = Rect {
        x: (w - img_w) as f32 / 2.0,
        y: (h - img_w) as f32 / 2.0,
        w: img_w as f32,
        h: img_w as f32,
    };

    let mut event = Event::new();
    let mut t = unsafe { sdl2::SDL_GetTicks() };
    let mut dt;
    while !event.quit {
        dt = unsafe { sdl2::SDL_GetTicks() } - t;
        t += dt;

        event.update(dt, &camera, &screen);

        match event.get_key(sdl2::SDL_KeyCode::SDLK_SPACE) {
            Some(kb) => {
                if kb.held() {
                    println!("_ {}", kb.duration)
                }
            }
            None => (),
        }

        let l = event.get_mouse(event::Mouse::Left);
        if l.clicked() {
            rect.set_pos(
                l.click_pos.x as f32,
                l.click_pos.y as f32,
                rect::Align::Center,
                rect::Align::Center,
            );
            rect.fit_within(&camera);
        }

        // Clear the screen
        globals.rs.r.clear();

        draw!(globals.rs, tex, std::ptr::null(), &rect.to_sdl_rect());

        // Update the screen
        globals.rs.r.present();

        dt = unsafe { sdl2::SDL_GetTicks() } - t;
        if dt < FRAME_TIME {
            unsafe { sdl2::SDL_Delay(FRAME_TIME - dt) };
        }
    }

    // Destroy globals
    drop(globals);

    // Destroy the window and quit SDL2
    unsafe {
        sdl2_image::IMG_Quit();
        sdl2::SDL_Quit();
    }
}
