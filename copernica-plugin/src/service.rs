use obs::{
    prelude::*,
    service::*,
    properties::*,
    obs_string,
};

pub struct CopernicaService;

pub struct ServiceData {
    feed_name: u32,
    feed_owner: u32,
}

impl Serviceable for CopernicaService{
    fn get_id() -> ObsString {
        obs_string!("copernica_service")
    }
}

impl GetNameService<ServiceData> for CopernicaService{
    fn get_name() -> ObsString {
        obs_string!("Copernica Service")
    }
}

impl GetPropertiesService<ServiceData> for CopernicaService {
    fn get_properties(_data: &mut Option<ServiceData>, properties: &mut Properties) {
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

impl CreatableService<ServiceData> for CopernicaService {
    fn create(settings: &mut SettingsContext, mut _source: ServiceContext) -> ServiceData {
        let feed_name  = settings.get_int(obs_string!("feed_name")).unwrap_or(0) as u32;
        let feed_owner = settings.get_int(obs_string!("feed_owner")).unwrap_or(0) as u32;
        return ServiceData {
            feed_name,
            feed_owner,
        };
    }
}

impl UpdateService<ServiceData> for CopernicaService {
    fn update(
        data: &mut Option<ServiceData>,
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
