use std::io::{self, Write};

fn main() {
    println!("Калькулятор\nВведите выражение в формате (<число1> <оператор> <число2>) или 0 для выхода.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения");

        let input = input.trim();

        if input == "0" {
            println!("Выход из программы.");
            break;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Введите выражение в формате: <число1> <оператор> <число2>");
            continue;
        }

        let a: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Первый аргумент не число.");
                continue;
            }
        };
        let op = parts[1];
        let b: f64 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Второй аргумент не число.");
                continue;
            }
        };

        let result = match op {
            "+" => a + b,
            "-" => a - b,
            "*" | "x" => a * b,
            "/" => {
                if b == 0.0 {
                    println!("Ошибка: деление на ноль!");
                    continue;
                }
                a / b
            }
            _ => {
                println!("Неизвестная операция: {}. Доступны +, -, *, /", op);
                continue;
            }
        };

        println!("Результат: {}", result);
    }
}