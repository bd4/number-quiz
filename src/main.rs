use std::io;
use std::io::Write;

use rand::Rng;

const num_ru: &[&str] = &[
    "ноль", "один",
    "два", "три",
    "четыре", "пять", "шесть",
    "семь", "восемь",
    "девять", "десять",
    "одиннадцать",
    "двенадцать",
    "тринадцать",
    "четырнадцать",
    "пятнадцать",
    "шестнадцать",
    "семнадцать",
    "восемнадцать",
    "девятнадцать",
    "двадцать"];

const months_ru: &[&str] = &[
    "январь",
    "февраль",
    "март",
    "апрель",
    "май",
    "июнь",
    "июль",
    "август",
    "сентябрь",
    "октябрь",
    "ноябрь",
    "декабрь",
];

const days_ru:&[&str] = &[
    "понедельник",
    "вторник",
    "среда",
    "четверг",
    "пятница",
    "суббота",
    "воскресенье",
];


fn quiz_time(words: &[&str]) {
    let n : usize = words.len();
    let mut rng = rand::thread_rng();
    let start = rng.gen_range(0, n-1);
    let delta = rng.gen_range(1, n-1);
    let correct = words[(start + delta) % n];
    print!("{} + {} = ", words[start], num_ru[delta]);
    io::stdout().flush().unwrap();
    let ans = read_str();
    if ans.trim() == correct {
        println!("Correct!");
    } else {
        println!("Answer: {} (guess {})", correct, ans.trim());
    }
}


fn read_str() -> String {
    let mut ans = String::new();

    io::stdin().read_line(&mut ans)
        .expect("Failed to read line from stdin");

    return ans;
}


fn read_num() -> usize {
    let mut ans = read_str();
    let num: usize = ans.trim().parse().expect("Not a number!");
    return num;
}


fn main() {
    print!("Enter number: ");
    io::stdout().flush().unwrap();

    let num = read_num();

    if num < num_ru.len() {
        println!("The number {} is {}", num, num_ru[num]);
    } else {
        println!("The number {} is too big for me!", num);
    }

    quiz_time(days_ru);
}
