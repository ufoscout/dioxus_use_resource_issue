use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::server_fn::session::*;

// Globals are created the first time you access them with the closure you pass to Global::new
pub static SESSION: GlobalSignal<Option<Session>> = Global::new(|| None);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub username: String,
}

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Login() -> Element {

    let mut name = use_signal(|| "".to_string());


    if let Some(session) = SESSION.read().as_ref() {
        rsx! {
            label { "Username is: [{session.username}] " }
            button { 
                class: "btn",
                onclick: move |_event| async move {
                    delete_user_session().await.unwrap();
                    *SESSION.write() = None;
                },
                "Logout",
             }
        }
    } else {
        rsx! {
            label { "Enter username: " }
            input {
                placeholder: "Type here to login...",
                class: "input input-bordered w-full max-w-xs",
                // we tell the component what to render
                value: "{name}",
                // and what to do when the value changes
                oninput: move |event| name.set(event.value()),
            }
            button { 
                class: "btn",
                onclick: move |_event| async move {
                    let username = set_user_session(name.read().clone()).await.unwrap();
                    *SESSION.write() = Some(Session { username });
                },
                "Login",
             }
        }
    }


}

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn SessionEcho() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            label { "Session Echo:" }
            input {
                class: "input input-bordered w-full max-w-xs",
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = session_echo(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}