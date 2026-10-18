use std::io::{self, Read};

fn main() {
    // Читаем весь поток ввода в строку
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Разделяем по любому пробельному символу (пробелы, переносы, табы)
    // и вытаскиваем только те куски, которые не пустые
    let mut nums = input
        .split_whitespace()
        .map(|s| s.parse::<i128>().unwrap()); // i128 защитит от переполнения при сложении краевых значений i64

    let a = nums.next().expect("Ожидалось первое число");
    let b = nums.next().expect("Ожидалось второе число");

    println!("{}", a + b);
}