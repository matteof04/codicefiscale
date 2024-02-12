/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 * 
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
use chrono::NaiveDate;

use clap::{Args, Parser, Subcommand};
use codicefiscale::Sex;

use crate::parse_birth_date;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    ///Generate the code
    Generate(GenerateArgs),
    ///Build the nations and city database
    BuildDatabase,
    ///Build autocomplete scripts for all the shells supported and save them into the complete folder
    BuildComplete,
}

#[derive(Args)]
pub(crate) struct GenerateArgs {
    ///Name
    pub(crate) name: String,
    ///Surname
    pub(crate) surname: String,
    ///Sex
    #[arg(value_enum)]
    pub(crate) sex: Sex,
    ///Birth nation
    pub(crate) nation: String,
    ///Birth city (irrelevant if nation is different from italy, but still needed)
    pub(crate) city: String,
    ///Birth date in format YYYY-MM-DD
    #[arg(value_parser = parse_birth_date)]
    pub(crate) birth_date: NaiveDate,
    ///Substitution depth for homocodic code
    pub(crate) substitution_depth: Option<u32>,
}
