use crate::api::*;
use crate::components::http_client_provider::use_http_client;
use crate::routes::Route;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub scenario: Scenario,
}

#[function_component(ScenarioSummary)]
pub fn scenario_summary(props: &Props) -> Html {
    let show_raw = use_state(|| false);
    let history = use_history().unwrap();
    let client = use_http_client();

    let new_game = Callback::from({
        let scenario = props.scenario.name.to_owned();
        move |_| {
            let operation = NewGameMutation::build(NewGameArguments {
                new_game: NewGame {
                    scenario: scenario.clone(),
                    players: vec![],
                },
            });
            let history = history.clone();
            let client = client.clone();
            spawn_local(async move {
                let result = client
                    .post("graphql")
                    .run_graphql(operation)
                    .await
                    .map_err(ApiError::RequestError)
                    .and_then(|response| match response.errors {
                        Some(errors) => Err(ApiError::GraphQlErrors(errors)),
                        None => Ok(response.data.unwrap()),
                    })
                    .and_then(|response| {
                        response
                            .game
                            .id
                            .parse::<uuid::Uuid>()
                            .map_err(ApiError::other)
                    });

                match result {
                    Ok(id) => {
                        history.push(Route::Game { id });
                    }
                    Err(error) => {
                        gloo::console::error!(format!("{:?}", error));
                    }
                }
            });
        }
    });

    let view_raw = Callback::from({
        let show_raw = show_raw.clone();
        move |_| {
            show_raw.set(!*show_raw);
        }
    });

    html! {
        <div class="text-start flex flex-col gap-2 border border-black/20 p-4">
            <div class="flex justify-between">
                <div class="flex flex-col gap-2">
                    <div class="font-semibold">
                        {&props.scenario.name}
                    </div>
                    if let Some(description) = &props.scenario.description {
                        <div class="text-sm text-black/80">
                            {description}
                        </div>
                    }
                </div>

                <div class="flex flex-col items-end justify-end">
                    if props.scenario.errors.is_empty() {
                        <div class="text-green-600 text-xs uppercase font-medium">
                            {"Valid"}
                        </div>
                    } else {
                        <div class="text-red-600 text-xs uppercase font-medium">
                            {format!("{} Errors", props.scenario.errors.len())}
                        </div>
                    }
                    <div class="font-mono text-xs text-black/60">
                        {&props.scenario.path}
                    </div>
                </div>
            </div>

            <div class="flex gap-4 justify-end">
                <button class="uppercase text-sm border border-black p-1 bg-neutral-100" onclick={view_raw}>
                    {"View Raw"}
                </button>

                <button class="uppercase text-sm border border-black p-1 bg-blue-400 text-white" onclick={new_game} disabled={!props.scenario.errors.is_empty()}>
                    {"Start"}
                </button>
            </div>

            if *show_raw {
                <pre class="bg-neutral-100 p-4"><code>{serde_json::to_string_pretty(&props.scenario).unwrap()}</code></pre>
            }
        </div>
    }
}
