
#ifndef CALCULATOR__FRACTION_H
#define CALCULATOR__FRACTION_H

struct fraction
{
    public:
    fraction(int64_t GivenInt, int64_t GivenNum, int64_t GivenDen);
    
    fraction(int64_t GivenNum, int64_t GivenDen);
    
    fraction()
    {
        integer = 0;
        numerator = 0;
        denominator = 1;
    }
    
    explicit fraction(const std::string_view &Number);
    
    int64_t integer;
    int64_t numerator;
    int64_t denominator;
    
    fraction operator+(fraction) const;
    
    fraction operator-(fraction) const;
    
    fraction operator*(fraction) const;
    
    fraction operator/(fraction) const;
    
    explicit operator int64_t() const
    { return integer; }
    
    explicit operator float() const
    { return integer + (float) numerator / denominator; }
    
    explicit operator double() const
    { return integer + double(numerator) / denominator; }
    
    void normalise();
    
};

std::ostream &operator<<(std::ostream &Strm, const fraction &);

#endif //CALCULATOR__FRACTION_H
