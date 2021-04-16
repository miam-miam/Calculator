#include "number.h"
#include "myMath.h"


Number::Number()
{
    type = INTEGER_TYPE;
    integer = 0;
}

Number::Number(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen)
{
    type = FRACTION_TYPE;
    fraction = {GivenInt, GivenNum, GivenDen};
    fraction.normalise();
    if (fraction.numerator == 0)
    {
        integer = fraction.integer;
        type = INTEGER_TYPE;
    }
}

Number::Number(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen)
{
    type = FRACTION_TYPE;
    fraction = {GivenNum, GivenDen};
    if (fraction.numerator == 0)
    {
        integer = fraction.integer;
        type = INTEGER_TYPE;
    }
}

Number::Number(Fraction GivenFraction)
{
    fraction = GivenFraction;
    if (fraction.numerator == 0)
    {
        integer = fraction.integer;
        type = INTEGER_TYPE;
    }
    else
    {
        type = FRACTION_TYPE;
    }
}

Number::Number(SafeInt<int64_t> GivenInt)
{
    type = INTEGER_TYPE;
    integer = GivenInt;
}

Number::Number(const std::string_view &Number, int Offset)  // Assumes there is only one decimal point
// For Offset -2 means not searched and -1 not found
{
    if (Offset == -2)
    {
        Offset = Number.find('.');
    }
    try
    {
        if (Offset != -1)
        {
            type = FRACTION_TYPE;
            fraction.integer = std::stoll(Number.substr(0, Offset).data());
            fraction.numerator = std::stoll(Number.substr(Offset + 1).data());
            if (fraction.numerator == 0)
            {
                type = INTEGER_TYPE;
                integer = fraction.integer;
                return;
            }
            if (fraction.integer < 0)
            {
                fraction.numerator = -fraction.numerator;
            }
            fraction.denominator = tenPowll(Number.length() - (Offset + 1));
        
            fraction.normalise();
        }
        else
        {
            type = INTEGER_TYPE;
            integer = std::stoll(Number.data());
        }
    }
    catch (const std::out_of_range &e)
    {
        try
        {
            type = DOUBLE_TYPE;
            double_num = std::stod(Number.data());
        }
        catch (const std::out_of_range &e)
        {
            if (Number[0] == '0')
            {
                type = INTEGER_TYPE;
                integer = 0;
            }
            else
            {
                throw Overflow();
            }
        }
    }
}

Number::Number(Fraction GivenMultiplicand, SimpleFraction GivenBase, SimpleFraction GivenExponent)
{
    type = POWER_TYPE;
    power = Power(GivenMultiplicand, GivenBase, GivenExponent);
}

Number::Number(Fraction GivenMultiplicand, SafeInt<int64_t> GivenBase, SimpleFraction GivenExponent)
{
    type = POWER_TYPE;
    power = Power(GivenMultiplicand, GivenBase, GivenExponent);
}

Number::operator double() const
{
    switch (type)
    {
        case INTEGER_TYPE:
        {
            return double(integer);
        }
        case FRACTION_TYPE:
        {
            return double(fraction);
        }
        case DOUBLE_TYPE:
        {
            return double_num;
        }
        case POWER_TYPE:
        {
            return double(power);
        }
    }
}

Number& Number::operator=(const Number& Other)
{
    type = Other.type;
    switch (type)
    {
        case INTEGER_TYPE:
        {
            integer = Other.integer;
            break;
        }
        case FRACTION_TYPE:
        {
            fraction = Other.fraction;
            break;
        }
        case DOUBLE_TYPE:
        {
            double_num = Other.double_num;
            break;
        }
        case POWER_TYPE:
        {
            power = Other.power;
            break;
        }
    }
    return *this;
}

Number Number::operator+(const Number N1) const
{
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(integer + N1.integer);
        }
        if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(fraction.integer + N1.integer, fraction.numerator, fraction.denominator);
        }
        if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(integer + N1.fraction.integer, N1.fraction.numerator, N1.fraction.denominator);
        }
        if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(fraction + N1.fraction);
        }
        if (type == POWER_TYPE && N1.type == POWER_TYPE && power.exponent == N1.power.exponent && power.base == N1.power.base)
        {
            return Number(power.multiplicand + N1.power.multiplicand, power.base, power.exponent);
        }
    }
    catch (const SafeIntException& err) {}
    
    Number result;
    result.type = DOUBLE_TYPE;
    std::feclearexcept(FE_OVERFLOW);
    result.double_num = double(*this) + double(N1);
    if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
    {
        throw Overflow();
    }
    return result;
}

