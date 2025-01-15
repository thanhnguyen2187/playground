use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::Form;
use log::{debug, info, warn};
use maud::{html, Markup};
use serde::Deserialize;
use crate::AppState;
use crate::common::{header, home_back_link};

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

pub fn validate_range(from: &String, to: &String) -> bool {
    if !validate_date(from) {
        return false;
    }
    if !validate_date(to) {
        return false;
    }
    let from_parts = from.split('.').collect::<Vec<&str>>();
    let to_parts = to.split('.').collect::<Vec<&str>>();
    let from_day = from_parts[0].parse::<u32>().unwrap_or(0);
    let from_month = from_parts[1].parse::<u32>().unwrap_or(0);
    let from_year = from_parts[2].parse::<u32>().unwrap_or(0);
    let to_day = to_parts[0].parse::<u32>().unwrap_or(0);
    let to_month = to_parts[1].parse::<u32>().unwrap_or(0);
    let to_year = to_parts[2].parse::<u32>().unwrap_or(0);

    if from_day > to_day {
        return false;
    }
    if from_month > to_month {
        return false;
    }
    if from_year > to_year {
        return false;
    }

    true
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

            isValidRange(from, to) {
                // TODO: think about how to cache this, as the splitting is
                //       redone again and again
                if (this.isValidDate(from) && this.isValidDate(to)) {
                    const fromParts = from.split('.');
                    const toParts = to.split('.');
                    const fromDay = parseInt(fromParts[0], 10);
                    const fromMonth = parseInt(fromParts[1], 10);
                    const fromYear = parseInt(fromParts[2], 10);
                    const toDay = parseInt(toParts[0], 10);
                    const toMonth = parseInt(toParts[1], 10);
                    const toYear = parseInt(toParts[2], 10);

                    if (fromDay > toDay) {
                        return false;
                    }
                    if (fromMonth > toMonth) {
                        return false;
                    }
                    if (fromYear > toYear) {
                        return false;
                    }
                    return true;
                }
            },

            canBook() {
                if (this.flightType === 'one-way') {
                    return this.isValidDate(this.from);
                } else {
                    return this.isValidRange(this.from, this.to);
                }
                console.warn('canBook: unreachable code');
                return false;
            },

            reset() {
                this.from = '';
                this.fromTouched = false;
                this.fromMessage = '';
                this.to = '';
                this.toTouched = false;
                this.toMessage = '';
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
                            } else if (!isValidRange(from, to)) {
                                toMessage = 'From date must be before to date';
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
                    hx-post="/flight-booker-submit"
                    hx-target="#flight-booker"
                    hx-swap="outerHTML"
                    ":disabled"="canBook() ? undefined : 'disabled'"
                    { "Book" };
                button
                    type="button"
                    "@click"="reset()"
                    { "Reset" };
            }
        }
    }
}

pub fn component_success() -> Markup {
    html! {
        form #flight-booker x-data="{}" {
            fieldset {
                label {
                    "Booked successfully! See you at the airport!"
                }
            }
        }
    }
}

pub fn component_unreachable() -> Markup {
    html! {
        form #flight-booker x-data="{}" {
            fieldset {
                label {
                    "An unexpected error happened. Please try again later!"
                }
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

pub fn check_state(state: &mut FlightBookerState, form_data: FormData) {
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

pub fn check_submission(form_data: &FormData) -> bool {
    match (&form_data.flight_type, &form_data.from, &form_data.to) {
        (Some(ref flight_type), Some(from), _) if flight_type == "one-way" => {
            validate_date(from)
        }
        (Some(ref flight_type), Some(from), Some(to)) if flight_type == "return" => {
            validate_range(from, to)
        }
        _ => {
            false
        }
    }
}

pub async fn page_submit(
    Form(form_data): Form<FormData>,
) -> Markup {
    debug!("Form data: {:?}", form_data);
    if check_submission(&form_data) {
        component_success()
    } else {
        component_unreachable()
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
