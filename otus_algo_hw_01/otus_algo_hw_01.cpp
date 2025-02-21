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

#include <string>
#include <iostream>
#include <vector>
#include <functional>
#include <cmath>

namespace ConsolePaint {
    enum ColorCode {
        FG_RESET = 0,
        FG_BLACK = 30,
        FG_RED = 31,
        FG_GREEN = 32,
        FG_YELLOW = 33,
        FG_BLUE = 34,
        FG_MAGENTA = 35,
        FG_CYAN = 36,
        FG_WHITE = 37,
        FG_DEFAULT = 39,
        BG_RED = 41,
        BG_GREEN = 42,
        BG_BLUE = 44,
        BG_DEFAULT = 49
    };

    class ColorModifier {
    protected:
        ColorCode color;
    public:
        ColorModifier(ColorCode color) : color(color){}

        friend std::ostream &
        operator<<(std::ostream &os, const ColorModifier &mod) {
            return os << "\033[" + std::to_string(mod.color) + "m";
        }
    };

    class PositionModifier
    {
    protected:
        int x;
        int y;
    public:
        PositionModifier(int x, int y): x(x), y(y){};
        friend std::ostream &
        operator<<(std::ostream &os, const PositionModifier &mod) {
            return os << "\033[" + std::to_string(mod.y) +";"+ std::to_string(mod.x) +"H";
        }
    };

    class ColorPositionModifier: public ColorModifier, PositionModifier
    {
    public:
        ColorPositionModifier(ColorCode color, int x, int y): ColorModifier(color), PositionModifier(x, y) {}

        friend std::ostream &
        operator<<(std::ostream &os, const ColorPositionModifier &mod) {
            return os << "\033[" + std::to_string(mod.color) + "m"
                    << "\033[" + std::to_string(mod.y) +";"+ std::to_string(mod.x) +"H";
        }
    };


}

int main()
{
    int MaxX = 25;
    int MaxY = 25;
    std::vector<std::function<bool(int, int)>> spells;
    //JUNIOR
    spells.push_back([](int x, int y){return x < y; }); //1
    //MIDDLE
    spells.push_back([](int x, int y){return x == y; }); //2
    spells.push_back([MaxY](int x, int y){return x == MaxY - 1 - y; }); //3
    spells.push_back([](int x, int y){return x + y < 30; }); //4
    //SENIOR
    spells.push_back([](int x, int y){return std::floor(y / 2) == x; }); // 5
    spells.push_back([](int x, int y){return (x < 10 || y < 10); }); // 6
    spells.push_back([](int x, int y){return x > 15 && y > 15; }); //7
    spells.push_back([](int x, int y){return x * y == 0; }); //8
    spells.push_back([](int x, int y){return std::abs(x - y) > 10; }); //9
    spells.push_back([](int x, int y){return std::floor (y / (x + 1)) == 1; }); //10
    spells.push_back([MaxX, MaxY](int x, int y){return x == 1 || y == 1 || x == MaxX - 2 || y == MaxY - 2; }); //11
    spells.push_back([](int x, int y){return x*x + y*y <= 400; }); //12
    spells.push_back([](int x, int y){return x + y >= 20 && x + y <= 28; }); //13
    spells.push_back([](int x, int y){return x*y <= 100; }); //14
    spells.push_back([](int x, int y){return std::abs(x - y) >= 10 && std::abs (x - y) <= 20; }); //15
    spells.push_back([](int x, int y){return std::abs(x - 12) + std::abs(y - 12) < 10; }); //16
    spells.push_back([](int x, int y){return std::sin(double(y)/3) <= double(x)/8 - 2; }); //17 ?
    spells.push_back([](int x, int y){return x * y < x + y; }); //18
    spells.push_back([MaxX, MaxY](int x, int y){return x * y == 0 || x == MaxX - 1 || y == MaxY - 1; }); //19
    spells.push_back([](int x, int y){return (x + y) % 2 == 0; }); //20
    spells.push_back([](int x, int y){return y % (x + 1) == 0; }); //21
    spells.push_back([](int x, int y){return (x + y) % 3 == 0; }); //22
    spells.push_back([](int x, int y){return x % 3 + y % 2 == 0; }); //23
    spells.push_back([](int x, int y){return x == y || x == 24 - y; }); //24
    spells.push_back([](int x, int y){return x % 6 == 0 || y % 6 == 0; }); //25
    using namespace ConsolePaint;
    uint SelectedSpell = 1;
    while (SelectedSpell != 0) {
        std::cout << ColorModifier(FG_RESET) << "Please enter number of spell: ";
        std::cin >> SelectedSpell;
        system("clear");
        if(SelectedSpell == 1)
            std::cout << ColorModifier(FG_GREEN)
            << std::to_string(SelectedSpell)
            <<" Welcome to Hogwarts, JUNIOR!" << std::endl;
        if(SelectedSpell > 1 && SelectedSpell < 5)
            std::cout << ColorModifier(FG_YELLOW)
            << std::to_string(SelectedSpell)
            << " Welcome to Askaban, MIDDLE!" << std::endl;
        if(SelectedSpell >= 5 && SelectedSpell <= 15)
            std::cout << ColorModifier(FG_BLACK)
            << std::to_string(SelectedSpell)
            << " What a hall are you doing here, SENIOR?!" << std::endl;
        if(SelectedSpell > 15)
            std::cout << ColorModifier(FG_RED)
            << std::to_string(SelectedSpell)
            << " Who are you?!" << std::endl;

        if(SelectedSpell == 0 || SelectedSpell > spells.size()) break;
        for (int x = 0; x < MaxX; ++x) {
            for (int y = 0; y < MaxY; ++y) {
                    if (spells[SelectedSpell - 1](x,y))
                        std::cout << ColorModifier(FG_YELLOW)
                                  << '#';
                    else
                        std::cout << ColorModifier(FG_BLUE)
                                  << '.';
                }
            std::cout << std::endl;
            }
    }
    return 0;
}