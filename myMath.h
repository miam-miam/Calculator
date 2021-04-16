

#ifndef CALCULATOR__MYMATH_H
#define CALCULATOR__MYMATH_H
#include "number.h"

int64_t tenPowll(unsigned long long Exp);
SafeInt<int64_t> powSI(int64_t Base, int64_t Exponent);
int ceilLog2(unsigned long long Base);
Fraction binomialSeries(Fraction FractionBase, SafeInt<int64_t> Exponent);
SafeInt<int64_t> saferMultiplyDivide(SafeInt<int64_t> Multiplicand1,
                                     SafeInt<int64_t> Multiplicand2,
                                     SafeInt<int64_t> Divisor);
SafeInt<int64_t> saferMultiplyDivide(SafeInt<int64_t> Multiplicand1,
                                     SafeInt<int64_t> Multiplicand2,
                                     SafeInt<int64_t> Multiplicand3,
                                     SafeInt<int64_t> Divisor);

Number powNum(Number Base, Number Exponent);

#endif //CALCULATOR__MYMATH_H
