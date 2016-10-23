#![cfg_attr(feature = "serde_derive", feature(rustc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate chrono;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use std::vec::Vec;
use std::option::Option;
use chrono::{DateTime, TimeZone};

// request model objects and their builders

#[derive(PartialEq, Debug)]
pub struct ForecastRequest<'a> {
    api_key: &'a str,
    latitude: f64,
    longitude: f64,
    exclude: Vec<ExcludeBlock>,
    extend: Option<ExtendBy>,
    lang: Option<Lang>,
    units: Option<Units>
}

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

    pub fn exclude_block(mut self, exclude_block: ExcludeBlock) -> ForecastRequestBuilder<'a> {
        self.exclude.push(exclude_block);
        self
    }

    pub fn exclude_blocks(
        mut self, exclude_blocks: &mut Vec<ExcludeBlock>
    ) -> ForecastRequestBuilder<'a> {
        self.exclude.append(exclude_blocks);
        self
    }

    pub fn extend(mut self, extend: ExtendBy) -> ForecastRequestBuilder<'a> {
        self.extend = Some(extend);
        self
    }

    pub fn lang(mut self, lang: Lang) -> ForecastRequestBuilder<'a> {
        self.lang = Some(lang);
        self
    }

    pub fn units(mut self, units: Units) -> ForecastRequestBuilder<'a> {
        self.units = Some(units);
        self
    }

    pub fn build(self) -> ForecastRequest<'a> {
        ForecastRequest {
            api_key: self.api_key,
            latitude: self.latitude,
            longitude: self.longitude,
            exclude: self.exclude,
            extend: self.extend,
            lang: self.lang,
            units: self.units
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct TimeMachineRequest<'a> {
    api_key: &'a str,
    latitude: f64,
    longitude: f64,
    time: u64,
    exclude: Vec<ExcludeBlock>,
    lang: Option<Lang>,
    units: Option<Units>
}

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

    pub fn exclude_block(mut self, exclude_block: ExcludeBlock) -> TimeMachineRequestBuilder<'a> {
        self.exclude.push(exclude_block);
        self
    }

    pub fn exclude_blocks(
        mut self, exclude_blocks: &mut Vec<ExcludeBlock>
    ) -> TimeMachineRequestBuilder<'a> {
        self.exclude.append(exclude_blocks);
        self
    }

    pub fn lang(mut self, lang: Lang) -> TimeMachineRequestBuilder<'a> {
        self.lang = Some(lang);
        self
    }

    pub fn units(mut self, units: Units) -> TimeMachineRequestBuilder<'a> {
        self.units = Some(units);
        self
    }

    pub fn build(self) -> TimeMachineRequest<'a> {
        TimeMachineRequest {
            api_key: self.api_key,
            latitude: self.latitude,
            longitude: self.longitude,
            time: self.time,
            exclude: self.exclude,
            lang: self.lang,
            units: self.units
        }
    }
}

fn get_forecast<T: TimeZone>(
    request: ForecastRequest
) -> Option<Response<T>> where DateTime<T>: Serialize + Deserialize {
    return None;
}

#[cfg(test)]
mod tests {
    use super::{ForecastRequestBuilder, ForecastRequest, TimeMachineRequestBuilder,
                TimeMachineRequest, Icon, PrecipType, ExcludeBlock, ExtendBy, Lang, Units,
                DataPoint, DataBlock, Alert, Flag, Response};

    use std::vec::Vec;

    // tests for serde models

    #[test]
    fn test_response_serde() {

    }

    // tests for request models and their builders

    #[test]
    fn test_forecast_request_builder_defaults() {
        let request = ForecastRequestBuilder::new("some_api_key", 6.66, 66.6).build();
        let expected = ForecastRequest {
            api_key: "some_api_key",
            latitude: 6.66,
            longitude: 66.6,
            exclude: Vec::new(),
            extend: None,
            lang: None,
            units: None
        };

        assert_eq!(expected.api_key, request.api_key);
        assert_eq!(expected.latitude, request.latitude);
        assert_eq!(expected.longitude, request.longitude);
        assert_eq!(expected.exclude, request.exclude);
        assert_eq!(expected.extend, request.extend);
        assert_eq!(expected.lang, request.lang);
        assert_eq!(expected.units, request.units);

        assert_eq!(expected, request);
    }

