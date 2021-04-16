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
    type = FRACTION_TYPE;
    fraction = GivenFraction;
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
            return double(power.multiplicand) + pow(double(power.base), double(power.exponent));
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
    Number result;
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            result = Number(integer + N1.integer);
        }
        else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            result = Number(fraction.integer + N1.integer, fraction.numerator, fraction.denominator);
        }
        else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            result = Number(integer + N1.fraction.integer, N1.fraction.numerator, N1.fraction.denominator);
        }
        else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            result.fraction = fraction + N1.fraction;
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
            else
            {
                result.type = FRACTION_TYPE;
            }
        }
        else if (type == POWER_TYPE && N1.type == POWER_TYPE && power.exponent == N1.power.exponent && power.base == N1.power.base)
        {
            result.type = POWER_TYPE;
            result.power.base = power.base;
            result.power.exponent = power.exponent;
            result.power.multiplicand = power.multiplicand + N1.power.multiplicand;
        }
        else
        {
            result.type = DOUBLE_TYPE;
            std::feclearexcept(FE_OVERFLOW);
            result.double_num = double(*this) + double(N1);
            if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
            {
                throw Overflow();
            }
        }
    }
    catch (const SafeIntException& err)
    {
        result.type = DOUBLE_TYPE;
        std::feclearexcept(FE_OVERFLOW);
        result.double_num = double(*this) + double(N1);
        if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
        {
            throw Overflow();
        }
    }
    
    return result;
}

Number Number::operator-(const Number N1) const
{
    Number result;
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            result = Number(integer - N1.integer);
        }
        else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            result = Number(fraction.integer - N1.integer, fraction.numerator, fraction.denominator);
        }
        else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            result = Number(integer - N1.fraction.integer, -N1.fraction.numerator, N1.fraction.denominator);
        }
        else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            result.fraction = fraction - N1.fraction;
            
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
            else
            {
                result.type = FRACTION_TYPE;
            }
        }
        else if (type == POWER_TYPE && N1.type == POWER_TYPE && power.exponent == N1.power.exponent && power.base == N1.power.base)
        {
            result.type = POWER_TYPE;
            result.power.base = power.base;
            result.power.exponent = power.exponent;
            result.power.multiplicand = power.multiplicand - N1.power.multiplicand;
        }
        else
        {
            result.type = DOUBLE_TYPE;
            std::feclearexcept(FE_OVERFLOW);
            result.double_num = double(*this) - double(N1);
            if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
            {
                throw Overflow();
            }
        }
    }
    catch (const SafeIntException& err)
    {
        result.type = DOUBLE_TYPE;
        std::feclearexcept(FE_OVERFLOW);
        result.double_num = double(*this) - double(N1);
        if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
        {
            throw Overflow();
        }
    }
    return result;
}

Number Number::operator*(const Number N1) const
{
    Number result;
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            result = Number(integer * N1.integer);
        }
        else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            result.fraction.integer = fraction.integer * N1.integer;
            result.fraction.numerator = N1.integer * fraction.numerator;
            result.fraction.denominator = fraction.denominator;
        
            result.fraction.normalise();
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
            else
            {
                result.type = FRACTION_TYPE;
            }
        }
        else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            result.fraction.integer = N1.fraction.integer * integer;
            result.fraction.numerator = integer * N1.fraction.numerator;
            result.fraction.denominator = N1.fraction.denominator;
        
            result.fraction.normalise();
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
            else
            {
                result.type = FRACTION_TYPE;
            }
        }
        else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            result.fraction = N1.fraction * fraction;
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
            else
            {
                result.type = FRACTION_TYPE;
            }
        }
        else if (type == POWER_TYPE && N1.type == POWER_TYPE)
        {
            if(power.base == N1.power.base)
            {
                result.type = POWER_TYPE;
                result.power.multiplicand = power.multiplicand * N1.power.multiplicand;
                result.power.base = power.base;
                result.power.exponent = power.exponent + N1.power.exponent;
            }
            else if(power.exponent == N1.power.exponent)
            {
                result.type = POWER_TYPE;
                result.power.multiplicand = power.multiplicand * N1.power.multiplicand;
                result.power.base = power.base * N1.power.base;
                result.power.exponent = power.exponent;
            }
        }
        else if (type == POWER_TYPE)
        {
        
        }
        else if (N1.type == POWER_TYPE)
        {
    
        }
        else
        {
            result.type = DOUBLE_TYPE;
            std::feclearexcept(FE_OVERFLOW);
            result.double_num = double(*this) * double(N1);
            if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
            {
                throw Overflow();
            }
        }
    }
    catch (const SafeIntException& err)
    {
        result.type = DOUBLE_TYPE;
        std::feclearexcept(FE_OVERFLOW);
        result.double_num = double(*this) * double(N1);
        if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
        {
            throw Overflow();
        }
    }
    
    return result;
}

Number Number::operator/(const Number N1) const
{
    Number result = Number(0, 0, 1);
    
    try
    {
        if (type == INTEGER_TYPE && N1.type == INTEGER_TYPE)
        {
            result = Number(integer, N1.integer);
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
        }
        else if (type == FRACTION_TYPE && N1.type == INTEGER_TYPE)
        {
            result.fraction.numerator = fraction.numerator + N1.integer * fraction.denominator;
            result.fraction.denominator = N1.integer * fraction.denominator;
        
            result.fraction.normalise();
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
        }
        else if (type == INTEGER_TYPE && N1.type == FRACTION_TYPE)
        {
            result.fraction.numerator = integer * N1.fraction.denominator;
            result.fraction.denominator = N1.fraction.numerator + integer * N1.fraction.denominator;
        
            result.fraction.normalise();
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
        }
        else if (type == FRACTION_TYPE && N1.type == FRACTION_TYPE)
        {
            result.fraction.numerator = fraction.integer * fraction.denominator * N1.fraction.denominator
                + N1.fraction.denominator * fraction.numerator;
            result.fraction.denominator = N1.fraction.denominator * fraction.denominator * N1.fraction.integer
                + fraction.denominator * N1.fraction.numerator;
        
            result.fraction.normalise();
            if (result.fraction.numerator == 0)
            {
                result.integer = result.fraction.integer;
                result.type = INTEGER_TYPE;
            }
        }
        else if (type == DOUBLE_TYPE || N1.type == DOUBLE_TYPE)
        {
            const auto doubleN1 = double(N1);
            if (doubleN1 == 0)
            {
                throw DivisionByZero();
            }
            result.type = DOUBLE_TYPE;
            std::feclearexcept(FE_OVERFLOW);
            result.double_num = double(*this) / doubleN1;
            if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
            {
                throw Overflow();
            }
        }
    }
    catch (const SafeIntException& err)
    {
        const auto doubleN1 = double(N1);
        if (doubleN1 == 0)
        {
            throw DivisionByZero();
        }
        result.type = DOUBLE_TYPE;
        std::feclearexcept(FE_OVERFLOW);
        result.double_num = double(*this) / doubleN1;
        if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
        {
            throw Overflow();
        }
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
