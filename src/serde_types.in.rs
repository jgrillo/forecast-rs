use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum PrecipType {
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "sleet")]
    Sleet
}

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

#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum ExtendBy {
    #[serde(rename = "hourly")]
    Hourly
}

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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataPoint<T: TimeZone> where DateTime<T>: Serialize + Deserialize {
    #[serde(rename = "apparentTemperature")]
    pub apparent_temperature: Option<f64>,
    #[serde(rename = "apparentTemperatureMax")]
    pub apparent_temperature_max: Option<f64>,
    #[serde(rename = "apparentTemperatureMaxTime")]
    pub apparent_temperature_max_time: Option<DateTime<T>>,
    #[serde(rename = "apparentTemperatureMin")]
    pub apparent_temperature_min: Option<f64>,
    #[serde(rename = "apparentTemperatureMinTime")]
    pub apparent_temperature_min_time: Option<DateTime<T>>,
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
    pub precip_intensity_max_time: Option<DateTime<T>>,
    #[serde(rename = "precipProbability")]
    pub precip_probability: Option<f64>,
    #[serde(rename = "precipType")]
    pub precip_type: Option<PrecipType>,
    pub pressure: Option<f64>,
    pub summary: Option<String>,
    #[serde(rename = "sunriseTime")]
    pub sunrise_time: Option<DateTime<T>>,
    #[serde(rename = "sunsetTime")]
    pub sunset_time: Option<DateTime<T>>,
    pub temperature: Option<f64>,
    #[serde(rename = "temperatureMax")]
    pub temperature_max: Option<f64>,
    #[serde(rename = "temperatureMaxTime")]
    pub temperature_max_time: Option<DateTime<T>>,
    #[serde(rename = "temperatureMin")]
    pub temperature_min: Option<f64>,
    #[serde(rename = "temperatureMinTime")]
    pub temperature_min_time: Option<DateTime<T>>,
    pub time: Option<DateTime<T>>,
    pub visibility: Option<f64>,
    #[serde(rename = "windBearing")]
    pub wind_bearing: Option<f64>,
    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<f64>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DataBlock<T: TimeZone> where DateTime<T>: Serialize + Deserialize {
    pub data: Vec<DataPoint<T>>,
    pub summary: Option<String>,
    pub icon: Option<Icon>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Alert<T: TimeZone> where DateTime<T>: Serialize + Deserialize {
    pub description: String,
    pub expires: DateTime<T>,
    pub title: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Flag {
    #[serde(rename = "darksky-unavailable")]
    pub darksky_unavailable: Option<String>,
    #[serde(rename = "metno-license")]
    pub metno_license: Option<String>,
    pub sources: Vec<String>,
    pub units: Units
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Response<T: TimeZone> where DateTime<T>: Serialize + Deserialize {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub offset: String,
    pub currently: DataPoint<T>,
    pub minutely: DataBlock<T>,
    pub hourly: DataBlock<T>,
    pub daily: DataBlock<T>,
    pub alerts: Vec<Alert<T>>,
    pub flags: Vec<Flag>
}
