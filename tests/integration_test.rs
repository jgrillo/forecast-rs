/*Copyright 2016 Jesse C. Grillo

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.*/

extern crate serde;
extern crate serde_json;

extern crate hyper;

extern crate forecast;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use hyper::Client;

use forecast::{ApiResponse, ApiClient, ForecastRequestBuilder,
               TimeMachineRequestBuilder, ExcludeBlock, ExtendBy, 
               Lang, Units};

// constants

const LAT: f64 = 6.66;
const LONG: f64 = 66.6;
const TIME: u64 = 666;

// tests for serde models

#[test]
fn test_response_serde() {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("resources/tests/forecast_response_10-23-2016.json");

    let path = path_buf.as_path();

    let file = match File::open(&path) {
        Err(reason) => panic!("couldn't open {}: {}", path.display(), reason.description()),
        Ok(file) => file
    };

    let deserialized_response: ApiResponse = serde_json::from_reader(file).unwrap();

    let serialized_response: String = serde_json::to_string(&deserialized_response).unwrap();

    let deserialized_again: ApiResponse = serde_json::from_str(&serialized_response).unwrap();

    assert_eq!(deserialized_response, deserialized_again);
}

// tests which perform network calls
// To execute these tests, run the following command in the project root:
//
// FORECAST_API_KEY=$YOUR_FORECAST_API_KEY cargo test --features integration

#[test]
#[cfg(feature = "integration")]
fn test_get_forecast_request_default() {
    let api_key = env!("FORECAST_API_KEY");

    let hyper_client = Client::default();
    let api_client = ApiClient::new(&hyper_client);

    let forecast_request = ForecastRequestBuilder::new(api_key, LAT, LONG).build();

    let response = api_client.get_forecast(forecast_request).unwrap();

    assert_eq!(response.status, hyper::Ok);
}

#[test]
#[cfg(feature = "integration")]
fn test_get_forecast_request_full() {
    let api_key = env!("FORECAST_API_KEY");

    let hyper_client = Client::default();
    let api_client = ApiClient::new(&hyper_client);

    let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

    let forecast_request = ForecastRequestBuilder::new(api_key, LAT, LONG)
        .exclude_block(ExcludeBlock::Hourly)
        .exclude_blocks(&mut blocks)
        .extend(ExtendBy::Hourly)
        .lang(Lang::Arabic)
        .units(Units::Imperial)
        .build();

    let response = api_client.get_forecast(forecast_request).unwrap();

    assert_eq!(response.status, hyper::Ok);
}

#[test]
#[cfg(feature = "integration")]
fn test_get_time_machine_request_default() {
    let api_key = env!("FORECAST_API_KEY");

    let hyper_client = Client::default();
    let api_client = ApiClient::new(&hyper_client);

    let time_machine_request = TimeMachineRequestBuilder::new(api_key, LAT, LONG, TIME)
        .build();

    let response = api_client.get_time_machine(time_machine_request).unwrap();

    assert_eq!(response.status, hyper::Ok);
}

#[test]
#[cfg(feature = "integration")]
fn test_get_time_machine_request_full() {
    let api_key = env!("FORECAST_API_KEY");

    let hyper_client = Client::default();
    let api_client = ApiClient::new(&hyper_client);

    let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

    let time_machine_request = TimeMachineRequestBuilder::new(api_key, LAT, LONG, TIME)
        .exclude_block(ExcludeBlock::Hourly)
        .exclude_blocks(&mut blocks)
        .lang(Lang::Arabic)
        .units(Units::Imperial)
        .build();

    let response = api_client.get_time_machine(time_machine_request).unwrap();

    assert_eq!(response.status, hyper::Ok);
}