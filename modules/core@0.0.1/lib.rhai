fn init(context, state) {
    state["core.turn"] = 0;
}

fn commands(context, state) {
    if state["core.turn"] == context.current_player?.id {
        return ["my turn"];
    }
    []
}

fn perform(context, state, command) {}
