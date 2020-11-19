extern crate sdl2;
extern crate ash;

use ash::version::{InstanceV1_0, EntryV1_0};
use sdl2::video::{Window};
use ash::{vk, Entry, Instance};
use std::ffi::{CString};

struct VulkanGod {
    window: Window,
    entry: Entry,
    instance: Instance
}

impl VulkanGod {
    fn new(width: u32, height: u32) -> Self {
        unsafe {
            let entry: Entry = Entry::new().unwrap();
            let window = Self::create_window(width, height);
            let instance = Self::create_instance(&window, &entry);

            VulkanGod{
                entry,
                window,
                instance
            }
        }
    }

    fn create_window(width: u32, height: u32) -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        video_subsystem.window("rsvd", width, height)
            .position_centered()
            .build()
            .unwrap()
    }

    unsafe fn create_instance(window: &Window, entry: &Entry) -> Instance {
        let name = CString::new("rsvd").unwrap();
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(&name)
            .application_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(1, 2, 0));

        let required_extensions = window.vulkan_instance_extensions().unwrap();

        let required_extensions_raw = required_extensions.iter()
            .map(|ext| CString::new(*ext).unwrap().as_ptr())
            .collect::<Vec<_>>();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&appinfo)
            .enabled_extension_names(&required_extensions_raw);
        
        let instance = entry
            .create_instance(&create_info, None)
            .expect("Instance creation error");

        return instance
    }
}

impl Drop for VulkanGod {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

pub fn main() {    
    let _vg = VulkanGod::new(800, 600);
}
