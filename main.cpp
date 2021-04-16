#include "number.h"
#include "expression.h"
#include "myMath.h"

int main()
{
    // Setting the precision to 13 decimal places
    std::cout << std::setprecision(14);
    std::cout << "Hello, Calculator!" << std::endl;
    
    auto base = Number(-10, 9);
    auto exp = Number(2, 5);
    std::cout << powNum(base, exp) << std::endl;
    
    while (true)
    {
        try
        {
            std::cout << "Input expression: " << std::endl;
            std::string userInput;
            std::getline(std::cin, userInput);
            std::string_view userView = userInput;
            if (userView == "stop" || userView == "Stop")
            {
                break;
            }
            auto e = Expression(userView);
            std::cout << e << std::endl;
        }
        catch (CalculatorException &e)
        {
            std::cout << "Caught error: " << e.what() << std::endl;
        }
    }
    return 0;
}

