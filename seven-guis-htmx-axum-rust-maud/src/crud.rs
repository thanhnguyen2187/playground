use std::error::Error;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, State};
use axum::Form;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use maud::{html, Markup};
use snafu::{whatever, ResultExt};
use crate::AppState;
use crate::common::{header, home_back_link};
use crate::err::{Result};

pub mod components {
    use maud::{html, Markup};
    use crate::crud::state_mod::Person;

    pub fn input_filter(filter: String) -> Markup {
        html! {
            input
                type="text"
                name="filter"
                value=(filter)
            ;
        }
    }

    pub fn select_persons(persons: &Vec<Person>) -> Markup {
        unimplemented!()
    }

    pub fn form_person(Person { id, name, surname }: &Person) -> Markup {
        unimplemented!()
    }

    pub fn form_buttons() -> Markup {
        unimplemented!()
    }
}

pub mod state_mod {
    use serde::Deserialize;

    #[derive(Debug)]
    pub struct Person {
        pub id: String,
        pub name: String,
        pub surname: String,
    }

    #[derive(Debug)]
    pub struct Impl {
        pub persons: Vec<Person>,
        pub filter: String,
        pub id_selected: String,
        pub name: String,
        pub surname: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct FormData {
        pub filter: Option<String>,
        pub id_selected: Option<String>,
        pub name: Option<String>,
        pub surname: Option<String>,
    }

    pub fn new() -> Impl {
        Impl {
            persons: Vec::new(),
            filter: String::new(),
            id_selected: String::new(),
            name: String::new(),
            surname: String::new(),
        }
    }
}

pub async fn mutate_state(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Path(field): Path<String>,
    Form(form_data): Form<state_mod::FormData>,
) -> Result<StatusCode> {
    if let Ok(mut state) = state_arc.lock() {
        match field.as_str() {
            "filter" => {
                state.crud_state.filter = form_data.filter.unwrap_or(
                    state.crud_state.filter.clone(),
                );
            }
            "id_selected" => {
                state.crud_state.id_selected = form_data.id_selected.unwrap_or(
                    state.crud_state.id_selected.clone(),
                );
            }
            "name" => {
                state.crud_state.name = form_data.name.unwrap_or(
                    state.crud_state.name.clone(),
                );
            }
            "surname" => {
                state.crud_state.surname = form_data.surname.unwrap_or(
                    state.crud_state.surname.clone(),
                );
            }
            _ => {
                whatever!("Invalid field: {}", field);
            }
        }
    } else {
        whatever!("Unable to get global state");
    }

    Ok(StatusCode::CREATED)
}

pub async fn page() -> Markup {
    html! {
        (header("CRUD"))
        body {
            h1 { "CRUD" }
            form {
                fieldset {
                    label {
                        "Filter: "
                        input
                            type="text"
                            name="name"
                        ;
                    }
                }

                fieldset .flex {
                    label .mr-2 style="width: 14em;" {
                        select size="5" name="status" {
                            option value="all" { "All" }
                            option value="active" { "Active" }
                            option value="inactive" { "Inactive" }
                        }
                    }

                    div {
                        label {
                            "Name: "
                            input
                                type="text"
                                name="name"
                            ;
                        }

                        label {
                            "Surname: "
                            input
                                type="text"
                                name="surname"
                            ;
                        }
                    }
                }

                fieldset {
                    button { "Create" };
                    button { "Update" };
                    button { "Delete" };
                }
            }
            (home_back_link())
        }
    }
}
