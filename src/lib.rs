/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 *
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
//! # codicefiscale
//! A library with useful functions to calculate a person's fiscal code, the italian equivalent of the social security number.
//! ## Usage
//! To use this library, a database populated with all the italian cities and all the nations is needed.\
//! To build one, download `gi_comuni.json` and `gi_nazioni.json` from [here](https://www.gardainformatica.it/database-comuni-italiani), place them in the root directory of the project and call [db_utils::populate_db]
use chrono::{Datelike, Month, NaiveDate};

use clap::ValueEnum;
use lazy_static::lazy_static;
use models::{City, Nation};
use std::collections::HashMap;

/// Functions to search and populate the database
pub mod db_utils;
/// Representations of nations and cities in the database
pub mod models;
pub(crate) mod schema;
mod utils;

lazy_static! {
    static ref ALPHABET_VEC: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    static ref ODD_LOOKUP_TABLE: HashMap<char, u32> = HashMap::from([
        ('0', 1),
        ('1', 0),
        ('2', 5),
        ('3', 7),
        ('4', 9),
        ('5', 13),
        ('6', 15),
        ('7', 17),
        ('8', 19),
        ('9', 21),
        ('A', 1),
        ('B', 0),
        ('C', 5),
        ('D', 7),
        ('E', 9),
        ('F', 13),
        ('G', 15),
        ('H', 17),
        ('I', 19),
        ('J', 21),
        ('K', 2),
        ('L', 4),
        ('M', 18),
        ('N', 20),
        ('O', 11),
        ('P', 3),
        ('Q', 6),
        ('R', 8),
        ('S', 12),
        ('T', 14),
        ('U', 16),
        ('V', 10),
        ('W', 22),
        ('X', 25),
        ('Y', 24),
        ('Z', 23),
    ]);
    static ref HOMOCODIC_LOOKUP_TABLE: HashMap<u32, char> = HashMap::from([
        (0, 'L'),
        (1, 'M'),
        (2, 'N'),
        (3, 'P'),
        (4, 'Q'),
        (5, 'R'),
        (6, 'S'),
        (7, 'T'),
        (8, 'U'),
        (9, 'V'),
    ]);
}

/// Generate the code with the given data
pub fn generate_code(
    name: String,
    surname: String,
    sex: Sex,
    birth_nation: Nation,
    birth_city: City,
    birth_date: NaiveDate,
) -> String {
    let name_code = extract_name_letters(&name);
    let surname_code = extract_surname_letters(&surname);
    let year_code = get_year(&birth_date.year().to_string());
    let born_month = Month::try_from(birth_date.month() as u8).unwrap();
    let month_code = get_month_letter(&born_month).to_string();
    let day_code = get_day(birth_date.day(), sex);
    let location_code = if birth_nation.nation_code == "0000" {
        birth_city.city_code
    } else {
        birth_nation.nation_code
    };
    let preliminary_code =
        format!("{surname_code}{name_code}{year_code}{month_code}{day_code:0>2}{location_code}");
    let check_code = get_control_character(&preliminary_code);
    format!("{preliminary_code}{check_code}")
}

/// Generate the homocodic version of the code, in case of homonymy
pub fn generate_homocodic_from_code(code: &str, substitution_depth: u32) -> String {
    let preliminary_code: String = code
        .char_indices()
        .filter(|c| c.0 != 15)
        .map(|c| c.1)
        .collect();
    let preliminary_code =
        generate_homocodic_preliminary_code(&preliminary_code, substitution_depth);
    let check_code = get_control_character(&preliminary_code);
    format!("{preliminary_code}{check_code}")
}

