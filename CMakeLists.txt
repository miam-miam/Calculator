cmake_minimum_required(VERSION 3.16)
project(Calculator)

set(CMAKE_CXX_STANDARD 20)

add_executable(Calculator main.cpp number.h number.cpp myMath.h myMath.cpp expression.cpp expression.h safeInt.hpp exceptions.h)

target_precompile_headers(Calculator PRIVATE pch.h)