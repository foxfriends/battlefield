import "extras" as extras;

fn commands(scenario, state, commands) {
    commands.push(extras::test());
    commands
}

fn perform(scenario, state, command) {
    extras::perform(scenario, state, command)
}
