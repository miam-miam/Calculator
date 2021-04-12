
#ifndef CALCULATOR__NUMBER_H
#define CALCULATOR__NUMBER_H

struct Fraction
{
    Fraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    
    Fraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    
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
    
    explicit operator float() const
    { return float(integer) +  float(numerator) / float(denominator); }
    
    explicit operator double() const
    { return double(integer) + double(numerator) / double(denominator); }
    
    void normalise();
};

struct Number
{
    enum NumberType
    {
        INTEGER_TYPE,
        FRACTION_TYPE,
        DOUBLE_TYPE
    };
    NumberType type;
    union
    {
        SafeInt<int64_t> integer;
        Fraction fraction;
        double double_num;
    };
    
    Number(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    Number(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    explicit Number(SafeInt<int64_t> GivenInt);
    explicit Number(const std::string_view &Number, int Offset=-2);
    Number();
    
    explicit operator double() const;
    
    Number& operator=(const Number& Other);
    
    Number operator+(Number) const;
    
    Number operator-(Number) const;
    
    Number operator*(Number) const;
    
    Number operator/(Number) const;
};

std::ostream &operator<<(std::ostream &Strm, const Number &);

#endif //CALCULATOR__NUMBER_H
