#include "number.h"
#include "expression.h"

void expression::tokenise(std::string_view &String)
{
    bool decimalPoint;
    for (int i = 0; i < String.length(); i++)
    {
        if ('0' <= (int) String[i] && (int) String[i] <= '9')
        {
            decimalPoint = false;
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
                    throw std::runtime_error("Should only have one decimal point.");
                }
                else
                {
                    break;
                }
            }
    
            number tempNumber = number(String.substr(i, j - i));
            infix_tokens.emplace(tempNumber);
            i = j - 1;
    
        }
        else
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
                        throw std::runtime_error("Unmatched brackets.");
                    }
                }
                // Discard Left bracket
                operatorStack.pop_back();
                break;
            }
            case NONE:throw std::runtime_error("Had an empty tokenInfix.");
        }
    }
    
    while (!operatorStack.empty())
    {
        operatorOperator = operatorStack.back();
        if (operatorOperator.type == LBRACKET || operatorOperator.type == RBRACKET)
        {
            throw std::runtime_error("Unmatched brackets.");
        }
        postfix_tokens.push_back(operatorOperator);
        operatorStack.pop_back();
    }
    
}

number expression::evaluatePostfix()
{
    //TODO do deep copy
    std::vector<number> result;
    for (auto it = postfix_tokens.begin(); it != postfix_tokens.end(); ++it)
    {
        switch (it->type)
        {
            case NUMBER:
            {
                result.push_back(it->num);
                break;
            }
            case PLUS:
            {
                number x = result.back();
                result.pop_back();
                number y = result.back();
                result.pop_back();
                result.push_back(y + x);
                break;
            }
            case MINUS:
            {
                number x = result.back();
                result.pop_back();
                number y = result.back();
                result.pop_back();
                result.push_back(y - x);
                break;
            }
            case MULTIPLY:
            {
                number x = result.back();
                result.pop_back();
                number y = result.back();
                result.pop_back();
                result.push_back(y * x);
                break;
            }
            case DIVIDE:
            {
                number x = result.back();
                result.pop_back();
                number y = result.back();
                result.pop_back();
                result.push_back(y / x);
                break;
            }
            default:
            {
                throw std::runtime_error("Should be operator.");
            }
        }
    }
    
    return result.front();
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
            throw std::runtime_error("Unknown operator.");
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
