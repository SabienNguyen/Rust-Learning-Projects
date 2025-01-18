fn main() {
    let response = reqwest::blocking::get("https://weather.com/weather/today/l/ca18fda8e3817d7d6773f7d229ea3038afe2a86f11ed4363d0d75a82d9ada22c");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_weather_selector = scraper::Selector::parse("span.CurrentConditions--tempValue--zUBSz").unwrap();
    let html_weather = document.select(&html_weather_selector);
    println!("{:?}", html_weather);
}
