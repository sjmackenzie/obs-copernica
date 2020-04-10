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
        obs_output_info, obs_output_t,
        /*
        obs_get_output_flags, obs_get_output_properties, obs_output_active, obs_output_addref,
        obs_output_audio, obs_output_begin_data_capture, obs_output_can_begin_data_capture,
        obs_output_can_pause, obs_output_create, obs_output_defaults, obs_output_end_data_capture,
        obs_output_force_stop, obs_output_get_active_delay, obs_output_get_audio_encoder,
        obs_output_get_congestion, obs_output_get_connect_time_ms, obs_output_get_delay,
        obs_output_get_display_name, obs_output_get_flags, obs_output_get_frames_dropped,
        obs_output_get_height, obs_output_get_id, obs_output_get_last_error, obs_output_get_mixer,
        obs_output_get_mixers, obs_output_get_name, obs_output_get_pause_offset,
        obs_output_get_proc_handler, obs_output_get_ref, obs_output_get_service,
        obs_output_get_settings, obs_output_get_signal_handler,
        obs_output_get_supported_audio_codecs, obs_output_get_supported_video_codecs,
        obs_output_get_total_bytes, obs_output_get_total_frames, obs_output_get_type_data,
        obs_output_get_video_encoder, obs_output_get_weak_output, obs_output_get_width,
        obs_output_initialize_encoders, obs_output_pause, obs_output_paused, obs_output_properties,
        obs_output_reconnecting, obs_output_release, obs_output_set_audio_conversion,
        obs_output_set_audio_encoder, obs_output_set_delay, obs_output_set_last_error,
        obs_output_set_media, obs_output_set_mixer, obs_output_set_mixers,
        obs_output_set_preferred_size, obs_output_set_reconnect_settings, obs_output_set_service,
        obs_output_set_video_conversion, obs_output_set_video_encoder, obs_output_signal_stop,
        obs_output_start, obs_output_stop, obs_output_update, obs_output_video,
        obs_weak_output_addref, obs_weak_output_get_output, obs_weak_output_references_output,
        obs_weak_output_release,
        */
    },
    std::marker::PhantomData,
};

pub struct OutputContext {
    output: *mut obs_output_t,
}

impl OutputContext {
    /// Run a function on the next output in the filter chain.
    ///
    /// Note: only works with outputs that are filters.
    pub fn do_with_target<F: FnOnce(&mut OutputContext)>(&mut self, func: F) {
        unsafe {
        }
    }

    /// Return a unique id for the filter
    pub fn id(&self) -> usize {
        self.output as usize
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
                create: None,
                destroy: None,
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
    get_properties => GetPropertiesOutput
}
