use crate::api::*;
use crate::components::http_client_provider::use_http_client;
use crate::hooks::use_query::use_query;
use yew::prelude::*;

mod scenario_summary;

use scenario_summary::ScenarioSummary;

#[function_component(MenuPage)]
pub fn menu_page() -> Html {
    let client = use_http_client();
    let scenarios = use_query(
        move |_| {
            let client = client.clone();
            async move {
                let operation = ListScenariosQuery::build(ListScenariosArguments::default());
                client
                    .post("graphql")
                    .run_graphql(operation)
                    .await
                    .map_err(ApiError::RequestError)
                    .and_then(|response| match response.errors {
                        Some(errors) => Err(ApiError::GraphQlErrors(errors)),
                        None => Ok(response.data.unwrap()),
                    })
            }
        },
        (),
    );

    html! {
        <div class="flex flex-col gap-4 p-4">
            {
                match scenarios.data.as_deref() {
                    Some(Err(error)) => html!{format!("Error: {:?}", error)},
                    Some(Ok(scenarios)) => {
                        scenarios
                            .scenarios_connection
                            .edges
                            .iter()
                            .map(|edge| html! { <ScenarioSummary key={edge.cursor.clone()} scenario={edge.node.clone()} /> })
                            .collect::<Html>()
                    }
                    None => html!{"Loading"}
                }
            }
        </div>
    }
}
