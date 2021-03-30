#include <iostream>
#include <iomanip>
#include <string_view>
#include "fraction.h"
#include "expression.h"

int main()
{
    
    // Setting the precision to 9 decimal places
    std::cout << std::setprecision(10);
    
    std::cout << "Hello, Calculator!" << std::endl;
    /*fraction a = fraction(1, 1000000016531 * 1000000);
    fraction b = fraction(1, 1000000016347);
    std::cout << "A: " << a << ", B: " << b << std::endl;
    fraction c;
    */
    std::string userInput;
    std::cin >> userInput;
    std::string_view userView = userInput;
    auto e = expression(userView);
    /*
    c = a + b;
    std::cout << "C: " << c << std::endl;
    c = a - b;
    std::cout << "C: " << c << std::endl;
    c = a * b;
    std::cout << "C: " << c << std::endl;
    c = a / b;
    std::cout << "C: " << c << std::endl;
    
    auto d = (double) c;
    std::cout << "D: " << d << std::endl;
    
    std::cout << (float) c << std::endl;
    std::cout << (double) c << std::endl;
    */
    return 0;
}

