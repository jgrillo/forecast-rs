/*Copyright 2016-2017 Jesse C. Grillo

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.*/

#![doc(html_root_url = "https://jgrillo.github.io/forecast-rs/")]

#![cfg_attr(feature = "serde_derive", feature(rustc_macro))]

//! This module exposes an SDK for interacting with the [Dark Sky
//! API](https://darksky.net/dev/docs/).
//!
//! # Overview
//!
//! The `ApiClient` is the main entrypoint. It exposes two methods for
//! sending HTTP requests to the Dark Sky API:
//!
//!   1. `ApiClient::get_forecast(request: ForecastRequest)` makes an
//!   HTTP request against the API and returns a deserialized response
//!   containing a weather forecast given the current weather
//!   conditions.
//!
//!   2. `ApiClient::get_time_machine(request: TimeMachineRequest)`
//!   makes a request against the API and returns a deserialized
//!   response containing weather data corresponding to the `time`
//!   parameter in the `TimeMachineRequest`.
//!
//! For your convenience, there are two builder objects
//! `ForecastRequestBuilder` and `TimeMachineRequestBuilder` which you
//! can use to construct `ForecastRequest` and `TimeMachineRequest`
//! instances.
//!
//! # Examples
//!
//! The following example builds a `ForecastRequest` and a
//! `TimeMachineRequest` and executes them against the API:
//!
//! ```
//! extern crate hyper;
//! extern crate forecast;
//!
//! use hyper::Client;
//!
//! use forecast::{ApiResponse, ApiClient, ForecastRequestBuilder,
//!                TimeMachineRequestBuilder, ExcludeBlock, ExtendBy,
//!                Lang, Units};
//!
//! const LAT: f64 = 6.66;
//! const LONG: f64 = 66.6;
//! const TIME: u64 = 666;
//!
//! fn main() {
//!     let api_key = "my_dark_sky_api_key"; // please don't actually hardcode your API key!
//!
//!     let hyper_client = Client::default();
//!     let api_client = ApiClient::new(&hyper_client);
//!
//!     let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];
//!
//!     let forecast_request = ForecastRequestBuilder::new(api_key, LAT, LONG)
//!         .exclude_block(ExcludeBlock::Hourly)
//!         .exclude_blocks(&mut blocks)
//!         .extend(ExtendBy::Hourly)
//!         .lang(Lang::Arabic)
//!         .units(Units::Imperial)
//!         .build();
//!
//!     let time_machine_request = TimeMachineRequestBuilder::new(api_key, LAT, LONG, TIME)
//!         .exclude_block(ExcludeBlock::Hourly)
//!         .exclude_blocks(&mut blocks)
//!         .lang(Lang::Arabic)
//!         .units(Units::Imperial)
//!         .build();
//!
//!     // let forecast_response = api_client.get_forecast(forecast_request).unwrap();
//!     // let time_machine_response = api_client.get_time_machine(time_machine_request).unwrap();
//! }
//! ```

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate itertools;

extern crate hyper;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use std::vec::Vec;
use std::option::Option;

use itertools::join;

use hyper::Url;
use hyper::error::{Result as ApiResult, ParseError};
use hyper::client::{Client, Response, IntoUrl};

// constants

static FORECAST_URL: &'static str = "https://api.darksky.net/forecast";
static EXCLUDE: &'static str = "exclude";
static EXTEND: &'static str = "extend";
static LANG: &'static str = "lang";
static UNITS: &'static str = "units";

// api objects

/// The ApiClient is a thin wrapper around a `hyper::Client` which
/// sends requests to the Forecast and Time Machine APIs.
#[derive(Debug)]
pub struct ApiClient<'a> {
    client: &'a Client
}

