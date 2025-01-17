fn main() {
    let response = reqwest::blocking::get("https://www.wunderground.com/weather/us/ca/los-angeles");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_weather_selector = scraper::Selector::parse("div.current-temp").unwrap();
    let html_weather = document.select(&html_weather_selector);
    println!("{:?}", html_weather);
}
