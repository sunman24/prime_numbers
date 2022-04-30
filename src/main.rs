mod lib;
mod tests;

use std::io::{self, Write};
use std::time::Instant;

use crate::lib::{check_prime1, check_prime2, check_prime3};

#[allow(unused_must_use)]
//noinspection ALL
fn main() {
    let mut input = String::new();
    let line = "-".repeat(80);

    println!("{line}");
    print!("Введите число, до которого будут проверятся простые числа: ");
    io::stdout().flush();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Ошибка: не удалось прочитать данные! Сообщение ошибки: {err}");
            println!("{line}\n");

            return;
        }
    }

    let num: usize = match input.trim().parse() {
        Ok(value) => {
            println!("{line}");
            value
        },
        Err(_) => {
            eprintln!("Ошибка: введённое значение не является целым положительным числом!");
            println!("{line}\n");

            return;
        }
    };

    println!("Тестирование первого алгоритма (функция check_prime1):");
    let time = Instant::now();
    let count = (0..num + 1).fold(0usize, |acc, n| {
        if check_prime1(n) {
            acc + 1
        } else {
            acc
        }
    });
    let time = time.elapsed();
    println!("Результат: число простых чисел в диапазоне от 0 до {num} - {count}.");
    println!("Время выполнения: {} милисекунд.\n", time.as_millis());

    println!("Тестирование второго алгоритма (функция check_prime2):");
    let time = Instant::now();
    let count = (0..num + 1).fold(0usize, |acc, n| {
        if check_prime2(n) {
            acc + 1
        } else {
            acc
        }
    });
    let time = time.elapsed();
    println!("Результат: число простых чисел в диапазоне от 0 до {num} - {count}.");
    println!("Время выполнения: {} милисекунд.\n", time.as_millis());

    println!("Тестирование третьего алгоритма (функция check_prime3):");
    let time = Instant::now();
    let count = (0..num + 1).fold(0usize, |acc, n| {
        if check_prime3(n) {
            acc + 1
        } else {
            acc
        }
    });
    let time = time.elapsed();
    println!("Результат: число простых чисел в диапазоне от 0 до {num} - {count}.");
    println!("Время выполнения: {} милисекунд.", time.as_millis());

    println!("{line}\n");
}
