use std::collections::HashMap;
use regex::Regex;
use postgres::{Client, NoTls};


pub(crate) fn db_update(){
    let mut client: Client= Client::connect("postgresql://postgres:postgres@localhost:5243/zodiak_db", NoTls).unwrap();

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS zodiak_db (
            zodiak_id text,
            forecast_one text,
            forecast_two text,
            forecast_three text,
            query text,
            CONSTRAINT zodiak_unique UNIQUE (zodiak_id)
        )
    ").expect("Error create data table.");

    let zodiak_signs: HashMap<&str, &str> = HashMap::from([
        ("овен", "aries"),
        ("телец",    "taurus"),
        ("близнецы", "gemini"),
        ("рак",      "cancer"),
        ("лев",      "leo"),
        ("дева",     "virgo"),
        ("весы",     "libra"),
        ("скорпион", "scorpio"),
        ("стрелец",  "sagittarius"),
        ("козерог",  "capricorn"),
        ("водолей",  "aquarius"),
        ("рыбы",     "pisces"),
    ]);

    for (k, v) in &zodiak_signs {
        let forecast: String = get_forecast(v);
        let forecast_two: String = get_forecast_two(v);
        let forecast_three: String = get_forecast_three(v);
        client.execute("INSERT INTO zodiak_db (zodiak_id, forecast_one, forecast_two, forecast_three, query) VALUES ($1, $2, $3, $4, $5) 
                                ON CONFLICT ON CONSTRAINT zodiak_unique 
                                    DO UPDATE SET forecast_one = $2, forecast_two = $3, forecast_three = $4, query = $5",
                                &[&k, &forecast, &forecast_two, &forecast_three, &"1".to_string()],).expect("Error. Not insert values to table.");
    }
}

fn get_forecast(sign: &str) -> String {
    let resp: reqwest::blocking::Response = reqwest::blocking::get(format!("https://horo.mail.ru/prediction/{}/today/", &sign))
                                                            .expect("Error! Connection failed.");
    let binding: String = resp.text().unwrap();
    let re: Regex = Regex::new("<p>.*[\n\r]*.*</p>").unwrap();
    let horoscope_raw: Option<regex::Match<'_>> = re.find(binding.as_str());

    let horoscope: String = match horoscope_raw {
        Some(x) => x.as_str().to_string(),
        None => todo!(),
    };
    horoscope.replace("<p>", "").replace("</p>", "")
                                            .replace(" &ndash;", "")
                                            .replace("&nbsp;", " ") + &"\n\nГороскоп подготовлен horo.mail.ru.".to_string()
}

fn get_forecast_two(sign: &str) -> String {
    let resp: reqwest::blocking::Response = reqwest::blocking::get(format!("https://horoscopes.rambler.ru/{}/", sign)).unwrap();
    let html_dom: String = resp.text().unwrap();
    let horoscope: &str = html_dom.split("grabContent\":[{\"type\":\"paragraph\",\"content\":\"")
                                .collect::<Vec<&str>>()[1]
                                .split("\"}],\"date\":\"")
                                .collect::<Vec<&str>>()[0];
    horoscope.to_string() + &"\n\nГороскоп подготовлен horoscopes.rambler.ru.".to_string()
}

fn get_forecast_three(sign: &str) -> String {
    let resp: reqwest::blocking::Response = reqwest::blocking::get(format!("https://1001goroskop.ru/?znak={}", sign)).unwrap();
    let bind: String = resp.text().unwrap();
    let horoscope: &str = bind.split("<div itemprop=\"description\"><p>").collect::<Vec<&str>>()[1].split("</p></div>").collect::<Vec<&str>>()[0];
    horoscope.to_string() + &"\n\nГороскоп подготовлен 1001goroskop.ru.".to_string()
}