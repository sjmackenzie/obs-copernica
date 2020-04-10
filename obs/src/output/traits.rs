use super::context::{ActiveContext};
use {
    super::{
        OutputContext,
    },
    crate::{
        string::ObsString,
        properties::{Properties, SettingsContext},
    },
};

pub trait Outputable {
    fn get_id() -> ObsString;
}

pub trait GetNameOutput<D> {
    fn get_name() -> ObsString;
}

pub trait GetPropertiesOutput<D> {
    fn get_properties(data: &mut Option<D>, properties: &mut Properties);
}

pub trait CreatableOutput<D> {
    fn create(settings: &mut SettingsContext, output: OutputContext) -> D;
}

pub trait UpdateOutput<D> {
    fn update(data: &mut Option<D>, settings: &mut SettingsContext, context: &mut ActiveContext);
}
