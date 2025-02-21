#include "tickets.h"
#include <iostream>
#include <string> 
#include <fstream>
#include <chrono>

//#include "version.h"
/**
 * happy_tickets program OTUS algo hw 02
 */

bool TestOut(unsigned long out, unsigned long count)
{
    if (out == count) 
    {
        std::cout << "Ok" << std::endl;
        return true;
    }
    else 
    {
        std::cout << "Err" << std::endl;
        return false;
    }
}

int main() {
    std::string s = "";
    std::ifstream file;
    int in;
    unsigned long out;
    using namespace std::literals;
    const std::chrono::time_point<std::chrono::steady_clock> start =
        std::chrono::steady_clock::now();

    std::string path = "1.Tickets/test.";
    for (size_t i = 0; i < 10; i++)
    {
        file.open(path + std::to_string(i) + ".in" );
        getline(file, s);
        in = std::stol(s);
        file.close();
        s = "";
        file.open(path + std::to_string(i) + ".out" );
        getline(file, s);
        out = std::stol(s);
        file.close();
        s = "";
        FastTickets Ftickets(in);
        unsigned long count = Ftickets.GetHappyCount();
        TestOut(out, count);
        std::cout << "cpp fast count: " << in << " : " << count << std::endl;
    }
    const auto end = std::chrono::steady_clock::now();

    std::cout << "Time : " << (end - start) / 1s  << " sec" << std::endl;
    return 0;
}
