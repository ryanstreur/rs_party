use rocket_db_pools::sqlx::PgPool;
use rocket_db_pools::{Connection, Database};

use rocket_db_pools::sqlx;

use crate::model::{NewUserParams, User};

#[derive(Database)]
#[database("rs_party")] // Maps to key under 'default.databases' in Rocket.toml
pub struct AppDb(PgPool);

/// A struct of named strings which refer to filenames of SQL queries in the program
pub struct QueryFiles {
    query1: &'static str,
}

/// The Query files themselves
static QUERY_FILES: QueryFiles = QueryFiles {
    query1: "src/sql/query1.sql",
};

// TODO: write a test which iterates over the queries and tests that each of the files is present

pub async fn get_first_user(mut db: Connection<AppDb>) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user LIMIT 1;"#)
        .fetch_one(&mut **db)
        .await
}

pub async fn get_all_users(mut db: Connection<AppDb>) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM rs_party.user"#)
        .fetch_all(&mut **db)
        .await
}

pub async fn insert_user(mut db: Connection<AppDb>, new_user: &NewUserParams) -> Result<User, sqlx::Error> {
    // Create a secure hash with the password

    sqlx::query_as::<_, User>(r#"INSERT INTO rs_party.user (email_address, name) VALUES ( $1, $2 ) RETURNING id, email_address, name, is_superuser;"#
    ).bind(&new_user.email).bind(&new_user.name).fetch_one(&mut **db).await
}
