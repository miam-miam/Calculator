#include <string>
#include <numeric>
#include <iostream>
#include "fraction.h"
#include "myMath.h"

fraction::fraction(int64_t GivenInt, int64_t GivenNum, int64_t GivenDen)
{
    integer = GivenInt;
    numerator = GivenNum;
    denominator = GivenDen;
}

fraction::fraction(int64_t GivenNum, int64_t GivenDen)
{
    integer = 0;
    numerator = GivenNum;
    denominator = GivenDen;
    normalise();
}

fraction fraction::operator-(const fraction F1) const
{
    fraction Temp;
    Temp.integer = integer - F1.integer;
    if (denominator == 1 && F1.denominator == 1)
    {
        return Temp;
    }
    Temp.denominator = F1.denominator * denominator;
    Temp.numerator = F1.denominator * numerator - F1.numerator * denominator;
    Temp.normalise();
    return Temp;
}

fraction fraction::operator+(const fraction F1) const
{
    fraction Temp;
    Temp.integer = F1.integer + integer;
    if (denominator == 1 && F1.denominator == 1)
    {
        return Temp;
    }
    Temp.denominator = F1.denominator * denominator;
    Temp.numerator = F1.numerator * denominator + F1.denominator * numerator;
    Temp.normalise();
    return Temp;
}

fraction fraction::operator*(const fraction F1) const
{
    fraction Temp;
    Temp.integer = F1.integer * integer;
    if (denominator == 1 && F1.denominator == 1)
    {
        return Temp;
    }
    Temp.denominator = F1.denominator * denominator;
    Temp.numerator =
        F1.numerator * numerator + F1.denominator * F1.integer * numerator + denominator * integer * F1.numerator;
    Temp.normalise();
    return Temp;
}

fraction fraction::operator/(const fraction F1) const
{
    fraction Temp;
    Temp.denominator = F1.denominator * denominator * F1.integer + denominator * F1.numerator;
    Temp.numerator = integer * denominator * F1.denominator + F1.denominator * numerator;
    Temp.normalise();
    return Temp;
}

void fraction::normalise()
{
    
    if (denominator == 1)
    {
        integer = numerator;
        numerator = 0;
        return;
    }
    
    if (numerator >= denominator)
    {
        integer += numerator / denominator;
        numerator -= (numerator / denominator) * denominator;
    }
    
    if (numerator == 0)
    {
        denominator = 1;
        return;
    }
    
    const int64_t Gcd = std::gcd((int64_t) numerator, (int64_t) denominator);
    
    if (Gcd != 1)
    {
        numerator = numerator / Gcd;
        denominator = denominator / Gcd;
    }
    
}

fraction::fraction(const std::string_view &Number)  // Assumes there is only one decimal point
{
    const std::size_t Offset = Number.find('.');    // TODO: Test for ','
    if (Offset != std::string::npos)
    {
        integer = std::stoll(Number.substr(0, Offset).data());
        numerator = std::stoll(Number.substr(Offset + 1).data());
        if (integer < 0)
        {
            numerator = -numerator;
        }
        
        denominator = powll(10LL, Number.length() - (Offset + 1));
        
        normalise();
    }
    else
    {
        integer = std::stoll(Number.data());
        denominator = 1;
        numerator = 0;
    }
    
}

std::ostream &operator<<(std::ostream &Strm, const fraction &F1)
{
    if (F1.denominator == 1)
    {
        Strm << F1.numerator;
    }
    else
    {
        Strm << F1.integer << "+" << F1.numerator << "/" << F1.denominator;
    }
    return Strm;
}

