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

struct Spells {
    max_x: i32,
    max_y: i32,
    spells: Vec<Box<dyn Fn(i32, i32) -> bool>>,
}

impl Default for Spells {
    fn default() -> Self {
        Spells {
            max_x: 25,
            max_y: 25,
            spells:  vec!( Box::new(|x, y| x < y )),           
        }
    }
}

impl Spells {
    pub fn get_max_x(&self) -> i32
    {
        self.max_x
    }

    pub fn get_max_y(&self) -> i32
    {
        self.max_y
    }
    
    pub fn new(_max_x: &'static i32, _max_y: &'static i32) -> Self{
        Self {
            max_x : *_max_x,
            max_y : *_max_y,
        //JUNIOR
            spells: vec!( Box::new(|x, y| x < y ), //1
        //MIDDLE
            Box::new(|x, y| x == y ), //2
            Box::new(|x, y| x == *_max_x - 1 - y ),//3
            Box::new(|x, y| x + y < 30 ), //4
            Box::new(|x, y| ((y / 2) as f32).floor() as i32 == x), // 5
            Box::new(|x, y| (x < 10 || y < 10 )), // 6
            Box::new(|x, y| x > 15 && y > 15 ), //7
            Box::new(|x, y| x * y == 0 ), //8
            Box::new(|x, y| (x - y).abs() > 10 ), //9
            Box::new(|x, y| ((y / (x + 1)) as f32).floor() as i32 == 1 ), //10
            Box::new(|x, y| x == 1 || y == 1 || x == *_max_x - 2 || y == *_max_y - 2 ), //11
            Box::new(|x, y| x*x + y*y <= 400 ), //12
            Box::new(|x, y| x + y >= 20 && x + y <= 28 ), //13
            Box::new(|x, y| x*y <= 100 ), //14
            Box::new(|x, y| (x - y).abs() >= 10 && (x - y).abs() <= 20 ), //15
            Box::new(|x, y| (x - 12).abs() + (y - 12).abs() < 10 ), //16
            Box::new(|x, y| ((y / 3) as f32).sin()  <=  ((x/8) as f32) - 2_f32 ), //17 ?
            Box::new(|x, y| x * y < x + y ), //18
            Box::new(|x, y| x * y == 0 || x == *_max_x - 1 || y == *_max_y - 1 ), //19
            Box::new(|x, y| (x + y) % 2 == 0 ), //20
            Box::new(|x, y| y % (x + 1) == 0 ), //21
            Box::new(|x, y| (x + y) % 3 == 0 ), //22
            Box::new(|x, y| x % 3 + y % 2 == 0 ), //23
            Box::new(|x, y| x == y || x == 24 - y ), //24
            Box::new(|x, y| x % 6 == 0 || y % 6 == 0 ), //25
            ),
        }
        
    }

    pub fn get_spell_string(&self, i: usize) -> String {
        let mut out_string: String = String::new();
        for x in 0 .. self.max_x {
            for y in 0 .. self.max_y {
                if self.spells[i - 1](x, y) {
                    out_string.push('#');
                }
                else {
                    out_string.push('.');
                }
            }
            out_string.push('\n');
        }
        return out_string; 
    }


    pub fn draw_spell(&self, i: usize) {
        clearscreen::clear().unwrap(); // Очистка
        match i {
            1 => println!("{}", "Welcome to Hogwarts, JUNIOR!".green()),
            1..=4 => println!("{}", "Welcome to Askaban, MIDDLE!".yellow()),
            5..=14 => println!("{}","What a hall are you doing here, SENIOR?!".black()),
            15..=25 => println!("{}","Who are you?!".red()),
            _ => println!("{}", "Wrong spell".cyan()),
        }
        for x in 0 .. self.max_x {
            for y in 0 .. self.max_y {
                if self.spells[i - 1](x, y) {
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


fn main() {
    
 
    let mut selected_spell: usize = 1;
    let spells = Spells::new(&25, &25);

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
       
        spells.draw_spell(selected_spell);      
    } 
}



#[cfg(test)]
mod tests {
    #[test]
    fn check_spells() {
        let test_spells = super::Spells::new(&25, &25);
        let i = 25;
        let path = "test\" + i.to_string() + ".txt";
        match std::fs::read_to_string(path)  {
            Ok(content) => assert_eq!(content, test_spells.get_spell_string(i)),
            Err(e) => eprint!("{:?}", e)        
        }
        

        for i in 0 .. 25
        {
            todo!();
        }
    }
}