use gloo::console;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct Data {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "test";
    let obj = Data {
        username: name.to_owned(),
        favorite_language: "C++".to_owned(),
    };

    console::log!(std::format!("this is a {} message", name));
    console::log!(serde_json::to_string_pretty(&obj).unwrap());

    html! {
        <h1>{"Hello World"}</h1>
    }
}
