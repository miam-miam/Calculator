#include <stdexcept>
#include <iostream>
#include "fraction.h"
#include "expression.h"

void expression::tokenise(std::string_view &String)
{
    bool decimalPoint;
    for (int i = 0; i < String.length(); i++)
    {
        if ('0' <= (int) String[i] and (int) String[i] <= '9')
        {
            decimalPoint = false;
            int j = i;
            for (; j < String.length(); j++)
            {
                if ('0' <= (int) String[j] and (int) String[j] <= '9')
                {
                    continue;
                }
                else if (!decimalPoint and (String[j] == '.'))
                {
                    decimalPoint = true;
                }
                else if (decimalPoint and (String[j] == '.'))
                {
                    throw std::runtime_error("Should only have one decimal point.");
                }
                else
                {
                    break;
                }
            }
            
            fraction tempFraction = fraction(String.substr(i, j - i));
            infix_tokens.emplace(tempFraction);
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
    fraction result = evaluatePostfix();
    std::cout << result << std::endl;
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
            case FRACTION:
            {
                postfix_tokens.push(tokenInfix);
                break;
            }
            case PLUS:
            case MINUS:
            case MULTIPLY:
            case DIVIDE:
            {
                while (!operatorStack.empty() and (operatorOperator = operatorStack.back()).type != LBRACKET
                    and (Precedence[operatorOperator.type]
                        >= Precedence[tokenInfix.type])) //If implementing powers precedence cannot be equal as it is right associative and not left.
                {
                    operatorStack.pop_back();
                    postfix_tokens.push(operatorOperator);
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
                    postfix_tokens.push(operatorOperator);
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
        if (operatorOperator.type == LBRACKET or operatorOperator.type == RBRACKET)
        {
            throw std::runtime_error("Unmatched brackets.");
        }
        postfix_tokens.push(operatorOperator);
        operatorStack.pop_back();
    }
    
}

fraction expression::evaluatePostfix()
{
    //TODO do deep copy
    std::vector<fraction> result;
    while (postfix_tokens.size() > 1)
    {
        token x = postfix_tokens.front();
        postfix_tokens.pop();
        token y = postfix_tokens.front();
        postfix_tokens.pop();
        token op = postfix_tokens.front();
        postfix_tokens.pop();
        
        switch (op.type)
        {
            case PLUS:
            {
                result.push_back(y.frac + x.frac);
                break;
            }
            case MINUS:
            {
                result.push_back(y.frac - x.frac);
                break;
            }
            case MULTIPLY:
            {
                result.push_back(y.frac * x.frac);
                break;
            }
            case DIVIDE:
            {
                result.push_back(y.frac / x.frac);
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

token::token(fraction F1)
{
    frac = F1;
    type = FRACTION;
}

