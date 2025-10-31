use std::sync::RwLock;
use std::sync::{OnceLock, RwLockReadGuard, RwLockWriteGuard};

use ash::{Entry, Instance, vk};

use std::ffi::{CStr, CString, c_void};

type ContextReadGuard = RwLockReadGuard<'static, Context>;
type ContextWriteGuard = RwLockWriteGuard<'static, Context>;

struct DebugObjects {
    debug_utils: ash::ext::debug_utils::Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

pub struct Context {
    entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    debug_objs: Option<DebugObjects>,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ApiVersion {
    V1_0 = vk::API_VERSION_1_0,
    V1_1 = vk::API_VERSION_1_1,
    V1_2 = vk::API_VERSION_1_2,
    V1_3 = vk::API_VERSION_1_3,
}

#[derive(utils::Paramters)]
pub struct ContextInfo {
    app_name: CString,
    engine_name: CString,
    version: ApiVersion,
    debugging: bool,
}

impl Default for ContextInfo {
    fn default() -> Self {
        Self {
            app_name: CString::from(c"Vulkan App"),
            engine_name: CString::from(c"Engine"),
            version: ApiVersion::V1_0,
            debugging: false,
        }
    }
}

static CONTEXT: OnceLock<RwLock<Context>> = OnceLock::new();

impl Context {
    const VALIDATION_LAYER: &'static CStr = &c"VK_LAYER_KHRONOS_validation";

    unsafe extern "system" fn debug_callback(
        _severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        _type_flags: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
        _user_data: *mut c_void,
    ) -> u32 {
        if let Some(msg) = unsafe { (*callback_data).message_as_c_str() } {
            println!("Validation Layer:\n {}", msg.to_string_lossy());
        }

        vk::FALSE
    }

    fn create_instance(entry: &Entry, info: &ContextInfo) -> Instance {
        let mut required_instance_extensions: Vec<_> = vec![];

        if info.debugging {
            let layers = unsafe { entry.enumerate_instance_layer_properties().unwrap() }
                .into_iter()
                .filter_map(|layer_prop| {
                    Some(CString::from(layer_prop.layer_name_as_c_str().ok()?))
                })
                .collect::<Vec<_>>();

            dbg!(layers.contains(&Self::VALIDATION_LAYER.into()));

            required_instance_extensions.push(ash::ext::debug_utils::NAME);
        }

        let instance_extensions =
            unsafe { entry.enumerate_instance_extension_properties(None).unwrap() }
                .into_iter()
                .filter_map(|ext_prop| {
                    Some(CString::from(ext_prop.extension_name_as_c_str().ok()?))
                })
                .collect::<Vec<_>>();

        for ext in required_instance_extensions.iter() {
            if !instance_extensions.contains(&CString::from(*ext)) {
                panic!(
                    "The required extension '{}' is not supported",
                    ext.to_string_lossy()
                );
            }
        }

        let app_info = vk::ApplicationInfo::default()
            .application_name(&info.app_name)
            .application_version(0)
            .engine_name(&info.engine_name)
            .engine_version(0)
            .api_version(info.version as u32);

        let enabled_layers = [Self::VALIDATION_LAYER.as_ptr()];
        let enabled_extensions: Vec<_> = required_instance_extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect();

        let mut instance_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(enabled_layers.as_slice())
            .enabled_extension_names(enabled_extensions.as_slice());

        let mut debug_messenger_info;

        if info.debugging {
            use vk::DebugUtilsMessageSeverityFlagsEXT as Severity;
            use vk::DebugUtilsMessageTypeFlagsEXT as Type;

            debug_messenger_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(Severity::VERBOSE | Severity::WARNING | Severity::ERROR)
                .message_type(Type::GENERAL | Type::PERFORMANCE | Type::VALIDATION)
                .pfn_user_callback(Some(Self::debug_callback));

            instance_info = instance_info.push_next(&mut debug_messenger_info);
        }

        unsafe {
            entry
                .create_instance(&instance_info, None)
                .expect("Failed to create VkInstance")
        }
    }

    fn create_debug_utils(entry: &Entry, instance: &Instance) -> DebugObjects {
        use vk::DebugUtilsMessageSeverityFlagsEXT as Severity;
        use vk::DebugUtilsMessageTypeFlagsEXT as Type;

        let debug_utils = ash::ext::debug_utils::Instance::new(entry, &instance);

        let debug_messenger_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(Severity::VERBOSE | Severity::WARNING | Severity::ERROR)
            .message_type(Type::GENERAL | Type::PERFORMANCE | Type::VALIDATION)
            .pfn_user_callback(Some(Self::debug_callback));

        let debug_messenger =
            unsafe { debug_utils.create_debug_utils_messenger(&debug_messenger_info, None) }
                .expect("Failed to create debug messenger");

        DebugObjects {
            debug_utils,
            debug_messenger,
        }
    }

    pub fn init(info: &ContextInfo) {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan entry") };

        let instance = Self::create_instance(&entry, info);

        let debug_objs = if info.debugging {
            Some(Self::create_debug_utils(&entry, &instance))
        } else {
            None
        };

        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };

        for physical_device in physical_devices {
            let mut props = vk::PhysicalDeviceProperties2::default();
            let mut features = vk::PhysicalDeviceFeatures2::default();

            unsafe {
                instance.get_physical_device_properties2(physical_device, &mut props);
                instance.get_physical_device_features2(physical_device, &mut features);
            }

            println!(
                "device: {}",
                props
                    .properties
                    .device_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
        }

        if let Err(_) = CONTEXT.set(RwLock::new(Context {
            entry,
            instance,
            surface: vk::SurfaceKHR::null(),
            debug_objs,
        })) {
            panic!("Failed to initialize Vulkan context");
        }
    }

    pub fn get() -> ContextReadGuard {
        CONTEXT
            .get()
            .expect("Vulkan context is not initialized")
            .read()
            .unwrap()
    }

    pub fn try_get() -> Option<ContextReadGuard> {
        CONTEXT.get()?.read().ok()
    }

    pub fn get_mut() -> ContextWriteGuard {
        CONTEXT
            .get()
            .expect("Vulkan context is not initialized")
            .write()
            .unwrap()
    }

    pub fn try_get_mut() -> Option<ContextWriteGuard> {
        CONTEXT.get()?.write().ok()
    }

    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    pub fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("dropping the context");
        unsafe {
            if let Some(DebugObjects {
                ref debug_utils,
                debug_messenger,
            }) = self.debug_objs
            {
                debug_utils.destroy_debug_utils_messenger(debug_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
