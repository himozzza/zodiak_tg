use regex::Regex;
use postgres::{Client, NoTls};
use std::{collections::HashMap, sync::mpsc};

pub(crate) fn get_forecast(msg: teloxide::types::Message) -> String {
    let (tx, rx) = mpsc::channel();

    let thread: std::thread::JoinHandle<()> = std::thread::spawn(move ||{
        let msg_str: String = prepair_msg(msg.text().unwrap().to_string());
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

    if zodiak_signs.contains_key(&msg_str.as_str()) {
        let mut client: Client = Client::connect("postgresql://postgres:postgres@localhost:5243/zodiak_db", NoTls,)
                                    .expect("Not connect to data base.");
        
        let row: Vec<postgres::Row> = client.query("SELECT query FROM zodiak_db WHERE zodiak_id = $1", &[&msg_str],).unwrap();
        let query: usize = row.iter().map(|f| f.get(0)).collect::<Vec<&str>>()[0].parse::<usize>().unwrap();

        let answer: String = match query {
            1 => push_forecast(&mut client, "2".to_string(), &msg_str, 1),
            2 => push_forecast(&mut client, "3".to_string(), &msg_str, 2),
            _ => push_forecast(&mut client, "1".to_string(), &msg_str, 3),
        };

        tx.send(answer).unwrap()

    } else {
        tx.send("Возможно Вы допустили ошибку.".to_string()).unwrap()
    }
    });

    thread.join().unwrap();
    rx.recv().unwrap()

}

fn prepair_msg(msg: String) -> String {
    let re: Regex = Regex::new(r"[а-яА-Я]").unwrap();
    let message: Vec<_> = re.captures_iter(msg.as_str())
                        .into_iter().map(|f: regex::Captures<'_>|f.iter()
                            .map(|s: Option<regex::Match<'_>>|s.unwrap().as_str())
                                .collect::<Vec<&str>>().join("")).collect();
    message.join("").to_lowercase().to_string()
}

fn push_forecast(client: &mut postgres::Client, count: String, msg_str: &String, forecast: usize, ) -> String {
    client.execute("UPDATE zodiak_db SET query = $1 WHERE zodiak_id = $2", &[&count, &msg_str],).unwrap();
    let zodiak_row: Vec<postgres::Row> = client.query("SELECT * FROM zodiak_db WHERE zodiak_id = $1", &[&msg_str],).unwrap();
    zodiak_row.iter().map(|f| f.get(forecast)).collect::<Vec<&str>>()[0].to_string()
}