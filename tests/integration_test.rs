extern crate serde;
extern crate serde_json;

extern crate chrono;

extern crate forecast;

use std::error::Error;
use std::fs::File;
use std::io::prelude::{BufRead, Read, Seek, Write};
use std::path::{PathBuf, Path};

use chrono::FixedOffset;

use forecast::Response;

// tests for serde models

#[test]
fn test_response_serde() {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("resources/tests/forecast_response_10-23-2016.json");

    let path = path_buf.as_path();

    let mut file = match File::open(&path) {
        Err(reason) => panic!("couldn't open {}: {}", path.display(), reason.description()),
        Ok(file) => file
    };

    let tz = FixedOffset::east(60 * 60 * 4);
    let deser_response: Response<FixedOffset> = serde_json::from_reader(file).unwrap();
}
