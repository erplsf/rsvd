extern crate sdl2;
extern crate ash;

use crate::ash::version::EntryV1_0;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use ash::{vk, Entry, Instance};
use std::ffi::{CString};

fn create_instance() -> Instance {
    let name = CString::new("rsvd").unwrap();
    let appinfo = vk::ApplicationInfo::builder()
        .application_name(&name)
        .application_version(vk::make_version(0, 0, 1))
        .api_version(vk::make_version(1, 2, 0));

    let entry: Entry = Entry::new().unwrap();
    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo);
    
    unsafe {
        let instance = entry
            .create_instance(&create_info, None)
            .expect("Instance creation error");

        return instance
    }
}

fn init_vulkan() {
    create_instance();
}

pub fn main() {
    init_vulkan();
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 10000000));
    }
}
