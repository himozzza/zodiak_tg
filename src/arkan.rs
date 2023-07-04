pub(crate) async fn arkan(msg: String) -> (String, String) {
    let mut arkan_one: String = String::new();
    let mut arkan_two: String = String::new();
    let verify_date: Vec<usize> = msg.split(".").map(|f| f.parse::<usize>().unwrap()).collect();
    if verify_date[0] <= 31 && verify_date[1] <= 12 {
        let arkan_array: [&str; 22] = ["Маг", "Верховная жрица", "Императрица", "Император", "Верховный жрец", "Влюбленные", "Колесница", "Правосудие", "Отшельник", "Колесо фортуны", "Сила", "Повешенный", "Смерть", "Умеренность", "Дьявол", "Башня", "Звезда", "Луна", "Солнце", "Суд", "Мир", "Шут"];
        let date_raw = msg.replace(".", "");
        let mut arkan_two_raw: usize = 0;
        for i in date_raw.chars() {
            arkan_two_raw += i.to_string().parse::<usize>().unwrap();
        }
        let mut arkan_one_raw = date_raw[0..2].to_string().parse::<usize>().unwrap();
        if arkan_one_raw > 22 {
            arkan_one_raw -= arkan_one_raw - 22
        }
        if arkan_two_raw > 22 {
            arkan_two_raw = &arkan_two_raw - 22
        }
        arkan_one = arkan_array[arkan_one_raw - 1].to_string();
        arkan_two = arkan_array[arkan_two_raw - 1].to_string();

    }
    (arkan_one.to_string(), arkan_two.to_string())
}