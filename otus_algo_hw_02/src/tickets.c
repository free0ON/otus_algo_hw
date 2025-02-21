#include <math.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>

unsigned long get_sum_number(unsigned long Number, int N)
{
    unsigned long Sum = 0;
        for (int i = N; i >= 0; i--) {
            Sum += Number % 10;
            Number /= 10;
        }
    return Sum;
}

unsigned long brut_get_happy_count(int N)
{
    unsigned long Count = 0;
    unsigned long MaxNum = pow(10,N);
    for(unsigned long left = 0; left < MaxNum; left++)
        for(unsigned long right = 0; right < MaxNum; right++)
            if(get_sum_number(left, N) == get_sum_number(right, N)) Count++;
    return Count;
}


unsigned long force_get_happy_count(int N)
{
    unsigned long Count = 0;
    unsigned long MaxNumber = pow(10,N);
    unsigned long SumsNumber = N*9 + 1;
    unsigned long SumsArray[100];
    for (int i = 0; i < SumsNumber; i++)
            SumsArray[i] = 0;
    for (unsigned long Number = 0; Number < MaxNumber; Number++)
        SumsArray[get_sum_number(Number, N)]++;

    for (int i = 0; i < SumsNumber; i++)
        Count += SumsArray[i] * SumsArray[i];

    return Count;
}

int test_brut_get_happy_count(void) 
{
    int result = -1;
    unsigned int in = 0;
    unsigned long out = 0;
    char filename[255];
    clock_t start = clock(), diff;
    for (unsigned int i = 0; i < 5; i++)
    {
        result = -1;
        FILE *file;
        if (sprintf(filename, "1.Tickets//test.%u.in",i) <= 0)
            return result;
        file = fopen(filename, "r");
        if (file == NULL) 
            return result;
        char in_str[100];
        if(fgets(in_str, 100, file) == NULL)
            return result; 
        in = atoi(in_str);
        fclose(file);
        sprintf(filename, "1.Tickets//test.%u.out",i);
        file = fopen(filename, "r");
        if (file == NULL) 
            return result;
        char out_str[100];
        if(fgets(out_str, 100, file) == NULL)
            return result; 
        out = atol(out_str);
        fclose(file);
        long count = brut_get_happy_count(in);
        printf("brut count: %d  %lu \n", in, count);
        if ((in == i+1) && (count == out)) 
            result = 1;
        else 
            result = 0;
    }
    diff = clock() - start;

    int msec1 = diff * 1000 / CLOCKS_PER_SEC;
    printf("Time taken %d seconds %d milliseconds \n", msec1/1000, msec1%1000);
    return result;
}

int test_fast_get_happy_count(void) 
{
    int result = -1;
    unsigned int in = 0;
    unsigned long out = 0;
    char filename[255];
    clock_t start = clock(), diff;
    for (unsigned int i = 0; i < 10; i++)
    {
        result = -1;
        FILE *file;
        if (sprintf(filename, "1.Tickets//test.%u.in",i) <= 0)
            return result;
        file = fopen(filename, "r");
        if (file == NULL) return result;
        char in_str[100];
        if(fgets(in_str, 100, file) == NULL)
            return result; 
        in = atoi(in_str);
        fclose(file);
        sprintf(filename, "1.Tickets//test.%u.out",i);
        file = fopen(filename, "r");
        if (file == NULL) return result;
        char out_str[100];
        if(fgets(out_str, 100, file) == NULL)
            return result; 
        out = atol(out_str);
        fclose(file);
        long count = force_get_happy_count(in);
        printf("c fast count: %d  %lu \n", in, count);
        if ((in == i+1) && (count == out)) 
            result = 1;
        else 
            result = 0;
    }
    diff = clock() - start;

    int msec1 = diff * 1000 / CLOCKS_PER_SEC;
    printf("Time taken %d seconds %d milliseconds \n", msec1/1000, msec1%1000);
    return result;
}



int main(void)
{
    switch (test_fast_get_happy_count())
    {
    case 1:
        printf("OK");
        break;
    case 0:
        printf("Not OK");
        break;
    case -1:
        printf("Error");
        break;
    default:
        printf("Fatal");
        break;
    }

    return 0;
}