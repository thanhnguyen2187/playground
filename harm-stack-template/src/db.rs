use crate::err::Result;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use snafu::ResultExt;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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

#[cfg(test)]
mod tests {
    use super::*;
    use diesel_migrations::MigrationHarness;

    #[test]
    fn test_create_todo() {
        let mut conn = establish_connection(&":memory:".to_owned())
            .expect("Should be able to create in-memory database");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Should be able to run migrations");
        let new_todo = Todo {
            id: "1".to_string(),
            title: "Test".to_string(),
            completed: false,
        };
        create_todo(&mut conn, &new_todo).expect("Should be able to create todo");
    }
}
