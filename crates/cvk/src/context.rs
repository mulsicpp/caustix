use std::sync::{Mutex, MutexGuard, OnceLock, RwLockReadGuard, RwLockWriteGuard};
use std::{ffi::CStr, sync::RwLock};

use ash::{Device, Entry, Instance, vk};

use std::ffi::CString;

type ContextReadGuard = RwLockReadGuard<'static, Context>;
type ContextWriteGuard = RwLockWriteGuard<'static, Context>;

pub struct Context {
    entry: Entry,
    instance: Instance,
}

pub struct ContextInfo {
    app_name: CString,
    engine_name: CString,
    version: u32,
    debugging: bool,
}

impl Default for ContextInfo {
    fn default() -> Self {
        Self {
            app_name: CString::from(c"Vulkan App"),
            engine_name: CString::from(c"Engine"),
            version: vk::API_VERSION_1_0,
            debugging: false,
        }
    }
}

static CONTEXT: OnceLock<RwLock<Context>> = OnceLock::new();

impl Context {
    pub fn init(info: ContextInfo) {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan entry") };

        let app_info = vk::ApplicationInfo::default()
            .application_name(&info.app_name)
            .application_version(0)
            .engine_name(&info.engine_name)
            .engine_version(0)
            .api_version(info.version);

        let instance_info = vk::InstanceCreateInfo::default().application_info(&app_info);

        let instance = unsafe {
            entry
                .create_instance(&instance_info, None)
                .expect("Failed to create VkInstance")
        };

        if let Err(_) = CONTEXT.set(RwLock::new(Context { entry, instance })) {
            panic!("Failed to initialize Vulkan context");
        }
    }

    pub fn get() -> ContextReadGuard {
        CONTEXT.get().expect("Vulkan context is not initialized").read().unwrap()
    }

    pub fn try_get() -> Option<ContextReadGuard> {
        CONTEXT.get()?.read().ok()
    }
    
    pub fn get_mut() -> ContextWriteGuard {
        CONTEXT.get().expect("Vulkan context is not initialized").write().unwrap()
    }

    pub fn try_get_mut() -> Option<ContextWriteGuard> {
        CONTEXT.get()?.write().ok()
    }
}
