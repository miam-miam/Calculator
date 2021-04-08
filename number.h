
#ifndef CALCULATOR__NUMBER_H
#define CALCULATOR__NUMBER_H

struct fraction
{
    fraction(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    
    fraction(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    
    fraction()
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

struct number
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
        fraction fraction{};
        double double_num;
    };
    
    number(SafeInt<int64_t> GivenInt, SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    number(SafeInt<int64_t> GivenNum, SafeInt<int64_t> GivenDen);
    explicit number(SafeInt<int64_t> GivenInt);
    explicit number(const std::string_view &Number);
    number();
    
    explicit operator double() const;
    
    number& operator=(const number& Other);
    
    number operator+(number) const;
    
    number operator-(number) const;
    
    number operator*(number) const;
    
    number operator/(number) const;
};

std::ostream &operator<<(std::ostream &Strm, const number &);

#endif //CALCULATOR__NUMBER_H
