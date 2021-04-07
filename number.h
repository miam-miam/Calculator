
#ifndef CALCULATOR__NUMBER_H
#define CALCULATOR__NUMBER_H

struct fraction
{
    fraction(int64_t GivenInt, int64_t GivenNum, int64_t GivenDen);
    
    fraction(int64_t GivenNum, int64_t GivenDen);
    
    fraction()
    {
        integer = 0;
        numerator = 0;
        denominator = 1;
    }
    
    int64_t integer;
    int64_t numerator;
    int64_t denominator;
    
    explicit operator int64_t() const
    { return integer; }
    
    explicit operator float() const
    { return integer + (float) numerator / denominator; }
    
    explicit operator double() const
    { return integer + double(numerator) / denominator; }
    
    void normalise();
    
};

struct number
{
    enum NumberType
    {
        INTEGER_TYPE,
        FRACTION_TYPE
    };
    NumberType type;
    union
    {
        int64_t integer;
        fraction fraction{};
    };
    
    number(int64_t GivenInt, int64_t GivenNum, int64_t GivenDen);
    number(int64_t GivenNum, int64_t GivenDen);
    explicit number(int64_t GivenInt);
    explicit number(const std::string_view &Number);
    number();
    
    number operator+(number) const;
    
    number operator-(number) const;
    
    number operator*(number) const;
    
    number operator/(number) const;
};

std::ostream &operator<<(std::ostream &Strm, const number &);

#endif //CALCULATOR__NUMBER_H
