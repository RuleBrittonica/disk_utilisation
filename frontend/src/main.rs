use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello from Yew!" }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
