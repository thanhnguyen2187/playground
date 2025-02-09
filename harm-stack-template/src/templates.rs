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

pub fn home() -> Markup {
    html! {
        (header("Home TODO"))
        body {
            div .prose {
                h1 { "Home" }
            }
            div .container { "Test" }
            p { "Please check again later" }
        }
    }
}

pub async fn page_unimplemented() -> Markup {
    html! {
        (header("Unimplemented!!1"))
        body {
            div .container .mx-auto .p-4 {
                h1 .text-3xl .font-bold .mb-2 { "Unimplemented" }
                p { "Please check again later" }
            }
        }
    }
}
