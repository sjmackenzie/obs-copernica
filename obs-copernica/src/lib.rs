mod source;
mod output;
mod service;
use {
    crate::{
        source::{CopernicaSource, SourceData},
        output::{CopernicaOutput, OutputData},
        service::{CopernicaService, ServiceData},
    },
    obs::{
        prelude::*,
        obs_register_module,
        obs_string,
    },
};

pub struct CopernicaModule {
    context: ModuleContext
}

impl Module for CopernicaModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        let source = load_context
            .create_source_builder::<CopernicaSource, SourceData>()
            .enable_get_name()
            .enable_create()
            .enable_get_properties()
            .enable_update()
            .build();
        let output = load_context
            .create_output_builder::<CopernicaOutput, OutputData>()
            .enable_get_name()
            .enable_get_properties()
            .build();
        let service = load_context
            .create_service_builder::<CopernicaService, ServiceData>()
            .enable_get_name()
            .enable_get_properties()
            .build();
        load_context.register_source(source);
        load_context.register_output(output);
        load_context.register_service(service);
        true
    }

    fn description() -> ObsString {
        obs_string!("An OBS plugin that allows users to stream from OBS to directly to other OBS instances in a many-to-many fashion skipping a middle men, like twitch, facebook and youtube.")
    }

    fn name() -> ObsString {
        obs_string!("OBS Copernica")
    }

    fn author() -> ObsString {
        obs_string!("Stewart Mackenzie")
    }
}

obs_register_module!(CopernicaModule);
