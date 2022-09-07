fn init(context, state) {
    let teams = context.config["teams"];
    state["core.turn"] = teams[0];
}

fn commands(context, state) {
    if state["core.turn"] == context {
    }
}

fn perform(context, state, command) {}
