#include "dataTypes.h"

SimpleFraction::SimpleFraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen)
{
    numerator = GivenNum;
    denominator = GivenDen;
    normalise();
    numerator += GivenInt * GivenDen;
}

SimpleFraction::SimpleFraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen)
{
    numerator = GivenNum;
    denominator = GivenDen;
    normalise();
}

SimpleFraction SimpleFraction::operator+(SimpleFraction S1) const
{
    return SimpleFraction(numerator * S1.denominator + S1.numerator * denominator, denominator * S1.denominator);
}

SimpleFraction SimpleFraction::operator-(SimpleFraction S1) const
{
    return SimpleFraction(numerator * S1.denominator - S1.numerator * denominator, denominator * S1.denominator);
}

SimpleFraction SimpleFraction::operator*(SimpleFraction S1) const
{
    return SimpleFraction(numerator * S1.numerator, denominator * S1.denominator);
}

void SimpleFraction::normalise()
{
    if (denominator == 0)
    {
        throw DivisionByZero();
    }
    if (denominator < 0)
    {
        numerator *= -1;
        denominator *= -1;
    }
    if (denominator == 1)
    {
        return;
    }
    if (numerator == 0)
    {
        denominator = 1;
        return;
    }
    
    const SafeInt<int64_t> gcd = std::gcd((int64_t) numerator, (int64_t) denominator);
    
    if (gcd != 1)
    {
        numerator = numerator / gcd;
        denominator = denominator / gcd;
    }
}



Fraction::Fraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen)    // Not unused
{
    integer = GivenInt;
    numerator = GivenNum;
    denominator = GivenDen;
}

Fraction::Fraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen)  // Not unused
{
    integer = 0;
    numerator = GivenNum;
    denominator = GivenDen;
    normalise();
}

Fraction Fraction::operator+(Fraction F1) const
{
    Fraction result;
    result.integer = integer + F1.integer;
    result.numerator = F1.denominator * numerator + F1.numerator * denominator;
    result.denominator = F1.denominator * denominator;
    result.normalise();
    return result;
}

Fraction Fraction::operator-(Fraction F1) const
{
    Fraction result;
    result.integer = integer - F1.integer;
    result.numerator = F1.denominator * numerator - F1.numerator * denominator;
    result.denominator = F1.denominator * denominator;
    result.normalise();
    return result;
}

Fraction Fraction::operator*(Fraction F1) const
{
    Fraction result;
    result.integer = F1.integer * integer;
    result.numerator = F1.numerator * numerator + F1.denominator * F1.integer * numerator + denominator * integer * F1.numerator;
    result.denominator = F1.denominator * denominator;
    result.normalise();
    
    return result;
}

void Fraction::normalise()
{
    if (denominator == 0)
    {
        throw DivisionByZero();
    }
    
    if (denominator < 0)
    {
        numerator *= -1;
        denominator *= -1;
    }
    
    if (denominator == 1)
    {
        integer += numerator;
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
    
    const SafeInt<int64_t> gcd = std::gcd((int64_t) numerator, (int64_t) denominator);
    
    if (gcd != 1)
    {
        numerator = numerator / gcd;
        denominator = denominator / gcd;
    }
    
}



Power::Power(Fraction GivenMultiplicand, SimpleFraction GivenBase, SimpleFraction GivenExponent)
{
    multiplicand = GivenMultiplicand;
    base = GivenBase;
    exponent = GivenExponent;
}

Power::Power(Fraction GivenMultiplicand, SafeInt<int64_t> GivenBase, SimpleFraction GivenExponent)
{
    multiplicand = GivenMultiplicand;
    base = SimpleFraction(GivenBase, 1);
    exponent = GivenExponent;
}

Power::Power()
{
    multiplicand = Fraction(1,0,1);
    base = SimpleFraction(1,1);
    exponent = SimpleFraction(1,1);
}

Power::operator double() const
{
    std::feclearexcept(FE_OVERFLOW);
    const double result = pow(double (base) , double (exponent));
    if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
    {
        throw Overflow();
    }
    return result;
}