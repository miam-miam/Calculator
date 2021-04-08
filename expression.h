

#ifndef CALCULATOR__EXPRESSION_H
#define CALCULATOR__EXPRESSION_H

enum TokenType
{
    NUMBER,
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
    number num;
    TokenType type;
    explicit token(const number& N1);
    explicit token(char C1);
    token();
};

struct expression
{
    std::queue<token> infix_tokens;
    std::deque<token> postfix_tokens;
    
    explicit expression(std::string_view &String);
    
    void tokenise(std::string_view &String);
    void infixToPostfix();
    number evaluatePostfix();
};

#endif //CALCULATOR__EXPRESSION_H
