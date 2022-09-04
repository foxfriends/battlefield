use super::context_2d_provider::Context2dProvider;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Canvas2d)]
pub fn canvas_2d(props: &Props) -> Html {
    let canvas_ref = use_node_ref();
    let state = use_state(|| None);
    use_effect_with_deps(
        {
            let setter = state.clone();
            move |canvas_ref: &NodeRef| {
                setter.set(canvas_ref.cast::<HtmlCanvasElement>());
                || ()
            }
        },
        canvas_ref.clone(),
    );

    html! {
        <div class="relative w-screen h-screen">
            <canvas ref={canvas_ref} class="absolute w-full h-full" />
            <div class="absolute inset-0">
                <Context2dProvider canvas={(*state).clone()}>
                    {for props.children.iter()}
                </Context2dProvider>
            </div>
        </div>
    }
}
