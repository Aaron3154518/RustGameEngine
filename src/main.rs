#![feature(specialization)]
#![feature(trait_alias)]

mod sdl2_bindings;
use sdl2_bindings::sdl2_ as sdl2;

mod sdl2_image_bindings;
use sdl2_image_bindings::sdl2_image_ as sdl2_image;

use std::mem;

mod asset_manager;
use asset_manager::RenderSystem;

mod pointers;
use pointers::*;

mod globals;
use globals::Globals;

mod event;
mod rect;
use rect::Rect;

use num_traits::FromPrimitive;

fn main() {
    unsafe {
        // Initialize SDL2
        if sdl2::SDL_Init(sdl2::SDL_INIT_EVERYTHING) == 0 {
            println!("SDL Initialized");
        } else {
            eprintln!("SDL Failed to Initialize");
        }
        let img_init_flags = sdl2_image::IMG_InitFlags::IMG_INIT_PNG as i32
            | sdl2_image::IMG_InitFlags::IMG_INIT_JPG as i32;
        if sdl2_image::IMG_Init(img_init_flags) & img_init_flags == img_init_flags {
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

        let tex = globals.rs.get_image("res/bra_vector.png");
        let rect = Rect {
            x: (w - img_w) / 2,
            y: (h - img_w) / 2,
            w: img_w,
            h: img_w,
        };

        // Wait for a key press
        sdl2::SDL_EventState(
            sdl2::SDL_EventType::SDL_KEYDOWN as u32,
            sdl2::SDL_ENABLE as i32,
        );
        loop {
            let mut event: sdl2::SDL_Event = mem::zeroed();
            if sdl2::SDL_PollEvent(&mut event) != 1 {
                continue;
            }

            match FromPrimitive::from_u32(event.type_) {
                Some(sdl2::SDL_EventType::SDL_QUIT) => break,
                Some(sdl2::SDL_EventType::SDL_MOUSEBUTTONUP) => println!("Up"),
                _ => (),
            }

            // Clear the screen
            globals.rs.r.clear();

            draw!(globals.rs, tex, std::ptr::null(), &rect);

            // Update the screen
            globals.rs.r.present();
        }

        // Destroy globals
        drop(globals);

        // Destroy the window and quit SDL2
        sdl2_image::IMG_Quit();
        sdl2::SDL_Quit();
    }
}
