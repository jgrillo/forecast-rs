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

/// Model object representing an icon for display.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Icon {
    #[serde(rename = "clear-day")]
    ClearDay,

    #[serde(rename = "clear-night")]
    ClearNight,

    #[serde(rename = "rain")]
    Rain,

    #[serde(rename = "snow")]
    Snow,

    #[serde(rename = "sleet")]
    Sleet,

    #[serde(rename = "wind")]
    Wind,

    #[serde(rename = "fog")]
    Fog,

    #[serde(rename = "cloudy")]
    Cloudy,

    #[serde(rename = "partly-cloudy-day")]
    PartlyCloudyDay,

    #[serde(rename = "partly-cloudy-night")]
    PartlyCloudyNight,

    #[serde(rename = "hail")]
    Hail,

    #[serde(rename = "thunderstorm")]
    Thunderstorm,

    #[serde(rename = "tornado")]
    Tornado
}

/// Model object representing the kind of precipitation ocurring at a particular
/// time.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum PrecipType {
    #[serde(rename = "rain")]
    Rain,

    #[serde(rename = "snow")]
    Snow,

    #[serde(rename = "sleet")]
    Sleet
}

/// Model object representing a DataBlock to exclude from the response.
#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum ExcludeBlock {
    #[serde(rename = "currently")]
    Currently,

    #[serde(rename = "minutely")]
    Minutely,

    #[serde(rename = "hourly")]
    Hourly,

    #[serde(rename = "daily")]
    Daily,

    #[serde(rename = "alerts")]
    Alerts,

    #[serde(rename = "flags")]
    Flags
}

/// When present in a request, this feature causes response data to be reported
/// for 168 hours into the future instead of 48 hours.
#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum ExtendBy {
    #[serde(rename = "hourly")]
    Hourly
}

/// Model object representing language.
#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum Lang {
    #[serde(rename = "ar")]
    Arabic,

    #[serde(rename = "az")]
    Azerbaijani,

    #[serde(rename = "be")]
    Belarusian,

    #[serde(rename = "bs")]
    Bosnian,

    #[serde(rename = "cz")]
    Czech,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "el")]
    Greek,

    #[serde(rename = "en")]
    English,

    #[serde(rename = "es")]
    Spanish,

    #[serde(rename = "fr")]
    French,

    #[serde(rename = "hr")]
    Croatian,

    #[serde(rename = "hu")]
    Hungarian,

    #[serde(rename = "id")]
    Indonesian,

    #[serde(rename = "it")]
    Italian,

    #[serde(rename = "is")]
    Icelandic,

    #[serde(rename = "kw")]
    Cornish,

    #[serde(rename = "nb")]
    NorwegianBokmal,

    #[serde(rename = "nl")]
    Dutch,

    #[serde(rename = "pl")]
    Polish,

    #[serde(rename = "pt")]
    Portugese,

    #[serde(rename = "ru")]
    Russian,

    #[serde(rename = "sk")]
    Slovak,

    #[serde(rename = "sr")]
    Serbian,

    #[serde(rename = "sv")]
    Swedish,

    #[serde(rename = "tet")]
    Tetum,

    #[serde(rename = "tr")]
    Turkish,

    #[serde(rename = "uk")]
    Ukranian,

    #[serde(rename = "x-pig-latin")]
    IgpayAtinlay,

    #[serde(rename = "zh")]
    SimplifiedChinese,

    #[serde(rename = "zh-tw")]
    TraditionalChinese
}

/// Model object representing measurement units.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Units {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "ca")]
    CA,

    #[serde(rename = "uk2")]
    UK,

    #[serde(rename = "us")]
    Imperial,

    #[serde(rename = "si")]
    SI
}

/// Model object containing various properties, each representing the average
/// (unless otherwise specified) of a particular weather phenomenon ocurring
/// during a period of time.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataPoint {
    #[serde(rename = "apparentTemperature")]
    pub apparent_temperature: Option<f64>,

    #[serde(rename = "apparentTemperatureMax")]
    pub apparent_temperature_max: Option<f64>,

    #[serde(rename = "apparentTemperatureMaxTime")]
    pub apparent_temperature_max_time: Option<u64>,

    #[serde(rename = "apparentTemperatureMin")]
    pub apparent_temperature_min: Option<f64>,

    #[serde(rename = "apparentTemperatureMinTime")]
    pub apparent_temperature_min_time: Option<u64>,

    #[serde(rename = "cloudCover")]
    pub cloud_cover: Option<f64>,

    #[serde(rename = "dewPoint")]
    pub dew_point: Option<f64>,

    pub humidity: Option<f64>,

    pub icon: Option<Icon>,

    #[serde(rename = "moonPhase")]
    pub moon_phase: Option<f64>,

    #[serde(rename = "nearestStormBearing")]
    pub nearest_storm_bearing: Option<f64>,

    #[serde(rename = "nearestStormDistance")]
    pub nearest_storm_distance: Option<f64>,

    pub ozone: Option<f64>,

    #[serde(rename = "precipAccumulation")]
    pub precip_accumulation: Option<f64>,

    #[serde(rename = "precipIntensity")]
    pub precip_intensity: Option<f64>,

    #[serde(rename = "precipIntensityMax")]
    pub precip_intensity_max: Option<f64>,

    #[serde(rename = "precipIntensityMaxTime")]
    pub precip_intensity_max_time: Option<u64>,

    #[serde(rename = "precipProbability")]
    pub precip_probability: Option<f64>,

    #[serde(rename = "precipType")]
    pub precip_type: Option<PrecipType>,

    pub pressure: Option<f64>,

    pub summary: Option<String>,

    #[serde(rename = "sunriseTime")]
    pub sunrise_time: Option<u64>,

    #[serde(rename = "sunsetTime")]
    pub sunset_time: Option<u64>,

    pub temperature: Option<f64>,

    #[serde(rename = "temperatureMax")]
    pub temperature_max: Option<f64>,

    #[serde(rename = "temperatureMaxTime")]
    pub temperature_max_time: Option<u64>,

    #[serde(rename = "temperatureMin")]
    pub temperature_min: Option<f64>,

    #[serde(rename = "temperatureMinTime")]
    pub temperature_min_time: Option<u64>,

    pub time: u64,

    pub visibility: Option<f64>,

    #[serde(rename = "windBearing")]
    pub wind_bearing: Option<f64>,

    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<f64>
}

/// Model object representing the various weather phenomena ocurring over a
/// period of time.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataBlock {
    pub data: Vec<DataPoint>,

    pub summary: Option<String>,

    pub icon: Option<Icon>
}

/// Model object representing a severe weather warning issued by a government
/// authority for the requested location.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Alert {
    pub description: String,

    pub expires: u64,

    pub title: String,

    pub uri: String,
}

/// Model object representing a flag which contains miscellaneous metadata about
/// a request.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Flags {
    #[serde(rename = "darksky-unavailable")]
    pub darksky_unavailable: Option<String>,

    #[serde(rename = "metno-license")]
    pub metno_license: Option<String>,

    pub sources: Vec<String>,

    pub units: Units
}

/// Model object representing a Forecast or Time Machine API response.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ApiResponse {
    pub latitude: f64,

    pub longitude: f64,

    pub timezone: String,

    pub offset: Option<u64>,

    pub currently: Option<DataPoint>,

    pub minutely: Option<DataBlock>,

    pub hourly: Option<DataBlock>,

    pub daily: Option<DataBlock>,

    pub alerts: Option<Vec<Alert>>,

    pub flags: Option<Flags>
}
