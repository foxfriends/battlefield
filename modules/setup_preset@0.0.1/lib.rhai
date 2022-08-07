fn init(context, state) {
    let archetypes = context.config["archetypes"];
    let entities = context.config["entities"];
    for definition in entities {
        let entity = state.spawn_entity();

        if definition.archetype != () {
            let archetype = archetypes[definition.archetype];
            for component in archetype.keys() {
                entity.set_component(component, archetype[component]);
            }
        }
        for component in definition.components.keys() {
            entity.set_component(component, definition.components[component]);
        }
    }
}

fn commands(context, state) { [] }

fn perform(context, state, command) {}
