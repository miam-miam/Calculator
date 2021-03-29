#include <string>
#include <stdexcept>
#include "fraction.h"
#include "expression.h"

int *expression::tokenise(std::string_view &String)
{
    bool DecimalPoint;
    for (int I = 0; I < String.length(); I++)
    {
        if ('0' <= (int) String[I] and (int) String[I] <= '9')
        {
            DecimalPoint = false;
            for (int J = I; I < String.length(); J++)
            {
                if ('0' <= (int) String[J] and (int) String[J] <= '9')
                {
                    continue;
                }
                else if (!DecimalPoint and (String[J] == '.' or String[J] == ','))
                {
                    DecimalPoint = true;
                }
                else if (DecimalPoint and (String[J] == '.' or String[J] == ','))
                {
                    throw std::runtime_error("Should only have one decimal point.");
                }
                else
                {
                    break;
                }
                
                infixToken.emplace_back(String.substr(I, J - I));
                
            }
            
        }
        else if (String[I] == '+' or String[I] == '/' or String[I] == '*' or String[I] == '-' or String[I] == '(' or
            String[I] == ')')
        {
            infixToken.emplace_back(String[I]);
        }
    }
}

token::token(fraction F1)
{
    at = &F1;
    length = sizeof(F1);
    type = FRACTION;
}

token::token(char C1)
{
    at = &C1;
    length = sizeof(C1);
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
            throw std::runtime_error("Should only have one decimal point.");
        }
    }
}

