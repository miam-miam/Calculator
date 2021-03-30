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
    fraction temp;
    temp.integer = integer - F1.integer;
    if (denominator == 1 && F1.denominator == 1)
    {
        return temp;
    }
    temp.denominator = F1.denominator * denominator;
    temp.numerator = F1.denominator * numerator - F1.numerator * denominator;
    temp.normalise();
    return temp;
}

fraction fraction::operator+(const fraction F1) const
{
    fraction temp;
    temp.integer = F1.integer + integer;
    if (denominator == 1 && F1.denominator == 1)
    {
        return temp;
    }
    temp.denominator = F1.denominator * denominator;
    temp.numerator = F1.numerator * denominator + F1.denominator * numerator;
    temp.normalise();
    return temp;
}

fraction fraction::operator*(const fraction F1) const
{
    fraction temp;
    temp.integer = F1.integer * integer;
    if (denominator == 1 && F1.denominator == 1)
    {
        return temp;
    }
    temp.denominator = F1.denominator * denominator;
    temp.numerator =
        F1.numerator * numerator + F1.denominator * F1.integer * numerator + denominator * integer * F1.numerator;
    temp.normalise();
    return temp;
}

fraction fraction::operator/(const fraction F1) const
{
    fraction temp;
    temp.denominator = F1.denominator * denominator * F1.integer + denominator * F1.numerator;
    temp.numerator = integer * denominator * F1.denominator + F1.denominator * numerator;
    temp.normalise();
    return temp;
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
    
    const int64_t gcd = std::gcd((int64_t) numerator, (int64_t) denominator);
    
    if (gcd != 1)
    {
        numerator = numerator / gcd;
        denominator = denominator / gcd;
    }
    
}

fraction::fraction(const std::string_view &Number)  // Assumes there is only one decimal point
{
    const std::size_t offset = Number.find('.');    // TODO: Test for ','
    if (offset != std::string::npos)
    {
        integer = std::stoll(Number.substr(0, offset).data());
        numerator = std::stoll(Number.substr(offset + 1).data());
        if (integer < 0)
        {
            numerator = -numerator;
        }
        
        denominator = powll(10LL, Number.length() - (offset + 1));
        
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
        Strm << F1.integer;
    }
    else
    {
        Strm << F1.integer << "+" << F1.numerator << "/" << F1.denominator;
    }
    return Strm;
}

