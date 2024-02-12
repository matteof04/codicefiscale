/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
use crate::models::{City, Nation, NewCity, NewLoadedCity, NewLoadedNation, NewNation};
use crate::schema::cities::dsl::*;
use crate::schema::nations::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::{env, fs};
use thiserror::Error;

// Despite the errors found by rust-analyzer, the software compiles successfully

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database connection error")]
    NotExist,
    #[error("Cities table empty!")]
    CitiesTableEmpty,
    #[error("Nations table empty!")]
    NationsTableEmpty,
}

/// Checks if the database is not empty
pub fn check_db_not_empty() -> Result<(), DbError> {
    let database_url = env::var("DATABASE_URL").unwrap_or("data.db".to_string());
    if fs::read(&database_url).is_err() {
        eprintln!(
            "A database with all nations and cities is needed.\nCreate one using build-database command and set the DATABASE_URL environment variable to the database path.\nIf the database name is data.db and if is in the same path as this executable, the variable can be omitted."
        );
        return Err(DbError::NotExist);
    }
    let mut conn = establish_connection();
    let count: i64 = cities.count().get_result(&mut conn).unwrap();
    if count <= 0 {
        return Err(DbError::CitiesTableEmpty);
    }
    let count: i64 = nations.count().get_result(&mut conn).unwrap();
    if count <= 0 {
        return Err(DbError::NationsTableEmpty);
    }
    Ok(())
}

/// Try to enstablish a connection with the database
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or("data.db".to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Search the nation with the given name in the database
pub fn search_nation(conn: &mut SqliteConnection, name: &str) -> Vec<Nation> {
    nations
        .filter(nation_name.like(name))
        .limit(5)
        .select(Nation::as_select())
        .load(conn)
        .expect("Error loading nation")
}

/// Search the italian city with the given name in the database
pub fn search_city(conn: &mut SqliteConnection, name: &str) -> Vec<City> {
    cities
        .filter(city_name.like(name))
        .limit(5)
        .select(City::as_select())
        .load(conn)
        .expect("Error loading city")
}

/// Populate the database using the data in the `gi_nazioni.json` and `gi_comuni.json`.\
/// It also fixes some nation codes incompatibility.\
/// These files can be obtained [here](https://www.gardainformatica.it/database-comuni-italiani).
pub fn populate_db() {
    let mut conn = establish_connection();
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    let nations_file_content =
        fs::read_to_string("gi_nazioni.json").expect("Error in nations json file opening");
    let loaded_nations: Vec<NewLoadedNation> = serde_json::from_str(&nations_file_content).unwrap();
    let loaded_nations: Vec<NewNation> = loaded_nations.into_iter().map(NewNation::from).collect();
    let loaded_nations: Vec<NewNation> = loaded_nations
        .into_iter()
        .map(|n| {
            if n.nation_code.is_empty() {
                NewNation {
                    nation_code: "0000".to_string(),
                    ..n
                }
            } else {
                n
            }
        })
        .collect();
    diesel::insert_into(nations)
        .values(loaded_nations)
        .execute(&mut conn)
        .expect("Error during nation table population");
    let cities_file_content =
        fs::read_to_string("gi_comuni.json").expect("Error in cities json file opening");
    let loaded_cities: Vec<NewLoadedCity> = serde_json::from_str(&cities_file_content).unwrap();
    let loaded_cities: Vec<NewCity> = loaded_cities.into_iter().map(NewCity::from).collect();
    diesel::insert_into(cities)
        .values(loaded_cities)
        .execute(&mut conn)
        .expect("Error during cities table population");

    println!("Database successfully populated!");
}
