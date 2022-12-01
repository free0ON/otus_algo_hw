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
 */

#include <string>
#include <iostream>

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
    using namespace ConsolePaint;
    char OutSymbol = 0;
    for(int x = 0; x < 25; ++x)
    {
        for(int y = 0; y < 25; ++y)
        {
            if (x < y){
                std::cout << ColorModifier(ColorCode::FG_YELLOW)
                << '#';
            }
            else{
                std::cout << ColorModifier(ColorCode::FG_BLUE)
                << '.';
            }
        }

        std::cout << std::endl;
    }
    getchar();
    return 0;
}