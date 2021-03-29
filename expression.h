

#ifndef CALCULATOR__EXPRESSION_H
#define CALCULATOR__EXPRESSION_H

#include <vector>
#include <string>
#include "fraction.h"

enum TokenType
{
    FRACTION,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LBRACKET,
    RBRACKET
};

struct token
{
    void *at;
    uint32_t length;
    TokenType type;
    explicit token(fraction F1);
    explicit token(char C1);
    
};

struct expression
{
    std::vector<token> infixToken;
    std::vector<token> postfixToken;
    
    int *tokenise(std::string_view &String);
};

#endif //CALCULATOR__EXPRESSION_H
