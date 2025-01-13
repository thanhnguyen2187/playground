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
        FlightBookerState::OneWay(_) => "",
        FlightBookerState::Return(_) => "disabled",
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

pub fn to_input(state: &FlightBookerState) -> Markup {
    match state {
        FlightBookerState::OneWay(_) => {
            html! {
                input
                    type="text"
                    name="to"
                    disabled="disabled"
                ;
            }
        }
        FlightBookerState::Return(ReturnFlight { from: _, to: to_opt }) => {
            let to = to_opt.clone().unwrap_or(String::new());
            html! {
                input
                    type="text"
                    name="to"
                    class={ (calculate_to_class(state)) }
                    value={ (to) }
                ;
            }
        }
    }
}

pub fn from_input(state: &FlightBookerState) -> Markup {
    let from_opt = match state {
        FlightBookerState::OneWay(OneWayFlight { from: from_opt }) => {
            from_opt.clone()
        }
        FlightBookerState::Return(ReturnFlight { from: from_opt, to: _ }) => {
            from_opt.clone()
        }
    };
    let from = from_opt.unwrap_or(String::new());
    html! {
        input
            type="text"
            name="from"
            class={ (calculate_from_class(state)) }
            value={ (from) }
        ;
    }
}

pub fn component(state: &FlightBookerState) -> Markup {
    html! {
        form #flight-booker x-data="{
            flightType: 'one-way',

            from: '',
            fromTouched: false,
            fromMessage: '',

            to: '',
            toTouched: false,
            toMessage: '',

            isValidDate(date) {
                const parts = date.split('.');
                if (parts.length !== 3) {
                    return false;
                }

                const day = parseInt(parts[0], 10);
                const month = parseInt(parts[1], 10);
                const year = parseInt(parts[2], 10);

                if (isNaN(day) || isNaN(month) || isNaN(year)) {
                    return false;
                }
                if (day < 1 || day > 31) {
                    return false;
                }
                if (month < 1 || month > 12) {
                    return false;
                }
                if (year < 1900 || year > 2100) {
                    return false;
                }

                return true;
            },
        }" {
            fieldset {
                label {
                    "Type: "
                    select
                        name="flight-type"
                        x-model="flightType"
                    {
                        option value="one-way" { "One Way" }
                        option value="return" { "Return" }
                    }
                }
                label {
                    "From: "
                    input
                        type="text"
                        name="from"
                        x-model="from"
                        "@blur"="
                            fromTouched = true;
                            if (from === '') {
                                fromMessage = 'Date must not be empty';
                            } else if (!isValidDate(from)) {
                                fromMessage = 'Expected format: DD.MM.YYYY';
                            } else {
                                fromMessage = '';
                            }
                        "
                        ":class"="(fromTouched && !isValidDate(from)) ? 'fg-danger' : ''"
                    ;
                    span .smaller x-text="fromMessage";
                }
                label x-show="flightType === 'return'" {
                    "To: "
                    input
                        type="text"
                        name="to"
                        x-model="to"
                        "@blur"="
                            toTouched = true;
                            if (to === '') {
                                toMessage = 'Date must not be empty';
                            } else if (!isValidDate(to)) {
                                toMessage = 'Expected format: DD.MM.YYYY';
                            } else {
                                toMessage = '';
                            }
                        "
                        ":class"="(toTouched && !isValidDate(to)) ? 'fg-danger' : ''"
                    ;
                    span .smaller x-text="toMessage";
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