fn generate_homocodic_preliminary_code(preliminary_code: &str, substitution_depth: u32) -> String {
    if substitution_depth == 0 {
        return preliminary_code.to_string();
    }
    let mut substitution_depth = substitution_depth;
    let mut new_preliminary_code: Vec<char> = vec![];
    for c in preliminary_code.chars().rev() {
        let new_c = if c.is_ascii_digit() && substitution_depth != 0 {
            substitution_depth -= 1;
            let c = c.to_digit(10).unwrap();
            *HOMOCODIC_LOOKUP_TABLE.get(&c).unwrap()
        } else {
            c
        };
        new_preliminary_code.push(new_c);
    }
    new_preliminary_code.into_iter().rev().collect()
}

/// Represent a person sex
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Sex {
    /// Male
    #[value(alias("M"))]
    M,
    /// Female
    #[value(alias("F"))]
    F,
}

fn get_month_letter(month: &Month) -> char {
    const MONTH_LETTERS: [char; 12] = ['A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T'];
    let month_num = month.number_from_month() as usize;
    MONTH_LETTERS[month_num - 1]
}

fn get_year(year: &str) -> String {
    let year_char: Vec<_> = year.chars().collect();
    assert_eq!(year_char.len(), 4);
    year_char[2..4].iter().collect()
}

fn get_day(day: u32, sex: Sex) -> u32 {
    match sex {
        Sex::M => day,
        Sex::F => day + 40,
    }
}

fn extract_surname_letters(name: &str) -> String {
    let mut name_consonants: Vec<char> = name.chars().filter(utils::is_consonant).collect();
    let mut name_vowels: Vec<char> = name.chars().filter(utils::is_vowel).collect();
    let mut name_code = vec![];
    name_code.append(&mut name_consonants);
    name_code.append(&mut name_vowels);
    let name_code: Vec<_> = name_code.chunks(3).collect();
    let name_code = name_code.first().unwrap();
    let name_code: String = name_code.to_vec().iter().collect();
    let name_code = format!("{:X<3}", name_code);
    name_code.to_ascii_uppercase()
}

fn extract_name_letters(name: &str) -> String {
    let mut name_consonants: Vec<char> = name.chars().filter(utils::is_consonant).collect();
    let mut name_vowels: Vec<char> = name.chars().filter(utils::is_vowel).collect();
    if name_consonants.len() <= 3 {
        let mut name_code = vec![];
        name_code.append(&mut name_consonants);
        name_code.append(&mut name_vowels);
        let name_code: Vec<_> = name_code.chunks(3).collect();
        let name_code = name_code.first().unwrap();
        let name_code: String = name_code.to_vec().iter().collect();
        let name_code = format!("{:X<3}", name_code);
        name_code.to_ascii_uppercase()
    } else {
        let name_code = [name_consonants[0], name_consonants[2], name_consonants[3]];
        let name_code: String = name_code.iter().collect();
        name_code.to_ascii_uppercase()
    }
}

fn get_control_character(preliminary_code: &str) -> char {
    let even_characters: Vec<char> = preliminary_code
        .char_indices()
        .filter(|c| c.0 % 2 == 1)
        .map(|c| c.1)
        .collect();
    let even_sum: u32 = even_characters_lookup(even_characters).iter().sum();
    let odd_characters: Vec<char> = preliminary_code
        .char_indices()
        .filter(|c| c.0 % 2 == 0)
        .map(|c| c.1)
        .collect();
    let odd_sum: u32 = odd_characters_lookup(odd_characters).iter().sum();
    let code_sum: u32 = (even_sum + odd_sum) % 26;
    ALPHABET_VEC[code_sum as usize]
}

fn even_characters_lookup(chars: Vec<char>) -> Vec<u32> {
    chars
        .into_iter()
        .map(|c| c.to_ascii_uppercase())
        .map(|c| {
            if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else {
                (ALPHABET_VEC.binary_search(&c).unwrap()) as u32
            }
        })
        .collect()
}

fn odd_characters_lookup(chars: Vec<char>) -> Vec<u32> {
    chars
        .into_iter()
        .map(|c| c.to_ascii_uppercase())
        .map(|c| *ODD_LOOKUP_TABLE.get(&c).unwrap())
        .collect()
}
