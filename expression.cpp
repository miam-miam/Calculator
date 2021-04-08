#include "number.h"
#include "expression.h"
#include <algorithm>

void expression::tokenise(std::string_view &String)
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
            number tempNumber;
            if (space)
            {
                std::string stringNumber = String.substr(i, j - i).data();
                stringNumber.erase(std::remove(stringNumber.begin(), stringNumber.end(), ' '), stringNumber.end());
                tempNumber = number(std::string_view(stringNumber));
            }
            else
            {
                tempNumber = number(String.substr(i, j - i));
            }
            infix_tokens.emplace(tempNumber);
            i = j - 1;
    
        }
        else if (String[i] != ' ')
        {
            infix_tokens.emplace(String[i]);
        }
    }
}

expression::expression(std::string_view &String)
{
    tokenise(String);
    infixToPostfix();
    result = evaluatePostfix();
}

void expression::infixToPostfix()
{
    std::vector<token> operatorStack;
    token operatorOperator;
    while (!infix_tokens.empty())
    {
        token tokenInfix = infix_tokens.front();
        infix_tokens.pop();
        switch (tokenInfix.type)
        {
            case NUMBER:
            {
                postfix_tokens.push_back(tokenInfix);
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
                    postfix_tokens.push_back(operatorOperator);
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
                    postfix_tokens.push_back(operatorOperator);
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
        postfix_tokens.push_back(operatorOperator);
        operatorStack.pop_back();
    }
    
}

number expression::evaluatePostfix()
{
    //TODO do deep copy
    std::vector<number> resultVec;
    for (auto it = postfix_tokens.begin(); it != postfix_tokens.end(); ++it)
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
                number x = resultVec.back();
                resultVec.pop_back();
                number y = resultVec.back();
                resultVec.pop_back();
                resultVec.push_back(y + x);
                break;
            }
            case MINUS:
            {
                number x = resultVec.back();
                resultVec.pop_back();
                number y = resultVec.back();
                resultVec.pop_back();
                resultVec.push_back(y - x);
                break;
            }
            case MULTIPLY:
            {
                number x = resultVec.back();
                resultVec.pop_back();
                number y = resultVec.back();
                resultVec.pop_back();
                resultVec.push_back(y * x);
                break;
            }
            case DIVIDE:
            {
                number x = resultVec.back();
                resultVec.pop_back();
                number y = resultVec.back();
                resultVec.pop_back();
                resultVec.push_back(y / x);
                break;
            }
            default:
            {
                assert(0);
            }
        }
    }
    
    return resultVec.front();
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

token::token(const number& N1)
{
    num = N1;
    type = NUMBER;
}

std::ostream &operator<<(std::ostream &Strm, const expression &E1)
{
    Strm << E1.result;
    return Strm;
}
