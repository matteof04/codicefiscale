/*
 * Copyright (c) 2024 Matteo Franceschini
 * All rights reserved.
 * 
 * Use of this source code is governed by BSD-3-Clause-Clear
 * license that can be found in the LICENSE file
 */
use diesel::prelude::*;
use serde::Deserialize;

/// Represents a city in the database
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct City {
    pub id: i32,
    pub city_name: String,
    pub city_code: String,
}

/// Represents a nation in the database
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Nation {
    pub id: i32,
    pub nation_name: String,
    pub nation_code: String,
}

/// Represents a city to add to the database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::cities)]
pub struct NewCity {
    pub city_name: String,
    pub city_code: String,
}

impl From<NewLoadedCity> for NewCity {
    fn from(value: NewLoadedCity) -> Self {
        NewCity {
            city_name: value.city_name,
            city_code: value.city_code,
        }
    }
}

/// Represents a nation to add to the database
#[derive(Insertable)]
#[diesel(table_name = crate::schema::nations)]
pub struct NewNation {
    pub nation_name: String,
    pub nation_code: String,
}

impl From<NewLoadedNation> for NewNation {
    fn from(value: NewLoadedNation) -> Self {
        NewNation {
            nation_name: value.nation_name,
            nation_code: value.nation_code,
        }
    }
}

/// Represents a city as in the json file
#[derive(Deserialize)]
pub struct NewLoadedCity {
    #[serde(rename = "sigla_provincia")]
    pub province_initials: String,
    #[serde(rename = "codice_istat")]
    pub istat_code: String,
    #[serde(rename = "denominazione_ita_altra")]
    pub mixed_city_name: String,
    #[serde(rename = "denominazione_ita")]
    pub city_name: String,
    #[serde(rename = "denominazione_altra")]
    pub alternative_city_name: String,
    #[serde(rename = "flag_capoluogo")]
    pub is_province: String,
    #[serde(rename = "codice_belfiore")]
    pub city_code: String,
    pub lat: String,
    pub lon: String,
    #[serde(rename = "superficie_kmq")]
    pub surface: String,
    #[serde(rename = "codice_sovracomunale")]
    pub overmunicipal_code: String,
}

/// Represents a nation as in the json file
#[derive(Deserialize)]
pub struct NewLoadedNation {
    #[serde(rename = "sigla_nazione")]
    pub nation_initials: String,
    #[serde(rename = "codice_belfiore")]
    pub nation_code: String,
    #[serde(rename = "denominazione_nazione")]
    pub nation_name: String,
    #[serde(rename = "denominazione_cittadinanza")]
    pub citizen_name: String,
}
