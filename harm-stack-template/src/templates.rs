use crate::db::{read_todo, read_todos, toggle_todo, Todo};
use crate::err::Result;
use crate::AppState;
use axum::extract::{Path, State};
use maud::{html, Markup, DOCTYPE};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::sync::{Arc, Mutex};

pub fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { (page_title) };
            script src="https://unpkg.com/htmx.org@2.0.4" {""};
            script defer src="https://unpkg.com/alpinejs@3.14.8" {""};
            link rel="stylesheet" type="text/css" href="./styles.css";
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum TodoState {
    Default,
    Editing,
    Done,
}

pub fn todo_row(state: TodoState, todo: &Todo) -> Markup {
    match state {
        TodoState::Default => html! {
            tr {
                td {
                    (todo.title.clone())
                }
                td .flex .gap-2 {
                    button
                        .btn
                        .btn-success
                        hx-target="closest tr"
                        hx-post=(format!("/toggle/{}", todo.id.as_str()))
                        hx-swap="outerHTML"
                        { "Finish" }
                    ;
                    button .btn .btn-primary { "Edit" }
                    button .btn { "Delete" }
                }
            }
        },
        TodoState::Editing => html! {
            tr {
                td {
                    input .input .input-bordered type="text" value=(todo.title.clone());
                }
                td .flex .gap-2 {
                    button .btn .btn-primary { "Done" }
                }
            }
        },
        TodoState::Done => html! {
            tr {
                td .line-through {
                    (todo.title.clone())
                }
                td .flex .gap-2 {
                    button
                        .btn
                        .btn-warning
                        hx-target="closest tr"
                        hx-post=(format!("/toggle/{}", todo.id.as_str()))
                        hx-swap="outerHTML"
                        { "Reopen" }
                    ;
                    button .btn .btn-primary { "Edit" }
                    button .btn { "Delete" }
                }
            }
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoWithState {
    pub data: Todo,
    pub data_temp: Todo,
    pub state: TodoState,
}

pub fn todo_row_v2(todo: &Todo) -> Markup {
    let todo_with_state = TodoWithState {
        data: todo.clone(),
        data_temp: todo.clone(),
        state: TodoState::Default,
    };
    html! {
        tr x-data=(serde_json::to_string(&todo_with_state).unwrap()) {
            td x-text="data.title" x-show="state === 'Default'" {}
            td .flex .gap-2 x-show="state === 'Default'" {
                button
                    .btn
                    .btn-success
                    { "Finish" };
                button
                    .btn
                    .btn-primary
                    "x-on:click"="state = 'Editing'"
                    { "Edit" };
                button .btn { "Delete" }
            }
            td x-show="state === 'Editing'" {
                input
                    .input
                    .input-bordered
                    type="text"
                    x-model="data_temp.title";
            }
            td .flex .gap-2 x-show="state === 'Editing'" {
                button
                    .btn
                    .btn-primary
                    "x-on:click"="
                        state = 'Default';
                        data = {...data_temp};
                    "
                    { "Save" };
                button
                    .btn
                    "x-on:click"="
                        state = 'Default';
                        data_temp = {...data};
                    "
                    { "Cancel" };
            }
        }
    }
}

pub async fn home(State(state_arc): State<Arc<Mutex<AppState>>>) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        let todos = read_todos(&mut state.conn)
            .with_whatever_context(|err| format!("Failed to read todos: {}", err))?;
        html! {
            (header("TODO Home"))
            body {
                div .container .mx-auto .p-4 .flex .flex-col .gap-2 {
                    h1 .text-3xl .font-bold { "TODO Home" }
                    table .table .w-full {
                        thead {
                            tr {
                                th { "Title" }
                                th { "Actions" }
                            }
                        }
                        tbody {
                            @for todo in todos {
                                @if todo.completed {
                                    (todo_row(TodoState::Done, &todo))
                                } @else {
                                    (todo_row(TodoState::Default, &todo))
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        html! {
            "Unable to get global state"
        }
    };

    Ok(markup)
}

pub async fn page_toggle_todo(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Path(todo_id): Path<String>,
) -> Result<Markup> {
    let markup = if let Ok(mut state) = state_arc.lock() {
        toggle_todo(&mut state.conn, &todo_id)?;
        let todo = read_todo(&mut state.conn, &todo_id)?;
        todo_row(
            {
                if todo.completed {
                    TodoState::Done
                } else {
                    TodoState::Default
                }
            },
            &todo,
        )
    } else {
        html! {
            "Unable to get global state"
        }
    };

    Ok(markup)
}

pub async fn page_unimplemented() -> Markup {
    html! {
        (header("Unimplemented!!1"))
        body {
            div .container .mx-auto .p-4 {
                h1 .text-3xl .font-bold .mb-2 { "Unimplemented" }
                p { "Please check again later" }
            }
        }
    }
}
