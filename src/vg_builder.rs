use std::borrow::Cow;
use ash::version::{InstanceV1_0, EntryV1_0};
use sdl2::video::{Window};
use ash::{vk, Entry, Instance};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use ash::extensions::{ext::DebugUtils};

use crate::vg;

const APP_NAME: &str = "rsvd";

pub struct VgBuilder {
    width: u32,
    height: u32,
    window: Option<Window>,
    entry: Option<Entry>,
    instance: Option<Instance>,
    // physical_device: Option<PhysicalDevice>,
    debug_info: Option<vk::DebugUtilsMessengerCreateInfoEXT>,
    debug_utils_loader: Option<DebugUtils>,
    debug_call_back: Option<vk::DebugUtilsMessengerEXT>
}

impl VgBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        Self{
            width,
            height,
            window: None,
            entry: None,
            instance: None,
            // physical_device: None,
            debug_info: None,
            debug_utils_loader: None,
            debug_call_back: None
        }
    }
    
    pub fn build(mut self) -> vg::Vg {
        self.init_vulkan();
        
        vg::Vg::new(
            self.entry.unwrap(),
            self.window.unwrap(),
            self.instance.unwrap(),
            // physical_device,
            self.debug_utils_loader.unwrap(),
            self.debug_call_back.unwrap()
        )
    }

    fn init_vulkan(&mut self) -> &mut Self {
        self.create_instance();
        self.setup_debug_messenger();
        self
    }

    fn create_instance(&mut self) -> &mut Self {
        self.entry = Some(Entry::new().unwrap());

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        self.window = Some(video_subsystem.window(APP_NAME, self.width, self.height)
                           .vulkan()
                           .position_centered()
                           .build()
                           .unwrap());

        self.debug_info = Some(vk::DebugUtilsMessengerCreateInfoEXT::builder()
                               .message_severity(
                                   vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                                       | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                                       | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                               )
                               .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                               .pfn_user_callback(Some(vulkan_debug_callback))
                               .build());

        let name = CString::new(APP_NAME).unwrap();
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(&name)
            .application_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(1, 2, 0));

        // get required extensions from sdl2
        let required_extensions = self.window.as_ref().unwrap().vulkan_instance_extensions().unwrap();

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

        let debug_info_ref = &mut self.debug_info.unwrap();
        
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&required_extensions_raw_const)
            .push_next(debug_info_ref);
        
        unsafe {
            self.instance = Some(self.entry.as_ref().unwrap()
                                 .create_instance(&create_info, None)
                                 .expect("Instance creation error"));

            // recreate them from raw pointers so rust can deallocate them
            for raw in required_extensions_raw {
                CString::from_raw(raw);
            }
        }

        self
    }
    
    fn setup_debug_messenger(&mut self) -> &mut Self {        
        self.debug_utils_loader = Some(
            DebugUtils::new(self.entry.as_ref().unwrap(),
                            self.instance.as_ref().unwrap())
        );

        unsafe {
            self.debug_call_back = Some(self.debug_utils_loader.as_ref().unwrap()
                                        .create_debug_utils_messenger(&self.debug_info.unwrap(), None)
                                        .unwrap());
        }

        self
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
        "{:?}:\n{:?} [{} ({})]:\n{}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}
