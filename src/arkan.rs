use chrono::prelude::*;

pub(crate) async fn arkan(msg: String) -> String {
    let date: Vec<usize> = msg.split(".").map(|f| f.parse::<usize>().unwrap()).collect();
    let year: usize = Local::now().year().try_into().unwrap();
    let day_in_month: usize = verify_date(date[0], date[1], date[2]);

    if date[0] > 0 && date[0] <= day_in_month && date[1] > 0 &&  date[1] <= 12 && date[2] > year - 100 && date[2] <= year {
        let arkan_array: [&str; 22] = ["Маг", "Верховная жрица", "Императрица", "Император", "Верховный жрец", "Влюбленные", "Колесница", "Правосудие", "Отшельник", "Колесо фортуны", "Сила", "Повешенный", "Смерть", "Умеренность", "Дьявол", "Башня", "Звезда", "Луна", "Солнце", "Суд", "Мир", "Шут"];
        let date_raw: String = msg.replace(".", "");
        let mut arkan_two_raw: usize = 0;
        for i in date_raw.chars() {
            arkan_two_raw += i.to_string().parse::<usize>().unwrap();
        }
        let mut arkan_one_raw = msg.split(".").collect::<Vec<&str>>()[0].to_string().parse::<usize>().unwrap();
        if arkan_one_raw > 22 {
            arkan_one_raw = arkan_one_raw - 22
        }
        if arkan_two_raw > 22 {
            arkan_two_raw = &arkan_two_raw - 22
        }
        let arkan_one: String = arkan_array[arkan_one_raw - 1].to_string();
        let arkan_two: String = arkan_array[arkan_two_raw - 1].to_string();
        format!("Ваши арканы:\n1. {}.\n2. {}.", arkan_one.to_string(), arkan_two.to_string())
    } else {
            fine_exception_facts(date[2], year)
    }
}

fn verify_date(day: usize, month: usize, year: usize) -> usize { 
    if month >= 8 && month <= 12 && day > 0 && day <= 31 {
        if month % 2 == 0 {
            31
        } else {
            30
        }
    } else if month >= 1 && month <= 7 {
        if month != 2 {
            if month % 2 == 0 {
                30
            } else {
                31
            }
        } else {
            if year % 4 == 0 {
                29
            } else {
                28
            }
        }
    } else {
        0
    }
}

fn fine_exception_facts(year: usize, current_year: usize) -> String {
    if year < current_year - 100 {
        format!("Фатальная ошибка!\nХочешь сказать что ты родился {} лет назад?\nДиапазон даты 100 лет от текущего времени.", current_year - year)
    } else if year > current_year {
        format!("Этот человек ещё не родился.\nПопробуйте ещё раз в {} году." , year)
    } else {
        format!("Неверно введена дата.")
    }
}