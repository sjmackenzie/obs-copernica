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
    super::{
        properties::*,
        string::ObsString,
    },
    obs_sys::{
        obs_output_info, obs_output_t, obs_output_update
    },
    std::marker::PhantomData,
};

pub struct OutputContext {
    output: *mut obs_output_t,
}

impl OutputContext {
    pub fn id(&self) -> usize {
        self.output as usize
    }



    pub fn update_output_settings(&mut self, settings: &SettingsContext) {
        unsafe {
            obs_output_update(self.output, settings.as_raw());
        }
    }
}

pub struct OutputInfo {
    info: Box<obs_output_info>,
}

impl OutputInfo {
    /// # Safety
    /// Creates a raw pointer from a box and could cause UB is misused.
    pub unsafe fn into_raw(self) -> *mut obs_output_info {
        Box::into_raw(self.info)
    }
}

pub struct OutputInfoBuilder<T: Outputable, D> {
    __output: PhantomData<T>,
    __data: PhantomData<D>,
    info: obs_output_info,
}

impl<T: Outputable, D> OutputInfoBuilder<T, D> {
    pub(crate) fn new() -> Self {
        Self {
            __output: PhantomData,
            __data: PhantomData,
            info: obs_output_info {
                id: T::get_id().as_ptr(),
                get_name: None,
                flags: 0,
                create: Some(ffi::create_default_data::<D>),
                destroy: Some(ffi::destroy::<D>),
                start: None,
                stop: None,
                raw_video: None,
                raw_audio: None,
                encoded_packet: None,
                update: None,
                get_defaults: None,
                get_properties: None,
                unused1: None,
                get_total_bytes: None,
                get_dropped_frames: None,
                type_data: std::ptr::null_mut(),
                free_type_data: None,
                get_congestion: None,
                get_connect_time_ms: None,
                encoded_video_codecs: std::ptr::null_mut(),
                encoded_audio_codecs: std::ptr::null_mut(),
                raw_audio2: None,
            },
        }
    }

    pub fn build(mut self) -> OutputInfo {
        OutputInfo {
            info: Box::new(self.info),
        }
    }
}

macro_rules! impl_output_builder {
    ($($f:ident => $t:ident)*) => ($(
        item! {
            impl<D, T: Outputable + [<$t>]<D>> OutputInfoBuilder<T, D> {
                pub fn [<enable_$f>](mut self) -> Self {
                    self.info.[<$f>] = Some(ffi::[<$f>]::<D, T>);
                    self
                }
            }
        }
    )*)
}

impl_output_builder! {
    get_name => GetNameOutput
    create => CreatableOutput
    get_properties => GetPropertiesOutput
    update => UpdateOutput
}
