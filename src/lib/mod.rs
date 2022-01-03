use std::error::Error;
use csv::Writer;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
struct Student {
    #[serde(rename = "Broj indeksa")]
    index: String,
    #[serde(rename = "Ime i prezime")]
    name: String,
    #[serde(rename = "Broj poena")]
    points: Option<u32>,
    #[serde(rename = "Komentar")]
    comment: Option<String>,
}

const CSV_EXTENSION: &str = ".csv";

pub mod extract;
pub mod list;
pub mod table;
