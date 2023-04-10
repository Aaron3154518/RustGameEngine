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

use std::{ffi::CString, mem};

use num_traits::FromPrimitive;

fn get_texture(r: *mut sdl2::SDL_Renderer, file: &str) -> *mut sdl2::SDL_Texture {
    let cstr = CString::new(file).expect("Failed to creat CString");
    unsafe { sdl2_image::IMG_LoadTexture(r, cstr.as_ptr()) }
}
fn main() {
    unsafe {
        // Initialize SDL2
        let sdl_init = sdl2::SDL_Init(sdl2::SDL_INIT_EVERYTHING);
        let img_init = 0;

        // Create a window
        let window = sdl2::SDL_CreateWindow(
            "My SDL2 Window\0".as_ptr() as *const i8,
            sdl2::SDL_WINDOWPOS_CENTERED_MASK as i32,
            sdl2::SDL_WINDOWPOS_CENTERED_MASK as i32,
            640,
            480,
            sdl2::SDL_WindowFlags::SDL_WINDOW_SHOWN as u32,
        );

        let renderer = sdl2::SDL_CreateRenderer(window, -1, 0);
        assert!(!renderer.is_null(), "Failed to create renderer");
        let tex = get_texture(renderer, "res/bra_vector.png");
        if tex.is_null() {
            eprintln!("Failed to load image: {:?}", sdl2::SDL_GetError());
        }

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

            sdl2::SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            sdl2::SDL_RenderClear(renderer);

            // Draw the texture to the center of the screen
            let mut dest_rect = sdl2::SDL_Rect {
                x: (640 - 100) as i32 / 2,
                y: (480 - 100) as i32 / 2,
                w: 100 as i32,
                h: 100 as i32,
            };
            sdl2::SDL_RenderCopy(renderer, tex, std::ptr::null(), &mut dest_rect);

            // Update the screen
            sdl2::SDL_RenderPresent(renderer);
        }

        // Destroy the window and quit SDL2
        sdl2::SDL_DestroyTexture(tex);
        sdl2::SDL_DestroyWindow(window);
        sdl2_image::IMG_Quit();
        sdl2::SDL_Quit();
    }
}
