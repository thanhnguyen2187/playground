use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::Form;
use log::{debug, info, warn};
use maud::{html, Markup};
use serde::Deserialize;
use crate::AppState;
use crate::index::{header, home_back_link};

#[derive(Debug)]
pub struct OneWayFlight {
    pub from: Option<String>,
}

#[derive(Debug)]
pub struct ReturnFlight {
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug)]
pub enum FlightBookerState {
    OneWay(OneWayFlight),
    Return(ReturnFlight),
}

pub fn validate_date(date: &String) -> bool {
    let parts = date.split('.').collect::<Vec<&str>>();
    match parts[..] {
        [day, month, year] => {
            let day = day.parse::<u32>().unwrap_or(0);
            let month = month.parse::<u32>().unwrap_or(0);
            let year = year.parse::<u32>().unwrap_or(0);
            if day == 0 || month == 0 || year == 0 {
                return false;
            }
            if day > 31 || month > 12 || year < 1900 || year > 2100 {
                return false;
            }
            true
        }
        _ => false,
    }
}

pub fn calculate_from_class(state: &FlightBookerState) -> &'static str {
    let from_opt = match state {
        FlightBookerState::OneWay(OneWayFlight { from }) => from,
        FlightBookerState::Return(ReturnFlight { from, to: _ }) => from,
    };

    if let Some(from) = from_opt {
        if !validate_date(&from) {
            return "fg-danger";
        }
    }

    ""
}

pub fn calculate_flight_type(state: &FlightBookerState) -> &'static str {
    match state {
        FlightBookerState::OneWay(_) => "one-way",
        FlightBookerState::Return(_) => "return",
    }
}

pub fn calculate_to_class(state: &FlightBookerState) -> &'static str {
    let to_opt = match state {
        FlightBookerState::OneWay(_) => None,
        FlightBookerState::Return(ReturnFlight { from: _, to }) => to.clone(),
    };

    if let Some(to) = to_opt {
        if !validate_date(&to) {
            return "fg-danger";
        }
    }

    ""
}

pub fn calculate_to_disabled(state: &FlightBookerState) -> &'static str {
    match state {
        FlightBookerState::OneWay(_) => "false",
        FlightBookerState::Return(_) => "true",
    }
}

pub fn calculate_book_disabled(state: &FlightBookerState) -> &'static str {
    match state {
        FlightBookerState::OneWay(OneWayFlight { from: from_opt }) => {
            if let Some(from) = from_opt {
                if !validate_date(from) {
                    return "true";
                }
            }
            "false"
        }
        FlightBookerState::Return(ReturnFlight { from: from_opt, to: to_opt }) => {
            if let Some(from) = from_opt {
                if !validate_date(from) {
                    return "true";
                }
            }
            if let Some(to) = to_opt {
                if !validate_date(to) {
                    return "true";
                }
            }
            "false"
        }
    }
}

pub fn options(state: &FlightBookerState) -> Markup {
    match state {
        FlightBookerState::OneWay(_) => {
            html! {
                option value="one-way" selected="selected" { "One Way" }
                option value="return" { "Return" }
            }
        }
        FlightBookerState::Return(_) => {
            html! {
                option value="one-way" { "One Way" }
                option value="return" selected="selected" { "Return" }
            }
        }
    }
}

pub fn component(state: &FlightBookerState) -> Markup {
    html! {
        form #flight-booker {
            fieldset {
                label {
                    "Type: "
                    select
                        name="flight-type"
                        hx-trigger="change"
                        hx-target="#flight-booker"
                        hx-swap="outerHTML"
                        hx-post="/flight-booker-component"
                    {
                        (options(state))
                    }
                }
                label {
                    "From: "
                    input
                        type="text"
                        name="from"
                        class={ (calculate_from_class(state)) }
                    ;
                }
                label {
                    "To: "
                    input
                        type="text"
                        name="to"
                        class={ (calculate_to_class(state)) }
                        disabled={ (calculate_to_disabled(state)) }
                    ;
                }
                button
                    type="submit"
                    hx-post="/flight-booker-component"
                    hx-target="#flight-booker"
                    hx-swap="outerHTML"
                    { "Book" };
                button
                    type="button"
                    hx-post="/flight-booker-reset"
                    hx-target="#flight-booker"
                    hx-swap="outerHTML"
                    { "Reset" };
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FormData {
    flight_type: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

fn empty_string_to_none(string_opt: Option<String>) -> Option<String> {
    if Some(String::new()) == string_opt {
        None
    } else {
        string_opt
    }
}

pub fn mutate_state(state: &mut FlightBookerState, form_data: FormData) {
    match form_data.flight_type {
        Some(ref flight_type) if flight_type == "one-way" => {
            *state = FlightBookerState::OneWay(OneWayFlight {
                from: empty_string_to_none(form_data.from),
            });
        }
        Some(ref flight_type) if flight_type == "return" => {
            *state = FlightBookerState::Return(ReturnFlight {
                from: empty_string_to_none(form_data.from),
                to: empty_string_to_none(form_data.to),
            });
        }
        _ => {
            *state = FlightBookerState::OneWay(OneWayFlight {
                from: None,
            });
        }
    }
    debug!("State after mutation: {:?}", state);
}

pub fn reset_state(state: &mut FlightBookerState) {
    *state = FlightBookerState::OneWay(OneWayFlight {
        from: None,
    });
}

pub async fn page_component(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
    Form(form_data): Form<FormData>,
) -> Markup {
    debug!("Form data: {:?}", form_data);
    if let Ok(mut app_state) = app_state_arc.lock() {
        mutate_state(&mut app_state.flight_booker_state, form_data);
        component(&app_state.flight_booker_state)
    } else {
        html! {
            "Unable to get app state"
        }
    }
}

pub async fn page(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
) -> Markup {
    let data = if let Ok(app_state) = app_state_arc.lock() {
        component(&app_state.flight_booker_state)
    } else {
        html! {
            "Unable to get app state"
        }
    };

    html! {
        (header("Flight Booker"))
        body {
            h1 { "Flight Booker" }
            { (data) }
            (home_back_link())
        }
    }
}

pub async fn page_reset(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
) -> Markup {
    if let Ok(mut app_state) = app_state_arc.lock() {
        reset_state(&mut app_state.flight_booker_state);
        component(&app_state.flight_booker_state)
    } else {
        html! {
            "Unable to get app state"
        }
    }
}