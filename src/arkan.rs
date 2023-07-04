pub(crate) async fn arkan(msg: String) -> (String, String) {
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
    let arkan_one: &str = arkan_array[arkan_one_raw - 1];
    let arkan_two: &str = arkan_array[arkan_two_raw - 1];
    (arkan_one.to_string(), arkan_two.to_string())
}