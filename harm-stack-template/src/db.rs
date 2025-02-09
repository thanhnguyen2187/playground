use crate::err::Result;
use diesel::prelude::*;
use snafu::ResultExt;

pub fn establish_connection(database_url: &String) -> Result<SqliteConnection> {
    SqliteConnection::establish(database_url)
        .with_whatever_context(|err| format!("Failed to connect to {}: {}", database_url, err))
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

pub fn create_todo(conn: &mut SqliteConnection, item: &Todo) -> Result<usize> {
    use crate::schema::todos::dsl::*;

    diesel::insert_into(todos)
        .values(item)
        .execute(conn)
        .with_whatever_context(|err| format!("Failed to insert todo: {}", err))
}

pub fn read_todos(conn: &mut SqliteConnection) -> Result<Vec<Todo>> {
    use crate::schema::todos::dsl::*;

    let results = todos
        .select(Todo::as_select())
        .load(conn)
        .with_whatever_context(|err| format!("Failed to load persons: {}", err))?;
    Ok(results)
}

pub fn update_todo(conn: &mut SqliteConnection, item: &Todo) -> Result<usize> {
    use crate::schema::todos::dsl::*;

    diesel::update(todos.filter(id.eq(item.id.clone())))
        .set(item)
        .execute(conn)
        .with_whatever_context(|err| format!("Failed to update todo: {}", err))
}

pub fn delete_todo(conn: &mut SqliteConnection, todo_id: &String) -> Result<usize> {
    use crate::schema::todos::dsl::*;

    diesel::delete(todos.filter(id.eq(todo_id)))
        .execute(conn)
        .with_whatever_context(|err| format!("Failed to delete todo: {}", err))
}
