use teloxide::prelude::*;
use std::collections::HashMap;
use regex::Regex;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot = Bot::new("TOKEN");

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
    msg.to_lowercase().replace(".", "")
                      .replace(",", "")
                      .replace(" ", "")
                      .to_string()
}
