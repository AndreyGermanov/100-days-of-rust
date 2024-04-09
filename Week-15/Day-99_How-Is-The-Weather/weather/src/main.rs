use std::collections::HashMap;
use std::str::FromStr;
use csv;
use reqwest;
use serde_json::{Value};

#[derive(Debug)]
struct Forecast {
    municipality: String,
    state: String,
    forecast: Vec<ForecastDay>
}

#[derive(Debug)]
struct ForecastDay {
    morning: ForecastPartDay,
    day: ForecastPartDay,
    evening: ForecastPartDay
}

#[derive(Debug, Clone)]
struct ForecastPartDay {
    minimum: f64,
    maximum: f64,
    resume: String
}

struct JsonParseResult (
    HashMap<u8,ForecastPartDay>,
    HashMap<u8,ForecastPartDay>,
    HashMap<u8,ForecastPartDay>,
    Vec<u8>
);

fn main() {
    println!("{:?}",query_forecast("coriz","MT"))
}

fn query_forecast(input: &str, state: &str) -> Result<Forecast,String> {
    let municipality = get_closest_municipality(input, state)?;
    let (lat, lng) = get_municipality_location(municipality.as_str(), state)?;
    let api_key = "OPENWEATHER-API-KEY";
    let request = format!("https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&units=metric&appid={}",lat,lng,api_key);
    let response_result = reqwest::blocking::get(request);
    if response_result.is_err() { return Err("Could not request remove API".to_string()) }
    let response = response_result.unwrap();
    let text_result = response.text();
    if text_result.is_err() { return Err("Could not get response text".to_string()) };
    let text = text_result.unwrap();
    let json_result = serde_json::from_str(text.as_str());
    if json_result.is_err() { return Err("Could not parse response JSON".to_string()) };
    let json:Value = json_result.unwrap();
    Ok(parse_forecast("Acorizal", "MT", &json))
}

fn parse_forecast(municipality: &str, state: &str, json: &Value) -> Forecast {
    let JsonParseResult(
        morning_temps,
        day_temps,
        evening_temps,
        days) = parse_json(json);
    return Forecast {
        municipality: municipality.to_string(),
        state: state.to_string(),
        forecast: days.iter().map(|key| {
            let mut fallback = ForecastPartDay{minimum:0.0,maximum:0.0,resume:"".to_string()};
            let mut morning = morning_temps.get(&key).unwrap();
            if morning.minimum != 999999.0 { fallback = morning.clone(); }
            let mut day = day_temps.get(&key).unwrap();
            if day.minimum != 999999.0 { fallback = day.clone(); }
            let mut evening = evening_temps.get(&key).unwrap();
            if evening.minimum != 999999.0 { fallback = evening.clone(); }
            if morning.minimum == 999999.0 { morning = &fallback }
            if day.minimum == 999999.0 { day = &fallback }
            if evening.minimum == 999999.0 { evening = &fallback }
            ForecastDay { morning: morning.clone(), day: day.clone(), evening: evening.clone() }
        }).collect()
    }
}

fn parse_json(json: &Value) -> JsonParseResult {
    let mut morning_temps:HashMap<u8,ForecastPartDay> = HashMap::new();
    let mut day_temps:HashMap<u8,ForecastPartDay> = HashMap::new();
    let mut evening_temps:HashMap<u8,ForecastPartDay> = HashMap::new();
    let (mut morning_days, mut day_days, mut evening_days) = (vec![],vec![],vec![]);
    json["list"].as_array().unwrap().iter().for_each(|item| {
        let parts:Vec<_> = item["dt_txt"].as_str().unwrap().split(" ").collect();
        let day = u8::from_str(parts[0].split("-").collect::<Vec<_>>()[2]).unwrap();
        let hour = u8::from_str(parts[1].split(":").collect::<Vec<_>>()[0]).unwrap();
        let temp = item["main"]["temp"].as_f64().unwrap();
        let resume = item["weather"].as_array().unwrap()[0]["main"].as_str().unwrap().to_string();
        let mut temps = morning_temps.entry(day)
            .or_insert(ForecastPartDay{minimum:999999.0,maximum:-999999.0,resume:"".to_string()});
        if hour >= 8 && hour < 16 {
            temps = day_temps.entry(day)
                .or_insert(ForecastPartDay{minimum:999999.0,maximum:-999999.0, resume:"".to_string()});
            day_days.push(day);
        } else if hour >= 16 {
            evening_days.push(day);
            temps = evening_temps.entry(day)
                .or_insert(ForecastPartDay{minimum:999999.0,maximum:-999999.0, resume:"".to_string()});
        } else {
            morning_days.push(day)
        }
        if temp < temps.minimum { temps.minimum = temp; }
        if temp > temps.maximum { temps.maximum = temp }
        temps.resume = resume
    });

    let days:Vec<_> = if morning_days.len() < day_days.len() && morning_days.len() < day_days.len() {
        morning_days
    } else if day_days.len() < morning_days.len() && day_days.len() < evening_days.len() {
        day_days
    } else {
        evening_days
    };
    JsonParseResult(
        morning_temps,
        day_temps,
        evening_temps,
        days
    )
}

