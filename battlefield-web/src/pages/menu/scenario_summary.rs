use crate::api::Scenario;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Eq, PartialEq, Properties)]
pub struct Props {
    pub scenario: Scenario,
}

#[function_component(ScenarioSummary)]
pub fn scenario_summary(props: &Props) -> Html {
    html! {
        <Link<Route>
            to={Route::NewGame { scenario: props.scenario.name.to_owned() }}
            classes="flex justify-between border border-black/20 p-4 hover:bg-black/5"
            disabled={!props.scenario.errors.is_empty()}
        >
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
        </Link<Route>>
    }
}
