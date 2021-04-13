
#ifndef CALCULATOR__NUMBER_H
#define CALCULATOR__NUMBER_H

struct SimpleFraction
{
    SimpleFraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // does not normalise
    
    SimpleFraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // does not normalise
    
    SimpleFraction()
    {
        numerator = 0;
        denominator = 1;
    }
    
    SafeInt<int64_t> numerator;
    SafeInt<int64_t> denominator;
    
    explicit operator SafeInt<int64_t>() const
    { return numerator / denominator; }
    
    explicit operator double() const
    { return double(numerator) / double(denominator); }
    
    void normalise();
};

struct Fraction
{
    Fraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // normalises
    
    Fraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen); // does not normalise
    
    Fraction()
    {
        integer = 0;
        numerator = 0;
        denominator = 1;
    }
    
    SafeInt<int64_t> integer;
    SafeInt<int64_t> numerator;
    SafeInt<int64_t> denominator;
    
    explicit operator SafeInt<int64_t>() const
    { return integer; }
    
    explicit operator double() const
    { return double(integer) + double(numerator) / double(denominator); }
    
    explicit operator SimpleFraction() const;
    
    void normalise();
};

struct Power
{
    Fraction multiplicand;
    SafeInt<int64_t> base;
    SimpleFraction exponent;
    Power(Fraction GivenMultiplicand, SafeInt<int64_t> GivenBase, SimpleFraction GivenExponent);
    Power();
    
    explicit operator double() const;
};

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
    
    Number(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    Number(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    Number();
    explicit Number(Fraction GivenFraction);
    explicit Number(SafeInt<int64_t> GivenInt);
    explicit Number(const std::string_view &Number, int Offset=-2);
    Number(Fraction GivenMultiplicand, SafeInt<int64_t> GivenBase, SimpleFraction GivenExponent);
    
    explicit operator double() const;
    
    Number& operator=(const Number& Other);
    
    Number operator+(Number) const;
    
    Number operator-(Number) const;
    
    Number operator*(Number) const;
    
    Number operator/(Number) const;
};

std::ostream &operator<<(std::ostream &Strm, const Number &);

#endif //CALCULATOR__NUMBER_H
