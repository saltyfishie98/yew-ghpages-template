use gloo::console;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    console::log!("Hello World");

    html! {
        <h1>{"Hello World"}</h1>
    }
}
