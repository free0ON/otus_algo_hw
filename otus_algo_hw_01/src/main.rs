/**
 * «Квадрат Гарри Поттера»
 * https://www.youtube.com/watch?v=KmV3aBHfo5w
 * JUNIOR
+1 байт. Разгадать первое заклинание, тайм-код 3:20
Написать здесь в чате ответ.
MIDDLE
Скачать архив Harry-Potter-Square.zip
В папке pics находится 25 картинок для разгадывания заклинаний.
Также есть файл squares.txt с правильными ответами для проверки.
+1 байт. Написать программу, которая выводит квадратные заклинания.
+3 байта. Разгадать ещё шесть заклинаний (любых).
Написать в чате ответы.
SENIOR
+5 байт. Разгадать ещё десять заклинаний (любых).
Написать в чате ответы.
Написать, сколько времени ушло на выполнение задания.

Описание/Пошаговая инструкция выполнения домашнего задания:

Посмотреть видеоурок «Квадрат Гарри Поттера»
Выполнить задание и написать в чат ответы.
Написать, сколько заклинаний разгадано и сколько байт набрано.
Написать, сколько времени ушло на выполнение задания.
ps. перенесено из другого репозитория 
*/

use std::io;
use colored::*;
use clearscreen;

fn main() {
    let max_x = 25;
    let max_y = 25;
    let mut spells: Vec<Box<dyn Fn(i32, i32) -> bool>> = Vec::new();
 
    //JUNIOR
    spells.push(Box::new(|x, y| x < y )); //1
    //MIDDLE
    spells.push(Box::new(|x, y| x == y )); //2
    spells.push(Box::new(|x, y| x == max_x - 1 - y )); //3
    spells.push(Box::new(|x, y| x + y < 30 )); //4
      //SENIOR
    spells.push(Box::new(|x, y| ((y / 2) as f32).floor() as i32 == x)); // 5
    spells.push(Box::new(|x, y| (x < 10 || y < 10 ))); // 6
    spells.push(Box::new(|x, y| x > 15 && y > 15 )); //7
    spells.push(Box::new(|x, y| x * y == 0 )); //8
    spells.push(Box::new(|x, y| (x - y).abs() > 10 )); //9
    spells.push(Box::new(|x, y| ((y / (x + 1)) as f32).floor() as i32 == 1 )); //10
    spells.push(Box::new(|x, y| x == 1 || y == 1 || x == max_x - 2 || y == max_y - 2 )); //11
    spells.push(Box::new(|x, y| x*x + y*y <= 400 )); //12
    spells.push(Box::new(|x, y| x + y >= 20 && x + y <= 28 )); //13
    spells.push(Box::new(|x, y| x*y <= 100 )); //14
    spells.push(Box::new(|x, y| (x - y).abs() >= 10 && (x - y).abs() <= 20 )); //15
    spells.push(Box::new(|x, y| (x - 12).abs() + (y - 12).abs() < 10 )); //16
    spells.push(Box::new(|x, y| ((y / 3) as f32).sin()  <=  ((x/8) as f32) - 2_f32 )); //17 ?
    spells.push(Box::new(|x, y| x * y < x + y )); //18
    spells.push(Box::new(|x, y| x * y == 0 || x == max_x - 1 || y == max_y - 1 )); //19
    spells.push(Box::new(|x, y| (x + y) % 2 == 0 )); //20
    spells.push(Box::new(|x, y| y % (x + 1) == 0 )); //21
    spells.push(Box::new(|x, y| (x + y) % 3 == 0 )); //22
    spells.push(Box::new(|x, y| x % 3 + y % 2 == 0 )); //23
    spells.push(Box::new(|x, y| x == y || x == 24 - y )); //24
    spells.push(Box::new(|x, y| x % 6 == 0 || y % 6 == 0 )); //25

    let mut selected_spell: usize = 1;

    while selected_spell != 0 {
        let mut input_string = String::new(); // Создаем изменяемую (mutable) пустую строку
        println!("Please enter number of spell: ");
        io::stdin().read_line(&mut input_string)
            .expect("Не удалось прочитать строку"); // Обрабатываем возможную ошибку
        selected_spell = match input_string.trim().parse() { // Используем match для обработки результата parse
            Ok(num) => num, // Если успешно, возвращаем число
            Err(_) => { // Если произошла ошибка (не число)
                println!("Ошибка: Введено не число!");
                0 // Возвращаем 0, можно также завершить программу
            }
        };

        clearscreen::clear().unwrap(); // Очистка
        
        match selected_spell {
            1 => println!("{}", "Welcome to Hogwarts, JUNIOR!".green()),
            1..=4 => println!("{}", "Welcome to Askaban, MIDDLE!".yellow()),
            5..=14 => println!("{}","What a hall are you doing here, SENIOR?!".black()),
            15..=25 => println!("{}","Who are you?!".red()),
            _ => break, 
        }
        
        for x in 0 .. max_x {
            for y in 0 .. max_y {
                if spells[selected_spell - 1](x, y) {
                    print!("{}", "#".yellow());
                }
                else {
                    print!("{}", ".".blue());
                }
            }
            println!("");
        } 
    } 
}

