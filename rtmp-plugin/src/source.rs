use {
    obs::{
        prelude::*,
        source::*,
        properties::*,
        obs_string,
    },
    crate::{
        connection::{Connection, ConnectionError, ReadResult},
        server::{Server, ServerResult},
    },
};

pub struct RTMPSource;

pub struct SourceData {
    source: SourceContext,
    feed_name: u32,
    feed_owner: u32,
}

impl Sourceable for RTMPSource {
    fn get_id() -> ObsString {
        obs_string!("rtmp_source")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }
}

impl GetNameSource<SourceData> for RTMPSource {
    fn get_name() -> ObsString {
        obs_string!("Copernica RTMP Source")
    }
}

impl GetPropertiesSource<SourceData> for RTMPSource {
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
        properties.add_text(
            obs_string!("Stringer"),
            obs_string!("Stringer"),
            0,
        );
    }
}

impl CreatableSource<SourceData> for RTMPSource {
    fn create(settings: &mut SettingsContext, mut source: SourceContext) -> SourceData {
        let feed_name  = settings.get_int(obs_string!("feed_name")).unwrap_or(0) as u32;
        let feed_owner = settings.get_int(obs_string!("feed_owner")).unwrap_or(0) as u32;
        source.update_source_settings(settings);
        return SourceData {
            feed_name,
            feed_owner,
            source,
        };
    }
}

impl UpdateSource<SourceData> for RTMPSource {
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
