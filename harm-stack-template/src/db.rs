// pub fn establish_connection() -> Result<SqliteConnection, Error> {
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set either through environment variable or .env file");
//     SqliteConnection::establish(&database_url).with_whatever_context(
//         |err| format!("Failed to connect to {}: {}", database_url, err),
//     )
// }