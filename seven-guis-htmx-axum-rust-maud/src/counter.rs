use axum::extract::State;
use std::sync::{Arc, Mutex};
use maud::{html, Markup};
use crate::AppState;
use crate::common::{header, home_back_link};

pub async fn page(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
) -> Markup {
    let input = if let Ok(app_state) = app_state_arc.lock() {
        counter_input(&app_state)
    } else {
        html! {
            "Unable to get app state"
        }
    };

    html! {
        (header("Counter"))
        body {
            h1 { "Counter" }

        form {
            fieldset {
                label {
                    "Server value: "
                    (input)
                }
                button
                    type="submit"
                    hx-post="/counter-increase"
                    hx-target="#counter-input"
                    hx-swap="outerHTML"
                    hx-trigger="click"
                    { "Increment" };
            }
        }

            form x-data="{ count: 0 }" {
                fieldset {
                    label {
                        "Client value: "
                        input x-model="count";
                    }
                    button
                        type="button"
                        x-on:click="count++"
                        { "Increment" };
                }
            }
            (home_back_link())
        }
    }
}

pub fn counter_input(app_state: &AppState) -> Markup {
    html! {
        input
            #counter-input
            type="number"
            value=(app_state.counter)
            name="counter"
        ;
    }
}

pub async fn increase(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
) -> Markup {
    let data = if let Ok(mut app_state) = app_state_arc.lock() {
        app_state.counter += 1;
        counter_input(&app_state)
    } else {
        html! {
            "Unable to get app state"
        }
    };

    data
}