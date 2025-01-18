use std::error::Error;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, Query, State};
use axum::Form;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use maud::{html, Markup};
use snafu::{whatever, ResultExt};
use crate::{db, AppState};
use crate::common::{header, home_back_link};
use crate::crud::components::{form_person, select_persons};
use crate::db::Person;
use rand::distributions::{Alphanumeric, DistString};
use crate::err::Result;

pub mod components {
    use maud::{html, Markup};
    use crate::db::Person;

    pub fn select_persons(persons: &Vec<Person>) -> Markup {
        if persons.is_empty() {
            html! {
                span { "No data yet" };
            }
        } else {
            html! {
                select
                size="5"
                x-model="id_selected"
                name="id-selected" {
                    @for person in persons {
                        option
                            value=(person.id)
                            "@click"={
                                (format!(
                                    r#"
                                        id_selected = '{}';
                                        name = '{}';
                                        surname = '{}';
                                    "#,
                                    person.id,
                                    person.name,
                                    person.surname,
                                ))
                            }
                            {
                                (person.name)
                                ", "
                                (person.surname)
                            }
                    }
                };
            }
        }
    }

    pub fn form_person(Person { id, name, surname }: &Person) -> Markup {
        html! {
            label {
                "Name: "
                input
                    type="text"
                    name="name"
                    x-model="name"
                    value=(name)
                ;
            }

            label {
                "Surname: "
                input
                    type="text"
                    name="surname"
                    x-model="surname"
                    value=(surname)
                ;
            }
        }
    }

    pub fn form_buttons() -> Markup {
        unimplemented!()
    }
}

pub fn generate_id() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

pub mod state_mod {
    use serde::Deserialize;
    use crate::db::Person;

    #[derive(Debug)]
    pub struct Impl {
        pub persons: Vec<Person>,
        pub filter: String,
        pub id_selected: String,
        pub name: String,
        pub surname: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
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

pub async fn create(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Form(form_data): Form<state_mod::FormData>,
) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        let id = generate_id();
        let name = form_data.name.unwrap_or(String::new());
        let surname = form_data.surname.unwrap_or(String::new());
        let filter = state.crud_state.filter.clone();

        let conn = &mut state.sqlite_connection;
        db::insert_person(conn, &Person {
            id: id.clone(),
            name,
            surname,
        })?;
        let persons = &db::get_persons(
            conn,
            &filter,
        )?;
        select_persons(persons)
    } else {
        html! {
            "Unable to get global state"
        }
    };

    Ok(markup)
}

pub async fn update(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Form(form_data): Form<state_mod::FormData>,
) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        let id = form_data.id_selected.unwrap_or(String::new());
        let name = form_data.name.unwrap_or(String::new());
        let surname = form_data.surname.unwrap_or(String::new());
        let filter = form_data.filter.unwrap_or(String::new());

        let conn = &mut state.sqlite_connection;
        db::update_person(conn, &Person {
            id: id.clone(),
            name,
            surname,
        })?;
        let persons = &db::get_persons(
            conn,
            &filter,
        )?;
        select_persons(persons)
    } else {
        html! {
            "Unable to get global state"
        }
    };

    Ok(markup)
}

pub async fn delete(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Query(params): Query<state_mod::FormData>,
) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        let id = params.id_selected.unwrap_or(String::new());
        let filter = params.filter.unwrap_or(String::new());
        let conn = &mut state.sqlite_connection;
        db::delete_person(conn, &id)?;
        let persons = &db::get_persons(
            conn,
            &filter,
        )?;
        select_persons(persons)
    } else {
        html! {
            "Unable to get global state"
        }
    };

    Ok(markup)
}

pub async fn update_filter(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Form(form_data): Form<state_mod::FormData>,
) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        let filter = form_data.filter.unwrap_or(String::new());
        let conn = &mut state.sqlite_connection;
        let persons = &db::get_persons(
            conn,
            &filter,
        )?;
        select_persons(persons)
    } else {
        html! {
            "Unable to get global state"
        }
    };

    Ok(markup)
}

pub async fn page(
    State(state_arc): State<Arc<Mutex<AppState>>>,
) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        let persons = &db::get_persons(
            &mut state.sqlite_connection,
            &String::new(),
        )?;
        html! {
            (header("CRUD"))
            body {
                h1 { "CRUD" }
                form x-data="{
                    id_selected: '',
                    filter: '',
                    name: '',
                    surname: '',

                    reset() {
                        this.id_selected = '';
                        this.filter = '';
                        this.name = '';
                        this.surname = '';
                    },
                }" {
                    fieldset {
                        label {
                            "Filter: "
                            input
                                type="text"
                                name="filter"
                                x-model="filter"
                                hx-post="/crud/update-filter"
                                hx-target="#select-persons"
                            ;
                        }
                    }

                    fieldset .flex {
                        label
                            #select-persons
                            .mr-2
                            style="width: 14em;"
                            { (select_persons(persons)) };

                        div #form-person {
                            (form_person(&Person {
                                id: String::new(),
                                name: String::new(),
                                surname: String::new()
                            }))
                        }
                    }

                    fieldset {
                        button
                            type="button"
                            hx-post="/crud"
                            hx-target="#select-persons"
                            "@click"="
                                setTimeout(() => {
                                    reset();
                                }, 0);
                            "
                            { "Create" };
                        button
                            type="button"
                            hx-put="/crud"
                            hx-target="#select-persons"
                            "@click"="
                                setTimeout(() => {
                                    reset();
                                }, 0);
                            "
                            { "Update" };
                        button
                            type="button"
                            hx-delete="/crud"
                            hx-include="[name='id-selected']"
                            hx-target="#select-persons"
                            hx-confirm="Are you sure you want to delete?"
                            "@click"="
                                setTimeout(() => {
                                    reset();
                                }, 0);
                            "
                            { "Delete" };
                        button
                            type="button"
                            "@click"="
                                setTimeout(() => {
                                    reset();
                                }, 0);
                            "
                            { "Reset" };
                    }
                }
                (home_back_link())
            }
        }
    } else {
        html! {
            (header("CRUD"))
            body {
                p { "Error loading state" }
            }
        }
    };

    Ok(markup)
}
