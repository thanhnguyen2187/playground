use maud::{html, Markup};
use crate::common::{header, home_back_link};

pub async fn page() -> Markup {
    html! {
        (header("Timer"))
        body {
            h1 { "Timer" }
            p { "WIP" }
            (home_back_link())
        }
    }
}

