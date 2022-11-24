use gloo::console::log;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Gretting {
    en: String,
    de: String,
}

#[function_component(Get)]
pub fn app() -> Html {
    let state = use_state(|| Gretting::default());

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            log!("clicked the button");
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("http://127.0.0.1:8080/api/hello")
                    .send()
                    .await
                    .unwrap()
                    .json::<Gretting>()
                    .await
                    .unwrap();
                state.set(resp);
            });
        })
    };

    html! {
        <div>
            <h1>
                {"Get request example"}
            </h1>
            <div>
                <button {onclick}>{"http request"}</button>
            </div>
            <div>
                {"En: "} {state.en.clone()}
            </div>
            <div>
                {"De: "} {state.de.clone()}
            </div>
        </div>
    }
}
