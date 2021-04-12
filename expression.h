

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

struct Token
{
    Number* num;
    TokenType type;
    explicit Token(Number* N1);
    explicit Token(char C1);
    Token();
};

struct Expression
{
    Number result;
    
    explicit Expression(std::string_view &String);
    
    static void tokenise(std::queue<Token> &InfixTokens, std::string_view &String);
    static void infixToPostfix(std::deque<Token> &PostfixTokens, std::queue<Token> &InfixTokens);
    static Number evaluatePostfix(std::deque<Token> &PostfixTokens);
};

std::ostream &operator<<(std::ostream &Strm, const Expression &);

#endif //CALCULATOR__EXPRESSION_H
