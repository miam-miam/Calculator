#ifndef CALCULATOR__EXCEPTIONS_H
#define CALCULATOR__EXCEPTIONS_H
#include <exception>

class CalculatorException : public std::exception{};

class Overflow: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Overflow"; }
};

class Underflow: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Underflow"; }
};

class DivisionByZero: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Division by zero"; }
};

class UnmatchedBracket: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Unmatched bracket"; }
};

class UnknownOperator: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Unknown operator"; }
};

class IncorrectOperatorUsage: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Incorrect operator usage"; }
};

class InvalidDecimalPoint: public CalculatorException
{
    [[nodiscard]] char const * what() const override { return "Invalid decimal point"; }
};

#endif //CALCULATOR__EXCEPTIONS_H