Number Number::operator-(const Number N1) const
{
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(integer - N1.integer);
        }
        if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(fraction.integer - N1.integer, fraction.numerator, fraction.denominator);
        }
        if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(integer - N1.fraction.integer, -N1.fraction.numerator, N1.fraction.denominator);
        }
        if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(fraction - N1.fraction);
        }
        if (type == POWER_TYPE && N1.type == POWER_TYPE && power.exponent == N1.power.exponent && power.base == N1.power.base)
        {
            return Number(power.multiplicand - N1.power.multiplicand, power.base, power.exponent);
        }
    }
    catch (const SafeIntException& err) {}
    Number result;
    result.type = DOUBLE_TYPE;
    std::feclearexcept(FE_OVERFLOW);
    result.double_num = double(*this) - double(N1);
    if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
    {
        throw Overflow();
    }
    return result;
}

Number Number::operator*(const Number N1) const
{
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(integer * N1.integer);
        }
        if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(fraction * N1.integer);
        }
        if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(fraction * N1.integer);
        }
        if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(N1.fraction * fraction);
        }
        if (type == POWER_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(power.multiplicand * N1.integer, N1.power.base, N1.power.exponent);
        }
        if (type == INTEGER_TYPE && N1.type == POWER_TYPE)
        {
            return Number(N1.power.multiplicand * integer, N1.power.base, N1.power.exponent);
        }
        if (type == POWER_TYPE && N1.type == FRACTION_TYPE)
        {
            return Number(power.multiplicand * N1.fraction, power.base, power.exponent);
        }
        if (type == FRACTION_TYPE && N1.type == POWER_TYPE)
        {
            return Number(N1.power.multiplicand * fraction, N1.power.base, N1.power.exponent);
        }
        if (type == POWER_TYPE && N1.type == POWER_TYPE)
        {
            if(power.base == N1.power.base)
            {
                return Number(power.multiplicand * N1.power.multiplicand, power.base, power.exponent + N1.power.exponent);
            }
            else if(power.exponent == N1.power.exponent)
            {
                return Number(power.multiplicand * N1.power.multiplicand, power.base * N1.power.base, power.exponent);
            }
        }
    }
    catch (const SafeIntException& err) {}
    Number result;
    result.type = DOUBLE_TYPE;
    std::feclearexcept(FE_OVERFLOW);
    result.double_num = double(*this) * double(N1);
    if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
    {
        throw Overflow();
    }
    return result;
}

Number Number::operator/(const Number N1) const
{
    const auto doubleN1 = double(N1);
    if (doubleN1 == 0)
    {
        throw DivisionByZero();
    }
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            return Number(integer, N1.integer);
        }
        if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            const auto numerator = fraction.numerator + N1.integer * fraction.denominator;
            const auto denominator = N1.integer * fraction.denominator;
            return Number(numerator, denominator);
        }
        if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            const auto numerator = integer * N1.fraction.denominator;
            const auto denominator = N1.fraction.numerator + integer * N1.fraction.denominator;
            return Number(numerator, denominator);
        }
        if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            const auto numerator = fraction.integer * fraction.denominator * N1.fraction.denominator + N1.fraction.denominator * fraction.numerator;
            const auto denominator = N1.fraction.denominator * fraction.denominator * N1.fraction.integer + fraction.denominator * N1.fraction.numerator;
            return Number(numerator, denominator);
        }
        //TODO power types
    }
    catch (const SafeIntException& err){}
    
    Number result;
    result.type = DOUBLE_TYPE;
    std::feclearexcept(FE_OVERFLOW);
    result.double_num = double(*this) / doubleN1;
    if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
    {
        throw Overflow();
    }
    return result;
}

std::ostream &operator<<(std::ostream &Strm, const Number &N1)
{
    if (N1.type == Number::INTEGER_TYPE)
    {
        Strm << int64_t(N1.integer);
    }
    else if (N1.type == Number::FRACTION_TYPE && N1.fraction.denominator == 1)
    {
        assert(!Strm << int64_t(N1.fraction.integer));
    }
    else if (N1.type == Number::FRACTION_TYPE)
    {
        Strm << int64_t(N1.fraction.integer) << "+" << int64_t(N1.fraction.numerator) << "/" << int64_t(N1.fraction.denominator);
    }
    else if (N1.type == Number::DOUBLE_TYPE)
    {
        Strm << N1.double_num;
    }
    else if (N1.type == Number::POWER_TYPE)
    {
        Strm << "(" << int64_t(N1.power.multiplicand.integer) << "+" << int64_t(N1.power.multiplicand.numerator) << "/" << int64_t(N1.power.multiplicand.denominator) << ")*(" << int64_t(N1.power.base.numerator) << "/" << int64_t(N1.power.base.denominator) <<  ")^(" << int64_t(N1.power.exponent.numerator) << "/" << int64_t(N1.power.exponent.denominator) << ")";
    }
    else
    {
        assert(0);
    }
    return Strm;
}

//TODO write a normalise for power
