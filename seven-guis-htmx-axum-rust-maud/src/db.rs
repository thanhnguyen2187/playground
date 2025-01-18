use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use snafu::ResultExt;
use crate::err::Result;

pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set either through environment variable or .env file");
    SqliteConnection::establish(&database_url).with_whatever_context(
        |err| format!("Failed to connect to {}: {}", database_url, err),
    )
}

pub fn get_persons(conn: &mut SqliteConnection, query_string: &String) -> Result<Vec<Person>> {
    use crate::schema::persons::dsl::*;

    let query_string = format!("%{}%", query_string);
    let results = persons
        .filter(name.like(&query_string).or(surname.like(&query_string)))
        .select(Person::as_select())
        .load(conn)
        .with_whatever_context(
            |err| format!("Failed to load persons: {}", err),
        )?;
    Ok(results)
}

pub fn insert_person(conn: &mut SqliteConnection, person: &Person) -> Result<usize> {
    use crate::schema::persons::dsl::*;

    diesel::insert_into(persons)
        .values(person)
        .execute(conn)
        .with_whatever_context(|err| format!("Failed to insert person: {}", err))
}

pub fn update_person(conn: &mut SqliteConnection, person: &Person) -> Result<usize> {
    use crate::schema::persons::dsl::*;

    diesel::update(persons.filter(id.eq(person.id.clone())))
        .set(person)
        .execute(conn)
        .with_whatever_context(|err| format!("Failed to update person: {}", err))
}

pub fn delete_person(conn: &mut SqliteConnection, id_: &str) -> Result<usize> {
    use crate::schema::persons::dsl::*;

    diesel::delete(persons.filter(id.eq(id_)))
        .execute(conn)
        .with_whatever_context(|err| format!("Failed to delete person: {}", err))
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::persons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: String,
    pub name: String,
    pub surname: String,
}