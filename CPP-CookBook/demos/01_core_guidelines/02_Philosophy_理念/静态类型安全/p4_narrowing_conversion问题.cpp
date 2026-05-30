#include <cmath>
#include <cstdint>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <type_traits>
#include <utility>

#include "cookbook/config.hpp"
#include <gsl/gsl>

/*
 * narrowing conversion: 把一个值转换到“更窄”的类型中
 *
 * 常见风险：
 *   1. 浮点数 -> 整数：小数部分被截断
 *   2. 大整数 -> 小整数：超出目标类型范围后结果变得难以直觉判断
 *   3. signed -> unsigned：负数会变成很大的无符号数
 *
 * safe 写法可以使用：
 *   - gsl::narrow<T>(value)：转换不安全时抛出 gsl::narrowing_error
 *   - std::in_range<T>(value)：C++23 风格的整数范围检查
 */

template <typename To, typename From>
To manual_narrow(From value) {
    static_assert(std::is_arithmetic_v<To>);
    static_assert(std::is_arithmetic_v<From>);

    if constexpr (std::is_floating_point_v<From> && std::is_integral_v<To>) {
        if (!std::isfinite(value)) {
            throw std::runtime_error("value is not finite");
        }

        if (value < static_cast<From>(std::numeric_limits<To>::min()) ||
            value > static_cast<From>(std::numeric_limits<To>::max())) {
            throw std::runtime_error("value is out of target range");
        }

        if (std::trunc(value) != value) {
            throw std::runtime_error("fractional part would be lost");
        }
    }

    To result = static_cast<To>(value);

    if (static_cast<From>(result) != value) {
        throw std::runtime_error("round-trip check failed");
    }

    return result;
}

void unsafe_double_to_int() {
    double value = 1234.56;
    int result = value;

    std::cout << "unsafe double -> int: " << result << '\n';
}

void safe_double_to_int_gsl() {
    double value = 1234.56;

    try {
        int result = gsl::narrow<int>(value);
        std::cout << "safe gsl double -> int: " << result << '\n';
    } catch (const gsl::narrowing_error& e) {
        std::cout << "safe gsl double -> int blocked: " << e.what() << '\n';
    }
}

void safe_double_to_int_manual() {
    double value = 1234.56;

    try {
        int result = manual_narrow<int>(value);
        std::cout << "safe manual double -> int: " << result << '\n';
    } catch (const std::exception& e) {
        std::cout << "safe manual double -> int blocked: " << e.what() << '\n';
    }
}

void unsafe_int_to_uint8() {
    int value = 300;
    std::uint8_t result = value;

    std::cout << "unsafe int -> uint8_t: " << static_cast<int>(result) << '\n';
}

void safe_int_to_uint8_gsl() {
    int value = 300;

    try {
        auto result = gsl::narrow<std::uint8_t>(value);
        std::cout << "safe gsl int -> uint8_t: " << static_cast<int>(result) << '\n';
    } catch (const gsl::narrowing_error& e) {
        std::cout << "safe gsl int -> uint8_t blocked: " << e.what() << '\n';
    }
}

void safe_int_to_uint8_cpp23_in_range() {
    int value = 300;

#if COOKBOOK_CPP23
    if (std::in_range<std::uint8_t>(value)) {
        auto result = static_cast<std::uint8_t>(value);
        std::cout << "safe C++23 int -> uint8_t: " << static_cast<int>(result) << '\n';
    } else {
        std::cout << "safe C++23 int -> uint8_t blocked: out of range\n";
    }
#else
    std::cout << "safe C++23 int -> uint8_t skipped: requires C++23\n";
#endif
}

void unsafe_int_to_unsigned() {
    int value = -1;
    unsigned int result = value;

    std::cout << "unsafe int -> unsigned int: " << result << '\n';
}

void safe_int_to_unsigned_gsl() {
    int value = -1;

    try {
        auto result = gsl::narrow<unsigned int>(value);
        std::cout << "safe gsl int -> unsigned int: " << result << '\n';
    } catch (const gsl::narrowing_error& e) {
        std::cout << "safe gsl int -> unsigned int blocked: " << e.what() << '\n';
    }
}

void safe_int_to_unsigned_cpp23_in_range() {
    int value = -1;

#if COOKBOOK_CPP23
    if (std::in_range<unsigned int>(value)) {
        auto result = static_cast<unsigned int>(value);
        std::cout << "safe C++23 int -> unsigned int: " << result << '\n';
    } else {
        std::cout << "safe C++23 int -> unsigned int blocked: out of range\n";
    }
#else
    std::cout << "safe C++23 int -> unsigned int skipped: requires C++23\n";
#endif
}

int main() {
    unsafe_double_to_int();
    safe_double_to_int_gsl();
    safe_double_to_int_manual();

    unsafe_int_to_uint8();
    safe_int_to_uint8_gsl();
    safe_int_to_uint8_cpp23_in_range();

    unsafe_int_to_unsigned();
    safe_int_to_unsigned_gsl();
    safe_int_to_unsigned_cpp23_in_range();
}
