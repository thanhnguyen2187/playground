use maud::{html, Markup, DOCTYPE};

pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { (page_title) };
            script src="https://unpkg.com/htmx.org@2.0.4" {""};
            script defer src="https://unpkg.com/alpinejs@3.14.8" {""};
            link rel="stylesheet" type="text/css" href="./styles.css";
        }
    }
}

pub async fn page_unimplemented() -> Markup {
    html! {
        (header("Unimplemented"))
        body {
            div .prose {
                h1 { "Unimplemented" }
            }
            div .flex { "Test" }
            p { "Please check again later" }
        }
    }
}
