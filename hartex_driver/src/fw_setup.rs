use hartex_core::logging::tracing;

use hartex_cmdsys::framework::CommandFramework;

use hartex_eventsys::{
    emitter::EventEmitter,
    events::Events
};

pub fn framework_setup() -> (EventEmitter, Events) {
    tracing::trace!("setting up command framework");

    let framework = CommandFramework::default();

    let listeners = framework.clone().listeners();

    (EventEmitter::new(listeners), framework.events())
}