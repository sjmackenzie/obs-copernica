use super::context::{ActiveContext};
use {
    super::{
        ServiceContext,
    },
    crate::{
        string::ObsString,
        properties::{Properties, SettingsContext},
    },
};

pub trait Serviceable {
    fn get_id() -> ObsString;
}

pub trait GetNameService<D> {
    fn get_name() -> ObsString;
}

pub trait GetPropertiesService<D> {
    fn get_properties(data: &mut Option<D>, properties: &mut Properties);
}

pub trait CreatableService<D> {
    fn create(settings: &mut SettingsContext, service: ServiceContext) -> D;
}

pub trait UpdateService<D> {
    fn update(data: &mut Option<D>, settings: &mut SettingsContext, context: &mut ActiveContext);
}
