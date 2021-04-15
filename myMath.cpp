#include <cfenv>
#include "myMath.h"
#include "factorisation.h"

#define MAX_LONG_LONG_LOG_10 18
#define MAX_LONG_LONG_LOG_2 63

int ceilLog2(unsigned long long Base)   // Found on stack overflow https://stackoverflow.com/questions/3272424/compute-fast-log-base-2-ceiling
{
    static const unsigned long long t[6] = {
        0xFFFFFFFF00000000ull,
        0x00000000FFFF0000ull,
        0x000000000000FF00ull,
        0x00000000000000F0ull,
        0x000000000000000Cull,
        0x0000000000000002ull
    };
    
    int y = (((Base & (Base - 1)) == 0) ? 0 : 1);
    int j = 32;
    int i;
    
    for (i = 0; i < 6; i++) {
        int k = (((Base & t[i]) == 0) ? 0 : j);
        y += k;
        Base >>= k;
        j >>= 1;
    }
    
    return y;
}

int64_t tenPowll(unsigned long long Exp)
{
    if (Exp > MAX_LONG_LONG_LOG_10)
    {
        throw std::out_of_range("Too large an Exponent.");
    }
    int64_t count = 10;
    for (int i = 1; i < Exp; i++)
    {
        count *= 10;
    }
    return count;
}

SafeInt<int64_t> powSI(int64_t Base, int64_t Exponent)
{
    if (Exponent * ceilLog2(llabs(Base)) > MAX_LONG_LONG_LOG_2)
    {
        throw SafeIntException();
    }
    if (Exponent == 0)
    {
        if (Base == 0)
        {
            throw PowerError();
        }
        return 1;
    }
    
    int64_t count = Base;
    for (int i = 1; i < Exponent; i++)
    {
        count *= Base;
    }
    return count;
}

Fraction binomialSeries(Fraction FractionBase, SafeInt<int64_t> Exponent)
{
    SafeInt<int64_t> coefficient = Exponent;
    Fraction result = Fraction();
    result.integer = powSI(FractionBase.integer, Exponent);
    result.denominator = powSI(FractionBase.denominator, Exponent);
    SafeInt<int64_t> previousCalc = saferMultiplyDivide(result.integer, FractionBase.numerator,result.denominator,(FractionBase.denominator * FractionBase.integer));
    for (int i = 1; i <= Exponent; i++)
    {
        result.numerator += coefficient * previousCalc;
        coefficient = (coefficient * (Exponent - i)) / (i+1);
        previousCalc = saferMultiplyDivide(previousCalc,FractionBase.numerator,(FractionBase.integer * FractionBase.denominator));
    }
    result.normalise();
    return result;
}


