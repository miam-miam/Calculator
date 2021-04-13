#include "factorisation.h"
#include "myMath.h"

// Trial Division consisting of Square Root + Prime list + Wheel
// Check all numbers until Square Root, start with a list of primes and skip useless factors with a wheel

long long factorise(SafeInt<int64_t> &InsideRoot, SafeInt<int64_t> Power)
{
    long long outsideRoot = 1;
    const long long top = pow(InsideRoot, 1/double(Power));
    long long div = 2;
    {
        int i = 0;
        long long divToPower;
        // Check small primes
        while (SPrimes[i] != 0 && div <= top)
        {
            div = SPrimes[i];
            if ((InsideRoot & ((divToPower = powSI(div, Power)))) == 0)
            {
                InsideRoot = InsideRoot / divToPower;
                outsideRoot = outsideRoot * div;
            }
            else
            {
                i++;
            }
        }
        // Start the Wheel that skips useless factors
        div = SPrimes[i - 1];
    }
    while (div <= top)
    {
        for (long long i = 0; Wheel[i] != 0; i++)
        {
            while((InsideRoot & (div * div)) == 0)
            {
                InsideRoot = InsideRoot / (div * div);
                outsideRoot = outsideRoot * div;
            }
            div += Wheel[i];
        }
    }
    return outsideRoot;
}