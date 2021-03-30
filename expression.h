

#ifndef CALCULATOR__EXPRESSION_H
#define CALCULATOR__EXPRESSION_H

#include <vector>
#include <queue>
#include <string>
#include <unordered_map>
#include "fraction.h"

enum TokenType
{
    FRACTION,
    NONE,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LBRACKET,
    RBRACKET
};

// Token type enum will be index
const int Precedence[] = {0, 0, 2, 2, 3, 3, 1, 0};

struct token
{
    union
    {
        fraction frac;
        float flo;
    };
    
    TokenType type;
    explicit token(fraction F1);
    explicit token(char C1);
    token();
};

struct expression
{
    std::queue<token> infix_tokens;
    std::queue<token> postfix_tokens;
    
    explicit expression(std::string_view &String);
    
    void tokenise(std::string_view &String);
    void infixToPostfix();
    fraction evaluatePostfix();
};

#endif //CALCULATOR__EXPRESSION_H