Number powNum(Number Base, Number Exponent)
{
    Number result = Number();
    try
    {
        if (Base.type == Number::INTEGER_TYPE && Exponent.type == Number::INTEGER_TYPE)
        {
            try
            {
                if (Exponent.integer < 0)
                {
                    result = Number(0, 1, powSI(Base.integer, -Exponent.integer));
                }
                else
                {
                    result = Number(powSI(Base.integer, Exponent.integer));
                }
            }
            catch (const SafeIntException &e)
            {
                result.type = Number::POWER_TYPE;
                result.power.multiplicand = Fraction(1, 0,1);
                result.power.base = SimpleFraction(Base.integer, 1);
                result.power.exponent = SimpleFraction(Exponent.integer, 1);
            }
        }
        else if (Base.type == Number::FRACTION_TYPE && Exponent.type == Number::INTEGER_TYPE)
        {
            try
            {
                if (Exponent.integer < 0)
                {
                    result.fraction.denominator = powSI(Base.fraction.numerator + Base.fraction.integer * Base.fraction.denominator, -Exponent.integer);
                    result.fraction.numerator = powSI(Base.fraction.denominator, -Exponent.integer);
                }
                else
                {
                    result.fraction.numerator =
                        powSI(Base.fraction.numerator + Base.fraction.integer * Base.fraction.denominator,
                              Exponent.integer);
                    result.fraction.denominator = powSI(Base.fraction.denominator, Exponent.integer);
                }
                result.fraction.normalise();
            }
            catch (const SafeIntException &e)
            {
                try
                {
                    if (Exponent.integer < 0)
                    {
                        result.fraction = binomialSeries(Fraction(Base.fraction.denominator,
                                                                  Base.fraction.numerator + Base.fraction.denominator
                                                                      * Base.fraction.integer), -Exponent.integer);
                    }
                    else
                    {
                        result.fraction = binomialSeries(Base.fraction, Exponent.integer);
                    }
                    if (result.fraction.numerator == 0)
                    {
                        result.integer = result.fraction.integer;
                        result.type = Number::INTEGER_TYPE;
                    }
                    else
                    {
                        result.type = Number::FRACTION_TYPE;
                    }
                }
                catch (const SafeIntException &e)
                {
                    result.type = Number::POWER_TYPE;
                    result.power.multiplicand = Fraction(1, 0,1);
                    result.power.base = SimpleFraction(Base.fraction);
                    result.power.exponent = SimpleFraction(Exponent.integer, 1);
                }
            }
        }
        else if (Base.type == Number::INTEGER_TYPE && Exponent.type == Number::FRACTION_TYPE && Exponent.fraction.integer > 0)
        {
            SimpleFraction simpleExponent = SimpleFraction(Exponent.fraction);
            result.power.base = SimpleFraction(Base.integer, 1);
            result.type = Number::POWER_TYPE;
            if (Exponent.fraction.denominator == 1) {assert(0);} // Fraction should not be an integer
            try
            {
                if (simpleExponent.numerator != 1)
                {
                    try
                    {
                        result.power.base.numerator = powSI(Base.integer, simpleExponent.numerator);
                        simpleExponent.numerator = 1;
                    }
                    catch (const SafeIntException &e) {}
                }
                
                // Careful base is passed by ref
                result.power.multiplicand = Fraction(powSI(factorise(result.power.base.numerator, simpleExponent.denominator), simpleExponent.numerator), 0, 1);
                result.power.exponent = simpleExponent;
            }
            catch (const SafeIntException &e)
            {
                result.power.multiplicand = Fraction(1,0,1);
                result.power.base.numerator = Base.integer;
                result.power.exponent = simpleExponent;
            }
        }
        else if ((Base.type == Number::FRACTION_TYPE || Exponent.fraction.integer <= 0) && Exponent.type == Number::FRACTION_TYPE)
        {
            SimpleFraction simpleExponent = SimpleFraction(Exponent.fraction);
            SimpleFraction newBase = SimpleFraction();
            result.type = Number::POWER_TYPE;
            
            if (Base.type == Number::INTEGER_TYPE)
            {
                // Exponent must have been negative and so will need to change Base into a fraction
                Base.fraction = Fraction(0,1, Base.integer);
                Base.type = Number::FRACTION_TYPE;
                simpleExponent.numerator *= -1;
            }
            else if (simpleExponent.numerator < 0)
            {
                Base.fraction = Fraction(Base.fraction.denominator, Base.fraction.numerator + Base.fraction.denominator * Base.fraction.integer);
                simpleExponent.numerator *= -1;
                Exponent.fraction.numerator *= -1;
                Exponent.fraction.integer *= -1;
            }
            if (simpleExponent.numerator != 1)
            {
                try
                {
                    newBase.numerator =
                        powSI(Base.fraction.numerator + Base.fraction.integer * Base.fraction.denominator, simpleExponent.numerator);
                    newBase.denominator = powSI(Base.fraction.denominator, simpleExponent.numerator);
                    newBase.normalise();
                    simpleExponent.numerator = 1;
                }
                catch (const SafeIntException &e)
                {
                    try
                    {
                        newBase = SimpleFraction(binomialSeries(Base.fraction, simpleExponent.numerator));
                        simpleExponent.numerator = 1;
                    }
                    catch (const SafeIntException &e)   // Cannot simplify so must return as is
                    {
                        result.power.multiplicand = Fraction(1,0,1);
                        result.power.base = SimpleFraction(Base.fraction);
                        result.power.exponent = simpleExponent;
                        return result;
                    }
                }
            }
            else
            {
                newBase = SimpleFraction(Base.fraction);
            }
            // TODO implement if numerator of simpleExponent is not 1
            {
                SafeInt<int64_t> denominator = newBase.denominator;
                // Careful base is passed by ref
                result.power.multiplicand = Fraction(factorise(newBase.numerator, simpleExponent.denominator) * powSI(factorise(newBase.denominator, simpleExponent.denominator), simpleExponent.denominator - 1), denominator);
            }
            
            if (_abs64(newBase.numerator) == 1 && newBase.denominator == 1 && !(simpleExponent.denominator % 2 == 0 && newBase.numerator == -1)) // Can simplify to simpler types but must remove complex numbers
            {
                if (newBase.numerator == -1 && simpleExponent.numerator % 2 == 1)
                {
                    result.power.multiplicand.integer *= -1;
                    result.power.multiplicand.numerator *= -1;
                }
                if (result.power.multiplicand.numerator == 0)
                {
                    result.integer = result.power.multiplicand.integer;
                    result.type = Number::INTEGER_TYPE;
                    return result;
                }
                else
                {
                    result.fraction = result.power.multiplicand;
                    result.type = Number::FRACTION_TYPE;
                    return result;
                }
            }
            
            result.power.base = SimpleFraction(newBase.numerator * powSI(newBase.denominator, simpleExponent.denominator - 1), 1);
            result.power.exponent = simpleExponent;
        }
        else if (Base.type == Number::DOUBLE_TYPE || Exponent.type == Number::DOUBLE_TYPE)
        {
            result.type = Number::DOUBLE_TYPE;
            std::feclearexcept(FE_OVERFLOW);
            result.double_num = pow(double(Base), double(Exponent));
            if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
            {
                throw Overflow();
            }
        }
    }
    catch (const SafeIntException& err)
    {
        result.type = Number::DOUBLE_TYPE;
        std::feclearexcept(FE_OVERFLOW);
        result.double_num = pow(double(Base), double(Exponent));
        if (std::fetestexcept(FE_OVERFLOW) & FE_OVERFLOW)
        {
            throw Overflow();
        }
    }
    return result;
}

