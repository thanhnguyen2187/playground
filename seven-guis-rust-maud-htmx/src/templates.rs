use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use axum::extract::State;
use maud::{html, DOCTYPE, Markup};
use crate::AppState;

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

pub async fn page_index() -> Markup {
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

pub async fn page_unimplemented() -> Markup {
    html! {
        (header("Unimplemented"))
        body {
            h1 { "Unimplemented" }
            p { "Please check again later" }
            a href="/" { "Back" }
        }
    }
}

pub async fn page_counter(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
) -> Markup {
    let data = if let Ok(app_state) = app_state_arc.lock() {
        counter_component(&app_state)
    } else {
        html! {
            "Unable to get app state"
        }
    };

    html! {
        (header("Counter"))
        body {
            h1 { "Counter" }

            (data)

            a href="/" { "Back" }
        }
    }
}

pub fn counter_component(app_state: &AppState) -> Markup {
    html! {
        form #counter {
            fieldset {
                label {
                    "Value: "
                    input type="number" value=(app_state.counter) name="counter";
                }
                button
                    type="submit"
                    hx-post="/counter-increase"
                    hx-target="#counter"
                    hx-swap="outerHTML"
                    hx-trigger="click"
                    { "Increment" };
            }
        }
    }
}

pub async fn page_counter_increase(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
) -> Markup {
    let data = if let Ok(mut app_state) = app_state_arc.lock() {
        app_state.counter += 1;
        counter_component(&app_state)
    } else {
        html! {
            "Unable to get app state"
        }
    };

    data
}

// pub async fn page_counter_component(
//     State(mut app_state): State<Arc<Mutex<AppState>>>,
// ) -> Markup {
//     app_state.deref_mut().counter += 1;
//     html! {
//         input type="number" value=(app_state.counter) name="counter";
//         button
//             type="submit"
//             hx-get="/counter-component"
//             hx-target="#counter"
//             hx-swap="outerHTML"
//             hx-trigger="click"
//             { "Increment" };
//     }
// }
