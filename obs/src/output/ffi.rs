use super::traits::*;
use crate::properties::*;
use std::ffi::c_void;
use std::os::raw::c_char;

use obs_sys::{
    obs_properties, obs_properties_create,
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
