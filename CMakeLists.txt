cmake_minimum_required(VERSION 3.16)
project(Calculator)

set(CMAKE_CXX_STANDARD 20)

add_executable(Calculator main.cpp fraction.h fraction.cpp myMath.h myMath.cpp expression.h expression.cpp)