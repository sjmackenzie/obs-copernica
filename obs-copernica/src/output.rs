use obs::{
    prelude::*,
    output::*,
    properties::*,
    obs_string,
};

pub struct CopernicaOutput;

pub struct OutputData {
    feed_name: u32,
    feed_owner: u32,
}

impl Outputable for CopernicaOutput{
    fn get_id() -> ObsString {
        obs_string!("copernica_output")
    }
}

impl GetNameOutput<OutputData> for CopernicaOutput{
    fn get_name() -> ObsString {
        obs_string!("Copernica Output")
    }
}

impl GetPropertiesOutput<OutputData> for CopernicaOutput {
    fn get_properties(_data: &mut Option<OutputData>, properties: &mut Properties) {
        properties.add_int(
            obs_string!("feed_name"),
            obs_string!("Name of the desired feed"),
            0,
            3840 * 3,
            1,
        );
        properties.add_int(
            obs_string!("feed_owner"),
            obs_string!("Feed owner's identity"),
            0,
            3840 * 3,
            1,
        );
    }
}

impl CreatableOutput<OutputData> for CopernicaOutput {
    fn create(settings: &mut SettingsContext, mut _source: OutputContext) -> OutputData {
        let feed_name  = settings.get_int(obs_string!("feed_name")).unwrap_or(0) as u32;
        let feed_owner = settings.get_int(obs_string!("feed_owner")).unwrap_or(0) as u32;
        return OutputData {
            feed_name,
            feed_owner,
        };
    }
}

impl UpdateOutput<OutputData> for CopernicaOutput {
    fn update(
        data: &mut Option<OutputData>,
        settings: &mut SettingsContext,
        _context: &mut ActiveContext,
    ) {
        if let Some(data) = data {
            if let Some(feed_name) = settings.get_int(obs_string!("feed_name")) {
                data.feed_name = feed_name as u32;
            }
            if let Some(feed_owner) = settings.get_int(obs_string!("feed_owner")) {
                data.feed_owner = feed_owner as u32;
            }
        }
    }
}
