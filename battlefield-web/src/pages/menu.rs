use crate::api::*;
use crate::hooks::use_query::use_query;
use crate::pages::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MenuPage)]
pub fn menu_page() -> Html {
    let scenarios = use_query(
        |_| async move {
            let operation = ListScenariosQuery::build(ListScenariosArguments::default());
            surf::post("http://localhost:8080/graphql")
                .run_graphql(operation)
                .await
                .map_err(ApiError::RequestError)
                .and_then(|response| match response.errors {
                    Some(errors) => Err(ApiError::GraphQlErrors(errors)),
                    None => Ok(response.data.unwrap()),
                })
        },
        (),
    );

    html! {
        <div>
            {
                match scenarios.data.as_deref() {
                    Some(Err(error)) => html!{format!("Error: {:?}", error)},
                    Some(Ok(scenarios)) => {
                        scenarios
                            .scenarios_connection
                            .edges
                            .iter()
                            .map(|edge| {
                                let name = &edge.node.name;
                                html! {
                                    <Link<Route> to={Route::NewGame { scenario: name.to_owned() }}>
                                        {name}
                                    </Link<Route>>
                                }
                            })
                            .collect::<Html>()
                    }
                    None => html!{"Loading"}
                }
            }
        </div>
    }
}
