use crate::Route;
use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/blog.css");

#[component]
pub fn Blog(id: ReadOnlySignal<i32>) -> Element {
    let blog = use_server_future(move || {
        let id = id();
        async move {
            get_blog_post(id).await.unwrap()
        }
    })?;


    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "#{blog().unwrap()}" }

            // Navigation links
            Link {
                class: "link",
                to: Route::Blog { id: id() - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                class: "link",
                to: Route::Blog { id: id() + 1 },
                "Next"
            }
        }
    }
}

#[server(GetBlogPost)]
pub async fn get_blog_post(page: i32) -> Result<String, ServerFnError> {
    Ok(format!("Hello from the server! Blog: #{page}"))
}