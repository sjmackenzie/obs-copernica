use super::traits::*;
use super::context::{ActiveContext};
use super::{OutputContext};
use crate::properties::*;
use std::ffi::c_void;
use std::os::raw::c_char;

use obs_sys::{
    obs_properties, obs_properties_create, obs_data_t, obs_output_t,
};

struct DataWrapper<D> {
    data: Option<D>,
    properties: Vec<Property>,
}

impl<D> Default for DataWrapper<D> {
    fn default() -> Self {
        Self {
            data: None,
            properties: vec![],
        }
    }
}

impl<D> From<D> for DataWrapper<D> {
    fn from(data: D) -> Self {
        Self {
            data: Some(data),
            properties: vec![],
        }
    }
}

pub unsafe extern "C" fn get_name<D, F: GetNameOutput<D>>(
    _type_data: *mut c_void,
) -> *const c_char {
    F::get_name().as_ptr()
}

pub unsafe extern "C" fn get_properties<D, F: GetPropertiesOutput<D>>(
    data: *mut ::std::os::raw::c_void,
) -> *mut obs_properties {
    let wrapper: &mut DataWrapper<D> = &mut *(data as *mut DataWrapper<D>);

    let mut properties = Properties::from_raw(obs_properties_create(), &mut wrapper.properties);

    F::get_properties(&mut wrapper.data, &mut properties);

    properties.into_raw()
}

pub unsafe extern "C" fn create<D, F: CreatableOutput<D>>(
    settings: *mut obs_data_t,
    output: *mut obs_output_t,
) -> *mut c_void {
    let mut wrapper = DataWrapper::default();
    let mut settings = SettingsContext::from_raw(settings, &wrapper.properties);

    let output = OutputContext { output };

    let data = F::create(&mut settings, output);

    wrapper.data = Some(data);

    Box::into_raw(Box::new(wrapper)) as *mut c_void
}

pub unsafe extern "C" fn create_default_data<D>(
    _settings: *mut obs_data_t,
    _output: *mut obs_output_t,
) -> *mut c_void {
    let data = Box::new(DataWrapper::<D>::default());
    Box::into_raw(data) as *mut c_void
}

pub unsafe extern "C" fn destroy<D>(data: *mut c_void) {
    let wrapper: Box<DataWrapper<D>> = Box::from_raw(data as *mut DataWrapper<D>);
    drop(wrapper);
}

pub unsafe extern "C" fn update<D, F: UpdateOutput<D>>(
    data: *mut c_void,
    settings: *mut obs_data_t,
) {
    let mut active = ActiveContext::default();
    let data: &mut DataWrapper<D> = &mut *(data as *mut DataWrapper<D>);
    let mut settings = SettingsContext::from_raw(settings, &data.properties);
    F::update(&mut data.data, &mut settings, &mut active);
}
