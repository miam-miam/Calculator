#include <cstdint>
#include "myMath.h"

int64_t powll(int64_t Base, int Exp)
{
    int64_t Count = Base;
    for (int I = 1; I < Exp; I++)
    {
        Count *= Base;
    }
    return Count;
}