impl<'a> ApiClient<'a> {
    /// Construct a new ApiClient.
    pub fn new(client: &'a Client) -> ApiClient<'a> {
        ApiClient {
            client: client
        }
    }

    /// Send a [Forecast API](https://darksky.net/dev/docs/forecast)
    /// request, returns the corresponding Response.
    ///
    /// # Errors
    ///
    /// This function is a thin wrapper around
    /// `hyper::Client.get(..)`, so it will return an error under the
    /// same conditions in which hyper would.
    pub fn get_forecast(self, request: ForecastRequest) -> ApiResult<Response> {
        self.client.get(request).send()
    }

    /// Send a [Time Machine
    /// API](https://darksky.net/dev/docs/time-machine) request,
    /// returns the corresponding Response.
    ///
    /// # Errors
    ///
    /// This function is a thin wrapper around
    /// `hyper::Client.get(..)`, so it will return an error under the
    /// same conditions in which hyper would.
    pub fn get_time_machine(self, request: TimeMachineRequest) -> ApiResult<Response> {
        self.client.get(request).send()
    }
}

// request model objects and their builders

/// Model object representing a request to the Forecast API.
#[derive(PartialEq, Debug)]
pub struct ForecastRequest<'a> {
    api_key: &'a str,
    latitude: f64,
    longitude: f64,
    url: Url,
    exclude: Vec<ExcludeBlock>,
    extend: Option<ExtendBy>,
    lang: Option<Lang>,
    units: Option<Units>
}

impl<'a> ForecastRequest<'a> {
    pub fn new(
        api_key: &'a str,
        latitude: f64,
        longitude: f64,
        url: Url,
        exclude: Vec<ExcludeBlock>,
        extend: Option<ExtendBy>,
        lang: Option<Lang>,
        units: Option<Units>
    ) -> ForecastRequest<'a> {
        ForecastRequest {
            api_key: api_key,
            latitude: latitude,
            longitude: longitude,
            url: url,
            exclude: exclude,
            extend: extend,
            lang: lang,
            units: units
        }
    }
}

impl<'a> IntoUrl for ForecastRequest<'a> {
    fn into_url(self) -> Result<Url, ParseError> {
        Result::Ok(self.url)
    }
}

/// Builder object used to construct a ForecastRequest.
#[derive(PartialEq, Debug)]
pub struct ForecastRequestBuilder<'a> {
    api_key: &'a str,
    latitude: f64,
    longitude: f64,
    exclude: Vec<ExcludeBlock>,
    extend: Option<ExtendBy>,
    lang: Option<Lang>,
    units: Option<Units>
}

impl<'a> ForecastRequestBuilder<'a> {

    /// A Forecast API request is constructed with required params
    /// `api_key`, `latitude`, and `longitude`.
    pub fn new(api_key: &'a str, latitude: f64, longitude: f64) -> ForecastRequestBuilder {
        ForecastRequestBuilder {
            api_key: api_key,
            latitude: latitude,
            longitude: longitude,
            exclude: Vec::new(),
            extend: None,
            lang: None,
            units: None
        }
    }

    /// Add a DataBlock to exclude from the response.
    pub fn exclude_block(mut self, exclude_block: ExcludeBlock) -> ForecastRequestBuilder<'a> {
        self.exclude.push(exclude_block);
        self
    }

    /// Add multiple DataBlocks to exclude from the response.
    pub fn exclude_blocks(
        mut self, exclude_blocks: &mut Vec<ExcludeBlock>
    ) -> ForecastRequestBuilder<'a> {
        self.exclude.append(exclude_blocks);
        self
    }

    /// Extend the time window of the response data from 48 hours to
    /// 168 hours.
    pub fn extend(mut self, extend: ExtendBy) -> ForecastRequestBuilder<'a> {
        self.extend = Some(extend);
        self
    }

    /// Set the language for messages in the response data.
    pub fn lang(mut self, lang: Lang) -> ForecastRequestBuilder<'a> {
        self.lang = Some(lang);
        self
    }

    /// Set the measurement units for response data.
    pub fn units(mut self, units: Units) -> ForecastRequestBuilder<'a> {
        self.units = Some(units);
        self
    }

    /// Finalize the request.
    pub fn build(self) -> ForecastRequest<'a> {
        ForecastRequest::new(
            self.api_key,
            self.latitude,
            self.longitude,
            self.build_url(),
            self.exclude,
            self.extend,
            self.lang,
            self.units
        )
    }

    fn build_url(&self) -> Url {
        let url_string = format!(
            "{base}/{key}/{lat:.16},{long:.16}",
            base=FORECAST_URL,
            key=&self.api_key,
            lat=&self.latitude,
            long=&self.longitude
        );

        let mut url = Url::parse(&url_string).unwrap();

        {
            let mut query_pairs = url.query_pairs_mut();

            if !&self.exclude.is_empty() {
                let excludes = join(
                    &self.exclude
                        .iter()
                        .map(|e| {
                            let json = serde_json::to_string(e).unwrap();
                            json.trim_matches('"').to_string()
                        })
                        .collect::<Vec<String>>(),
                    ","
                );

                query_pairs.append_pair(EXCLUDE, &excludes);
            }

            if let &Some(ref extend) = &self.extend {
                query_pairs.append_pair(
                    EXTEND,
                    serde_json::to_string(&extend).unwrap().trim_matches('"')
                );
            }

            if let &Some(ref lang) = &self.lang {
                query_pairs.append_pair(
                    LANG,
                    serde_json::to_string(&lang).unwrap().trim_matches('"')
                );
            }

            if let &Some(ref units) = &self.units {
                query_pairs.append_pair(
                    UNITS,
                    serde_json::to_string(&units).unwrap().trim_matches('"')
                );
            }
        };

        url
    }
}

