/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 * 
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
use std::{
    env,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

use chrono::NaiveDate;
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::Shell;
use codicefiscale::{
    db_utils::{check_db_not_empty, establish_connection, populate_db, search_city, search_nation},
    generate_code, generate_homocodic_from_code,
    models::{City, Nation},
};

mod cli;

fn main() {
    let args: String = env::args().collect();
    assert!(args.is_ascii(), "Data must be in ASCII format!");
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Generate(args) => generate(args),
        cli::Commands::BuildDatabase => populate_db(),
        cli::Commands::BuildComplete => build_complete_file(),
    }
}

fn generate(args: cli::GenerateArgs) {
    if let Err(e) = check_db_not_empty() {
        eprintln!("{e}");
        return;
    }
    let mut conn = establish_connection();
    let city: Vec<City> = search_city(&mut conn, &args.city);
    let nation: Vec<Nation> = search_nation(&mut conn, &args.nation);
    let city = city.into_iter().next().expect("City not found");
    let nation = nation.into_iter().next().expect("Nation not found");
    let code = generate_code(
        args.name,
        args.surname,
        args.sex,
        nation,
        city,
        args.birth_date,
    );
    println!("Code: {code}");
    if let Some(substitution_depth) = args.substitution_depth {
        let homocodic_code = generate_homocodic_from_code(&code, substitution_depth);
        println!("Homocodic code: {homocodic_code}");
    }
}

fn parse_birth_date(birth_date: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(birth_date, "%F")
        .map_err(|_| "Invalid date format, should be YYYY-MM-DD.".to_string())
}

fn build_complete_file() {
    const BIN_NAME: &str = env!("CARGO_BIN_NAME");
    let base_dir = Path::new("complete");
    if !base_dir.exists() || !base_dir.is_dir() {
        create_dir(base_dir).unwrap_or_else(|_| panic!("Can't create the complete directory!"));
    }
    for shell in Shell::value_variants() {
        let file_name = format!(
            "{}/{BIN_NAME}.{shell}",
            base_dir.file_name().unwrap().to_str().unwrap()
        );
        let file_path = Path::new(&file_name);
        let mut file = File::create(file_path)
            .unwrap_or_else(|_| panic!("Can't create the complete file {file_name}!"));
        clap_complete::generate(
            shell.to_owned(),
            &mut cli::Cli::command(),
            BIN_NAME,
            &mut file,
        );
        println!("Generated complete file of {BIN_NAME} for {shell}");
    }
    let load_script = include_str!("../load_script_template");
    let load_script = load_script.replace(
        "COMPLETE_DIR",
        base_dir.file_name().unwrap().to_str().unwrap(),
    );
    let load_script = load_script.replace("BIN_NAME", BIN_NAME);
    let mut file = File::create("load").expect("Can't create the load file!");
    file.write_all(load_script.as_bytes())
        .expect("Can't write the load script!");
    println!("Generated load script");
}
