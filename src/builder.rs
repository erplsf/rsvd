use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk::PhysicalDevice;
use ash::{vk, Entry, Instance};
use sdl2::video::Window;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use ash_window;

const APP_NAME: &str = "rsvd";

pub fn create_entry() -> Entry {
    Entry::new().unwrap()
}

pub fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(vulkan_debug_callback))
        .build()
}

pub fn create_instance(
    (width, height): (u32, u32),
    entry: &Entry,
    debug_info: &mut vk::DebugUtilsMessengerCreateInfoEXT,
) -> (Window, Instance) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(APP_NAME, width, height)
        .vulkan()
        .position_centered()
        .build()
        .unwrap();

    let name = CString::new(APP_NAME).unwrap();
    let appinfo = vk::ApplicationInfo::builder()
        .application_name(&name)
        .application_version(vk::make_version(0, 1, 0))
        .api_version(vk::make_version(1, 2, 0));

    // get required extensions from sdl2
    let required_extensions = window.vulkan_instance_extensions().unwrap();

    // dbg!(&required_extensions);
    // dbg!(entry.enumerate_instance_extension_properties().unwrap());
    // dbg!(entry.enumerate_instance_layer_properties().unwrap());
    // convert them to raw pointers
    let required_extensions_raw: Vec<_> = required_extensions
        .iter()
        .map(|&ext| CString::new(ext).unwrap().into_raw())
        .collect();

    // cast them for the next call
    let mut required_extensions_raw_const: Vec<_> = required_extensions_raw
        .iter()
        .map(|&raw| raw as *const c_char)
        .collect();

    // add debug_utils extension so we can have messages on stdout
    required_extensions_raw_const.push(DebugUtils::name().as_ptr());

    // TODO: move this to compile time flags to be enabled conditionally
    let layer_names = vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()];

    let layers_names_raw: Vec<*const i8> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();
    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo)
        .enabled_layer_names(&layers_names_raw)
        .enabled_extension_names(&required_extensions_raw_const)
        .push_next(debug_info);

    let instance: Instance;
    unsafe {
        instance = entry
            .create_instance(&create_info, None)
            .expect("Instance creation error");

        // recreate them from raw pointers so rust can deallocate them
        for raw in required_extensions_raw {
            CString::from_raw(raw);
        }
    }

    (window, instance)
}

pub fn setup_debug_messenger(
    entry: &Entry,
    instance: &Instance,
    debug_info: &mut vk::DebugUtilsMessengerCreateInfoEXT,
) -> (DebugUtils, vk::DebugUtilsMessengerEXT) {
    let debug_utils_loader = DebugUtils::new(entry, instance);
    let debug_call_back: vk::DebugUtilsMessengerEXT;

    unsafe {
        debug_call_back = debug_utils_loader
            .create_debug_utils_messenger(debug_info, None)
            .unwrap();
    }

    (debug_utils_loader, debug_call_back)
}

pub fn create_surface(
    entry: &Entry,
    instance: &Instance,
    window: &Window,
) -> (Surface, vk::SurfaceKHR) {
    let surface: vk::SurfaceKHR;
    let surface_loader = Surface::new(entry, instance);

    unsafe { surface = ash_window::create_surface(entry, instance, window, None).unwrap() }
    (surface_loader, surface)
}

pub fn pick_physical_device(instance: &Instance) -> PhysicalDevice {
    let all_physical_devices: Vec<PhysicalDevice>;

    unsafe {
        all_physical_devices = instance.enumerate_physical_devices().unwrap();
    }

    for device in all_physical_devices {
        if is_device_suitable(instance, device) {
            return device;
        }
    }

    panic!("Couldn't find a suitable device!");
}

pub fn pick_queue_family_index(instance: &Instance, device: &PhysicalDevice) -> Option<u32> {
    let all_queue_family_properties: Vec<vk::QueueFamilyProperties>;

    unsafe {
        all_queue_family_properties = instance.get_physical_device_queue_family_properties(*device);
    }

    for (i, queue_family_property) in all_queue_family_properties.iter().enumerate() {
        if is_queue_family_suitable(queue_family_property) {
            return Some(i as u32);
        }
    }

    None
}

fn is_device_suitable(instance: &Instance, device: PhysicalDevice) -> bool {
    let features: vk::PhysicalDeviceFeatures;
    let properties: vk::PhysicalDeviceProperties;
    unsafe {
        features = instance.get_physical_device_features(device);
        properties = instance.get_physical_device_properties(device);
    }

    if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        && features.geometry_shader == vk::TRUE
    {
        return true;
    }
    false
}

fn is_queue_family_suitable(queue_family_properties: &vk::QueueFamilyProperties) -> bool {
    if queue_family_properties
        .queue_flags
        .contains(vk::QueueFlags::GRAPHICS)
    {
        return true;
    }

    false
}

pub fn create_logical_device(queue_family_index: u32) {
    let queue_priorities = [1.0];
    let queue_info = [
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&queue_priorities)
            .build()
    ];
    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&[]);
    // .enabled_features();    
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})]:\n{}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}
