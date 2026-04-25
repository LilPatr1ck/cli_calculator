use std::io::{self, Write};

fn main() {
    println!("Калькулятор\nВведите выражение в формате (<число> <операция> <число>) или 0 для выхода.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения");

        let input = input.trim();
        println!("Ваш ввод:{}",input);

        if input == "0" {
            println!("Выход из программы.");
            break;
        }
    }
}