use maud::{html, Markup};
use crate::common::{header, home_back_link};

pub async fn page() -> Markup {
    html! {
        (header("CRUD"))
        body {
            h1 { "CRUD" }
            p { "WIP" }
            (home_back_link())
        }
    }
}