    #[test]
    fn test_forecast_request_builder_simple() {
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        let request = ForecastRequestBuilder::new("some_api_key", 6.66, 66.6)
            .exclude_block(ExcludeBlock::Hourly)
            .exclude_blocks(&mut blocks)
            .extend(ExtendBy::Hourly)
            .lang(Lang::Arabic)
            .units(Units::Imperial)
            .build();

        let expected = ForecastRequest {
            api_key: "some_api_key",
            latitude: 6.66,
            longitude: 66.6,
            exclude: vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            extend: Some(ExtendBy::Hourly),
            lang: Some(Lang::Arabic),
            units: Some(Units::Imperial)
        };

        assert_eq!(expected, request);
    }

    #[test]
    fn test_forecast_request_builder_complex() {
        let mut builder = ForecastRequestBuilder::new("some_api_key", 6.66, 66.6);
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        builder = builder.exclude_block(ExcludeBlock::Hourly);
        builder = builder.exclude_blocks(&mut blocks);
        builder = builder.extend(ExtendBy::Hourly);
        builder = builder.lang(Lang::Arabic);
        builder = builder.units(Units::Imperial);

        let expected = ForecastRequest {
            api_key: "some_api_key",
            latitude: 6.66,
            longitude: 66.6,
            exclude: vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            extend: Some(ExtendBy::Hourly),
            lang: Some(Lang::Arabic),
            units: Some(Units::Imperial)
        };

        assert_eq!(expected, builder.build());
    }

    #[test]
    fn test_time_machine_request_builder_defaults() {
        let request = TimeMachineRequestBuilder::new("some_api_key", 6.66, 66.6, 666).build();
        let expected = TimeMachineRequest {
            api_key: "some_api_key",
            latitude: 6.66,
            longitude: 66.6,
            time: 666,
            exclude: Vec::new(),
            lang: None,
            units: None
        };

        assert_eq!(expected.api_key, request.api_key);
        assert_eq!(expected.latitude, request.latitude);
        assert_eq!(expected.longitude, request.longitude);
        assert_eq!(expected.time, request.time);
        assert_eq!(expected.exclude, request.exclude);
        assert_eq!(expected.lang, request.lang);
        assert_eq!(expected.units, request.units);

        assert_eq!(expected, request);
    }

    #[test]
    fn test_time_machine_request_builder_simple() {
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        let request = TimeMachineRequestBuilder::new("some_api_key", 6.66, 66.6, 666)
            .exclude_block(ExcludeBlock::Hourly)
            .exclude_blocks(&mut blocks)
            .lang(Lang::Arabic)
            .units(Units::Imperial)
            .build();

        let expected = TimeMachineRequest {
            api_key: "some_api_key",
            latitude: 6.66,
            longitude: 66.6,
            time: 666,
            exclude: vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            lang: Some(Lang::Arabic),
            units: Some(Units::Imperial)
        };

        assert_eq!(expected, request);
    }

    #[test]
    fn test_time_machine_request_builder_complex() {
        let mut builder = TimeMachineRequestBuilder::new("some_api_key", 6.66, 66.6, 666);
        let mut blocks = vec![ExcludeBlock::Daily, ExcludeBlock::Alerts];

        builder = builder.exclude_block(ExcludeBlock::Hourly);
        builder = builder.exclude_blocks(&mut blocks);
        builder = builder.lang(Lang::Arabic);
        builder = builder.units(Units::Imperial);

        let expected = TimeMachineRequest {
            api_key: "some_api_key",
            latitude: 6.66,
            longitude: 66.6,
            time: 666,
            exclude: vec![ExcludeBlock::Hourly, ExcludeBlock::Daily, ExcludeBlock::Alerts],
            lang: Some(Lang::Arabic),
            units: Some(Units::Imperial)
        };

        assert_eq!(expected, builder.build());
    }
}
