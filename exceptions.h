#ifndef CALCULATOR__EXCEPTIONS_H
#define CALCULATOR__EXCEPTIONS_H
#include <exception>

class CalculatorException : public std::exception{};

class Overflow: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Overflow"; }
};

class DivisionByZero: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Division by zero"; }
};

class UnmatchedBracket: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Unmatched bracket"; }
};

class UnknownOperator: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Unknown operator"; }
};

class IncorrectOperatorUsage: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Incorrect operator usage"; }
};

class PowerError: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Cannot compute 0^0"; }
};

class InvalidDecimalPoint: public CalculatorException
{
    [[nodiscard]] char const * what() const noexcept override { return "Invalid decimal point"; }
};

#endif //CALCULATOR__EXCEPTIONS_H
