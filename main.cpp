#include <iostream>
#include <iomanip>
#include <numeric>


class fraction
{
public:
    fraction(long long givenInt, long long givenNum, long long givenDen);

    fraction(long long givenNum, long long givenDen);

    fraction()
    {
        integer = 0;
        numerator = 0;
        denominator = 1;
    }

    long long integer;
    long long numerator;
    long long denominator;

    fraction operator+(fraction) const;

    fraction operator-(fraction) const;

    fraction operator*(fraction) const;

    fraction operator/(fraction) const;

    void normalise();

    explicit operator long long() const
    { return integer; }

    explicit operator float() const
    { return integer + float(numerator) / denominator; }

    explicit operator double()
    { return integer + double(numerator) / denominator; }

};

fraction::fraction(long long givenInt, long long givenNum, long long givenDen)
{
    integer = givenInt;
    numerator = givenNum;
    denominator = givenDen;
}

fraction::fraction(long long givenNum, long long givenDen)
{
    integer = 0;
    numerator = givenNum;
    denominator = givenDen;
    normalise();
}

fraction fraction::operator-(const fraction f1) const
{
    fraction temp;
    temp.integer = integer - f1.integer;
    if (denominator == 1 && f1.denominator == 1) return temp;
    temp.denominator = f1.denominator * denominator;
    temp.numerator = f1.denominator * numerator - f1.numerator * denominator;
    temp.normalise();
    return temp;
}

fraction fraction::operator+(const fraction f1) const
{
    fraction temp;
    temp.integer = f1.integer + integer;
    if (denominator == 1 && f1.denominator == 1) return temp;
    temp.denominator = f1.denominator * denominator;
    temp.numerator = f1.numerator * denominator + f1.denominator * numerator;
    temp.normalise();
    return temp;
}

fraction fraction::operator*(const fraction f1) const
{
    fraction temp;
    temp.integer = f1.integer * integer;
    if (denominator == 1 && f1.denominator == 1) return temp;
    temp.denominator = f1.denominator * denominator;
    temp.numerator =
            f1.numerator * numerator + f1.denominator * f1.integer * numerator + denominator * integer * f1.numerator;
    temp.normalise();
    return temp;
}

fraction fraction::operator/(const fraction f1) const
{
    fraction temp;
    temp.denominator = f1.denominator * denominator * f1.integer + denominator * f1.numerator;
    temp.numerator = integer * denominator * f1.denominator + f1.denominator * numerator;
    temp.normalise();
    return temp;
}


void fraction::normalise()
{

    if (denominator == 1)
    {
        integer = numerator;
        numerator = 0;
        return;
    }

    if (numerator >= denominator)
    {
        integer += numerator / denominator;
        numerator -= (numerator / denominator) * denominator;
    }

    if (numerator == 0)
    {
        denominator = 1;
        return;
    }

    const long long gcd = std::gcd((long long) numerator, (long long) denominator);
    if (gcd != 1)
    {
        numerator = numerator / gcd;
        denominator = denominator / gcd;
    }

}

fraction getFraction()
{
    long long integer = 0;
    long long numerator = 0;
    long long denominator = 0;
    std::cout << "Input fraction: ";
    std::cin >> integer;
    std::cin >> numerator;
    std::cin >> denominator;
    fraction temp(integer, numerator, denominator);
    temp.normalise();
    return temp;
}

std::ostream &operator<<(std::ostream &strm, const fraction &a)
{
    if (a.denominator == 1)
    {
        strm << a.numerator;
    }
    else
    {
        strm << a.integer << "+" << a.numerator << "/" << a.denominator;
    }
    return strm;
}


int main()
{
    // Setting the precision to 9 decimal places
    std::cout << std::setprecision(10);

    std::cout << "Hello, Calculator!" << std::endl;
    fraction a = fraction(6, 5);
    fraction b = fraction(1, 2, 7);
    std::cout << "A: " << a << ", B: " << b << std::endl;
    fraction c;

    c = a + b;
    std::cout << "C: " << c << std::endl;
    c = a - b;
    std::cout << "C: " << c << std::endl;
    c = a * b;
    std::cout << "C: " << c << std::endl;
    c = a / b;
    std::cout << "C: " << c << std::endl;


    double d = (double) c;
    std::cout << "D: " << d << std::endl;

    std::cout << (float) c << std::endl;
    std::cout << (double) c << std::endl;

    return 0;
}
