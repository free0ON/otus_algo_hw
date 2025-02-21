#include <gtest/gtest.h>
#include "tickets.h"
#include <iostream>
#include <string>
#include <fstream> 

/**
 * Google tests
 */
// TEST(tickets, BrutTickets) {
//     std::string s = "";
//     std::ifstream file;
//     int in[10];
//     unsigned long out[10];
//     std::string path = "1.Tickets/test.";
//     for (size_t i = 0; i < 5; i++)
//     {
//         file.open(path + std::to_string(i) + ".in" );
//         getline(file, s);
//         in[i] = std::stol(s);
//         file.close();
//         ASSERT_EQ(in[i],i+1) << s;
//         s = "";
//         file.open(path + std::to_string(i) + ".out" );
//         getline(file, s);
//         out[i] = std::stol(s);
//         file.close();
//         s = "";
//         BrutTickets tickets(in[i]);
//         ASSERT_EQ(tickets.GetHappyCount(),out[i]);
//     }
// }

TEST(tickets, FastTickets) {
    std::string s = "";
    std::ifstream file;
    int in;
    unsigned long out;
    std::string path = "1.Tickets/test.";
    for (size_t i = 0; i < 10; i++)
    {
        file.open(path + std::to_string(i) + ".in" );
        getline(file, s);
        in = std::stol(s);
        file.close();
        ASSERT_EQ(in,i+1) << s;
        s = "";
        file.open(path + std::to_string(i) + ".out" );
        getline(file, s);
        out = std::stol(s);
        file.close();
        s = "";
        FastTickets Ftickets(in);
        ASSERT_EQ(Ftickets.GetHappyCount(),out);
    }
}
