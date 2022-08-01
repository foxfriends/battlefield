fn init(context, state) {
    let entities = context.config["entities"];
    for entity_definition in entities {
        let entity = state.spawn_entity();
        for component in entity_definition.keys() {
            entity.set_component(component, entity_definition[component]);
        }
    }
}

fn commands(context, state) { [] }

fn perform(context, state, command) {}
