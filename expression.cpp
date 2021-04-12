#include "number.h"
#include "expression.h"
#include <algorithm>

void expression::tokenise(std::queue<token> &InfixTokens, std::string_view &String)
{
    bool decimalPoint;
    for (int i = 0; i < String.length(); i++)
    {
        if ('0' <= (int) String[i] && (int) String[i] <= '9')
        {
            decimalPoint = false;
            bool space = false;
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
            number* tempNumber = new number;
            if (space)
            {
                std::string stringNumber = String.substr(i, j - i).data();
                stringNumber.erase(std::remove(stringNumber.begin(), stringNumber.end(), ' '), stringNumber.end());
                *tempNumber = number(std::string_view(stringNumber));
            }
            else
            {
                *tempNumber = number(String.substr(i, j - i));
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

expression::expression(std::string_view &String)
{
    std::queue<token> infixTokens;
    std::deque<token> postfixTokens;
    tokenise(infixTokens, String);
    infixToPostfix(postfixTokens, infixTokens);
    result = evaluatePostfix(postfixTokens);
}

void expression::infixToPostfix(std::deque<token> &PostfixTokens, std::queue<token> &InfixTokens)
{
    std::vector<token> operatorStack;
    token operatorOperator;
    while (!InfixTokens.empty())
    {
        token tokenInfix = InfixTokens.front();
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

number expression::evaluatePostfix(std::deque<token> &PostfixTokens)
{
    //TODO do deep copy
    std::vector<number*> resultVec;
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
                number* x = resultVec.back();
                resultVec.pop_back();
                number* y = resultVec.back();
                resultVec.pop_back();
                *x = *y + *x;
                resultVec.push_back(x);
                delete y;
                break;
            }
            case MINUS:
            {
                number* x = resultVec.back();
                resultVec.pop_back();
                number* y = resultVec.back();
                resultVec.pop_back();
                *x = *y - *x;
                resultVec.push_back(x);
                delete y;
                break;
            }
            case MULTIPLY:
            {
                number* x = resultVec.back();
                resultVec.pop_back();
                number* y = resultVec.back();
                resultVec.pop_back();
                *x = *y * *x;
                resultVec.push_back(x);
                delete y;
                break;
            }
            case DIVIDE:
            {
                number* x = resultVec.back();
                resultVec.pop_back();
                number* y = resultVec.back();
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
    number result = *resultVec.front();
    delete resultVec.front();
    return result;
}

token::token(char C1)
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

token::token()
{
    type = NONE;
}

token::token(number* N1)
{
    num = N1;
    type = NUMBER;
}

std::ostream &operator<<(std::ostream &Strm, const expression &E1)
{
    Strm << E1.result;
    return Strm;
}
