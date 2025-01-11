use maud::{html, DOCTYPE, Markup};

fn header(page_title: &str) -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta charset="utf-8";
            title { (page_title) };
            link rel="stylesheet" type="text/css" href="https://matcha.mizu.sh/matcha.css";
            script src="https://unpkg.com/htmx.org@2.0.4" {""};
        }
    }
}

pub async fn index() -> Markup {
    html! {
        (header("Seven GUIs in Rust"))
        body {
            h1 { "Seven GUIs in Rust" }
            ul {
                li { a href="/counter" { "Counter" } }
                li { a href="/temperature-converter" { "Temperature Converter (Unimplemented)" } }
                li { a href="/flight-booker" { "Flight Booker (Unimplemented)" } }
                li { a href="/timer" { "Timer (Unimplemented)" } }
                li { a href="/crud" { "CRUD (Unimplemented)" } }
                li { a href="/circle-drawer" { "Circle Drawer (Unimplemented)" } }
            }
        }
    }
}

pub async fn unimplemented() -> Markup {
    html! {
        (header("Unimplemented"))
        body {
            h1 { "Unimplemented" }
            p { "Please check again later" }
            a href="/" { "Back" }
        }
    }
}

pub async fn counter() -> Markup {
    html! {
        (header("Counter"))
        body {
            h1 { "Counter" }

            p { "WIP" }

            a href="/" { "Back" }
        }
    }
}
