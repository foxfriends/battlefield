// Extras does not provide any commands, only helper functions
fn commands(scenario, state, commands) { commands }

fn test() { "test()"; }

fn perform(scenario, state, command) {
    switch command {
        "test()" => {
            state.set_data("tested", true);
            state
        }
    }
}
