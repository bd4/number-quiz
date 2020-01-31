extern crate clap;

use std::io;
use std::io::Write;
use std::collections::HashMap;

use std::time::{Duration, Instant};

use rand::Rng;

use clap::{Arg, App};

const NUM_RU: &[&str] = &[
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

const MONTHS_RU: &[&str] = &[
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

const DAYS_RU:&[&str] = &[
    "понедельник",
    "вторник",
    "среда",
    "четверг",
    "пятница",
    "суббота",
    "воскресенье",
];


fn parse_ru_num(ru_num: &HashMap<&str, usize>, s: &str) -> Option<usize> {
    let mut parts: Vec<&str> = s.trim().split_whitespace().collect();
    if parts.len() < 1 {
        return None;
    }
    let mut total: usize = 0;
    if parts[0] == "тысяча" {
        total = 1000;
        parts.remove(0);
    } else if parts.len() > 1 && parts[1].starts_with("тысяч") {
        let t : String = parts[0..=1].join(" ");
        match ru_num.get(&*t) {
            Some(n) if *n >= 1000 => total += *n,
            _ => return None,
        }
        parts.remove(0);
        parts.remove(0);
    }
    for p in parts {
        match ru_num.get(p) {
            Some(n) => total += *n,
            None => return None,
        }
    }
    return Some(total)
}


fn get_num_ru(n: usize) -> String {
    if n <= 20 {
        return NUM_RU[n].to_string();
    }
    let mut result = String::with_capacity(
                        f32::log10(n as f32).ceil() as usize * 6);
    let t = n / 1000;
    let n = n % 1000;
    match t {
        0 => {},
        1 =>         result.push_str(" тысяча"),
        2 =>         result.push_str(" две тысячи"),
        3 | 4 =>   { result.push_str(" "); result.push_str(NUM_RU[t]);
                     result.push_str(" тысячи"); },
        5 ..= 9 => { result.push_str(" "); result.push_str(NUM_RU[t]);
                     result.push_str(" тысяч"); },
        _ => {}
    }

    let h = n / 100;
    let n = n % 100;
    match h {
        0 => {},
        1 => result.push_str(" сто"),
        2 => result.push_str(" двести"),
        3 | 4 => { result.push_str(" "); result.push_str(NUM_RU[h]);
                   result.push_str("ста"); },
        5 ..= 9 => { result.push_str(" "); result.push_str(NUM_RU[h]);
                     result.push_str("сот"); },
        _ => {}
    }

    let tens = n / 10;
    let mut n = n % 10;
    match tens {
        0 => {},
        1 =>     { result.push_str(" ");
                   result.push_str(NUM_RU[tens * 10 + n]);
                   n=0 },
        2 | 3 => { result.push_str(" "); result.push_str(NUM_RU[tens]);
                   result.push_str("дцать"); },
        4 => result.push_str(" сорок"),
        5 ..= 8 => { result.push_str(" "); result.push_str(NUM_RU[tens]);
                     result.push_str("десят"); },
        9       => { result.push_str(" девяносто"); },
        _ => {},
    }

    if n > 0 {
        result.push_str(" ");
        result.push_str(NUM_RU[n]);
    }
    if result.starts_with(" ") {
        result.remove(0);
    }
    return result;
}


fn quiz(ru_num: &HashMap<&str, usize>, maxn: usize) {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(1, maxn);
    let b = rng.gen_range(1, maxn-a+1);
    let c = a + b;
 
    println!("{} + {} = ", get_num_ru(a), get_num_ru(b));
    io::stdout().flush().unwrap();

    let mut correct = false;
    for _ in 0..3 {
        let ans_s = read_str();
        match parse_ru_num(ru_num, ans_s.trim()) {
            Some(ans_n) if ans_n == c => { correct = true; break; },
            Some(ans_n)               => { 
                println!("value {} = {} is not correct, try again: ",
                         ans_s.trim(), ans_n);
            },
            None                      => {
                println!("value {} is mispelled, try again: ", ans_s.trim());
            },
        }
    }
    if correct {
        println!("Correct! {} + {} = {}", a, b, c)
    } else {
        println!("Too many attempts, correct answer: {}", get_num_ru(c));
    }
}


fn quiz_time(words: &[&str]) {
    let n : usize = words.len();
    let mut rng = rand::thread_rng();
    let start = rng.gen_range(0, n-1);
    let delta = rng.gen_range(1, n-1);
    let correct = words[(start + delta) % n];
    print!("{} + {} = ", words[start], NUM_RU[delta]);
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
    let ans = read_str();
    let num: usize = ans.trim().parse().expect("Not a number!");
    return num;
}


fn test(ru_num : &HashMap<&str, usize>, maxn : usize) -> bool {
    for n in 0..maxn {
        let name = get_num_ru(n);
        match parse_ru_num(ru_num, &*name) {
            Some(n2) if n2 != n => { println!("{} {} {}", n, name, n2);
                                   return false; },
            None => { println!("{} {} failed to parse", n, name);
                      return false; },

            _ => {},
        }
    }
    return true;
}


fn main() {
    let mut ru_num : HashMap<&str, usize> = HashMap::new();
    for i in 0..NUM_RU.len() {
        ru_num.insert(NUM_RU[i], i);
    }

    ru_num.insert("тридцать", 30);
    ru_num.insert("сорок", 40);
    ru_num.insert("пятьдесят", 50);
    ru_num.insert("шестьдесят", 60);
    ru_num.insert("семьдесят", 70);
    ru_num.insert("восемьдесят", 80);
    ru_num.insert("девяносто", 90);
    ru_num.insert("сто", 100);
    ru_num.insert("двести", 200);
    ru_num.insert("триста", 300);
    ru_num.insert("четыреста", 400);
    ru_num.insert("пятьсот", 500);
    ru_num.insert("шестьсот", 600);
    ru_num.insert("семьсот", 700);
    ru_num.insert("восемьсот", 800);
    ru_num.insert("девятьсот", 900);
    ru_num.insert("тысяча", 1000);
    ru_num.insert("одна тысяча", 1000);
    ru_num.insert("две тысячи", 2000);
    ru_num.insert("три тысячи", 3000);
    ru_num.insert("четыре тысячи", 4000);
    ru_num.insert("пять тысяч", 5000);
    ru_num.insert("шесть тысяч", 6000);
    ru_num.insert("семь тысяч", 7000);
    ru_num.insert("восемь тысяч", 8000);
    ru_num.insert("девять тысяч", 9000);


    let matches = App::new("numrs")
        .about("Russian language number quiz")
        .version("0.1")
        .author("Bryce Allen <bryce@bda.space>")
        .arg(Arg::with_name("numbers")
                .short("n").long("numbers")
                .help("Plain number quiz"))
        .arg(Arg::with_name("days")
                .short("d").long("days")
                .help("Days of the week quiz"))
        .arg(Arg::with_name("months")
                .short("m").long("months")
                .required(false)
                .help("Months of the year quiz"))
        .arg(Arg::with_name("test")
                .short("t").long("test")
                .help("Run roundtrip parse test"))
        .arg(Arg::with_name("max")
                .short("x").long("max")
                .takes_value(true)
                .default_value("9999")
                .help("Max value for numbers test"))
        .get_matches();

    let maxn;
    let max_str = matches.value_of("max");
    match max_str {
        None => { println!("Failed to parse max"); maxn = 9999 },
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n) => { maxn = n; },
                Err(_) => { println!("Failed to parse max"); maxn = 9999 },
            }
        }
    }
    
    if matches.is_present("test") {
        let start = Instant::now();
        let test_ok = test(&ru_num, maxn);
        let test_time = start.elapsed().as_secs_f32();
        println!("test {} {}", test_ok, test_time)
    }

    let mut quizes : Vec<Box<dyn Fn() -> ()>> = Vec::with_capacity(3);
    if matches.is_present("numbers") {
        quizes.push(Box::new(move || { quiz(&ru_num, maxn) }));
    }
    if matches.is_present("days") {
        quizes.push(Box::new(move || { quiz_time(DAYS_RU) }));
    }
    if matches.is_present("months") {
        quizes.push(Box::new(move || { quiz_time(MONTHS_RU) }));
    }
    if quizes.len() > 0 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, quizes.len());
        (*quizes[i])();
    }

    //quiz_time(DAYS_RU);
    //quiz(&ru_num, 20);
}
