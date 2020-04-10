#![allow(non_upper_case_globals)]

use paste::item;

mod ffi;

pub mod context;
pub mod properties;
pub mod traits;

pub use context::*;
pub use properties::*;
pub use traits::*;

use {
    super::string::ObsString,
    obs_sys::{
        obs_service_info, obs_service_t,
    },
    std::marker::PhantomData,
};

pub struct ServiceContext {
    service: *mut obs_service_t,
}

impl ServiceContext {
    /// Run a function on the next service in the filter chain.
    ///
    /// Note: only works with services that are filters.
    pub fn do_with_target<F: FnOnce(&mut ServiceContext)>(&mut self, func: F) {
        unsafe {
        }
    }

    /// Return a unique id for the filter
    pub fn id(&self) -> usize {
        self.service as usize
    }
}

pub struct ServiceInfo {
    info: Box<obs_service_info>,
}

impl ServiceInfo {
    /// # Safety
    /// Creates a raw pointer from a box and could cause UB is misused.
    pub unsafe fn into_raw(self) -> *mut obs_service_info {
        Box::into_raw(self.info)
    }
}

pub struct ServiceInfoBuilder<T: Serviceable, D> {
    __service: PhantomData<T>,
    __data: PhantomData<D>,
    info: obs_service_info,
}

impl<T: Serviceable, D> ServiceInfoBuilder<T, D> {
    pub(crate) fn new() -> Self {
        Self {
            __service: PhantomData,
            __data: PhantomData,
            info: obs_service_info {
                id: T::get_id().as_ptr(),
                get_name: None,
                create: None,
                destroy: None,
                activate: None,
                deactivate: None,
                update: None,
                get_defaults: None,
                get_properties: None,
                initialize: None,
                get_url: None,
                get_key: None,
                get_username: None,
                get_password: None,
                supports_multitrack: None,
                apply_encoder_settings: None,
                type_data: std::ptr::null_mut(),
                free_type_data: None,
                get_output_type: None,
            },
        }
    }

    pub fn build(mut self) -> ServiceInfo {
        ServiceInfo {
            info: Box::new(self.info),
        }
    }
}

macro_rules! impl_service_builder {
    ($($f:ident => $t:ident)*) => ($(
        item! {
            impl<D, T: Serviceable + [<$t>]<D>> ServiceInfoBuilder<T, D> {
                pub fn [<enable_$f>](mut self) -> Self {
                    self.info.[<$f>] = Some(ffi::[<$f>]::<D, T>);
                    self
                }
            }
        }
    )*)
}

impl_service_builder! {
    get_name => GetNameService
    get_properties => GetPropertiesService
}
