import "commands" as cmds;

fn commands(scenario, state, commands) {
    cmds::commands(scenario, state, commands);
}

fn perform(scenario, state, command) {
    switch command {
        "win()" => {
            state.set_data("won", true);
            state
        }
        "lose()" => {
            state.set_data("won", false);
            state
        }
    }
}