/// Model object representing a request to the Time Machine API.
#[derive(PartialEq, Debug)]
pub struct TimeMachineRequest<'a> {
    api_key: &'a str,
    latitude: f64,
    longitude: f64,
    time: u64,
    url: Url,
    exclude: Vec<ExcludeBlock>,
    lang: Option<Lang>,
    units: Option<Units>
}

impl<'a> TimeMachineRequest<'a> {
    pub fn new(
        api_key: &'a str,
        latitude: f64,
        longitude: f64,
        time: u64,
        url: Url,
        exclude: Vec<ExcludeBlock>,
        lang: Option<Lang>,
        units: Option<Units>
    ) -> TimeMachineRequest<'a> {
        TimeMachineRequest {
            api_key: api_key,
            latitude: latitude,
            longitude: longitude,
            time: time,
            url: url,
            exclude: exclude,
            lang: lang,
            units: units
        }
    }
}

impl<'a> IntoUrl for TimeMachineRequest<'a> {
    fn into_url(self) -> Result<Url, ParseError> {
        Result::Ok(self.url)
    }
}

/// Builder object used to construct a TimeMachineRequest.
#[derive(PartialEq, Debug)]
pub struct TimeMachineRequestBuilder<'a> {
    api_key: &'a str,
    latitude: f64,
    longitude: f64,
    time: u64,
    exclude: Vec<ExcludeBlock>,
    lang: Option<Lang>,
    units: Option<Units>
}

impl<'a> TimeMachineRequestBuilder<'a> {

    /// A Time Machine API request is constructed with required params
    /// `api_key`, `latitude`, `longitude`, and `time`.
    pub fn new(
        api_key: &'a str, latitude: f64, longitude: f64, time: u64
    ) -> TimeMachineRequestBuilder {
        TimeMachineRequestBuilder {
            api_key: api_key,
            latitude: latitude,
            longitude: longitude,
            time: time,
            exclude: Vec::new(),
            lang: None,
            units: None
        }
    }

    /// Add a DataBlock to exclude from the response.
    pub fn exclude_block(mut self, exclude_block: ExcludeBlock) -> TimeMachineRequestBuilder<'a> {
        self.exclude.push(exclude_block);
        self
    }

    /// Add multiple DataBlocks to exclude from the response.
    pub fn exclude_blocks(
        mut self, exclude_blocks: &mut Vec<ExcludeBlock>
    ) -> TimeMachineRequestBuilder<'a> {
        self.exclude.append(exclude_blocks);
        self
    }

    /// Set the language for messages in the response data.
    pub fn lang(mut self, lang: Lang) -> TimeMachineRequestBuilder<'a> {
        self.lang = Some(lang);
        self
    }

    /// Set the measurement units for response data.
    pub fn units(mut self, units: Units) -> TimeMachineRequestBuilder<'a> {
        self.units = Some(units);
        self
    }

    /// Finalize the request.
    pub fn build(self) -> TimeMachineRequest<'a> {
        TimeMachineRequest::new(
            self.api_key,
            self.latitude,
            self.longitude,
            self.time,
            self.build_url(),
            self.exclude,
            self.lang,
            self.units
        )
    }

    fn build_url(&self) -> Url {
        let url_string = format!(
           "{base}/{key}/{lat:.16},{long:.16},{time}",
            base=FORECAST_URL,
            key=self.api_key,
            lat=self.latitude,
            long=self.longitude,
            time=self.time
        );

        let mut url = Url::parse(&url_string).unwrap();

        {
            let mut query_pairs = url.query_pairs_mut();

            if !self.exclude.is_empty() {
                let excludes = join(
                    &self.exclude
                        .iter()
                        .map(|e| {
                            let json = serde_json::to_string(e).unwrap();
                            json.trim_matches('"').to_string()
                        })
                        .collect::<Vec<String>>(),
                    ","
                );

                query_pairs.append_pair(EXCLUDE, &excludes);
            }

            if let &Some(ref lang) = &self.lang {
                query_pairs.append_pair(
                    LANG,
                    serde_json::to_string(&lang).unwrap().trim_matches('"')
                );
            }

            if let &Some(ref units) = &self.units {
                query_pairs.append_pair(
                    UNITS,
                    serde_json::to_string(&units).unwrap().trim_matches('"')
                );
            }
        }

        url
    }
}

