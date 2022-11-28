use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(WSEcho)]
pub fn web_socket() -> Html {
    let history = use_list(vec![]);

    let ws = use_websocket("ws://0.0.0.0:8080/ws/".to_string());
    let onclick = {
        let ws = ws.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let message = "Hello, world!".to_string();
            ws.send(message.clone());
            history.push(format!("[send]: {}", message));
        })
    };
    {
        let history = history.clone();
        let ws = ws.clone();
        // Receive message by depending on `ws.message`.
        use_effect_with_deps(
            move |message| {
                if let Some(message) = &**message {
                    history.push(format!("[recv]: {}", message.clone()));
                }
                || ()
            },
            ws.message,
        );
    }

    html! {
        <>
            <p>
                <button {onclick} disabled={*ws.ready_state != UseWebSocketReadyState::Open}>{ "Send" }</button>
            </p>
            <p>
                <b>{ "Message history: " }</b>
            </p>
            {
                for history.current().iter().map(|message| {
                    html! {
                        <p>{ message }</p>
                    }
                })
            }
        </>
    }
}
