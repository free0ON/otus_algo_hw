#include <math.h>
#include <iostream>
#include <cassert>
/// @brief Happy tickets count Brut force algiritm
class BrutTickets
{
    private:
        int N;

    public:

    BrutTickets(int _N)
    {
        N = _N;
    }

    unsigned long GetSumNumber(unsigned long Number)
    {
        unsigned long Sum = 0;
        for (int i = N; i >= 0; i--) {
            Sum += Number % 10;
            Number /= 10;
        }
        return Sum;
    }

    unsigned long GetHappyCount()
    {
        unsigned long Count = 0;
        unsigned long MaxNum = pow(10,N);
        for(unsigned long left = 0; left < MaxNum; left++)
            for(unsigned long right = 0; right < MaxNum; right++)
                if(GetSumNumber(left) == GetSumNumber(right)) Count++;
    return Count;

    }
}; 

/// @brief Happy tickets count Fast algoritm
class FastTickets
{
    private:
        int N;
        int SumsNumber;
        unsigned long* SumsArray;
    public:

    FastTickets(int _N)
    {
        N = _N;
        SumsNumber = N*9 + 1;
        SumsArray = new unsigned long[SumsNumber];
        for (int i = 0; i < SumsNumber; i++)
            SumsArray[i] = 0;
    }

    ~FastTickets()
    {
        delete[] SumsArray;
    }


    unsigned long GetSumNumber(unsigned long Number)
    {
        unsigned long Sum = 0;
            for (int i = N; i >= 0; i--) {
                Sum += Number % 10;
                Number /= 10;
            }
        return Sum;
    }

    unsigned long GetHappyCount(void)
    {
        unsigned long Count = 0;
        unsigned long MaxNumber = pow(10,N);
        for (unsigned long Number = 0; Number < MaxNumber; Number++)
            SumsArray[GetSumNumber(Number)]++;

        for (int i = 0; i < SumsNumber; i++)
            Count += SumsArray[i] * SumsArray[i];
    
        return Count;
    }
};