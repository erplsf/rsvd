extern crate sdl2;
extern crate ash;

use std::borrow::Cow;
use std::ffi::CStr;
use ash::version::{InstanceV1_0, EntryV1_0};
use sdl2::video::{Window};
use ash::{vk, Entry, Instance};
use std::ffi::{CString};
use std::os::raw::c_char;
use ash::extensions::{ext::DebugUtils};

struct VulkanGod {
    window: Window,
    entry: Entry,
    instance: Instance,
    debug_info: vk::DebugUtilsMessengerCreateInfoEXT,
    debug_utils_loader: DebugUtils,
    debug_call_back: vk::DebugUtilsMessengerEXT
}

impl VulkanGod {
    fn new(width: u32, height: u32) -> Self {
        let entry: Entry = Entry::new().unwrap();
        let window = Self::create_window(width, height);
        let debug_info = Self::create_debug_info();
        let instance = Self::create_instance(&window, &entry, debug_info);
        let (debug_utils_loader, debug_call_back) = Self::create_debug_utils_loader(&entry, &instance, &debug_info);

        VulkanGod{
            entry,
            window,
            instance,
            debug_info,
            debug_utils_loader,
            debug_call_back
        }
    }

    fn create_window(width: u32, height: u32) -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        video_subsystem.window("rsvd", width, height)
            .vulkan()
            .position_centered()
            .build()
            .unwrap()
    }

    fn create_instance(window: &Window, entry: &Entry, mut debug_info: vk::DebugUtilsMessengerCreateInfoEXT) -> Instance {
        let name = CString::new("rsvd").unwrap();
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(&name)
            .application_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(1, 0, 0));

        // get required extensions from sdl2
        let required_extensions = window.vulkan_instance_extensions().unwrap();

        // dbg!(&required_extensions);
        // dbg!(entry.enumerate_instance_extension_properties().unwrap());
        // dbg!(entry.enumerate_instance_layer_properties().unwrap());
        
        // convert them to raw pointers
        let required_extensions_raw: Vec<_> = required_extensions.iter()
            .map(|&ext| CString::new(ext).unwrap().into_raw())
            .collect();

        // cast them for the next call
        let mut required_extensions_raw_const: Vec<_> = required_extensions_raw.iter()
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
            .push_next(&mut debug_info);
        
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
                       
        return instance
    }

    fn create_debug_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
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

    fn create_debug_utils_loader(entry: &Entry, instance: &Instance, debug_info: &vk::DebugUtilsMessengerCreateInfoEXT) -> (DebugUtils, ash::vk::DebugUtilsMessengerEXT) {
        let debug_utils_loader = DebugUtils::new(entry, instance);

        let debug_call_back: vk::DebugUtilsMessengerEXT;
        unsafe {
            debug_call_back = debug_utils_loader
                .create_debug_utils_messenger(debug_info, None)
                .unwrap();
        }

        (debug_utils_loader, debug_call_back)
    }
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
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}

impl Drop for VulkanGod {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_call_back, None);
            self.instance.destroy_instance(None);
        }
    }
}

pub fn main() {    
    let _vg = VulkanGod::new(800, 600);
}
