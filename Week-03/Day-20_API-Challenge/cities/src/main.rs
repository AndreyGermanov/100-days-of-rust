use std::sync::Mutex;
use sqlite::{Connection, Statement};
use rocket::{response::content, State};
use serde_derive::Serialize;

#[macro_use] extern crate rocket;

struct AppState {
    database: Mutex<Connection>
}

#[derive(Serialize)]
struct CityResponse {
    name: String,
    latitude: f64,
    longitude: f64,
    score: f64
}

#[rocket::main]
async fn main() {
    let connection = sqlite::open("cities.db").expect("Could not open database");
    rocket::build()
        .manage(AppState {
            database: Mutex::new(connection),
        })
        .mount("/", routes![index])
        .mount("/suggestions", routes![suggestions])
        .launch().await.unwrap();
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    content::RawHtml(std::fs::read_to_string("index.html").unwrap())
}

#[get("/?<q>&<latitude>&<longitude>")]
fn suggestions(q: String, latitude: Option<f64>, longitude: Option<f64>, state: &State<AppState>) -> String {
    let mut loc: Option<(f64,f64)> = None;
    if latitude.is_some() && longitude.is_some() {
        loc = Some((latitude.unwrap(), longitude.unwrap()))
    }
    return serde_json::to_string(&get_closest_cities(state,q,loc,10)).unwrap_or_default()
}

fn get_closest_cities(state: &AppState, name: String, loc: Option<(f64,f64)>, count: i32) -> Vec<CityResponse> {
    let query = format!("SELECT city,country,lat,lng FROM cities \
        WHERE UPPER(city) \
        LIKE '{}%'  AND population > 5000 \
        ORDER BY city \
        LIMIT {}", name, count
    );
    let connection = state.database.lock().unwrap();
    let statement = connection.prepare(query).unwrap();
    let result = parse_query_result(statement, loc);
    return result;
}

fn parse_query_result(mut statement: Statement,loc: Option<(f64,f64)>) -> Vec<CityResponse> {
    let mut result:Vec<CityResponse> = vec![];
    let mut min_distance = 99999_f64;
    while let Ok(sqlite::State::Row) = statement.next() {
        let mut row = CityResponse {
            name: statement.read::<String, _>("city").unwrap().clone()+
                ", "+&statement.read::<String, _>("country").unwrap().clone(),
            latitude: statement.read::<f64, _>("lat").unwrap(),
            longitude: statement.read::<f64, _>("lng").unwrap(),
            score: 0_f64
        };
        if let Some((lat,lng)) = loc {
            row.score = calc_distance((row.latitude,row.longitude),(lat,lng));
            if row.score < min_distance {
                min_distance = row.score;
            }
        };
        result.push(row);
    }
    if loc.is_some() {
        for item in result.iter_mut() {
            item.score = min_distance / item.score;
        }
        result.sort_by(|item1, item2| item1.score.partial_cmp(&item2.score).unwrap());
        result.reverse();
    }
    result
}

fn calc_distance(point1:(f64, f64),point2:(f64,f64)) -> f64 {
    let earth_radius_kilometer = 6371.0_f64;
    let point1_lat = deg_to_rad(point1.0);
    let point2_lat = deg_to_rad(point2.0);
    let delta_lat = deg_to_rad(point1.0-point2.0);
    let delta_lng = deg_to_rad(point1.1-point2.1);
    let central_angle_inner = (delta_lat / 2.0).sin().powi(2) + point1_lat.cos() *
        point2_lat.cos() * (delta_lng / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();
    return (earth_radius_kilometer * central_angle).abs();
}

fn deg_to_rad(degs: f64) -> f64 {
    degs*std::f64::consts::PI/180_f64
}
