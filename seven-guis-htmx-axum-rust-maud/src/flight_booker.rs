use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::Form;
use log::info;
use maud::{html, Markup};
use serde::Deserialize;
use crate::AppState;
use crate::index::{header, home_back_link};

pub struct OneWayFlight {
    pub from: Option<String>,
}

pub struct ReturnFlight {
    pub from: Option<String>,
    pub to: Option<String>,
}

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

pub fn calculate_to_disabled(state: &FlightBookerState) -> &'static str {
    match state {
        FlightBookerState::OneWay(_) => "true",
        FlightBookerState::Return(_) => "false",
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

pub fn component(state: &FlightBookerState) -> Markup {
    html! {
        form #flight-booker {
            fieldset {
                label {
                    "Type: "
                    select name="flight-type" {
                        option value="one-way" { "One Way" }
                        option value="return" { "Return" }
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
                        disabled={ (calculate_to_disabled(state)) }
                    ;
                }
                button
                    type="submit"
                    hx-post="/flight-booker-component"
                    hx-target="#flight-booker"
                    hx-swap="outerHTML"
                    { "Book" };
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

pub async fn page_component(
    State(app_state_arc): State<Arc<Mutex<AppState>>>,
    Form(form_data): Form<FormData>,
) -> Markup {
    info!("Form data: {:?}", form_data);
    if let Ok(app_state) = app_state_arc.lock() {
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