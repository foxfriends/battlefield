import "extras" as extras;

fn commands(scenario, state, commands) {
    commands.push(extras::test());
    commands
}
