#include "myMath.h"

#define MAX_LONG_LONG_LOG_10 18

int64_t tenPowll(unsigned long long Exp)
{
    if (Exp > MAX_LONG_LONG_LOG_10)
    {
        throw std::out_of_range("Too large an Exponent.");
    }
    int64_t count = 10;
    for (int i = 1; i < Exp; i++)
    {
        count *= 10;
    }
    return count;
}
