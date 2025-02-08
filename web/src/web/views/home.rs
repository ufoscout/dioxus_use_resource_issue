use dioxus::prelude::*;
use ui::{Echo, Hero};

use crate::web::ui::log::{Login, SessionEcho};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Login {} 
        SessionEcho {}
        Echo {}
    }
}
