use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use snafu::ResultExt;
use crate::err::Result;

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set either through environment variable or .env file");
    SqliteConnection::establish(&database_url).with_whatever_context(
        |err| format!("Failed to connect to {}: {}", database_url, err),
    )
}

pub fn get_persons() -> Result<Vec<Person>> {
    use crate::schema::persons::dsl::*;

    let conn = &mut establish_connection()?;
    let results = persons.select(Person::as_select()).load(conn).with_whatever_context(
        |err| format!("Failed to load persons: {}", err),
    )?;
    Ok(results)
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: String,
    pub name: String,
    pub surname: String,
}