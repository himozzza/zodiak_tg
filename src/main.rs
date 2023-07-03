use teloxide::prelude::*;
use std::collections::HashMap;
use regex::Regex;
    
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
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
            let re = Regex::new("[0-9]{2}.[0-9]{2}.[0-9]{4}").unwrap();
            if re.is_match(msg.text().unwrap()) {
                let (arkan_one, arkan_two) = arkan(msg.text().unwrap().to_string()).await;
                let s: String = format!("Ваши арканы:\n1. {}.\n2. {}.", arkan_one, arkan_two);
                bot.send_message(msg.chat.id, s).await?;
            }
            let msg_str: String = prepair_msg(msg.text().unwrap()).await;

            if zodiak_signs.contains_key(&msg_str.as_str()) {

                let q: String = get_forecast(zodiak_signs[&msg_str.as_str()]).await;
                bot.send_message(msg.chat.id, q).await?;
            }
            Ok(())
    })
    .await;
}

async fn get_forecast(r: &str) -> String {
    let resp: reqwest::Response = reqwest::get(format!("https://horo.mail.ru/prediction/{}/today/", r))
                                                            .await.expect("Error! Connection failed.");
    let re: Regex = Regex::new("<p>.*[\n\r]*.*</p>").unwrap();

    let binding: String = resp.text().await.unwrap();
    let horoscope_raw: Option<regex::Match<'_>> = re.find(binding.as_str());

    let horoscope: &str = match horoscope_raw {
        Some(x) => x.as_str(),
        None => todo!(),
    };
    horoscope.replace("<p>", "").replace("</p>", "")
                                         .replace(" &ndash;", "")
                                         .replace("&nbsp;", " ")
}

async fn prepair_msg(msg: &str) -> String {
    let mut message: Vec<&str> = Vec::new();
    let re: Regex = Regex::new(r"[а-яА-Я]").unwrap();
    let r: regex::CaptureMatches<'_, '_> = re.captures_iter(msg);
    for i in r.into_iter() {
        for k in i.iter() {
            message.push(k.unwrap().as_str())
        }
    }
    // let re: Regex = Regex::new(r"[-+<>=\ /.?!@$;:0-9\{\}\]\[*]").unwrap();
    // let s = re.replace_all(&msg, "").to_lowercase().to_owned();
    message.join("").to_lowercase()
}

async fn arkan(msg: String) -> (String, String) {
    let arkan_array: [&str; 22] = ["Маг", "Верховная жрица", "Императрица", "Император", "Первосвященник", "Влюбленные", "Колесница", "Сила", "Отшельник", "Колесо фортуны", "Справедливость", "Повешенный", "Смерть", "Умеренность", "Дьявол", "Башня", "Звезда", "Луна", "Солнце", "Суд", "Мир", "Шут"];
    let date_raw = msg.replace(".", "");
    let mut date_sum: usize = 0;
    for i in date_raw.chars() {
        date_sum += i.to_string().parse::<usize>().unwrap();
    }
    let mut arkan_one_raw = date_raw[0..2].to_string().parse::<usize>().unwrap();
    if arkan_one_raw > 22 {
        arkan_one_raw -= arkan_one_raw - 22
    }
    if date_sum > 22 {
        date_sum = &date_sum - 22
    }
    let arkan_one: &str = arkan_array[arkan_one_raw - 1];
    let arkan_two: &str = arkan_array[date_sum - 1];
    (arkan_one.to_string(), arkan_two.to_string())
}
