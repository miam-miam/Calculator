#include "number.h"
#include "expression.h"

int main()
{
    
    // Setting the precision to 9 decimal places
    std::cout << std::setprecision(10);
    std::cout << "Hello, Calculator!" << std::endl;
    
    while (true)
    {
        std::cout << "Input expression: " << std::endl;
        std::string userInput;
        std::cin >> userInput;
        std::string_view userView = userInput;
        if (userView == "stop" || userView == "Stop")
        {
            break;
        }
        auto e = expression(userView);
    }
    return 0;
}

