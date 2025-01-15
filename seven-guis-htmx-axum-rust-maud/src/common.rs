use maud::{html, Markup, DOCTYPE};

pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { (page_title) };
            link rel="stylesheet" type="text/css" href="https://matcha.mizu.sh/matcha.css";
            script src="https://unpkg.com/htmx.org@2.0.4" {""};
            script defer src="https://unpkg.com/alpinejs@3.14.8" {""};
        }
    }
}

pub fn home_back_link() -> Markup {
    html! {
        a href="/" { "Back" }
    }
}

pub async fn page_unimplemented() -> Markup {
    html! {
        (header("Unimplemented"))
        body {
            h1 { "Unimplemented" }
            p { "Please check again later" }
            (home_back_link())
        }
    }
}