fn get_municipality_location(municipality: &str, accr: &str) -> Result<(f32,f32),String> {
    let state = get_state_by_accr(accr)?;
    for record in csv::Reader::from_path("BrazilianMunicipalities.csv").unwrap()
        .records().into_iter().map(|record| record.unwrap()) {
        if record.get(1).unwrap() == municipality && record.get(2).unwrap() == state.as_str() {
            return Ok(
                (
                    f32::from_str(record.get(3).unwrap()).unwrap(),
                    f32::from_str(record.get(4).unwrap()).unwrap()
                )
            );
        }
    }
    return Err("Could not find location".to_string())
}

fn get_state_by_accr(accr:&str) -> Result<String,String> {
    let states:HashMap<&str,&str> = HashMap::from([
        ("AC","Acre"), ("AL","Alagoas"), ("AP","Amapá"), ("AM","Amazonas"), ("BA","Bahia"),
        ("CE","Ceará"), ("ES","Espírito Santo"), ("GO","Goiás"), ("MA","Maranhão"),
        ("MT","Mato Grosso"), ("MS","Mato Grosso do Sul"), ("MG","Minas Gerais"), ("PA","Pará"),
        ("PB","Paraíba"), ("PR","Paraná"),("PE","Pernambuco"), ("PI","Piauí"),
        ("RJ","Rio de Janeiro"), ("RN","Rio Grande do Norte"), ("RS","Rio Grande do Sul"),
        ("RO","Rondônia"), ("RR","Roraima"), ("SC","Santa Catarina"), ("SP","São Paulo"),
        ("SE","Sergipe"), ("TO","Tocantins"), ("DF","Distrito Federal")]);
    match states.get(accr) {
        Some(state) => return Ok(state.to_string()),
        _ => Err("Could not find specified state".to_string())
    }
}

fn get_closest_municipality(input: &str, accr: &str) -> Result<String,String> {
    let state = get_state_by_accr(accr)?;
    Ok(csv::Reader::from_path("BrazilianMunicipalities.csv").unwrap()
        .records().into_iter().map(|record| record.unwrap())
        .fold((99999,"".to_string()),|(mut min,mut result),record| {
            if record.get(2).unwrap() == state.as_str() {
                let distance = levenshtein(input, record.get(1).unwrap());
                if distance < min {
                    min = distance;
                    result = record.get(1).unwrap().to_string();
                }
            }
            return (min, result)
        }).1)
}

pub fn levenshtein(a: &str, b: &str) -> usize {
    let mut result = 0;
    if a == b { return result; }
    let (length_a,length_b) = (a.chars().count(),b.chars().count());
    if length_a == 0 { return length_b; }
    if length_b == 0 { return length_a; }
    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let (mut distance_a,mut distance_b) = (0, 0);
    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;
        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b { distance_a } else { distance_a + 1 };
            distance_a = cache[index_a];
            result = if distance_a > result {
                if distance_b > result { result + 1 } else { distance_b }
            } else if distance_b > distance_a { distance_a + 1 } else { distance_b };
            cache[index_a] = result;
        }
    }
    result
}
