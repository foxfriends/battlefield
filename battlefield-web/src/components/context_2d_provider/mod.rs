use web_sys::CanvasRenderingContext2d;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub value: Option<CanvasRenderingContext2d>,
    pub children: Children,
}

#[function_component(Context2dProvider)]
pub fn context_2d_provider(props: &Props) -> Html {
    html! {
        <ContextProvider<Option<CanvasRenderingContext2d>> context={props.value.clone()}>
            {for props.children.iter()}
        </ContextProvider<Option<CanvasRenderingContext2d>>>
    }
}

pub fn use_context_2d() -> CanvasRenderingContext2d {
    use_context::<Option<CanvasRenderingContext2d>>()
        .unwrap()
        .unwrap()
}
