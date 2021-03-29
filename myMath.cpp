#include <cstdint>
#include "myMath.h"


int64_t powll(int64_t base, int exp)
{
    int64_t count = base;
    for (int i = 1; i < exp; i++)
    {
        count *= base;
    }
    return count;
}