SafeInt<int64_t> saferMultiplyDivide(SafeInt<int64_t> Multiplicand1,
                                     SafeInt<int64_t> Multiplicand2,
                                     SafeInt<int64_t> Divisor)
{
    try
    {
        return (Multiplicand1 * Multiplicand2) / Divisor;
    }
    catch (const SafeIntException &e)
    {
        const SafeInt<int64_t> gcd1 = std::gcd((int64_t) Multiplicand1, (int64_t) Divisor);
        const SafeInt<int64_t> gcd2 = std::gcd((int64_t) Multiplicand2, int64_t (Divisor/gcd1));
        return ((Multiplicand1/gcd1) * (Multiplicand2/gcd2)) / (Divisor/(gcd1 * gcd2));
    }
    
}

SafeInt<int64_t> saferMultiplyDivide(SafeInt<int64_t> Multiplicand1,
                                     SafeInt<int64_t> Multiplicand2,
                                     SafeInt<int64_t> Multiplicand3,
                                     SafeInt<int64_t> Divisor)
{
    try
    {
        return (Multiplicand1 * Multiplicand2 * Multiplicand3) / Divisor;
    }
    catch (const SafeIntException &e)
    {
        const SafeInt<int64_t> gcd1 = std::gcd((int64_t) Multiplicand1, (int64_t) Divisor);
        const SafeInt<int64_t> gcd2 = std::gcd((int64_t) Multiplicand2, int64_t(Divisor / gcd1));
        const SafeInt<int64_t> gcd3 = std::gcd((int64_t) Multiplicand3, int64_t(Divisor / (gcd2 * gcd1)));
        return ((Multiplicand1 / gcd1) * (Multiplicand2 / gcd2) * (Multiplicand3 / gcd3)) / (Divisor / (gcd1 * gcd2 * gcd3));
    }
}
