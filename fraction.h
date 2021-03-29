
#ifndef CALCULATOR_FRACTION_H
#define CALCULATOR_FRACTION_H

#include <string>

class fraction
{
public:
    fraction(int64_t givenInt, int64_t givenNum, int64_t givenDen);

    fraction(int64_t givenNum, int64_t givenDen);

    fraction()
    {
        integer = 0;
        numerator = 0;
        denominator = 1;
    }

    explicit fraction(const std::string &number);

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

fraction getFraction();

std::ostream &operator<<(std::ostream &strm, const fraction &);

#endif //CALCULATOR_FRACTION_H
