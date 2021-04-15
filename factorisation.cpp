#include "factorisation.h"
#include "myMath.h"

// Trial Division consisting of Square Root + Prime list + Wheel
// Check all numbers until Square Root, start with a list of primes and skip useless factors with a wheel

long long factorise(SafeInt<int64_t> &InsideRoot, SafeInt<int64_t> Power)
{
    long long outsideRoot = 1;
    if (Power == 1)
    {
        outsideRoot = InsideRoot;
        InsideRoot = 1;
        return outsideRoot;
    }
    long long top = pow(InsideRoot, 1/double(Power));
    long long div;
    long long divToPower;
    {
        int i = 0;
        // Check small primes
        while (SPrimes[i] != 0 && SPrimes[i] <= top)
        {
            div = SPrimes[i];
            if ((InsideRoot % ((divToPower = powSI(div, Power)))) == 0)
            {
                InsideRoot = InsideRoot / divToPower;
                outsideRoot = outsideRoot * div;
                top = pow(InsideRoot, 1/double(Power));
            }
            else
            {
                i++;
            }
        }
        // Start the Wheel that skips useless factors
        div = 601;
    }
    while (div <= top)
    {
        for (long long i = 0; Wheel[i] != 0; i++)
        {
            div += Wheel[i];
            while((InsideRoot % (divToPower = powSI(div, Power))) == 0)
            {
                InsideRoot = InsideRoot / divToPower;
                outsideRoot = outsideRoot * div;
                top = pow(InsideRoot, 1/double(Power));
            }
        }
    }
    return outsideRoot;
}