#include <cstdint>
#include "myMath.h"

int64_t powll(int64_t Base, int Exp)
{
    int64_t count = Base;
    for (int i = 1; i < Exp; i++)
    {
        count *= Base;
    }
    return count;
}
