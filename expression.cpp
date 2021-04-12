#include "number.h"
#include "expression.h"
#include <algorithm>

void Expression::tokenise(std::queue<Token> &InfixTokens, std::string_view &String)
{
    bool decimalPoint;
    for (int i = 0; i < String.length(); i++)
    {
        if ('0' <= (int) String[i] && (int) String[i] <= '9')
        {
            decimalPoint = false;
            bool space = false;
            int decimalPointIndex = -1;
            int j = i;
            for (; j < String.length(); j++)
            {
                if ('0' <= (int) String[j] && (int) String[j] <= '9')
                {
                    continue;
                }
                else if (!decimalPoint && (String[j] == '.'))
                {
                    decimalPoint = true;
                    decimalPointIndex = j;
                }
                else if (decimalPoint && (String[j] == '.'))
                {
                    throw InvalidDecimalPoint();
                }
                else if (!space && String[j] == ' ')
                {
                    space = true;
                    continue;
                }
                else
                {
                    break;
                }
            }
            Number* tempNumber = new Number;
            if (space)
            {
                std::string stringNumber = String.substr(i, j - i).data();
                stringNumber.erase(std::remove(stringNumber.begin(), stringNumber.end(), ' '), stringNumber.end());
                *tempNumber = Number(std::string_view(stringNumber), decimalPointIndex - i);
            }
            else
            {
                *tempNumber = Number(String.substr(i, j - i), decimalPointIndex - i);
            }
            InfixTokens.emplace(tempNumber);
            i = j - 1;
    
        }
        else if (String[i] != ' ')
        {
            InfixTokens.emplace(String[i]);
        }
    }
}

Expression::Expression(std::string_view &String)
{
    std::queue<Token> infixTokens;
    std::deque<Token> postfixTokens;
    tokenise(infixTokens, String);
    infixToPostfix(postfixTokens, infixTokens);
    result = evaluatePostfix(postfixTokens);
}

void Expression::infixToPostfix(std::deque<Token> &PostfixTokens, std::queue<Token> &InfixTokens)
{
    std::vector<Token> operatorStack;
    Token operatorOperator;
    while (!InfixTokens.empty())
    {
        Token tokenInfix = InfixTokens.front();
        InfixTokens.pop();
        switch (tokenInfix.type)
        {
            case NUMBER:
            {
                PostfixTokens.push_back(tokenInfix);
                break;
            }
            case PLUS:
            case MINUS:
            case MULTIPLY:
            case DIVIDE:
            {
                while (!operatorStack.empty() && (operatorOperator = operatorStack.back()).type != LBRACKET
                    && (Precedence[operatorOperator.type]
                        >= Precedence[tokenInfix.type])) //If implementing powers precedence cannot be equal as it is right associative and not left.
                {
                    operatorStack.pop_back();
                    PostfixTokens.push_back(operatorOperator);
                }
                operatorStack.push_back(tokenInfix);
                break;
            }
            case LBRACKET:
            {
                operatorStack.push_back(tokenInfix);
                break;
            }
            case RBRACKET:
            {
                while ((operatorOperator = operatorStack.back()).type != LBRACKET)
                {
                    operatorStack.pop_back();
                    PostfixTokens.push_back(operatorOperator);
                    if (operatorStack.empty())
                    {
                        throw UnmatchedBracket();
                    }
                }
                // Discard Left bracket
                operatorStack.pop_back();
                break;
            }
            case NONE:assert(0);
        }
    }
    
    while (!operatorStack.empty())
    {
        operatorOperator = operatorStack.back();
        if (operatorOperator.type == LBRACKET || operatorOperator.type == RBRACKET)
        {
            throw UnmatchedBracket();
        }
        PostfixTokens.push_back(operatorOperator);
        operatorStack.pop_back();
    }
}

Number Expression::evaluatePostfix(std::deque<Token> &PostfixTokens)
{
    //TODO do deep copy
    std::vector<Number*> resultVec;
    Number* x;
    Number* y;
    try
    {
        for (auto it = PostfixTokens.begin(); it != PostfixTokens.end(); ++it)
        {
            switch (it->type)
            {
                case NUMBER:
                {
                    resultVec.push_back(it->num);
                    break;
                }
                case PLUS:
                {
                    x = resultVec.back();
                    resultVec.pop_back();
                    y = resultVec.back();
                    resultVec.pop_back();
                    *x = *y + *x;
                    resultVec.push_back(x);
                    delete y;
                    break;
                }
                case MINUS:
                {
                    x = resultVec.back();
                    resultVec.pop_back();
                    y = resultVec.back();
                    resultVec.pop_back();
                    *x = *y - *x;
                    resultVec.push_back(x);
                    delete y;
                    break;
                }
                case MULTIPLY:
                {
                    x = resultVec.back();
                    resultVec.pop_back();
                    y = resultVec.back();
                    resultVec.pop_back();
                    *x = *y * *x;
                    resultVec.push_back(x);
                    delete y;
                    break;
                }
                case DIVIDE:
                {
                    x = resultVec.back();
                    resultVec.pop_back();
                    y = resultVec.back();
                    resultVec.pop_back();
                    *x = *y / *x;
                    resultVec.push_back(x);
                    delete y;
                    break;
                }
                default:
                {
                    assert(0);
                }
            }
        }
    }
    catch (CalculatorException &e)
    {
        for (auto it = PostfixTokens.begin(); it != PostfixTokens.end(); ++it)
        {
            if (it -> type == NUMBER && it -> num != y && it -> num != x)
            {
                delete it -> num;
            }
        }
        for (auto & it : resultVec)
        {
            delete it;
        }
        delete y;
        delete x;
        throw;
    }
    Number result = *resultVec.front();
    delete resultVec.front();
    return result;
}

Token::Token(char C1)
{
    switch (C1)
    {
        case '+':
        {
            type = PLUS;
            break;
        }
        case '-':
        {
            type = MINUS;
            break;
        }
        case '*':
        {
            type = MULTIPLY;
            break;
        }
        case '/':
        {
            type = DIVIDE;
            break;
        }
        case '(':
        {
            type = LBRACKET;
            break;
        }
        case ')':
        {
            type = RBRACKET;
            break;
        }
        default:
        {
            throw UnknownOperator();
        }
    }
}

Token::Token()
{
    type = NONE;
}

Token::Token(Number* N1)
{
    num = N1;
    type = NUMBER;
}

std::ostream &operator<<(std::ostream &Strm, const Expression &E1)
{
    Strm << E1.result;
    return Strm;
}
