

#ifndef CALCULATOR_EXPRESSION_H
#define CALCULATOR_EXPRESSION_H

#include <list>
#include <string>
#include "fraction.h"

class expression
{
private:
    std::list<fraction> infixToken;
    std::list<fraction> postfixToken;

    int *tokenise(std::string &string);
};

#endif //CALCULATOR_EXPRESSION_H
