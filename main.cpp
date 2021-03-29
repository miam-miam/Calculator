#include <iostream>
#include <iomanip>
#include <string>
#include "fraction.h"

int main()
{
    // Setting the precision to 9 decimal places
    std::cout << std::setprecision(10);
    
    std::cout << "Hello, Calculator!" << std::endl;
    fraction A = fraction(1, 1000000016531 * 1000000);
    fraction B = fraction(1, 1000000016347);
    std::cout << "A: " << A << ", B: " << B << std::endl;
    fraction C;
    
    std::string UserInput;
    std::cin >> UserInput;
    auto E = fraction(UserInput);
    std::cout << "E: " << E << std::endl;
    
    C = A + B;
    std::cout << "C: " << C << std::endl;
    C = A - B;
    std::cout << "C: " << C << std::endl;
    C = A * B;
    std::cout << "C: " << C << std::endl;
    C = A / B;
    std::cout << "C: " << C << std::endl;
    
    auto D = (double) C;
    std::cout << "D: " << D << std::endl;
    
    std::cout << (float) C << std::endl;
    std::cout << (double) C << std::endl;
    
    return 0;
}
