use sdl2::video::{Window};
use ash::{vk, Entry, Instance};
use ash::extensions::{ext::DebugUtils};
use ash::version::InstanceV1_0;

pub struct Vg {
    entry: Entry,
    window: Window,
    instance: Instance,
    // physical_device: PhysicalDevice,
    debug_utils_loader: DebugUtils,
    debug_call_back: vk::DebugUtilsMessengerEXT
}

impl Vg {
    pub fn new(
        entry: Entry,
        window: Window,
        instance: Instance,
        // physical_device: PhysicalDevice,
        debug_utils_loader: DebugUtils,
        debug_call_back: vk::DebugUtilsMessengerEXT
    ) -> Self {
        Self{
            entry,
            window,
            instance,
            // physical_device,
            debug_utils_loader,
            debug_call_back
        }
    }
}

impl Drop for Vg {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_call_back, None);
            self.instance.destroy_instance(None);
        }
    }
}
