#include <string>
#include <numeric>
#include <iostream>
#include "fraction.h"
#include "myMath.h"


fraction::fraction(int64_t givenInt, int64_t givenNum, int64_t givenDen)
{
    integer = givenInt;
    numerator = givenNum;
    denominator = givenDen;
}

fraction::fraction(int64_t givenNum, int64_t givenDen)
{
    integer = 0;
    numerator = givenNum;
    denominator = givenDen;
    normalise();
}

fraction fraction::operator-(const fraction f1) const
{
    fraction temp;
    temp.integer = integer - f1.integer;
    if (denominator == 1 && f1.denominator == 1) return temp;
    temp.denominator = f1.denominator * denominator;
    temp.numerator = f1.denominator * numerator - f1.numerator * denominator;
    temp.normalise();
    return temp;
}

fraction fraction::operator+(const fraction f1) const
{
    fraction temp;
    temp.integer = f1.integer + integer;
    if (denominator == 1 && f1.denominator == 1) return temp;
    temp.denominator = f1.denominator * denominator;
    temp.numerator = f1.numerator * denominator + f1.denominator * numerator;
    temp.normalise();
    return temp;
}

fraction fraction::operator*(const fraction f1) const
{
    fraction temp;
    temp.integer = f1.integer * integer;
    if (denominator == 1 && f1.denominator == 1) return temp;
    temp.denominator = f1.denominator * denominator;
    temp.numerator =
            f1.numerator * numerator + f1.denominator * f1.integer * numerator + denominator * integer * f1.numerator;
    temp.normalise();
    return temp;
}

fraction fraction::operator/(const fraction f1) const
{
    fraction temp;
    temp.denominator = f1.denominator * denominator * f1.integer + denominator * f1.numerator;
    temp.numerator = integer * denominator * f1.denominator + f1.denominator * numerator;
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

fraction::fraction(const std::string &number)  // Assumes there is only one decimal point
{
    const std::size_t offset = number.find('.');    // TODO: Test for ','
    if (offset != std::string::npos)
    {
        integer = std::stoll(number.substr(0, offset));
        numerator = std::stoll(number.substr(offset + 1));
        if (integer < 0)
        {
            numerator = -numerator;
        }

        denominator = powll(10LL, number.length() - (offset + 1));

        normalise();
    }
    else
    {
        integer = std::stoll(number);
        denominator = 1;
        numerator = 0;
    }

}

std::ostream &operator<<(std::ostream &strm, const fraction &f1)
{
    if (f1.denominator == 1)
    {
        strm << f1.numerator;
    }
    else
    {
        strm << f1.integer << "+" << f1.numerator << "/" << f1.denominator;
    }
    return strm;
}

