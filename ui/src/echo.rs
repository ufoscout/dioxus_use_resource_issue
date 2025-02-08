use dioxus::prelude::*;

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            label { "ServerFn Echo:" }
            input {
                class: "input input-bordered w-full max-w-xs",
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = server::echo(event.value()).await.unwrap();
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
