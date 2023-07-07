use chrono::prelude::*;
pub(crate) async fn arkan(msg: String) -> String {
    let verify_date: Vec<usize> = msg.split(".").map(|f| f.parse::<usize>().unwrap()).collect();
    let month: usize = if verify_date[0] % 2 == 0 {
        30
    } else {
        31
    };
    let year: usize = Local::now().year().try_into().unwrap();
    if verify_date[0] > 0 && verify_date[0] <= month && verify_date[1] > 0 &&  verify_date[1] <= 12 && verify_date[2] > year - 100 && verify_date[2] <= year + 1 {
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
        format!("Неверно введена дата.")
    }
}
