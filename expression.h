

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
    number* num;
    TokenType type;
    explicit token(number* N1);
    explicit token(char C1);
    token();
};

struct expression
{
    number result;
    
    explicit expression(std::string_view &String);
    
    static void tokenise(std::queue<token> &InfixTokens, std::string_view &String);
    static void infixToPostfix(std::deque<token> &PostfixTokens, std::queue<token> &InfixTokens);
    static number evaluatePostfix(std::deque<token> &PostfixTokens);
};

std::ostream &operator<<(std::ostream &Strm, const expression &);

#endif //CALCULATOR__EXPRESSION_H
