#include "expression.h"
#include <string>
#include <stdexcept>


int *expression::tokenise(std::string &string)
{
    bool decimalPoint;
    for (int i = 0; i < string.length(); i++)
    {
        if ('0' <= (int) string[i] and (int) string[i] <= '9')
        {
            decimalPoint = false;
            for (int j = i; i < string.length(); j++)
            {
                if ('0' <= (int) string[j] and (int) string[j] <= '9')
                {
                    continue;
                }
                else if (!decimalPoint and (string[j] == '.' or string[j] == ','))
                {
                    decimalPoint = true;
                }
                else if (decimalPoint and (string[j] == '.' or string[j] == ','))
                {
                    throw std::runtime_error("Should only have one decimal point.");
                }
                else
                {
                    break;
                }
                infixToken.emplace_back(string.substr(i, j - i));
            }


        }
        else if (string[i] == '+' or string[i] == '/' or string[i] == '*' or string[i] == '-' or string[i] == '(' or
                 string[i] == ')')
        {

        }
    }
}
