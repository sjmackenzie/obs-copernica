use obs::{
    prelude::*,
    source::*,
    properties::*,
    obs_string,
};

pub struct CopernicaSource;

pub struct SourceData {
    feed_name: u32,
    feed_owner: u32,
}

impl Sourceable for CopernicaSource {
    fn get_id() -> ObsString {
        obs_string!("copernica_source")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }
}

impl GetNameSource<SourceData> for CopernicaSource {
    fn get_name() -> ObsString {
        obs_string!("Copernica Source")
    }
}

impl GetPropertiesSource<SourceData> for CopernicaSource {
    fn get_properties(_data: &mut Option<SourceData>, properties: &mut Properties) {
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

impl CreatableSource<SourceData> for CopernicaSource {
    fn create(settings: &mut SettingsContext, mut _source: SourceContext) -> SourceData {
        let feed_name  = settings.get_int(obs_string!("feed_name")).unwrap_or(0) as u32;
        let feed_owner = settings.get_int(obs_string!("feed_owner")).unwrap_or(0) as u32;
        return SourceData {
            feed_name,
            feed_owner,
        };
    }
}

impl UpdateSource<SourceData> for CopernicaSource {
    fn update(
        data: &mut Option<SourceData>,
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