// unit tests

#[cfg(test)]
mod tests {
    use super::{ForecastRequestBuilder, ForecastRequest, TimeMachineRequestBuilder,
                TimeMachineRequest, ExcludeBlock, Units, Lang, ExtendBy, FORECAST_URL,
                EXCLUDE, EXTEND, LANG, UNITS};

    use hyper::Url;

    use std::vec::Vec;

    // constants

    const LAT: f64 = 6.66;
    const LONG: f64 = 66.6;
    const TIME: u64 = 666;

    static API_KEY: &'static str = "some_api_key";

    // tests for request models and their builders

    #[test]
    fn test_forecast_request_builder_defaults() {
        let request = ForecastRequestBuilder::new(API_KEY, LAT, LONG).build();

        let expected_url = Url::parse(
            &format!(
                "{base}/{key}/{lat:.16},{long:.16}?",
                base=FORECAST_URL,
                key=API_KEY,
                lat=LAT,
                long=LONG
            )
        ).unwrap();

        let expected = ForecastRequest::new(
            API_KEY,
            LAT,
            LONG,
            expected_url,
            Vec::new(),
            None,
            None,
            None
        );

        assert_eq!(expected.api_key, request.api_key);
        assert_eq!(expected.latitude, request.latitude);
        assert_eq!(expected.longitude, request.longitude);
        assert_eq!(expected.exclude, request.exclude);
        assert_eq!(expected.extend, request.extend);
        assert_eq!(expected.lang, request.lang);
        assert_eq!(expected.units, request.units);
        assert_eq!(expected.url, request.url);

        assert_eq!(expected, request);
    }

    #[test]
    fn test_forecast_request_builder_simple() {
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        let request = ForecastRequestBuilder::new(API_KEY, LAT, LONG)
            .exclude_block(ExcludeBlock::Hourly)
            .exclude_blocks(&mut blocks)
            .extend(ExtendBy::Hourly)
            .lang(Lang::Arabic)
            .units(Units::Imperial)
            .build();

        let expected_url = {
            let mut url = Url::parse(
                &format!(
                    "{base}/{key}/{lat:.16},{long:.16}",
                    base=FORECAST_URL,
                    key=API_KEY,
                    lat=LAT,
                    long=LONG
                )
            ).unwrap();

            url.query_pairs_mut()
                .append_pair(EXCLUDE, "hourly,daily,alerts")
                .append_pair(EXTEND, "hourly")
                .append_pair(LANG, "ar")
                .append_pair(UNITS, "us");

            url
        };

        let expected = ForecastRequest::new(
            API_KEY,
            LAT,
            LONG,
            expected_url,
            vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            Some(ExtendBy::Hourly),
            Some(Lang::Arabic),
            Some(Units::Imperial)
        );

        assert_eq!(expected, request);
    }

