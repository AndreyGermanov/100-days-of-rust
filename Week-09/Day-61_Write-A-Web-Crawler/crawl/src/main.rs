use reqwest::StatusCode;
use scraper::{Html, Selector};

fn main() {
    let resp = scrape("https://germanov.dev", -1, &vec![]);
    if let Ok(list) = resp { println!("{:?}",list); }
}

fn scrape(url: &str, depth: i8, visited: &Vec<String>) -> Result<Vec<String>,String> {
    if depth == 0 || visited.contains(&url.to_string()) {
        return Err("Could not go further".to_string());
    }
    let (protocol, host) = match parse_url(url) {
        Ok(result) => result,
        _ => return Err("Could not parse URL!".to_string())
    };
    match reqwest::blocking::ClientBuilder::new()
        .danger_accept_invalid_certs(true).build().unwrap().get(url).send() {
        Ok(result) if result.status() == StatusCode::OK =>
            Ok(get_links(result.text().unwrap().as_str(),url,protocol.as_str(),host.as_str(),depth)),
        Err(error) => Err(format!("Could not process page: {}", error)),
        _ => Err("Could not parse page".to_string())
    }
}

fn parse_url(url: &str) -> Result<(String, String),String> {
    let root = url.to_lowercase();
    let mut arr:Vec<_> = root.split("//").collect();
    if arr.len() == 0 || arr[0].len() == 0 {
        return Err("Could not parse URL".to_string())
    }
    let protocol = arr[0];
    let root2 = url.to_lowercase().replace(&(protocol.to_string()+"//"),"");
    arr = root2.split("/").collect();
    if arr.len() == 0 {
        return Err("Could not parse URL".to_string());
    }
    let host = arr[0];
    return Ok((protocol.to_string(),host.to_string()));
}

fn get_links(text: &str, url: &str, protocol: &str, host: &str, depth: i8) -> Vec<String> {
    let mut links = parse_html(text, url, protocol, host);
    for link in &links.clone() {
        if let Ok(result) = scrape(link.as_str(),if depth>0 {depth-1} else {depth}, &links) {
            let _ = &links.append(&mut result.clone());
        }
    }
    links
}

fn parse_html(text: &str, url: &str, protocol: &str, host: &str) -> Vec<String> {
    let document = Html::parse_document(text);
    let selector = Selector::parse("[href],[src]").unwrap();
    let elements: Vec<_> = document.select(&selector).collect();
    let mut links = vec![];
    for element in elements {
        let mut link = if element.value().attr("href").is_some() {
            element.value().attr("href").unwrap().to_lowercase()
        } else {
            element.value().attr("src").unwrap().to_lowercase()
        };
        if !link.starts_with("http") {
            if link.starts_with("/") {
                link = protocol.to_string() + "//" + host + link.as_str();
            } else {
                link = url.to_string() + link.as_str();
            }
        }
        if !links.contains(&link) {
            links.push(link);
        }
    }
    links
}
