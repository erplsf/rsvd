use ash::Device;
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::version::{InstanceV1_0, DeviceV1_0};
use ash::vk::PhysicalDevice;
use ash::{vk, Entry, Instance};
use sdl2::video::Window;

use crate::builder;

pub struct Vg {
    entry: Entry,
    window: Window,
    instance: Instance,
    surface_loader: Surface,
    surface: vk::SurfaceKHR,
    debug_utils_loader: DebugUtils,
    debug_call_back: vk::DebugUtilsMessengerEXT,
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    device: Device,
    queue: vk::Queue,
}

impl Vg {
    pub fn new(dimensions: (u32, u32)) -> Self {
        // init vulkan
        let mut debug_info = builder::create_debug_info();
        let entry = builder::create_entry();
        let (window, instance) = builder::create_instance(dimensions, &entry, &mut debug_info);
        let (debug_utils_loader, debug_call_back) =
            builder::setup_debug_messenger(&entry, &instance, &mut debug_info);
        let (surface_loader, surface) = builder::create_surface(&entry, &instance, &window);
        let (physical_device, queue_family_index) = builder::pick_physical_device_and_index(&instance, &surface_loader, &surface);
        let device = builder::create_logical_device(queue_family_index, &instance, &physical_device);
        let queue = builder::get_device_queue(queue_family_index, 0, &device);

        Self {
            entry,
            window,
            instance,
            debug_utils_loader,
            debug_call_back,
            surface_loader,
            surface,
            physical_device,
            queue_family_index,
            device,
            queue
        }
    }
}

impl Drop for Vg {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_call_back, None);
            self.instance.destroy_instance(None);
        }
    }
}
