use regex::Regex;
use std::collections::HashMap;

pub(crate) async fn get_forecast(msg: String) -> String {
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

    let msg_str: String = prepair_msg(msg.to_string()).await;
    let mut horoscope: String = String::new();
    if zodiak_signs.contains_key(msg_str.as_str()) {
        let re: Regex = Regex::new("[а-яА-Я]*").unwrap();
        let resp: reqwest::Response = reqwest::get(format!("https://horo.mail.ru/prediction/{}/today/", zodiak_signs[&msg_str.as_str()]))
                                                                .await.expect("Error! Connection failed.");
    
        let binding: String = resp.text().await.unwrap();
        let horoscope_raw: Option<regex::Match<'_>> = re.find(binding.as_str());
    
        horoscope = match horoscope_raw {
            Some(x) => x.as_str().to_string(),
            None => todo!(),
        };

    }
    horoscope.replace("<p>", "").replace("</p>", "")
    .replace(" &ndash;", "")
    .replace("&nbsp;", " ")
}

async fn prepair_msg(msg: String) -> String {
    let mut message: Vec<&str> = Vec::new();
    let re: Regex = Regex::new(r"[а-яА-Я]").unwrap();
    let r: regex::CaptureMatches<'_, '_> = re.captures_iter(msg.as_str());
    for i in r.into_iter() {
        for k in i.iter() {
            message.push(k.unwrap().as_str())
        }
    }

    message.join("").to_lowercase()
}

