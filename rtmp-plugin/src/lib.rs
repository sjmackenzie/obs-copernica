mod server;
mod connection;
mod source;
use {
    crate::{
        source::{RTMPSource, SourceData},
    },
    obs::{
        prelude::*,
        obs_register_module,
        obs_string,
    },
};

pub struct RTMPModule {
    context: ModuleContext
}

impl Module for RTMPModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        let source = load_context
            .create_source_builder::<RTMPSource, SourceData>()
            .enable_get_name()
            .enable_create()
            .enable_get_properties()
            .enable_update()
            .build();
        load_context.register_source(source);
        true
    }

    fn description() -> ObsString {
        obs_string!("An OBS plugin that allows users to stream from OBS directly to other OBS instances skipping twitch, facebook and youtube middle men.")
    }

    fn name() -> ObsString {
        obs_string!("OBS RTMP Copernica Plugin")
    }

    fn author() -> ObsString {
        obs_string!("Stewart Mackenzie")
    }
}

obs_register_module!(RTMPModule);