    #[test]
    fn test_forecast_request_builder_complex() {
        let mut builder = ForecastRequestBuilder::new(API_KEY, LAT, LONG);
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        builder = builder.exclude_block(ExcludeBlock::Hourly);
        builder = builder.exclude_blocks(&mut blocks);
        builder = builder.extend(ExtendBy::Hourly);
        builder = builder.lang(Lang::Arabic);
        builder = builder.units(Units::Imperial);

        let expected_url = {
            let mut url = Url::parse(
                &format!(
                    "{base}/{key}/{lat:.16},{long:.16}",
                    base=FORECAST_URL,
                    key=API_KEY,
                    lat=LAT,
                    long=LONG
                )
            ).unwrap();

            url.query_pairs_mut()
                .append_pair(EXCLUDE, "hourly,daily,alerts")
                .append_pair(EXTEND, "hourly")
                .append_pair(LANG, "ar")
                .append_pair(UNITS, "us");

            url
        };

        let expected = ForecastRequest::new(
            API_KEY,
            LAT,
            LONG,
            expected_url,
            vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            Some(ExtendBy::Hourly),
            Some(Lang::Arabic),
            Some(Units::Imperial)
        );

        assert_eq!(expected, builder.build());
    }

    #[test]
    fn test_time_machine_request_builder_defaults() {
        let request = TimeMachineRequestBuilder::new(API_KEY, LAT, LONG, TIME).build();

        let expected_url = Url::parse(
            &format!(
                "{base}/{key}/{lat:.16},{long:.16},{time}?",
                base=FORECAST_URL,
                key=API_KEY,
                lat=LAT,
                long=LONG,
                time=TIME
            )
        ).unwrap();

        let expected = TimeMachineRequest::new(
            API_KEY,
            LAT,
            LONG,
            TIME,
            expected_url,
            Vec::new(),
            None,
            None
        );

        assert_eq!(expected.api_key, request.api_key);
        assert_eq!(expected.latitude, request.latitude);
        assert_eq!(expected.longitude, request.longitude);
        assert_eq!(expected.time, request.time);
        assert_eq!(expected.exclude, request.exclude);
        assert_eq!(expected.lang, request.lang);
        assert_eq!(expected.units, request.units);
        assert_eq!(expected.url, request.url);

        assert_eq!(expected, request);
    }

    #[test]
    fn test_time_machine_request_builder_simple() {
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        let request = TimeMachineRequestBuilder::new(API_KEY, LAT, LONG, TIME)
            .exclude_block(ExcludeBlock::Hourly)
            .exclude_blocks(&mut blocks)
            .lang(Lang::Arabic)
            .units(Units::Imperial)
            .build();

        let expected_url = {
            let mut url = Url::parse(
                &format!(
                    "{base}/{key}/{lat:.16},{long:.16},{time}",
                    base=FORECAST_URL,
                    key=API_KEY,
                    lat=LAT,
                    long=LONG,
                    time=TIME
                )
            ).unwrap();

            url.query_pairs_mut()
                .append_pair(EXCLUDE, "hourly,daily,alerts")
                .append_pair(LANG, "ar")
                .append_pair(UNITS, "us");

            url
        };

        let expected = TimeMachineRequest::new(
            API_KEY,
            LAT,
            LONG,
            TIME,
            expected_url,
            vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            Some(Lang::Arabic),
            Some(Units::Imperial)
        );

        assert_eq!(expected, request);
    }

    #[test]
    fn test_time_machine_request_builder_complex() {
        let mut builder = TimeMachineRequestBuilder::new(API_KEY, LAT, LONG, TIME);
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        builder = builder.exclude_block(ExcludeBlock::Hourly);
        builder = builder.exclude_blocks(&mut blocks);
        builder = builder.lang(Lang::Arabic);
        builder = builder.units(Units::Imperial);

        let expected_url = {
            let mut url = Url::parse(
                &format!(
                    "{base}/{key}/{lat:.16},{long:.16},{time}",
                    base=FORECAST_URL,
                    key=API_KEY,
                    lat=LAT,
                    long=LONG,
                    time=TIME
                )
            ).unwrap();

            url.query_pairs_mut()
                .append_pair(EXCLUDE, "hourly,daily,alerts")
                .append_pair(LANG, "ar")
                .append_pair(UNITS, "us");

            url
        };

        let expected = TimeMachineRequest::new(
            API_KEY,
            LAT,
            LONG,
            TIME,
            expected_url,
            vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            Some(Lang::Arabic),
            Some(Units::Imperial)
        );

        assert_eq!(expected, builder.build());
    }
}
