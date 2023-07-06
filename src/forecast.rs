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

    if zodiak_signs.contains_key(msg_str.as_str()) {
        let re: Regex = Regex::new("<p>.*[\n\r]*.*</p>").unwrap();
        let resp: reqwest::Response = reqwest::get(format!("https://horo.mail.ru/prediction/{}/today/", zodiak_signs[&msg_str.as_str()]))
                                                                .await.expect("Error! Connection failed.");
    
        let binding: String = resp.text().await.unwrap();
        let horoscope_raw: Option<regex::Match<'_>> = re.find(binding.as_str());
    
        let horoscope: String = match horoscope_raw {
            Some(x) => x.as_str().to_string(),
            None => todo!(),
        };
        horoscope.replace("<p>", "").replace("</p>", "")
        .replace(" &ndash;", "")
        .replace("&nbsp;", " ")

    } else {
        String::from("Возможно Вы допустили ошибку.")
    }

}

async fn prepair_msg(msg: String) -> String {
    // let mut message: Vec<&str> = Vec::new();
    let re: Regex = Regex::new(r"[а-яА-Я]").unwrap();
    let message: Vec<_> = re.captures_iter(msg.as_str())
                        .into_iter().map(|f: regex::Captures<'_>|f.iter()
                            .map(|s: Option<regex::Match<'_>>|s.unwrap().as_str())
                                .collect::<Vec<&str>>().join("")).collect();


    message.join("").to_lowercase().to_string()
}

