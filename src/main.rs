extern crate sdl2;
extern crate ash;

use crate::ash::version::EntryV1_0;

use sdl2::video::{Window};
use ash::{vk, Entry, Instance};
use std::ffi::{CString};

struct VulkanGod {
    window: Window,
    entry: Entry,
    instance: Instance
}

impl VulkanGod {
    fn new(window: Window) -> Self {
        unsafe {
            let entry: Entry = Entry::new().unwrap();
            let instance = Self::create_instance(&entry);

            VulkanGod{
                window,
                entry,
                instance
            }
        }
    }

    unsafe fn create_instance(entry: &Entry) -> Instance {
        let name = CString::new("rsvd").unwrap();
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(&name)
            .application_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(1, 2, 0));

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&appinfo);
        
        let instance = entry
            .create_instance(&create_info, None)
            .expect("Instance creation error");

        return instance
    }
}

pub fn main() {    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem.window("rsvd", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let vg = VulkanGod::new(window);
}
