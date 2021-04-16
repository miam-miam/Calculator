#ifndef CALCULATOR__NUMBER_H
#define CALCULATOR__NUMBER_H

#include "dataTypes.h"

struct Number
{
    enum NumberType
    {
        INTEGER_TYPE,
        FRACTION_TYPE,
        DOUBLE_TYPE,
        POWER_TYPE
    };
    NumberType type;
    union
    {
        SafeInt<int64_t> integer;
        Fraction fraction;
        double double_num;
        Power power;
    };
    
    Number();
    Number(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    Number(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    explicit Number(Fraction GivenFraction);
    explicit Number(SafeInt<int64_t> GivenInt);
    explicit Number(const std::string_view &Number, int Offset = -2);
    Number(Fraction GivenMultiplicand, SimpleFraction GivenBase, SimpleFraction GivenExponent);
    Number(Fraction GivenMultiplicand, SafeInt<int64_t> GivenBase, SimpleFraction GivenExponent);
    
    explicit operator double() const;
    
    Number &operator=(const Number &Other);
    
    Number operator+(Number) const;
    
    Number operator-(Number) const;
    
    Number operator*(Number) const;
    
    Number operator/(Number) const;
    
    [[nodiscard]] bool checkIfZero(Number) const;
    
};

std::ostream &operator<<(std::ostream &Strm, const Number &);

#endif //CALCULATOR__NUMBER_H
