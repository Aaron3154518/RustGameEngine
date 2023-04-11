#![allow(
    unused_variables,
    unused_assignments,
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]

mod sdl2 {
    include!(concat!(env!("OUT_DIR"), "/sdl2_bindings.rs"));
}
mod sdl2_image {
    include!(concat!(env!("OUT_DIR"), "/sdl2_image_bindings.rs"));
}

use std::mem;

mod pointers;
use pointers::*;

use num_traits::FromPrimitive;

fn main() {
    unsafe {
        // Initialize SDL2
        let sdl_init = sdl2::SDL_Init(sdl2::SDL_INIT_EVERYTHING);
        let img_init = sdl2_image::IMG_Init(sdl2_image::IMG_InitFlags::IMG_INIT_PNG as i32);

        // Create a window
        let window = Window::new().title("Game Engine").dimensions(960, 720);

        let renderer = Renderer::new(&window);
        let tex = Texture::new(&renderer, "res/bra_vector.png");

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

            renderer.clear();

            // Draw the texture to the center of the screen
            let mut dest_rect = sdl2::SDL_Rect {
                x: (640 - 100) as i32 / 2,
                y: (480 - 100) as i32 / 2,
                w: 100 as i32,
                h: 100 as i32,
            };
            tex.draw(&renderer, std::ptr::null(), &mut dest_rect);

            // Update the screen
            renderer.present();
        }

        // Destroy the window and quit SDL2
        sdl2_image::IMG_Quit();
        sdl2::SDL_Quit();
    }
}
