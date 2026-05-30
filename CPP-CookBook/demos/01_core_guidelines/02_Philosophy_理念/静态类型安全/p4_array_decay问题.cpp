#include <iostream>

#include "cookbook/config.hpp"

#if COOKBOOK_CPP20
#include <span>
#endif

/*
 * array decay 数组退化 问题：
 *
 * 在 main() 中，values 是一个数组：
 *
 *   int values[10]{};
 *
 * 内存结构可以理解为：
 *
 *   values
 *     |
 *     v
 *   +-----------+-----------+-----------+-----+-----------+
 *   | values[0] | values[1] | values[2] | ... | values[9] |
 *   +-----------+-----------+-----------+-----+-----------+
 *   <---------------- 10 个 int，长度信息还在 ---------------->
 *
 * 但是当数组传给 unsafe_api(const int arr[]) 时，函数参数中的 arr[]
 * 会退化成指针，等价于：
 *
 *   void unsafe_api(const int* arr)
 *
 * 此时 unsafe_api 只知道：
 *
 *   arr
 *    |
 *    v
 *   +-----------+-----------+-----------+-----+-----------+
 *   | values[0] | values[1] | values[2] | ... | values[9] |
 *   +-----------+-----------+-----------+-----+-----------+
 *
 * arr 只是指向 values[0] 的指针，不再携带“数组长度是 10”这个信息
 * 所以在 unsafe_api 里 sizeof(arr) 得到的是指针大小，不是整个数组大小
 */
void unsafe_api(const int arr[]) {
    // std::cout << "unsafe_api: sizeof(arr) = " << sizeof(arr)
    //           << " bytes, because arr is a pointer\n";
}

/*
 * 使用数组引用参数
 *
 * int (&arr)[N] 的意思是：
 *   arr 是一个引用，引用到“长度为 N 的 int 数组”
 *
 * 这样数组不会退化成指针，N 会在编译期被推导出来
 */
template <std::size_t N>
void safe_api(int (&arr)[N]) {
    std::cout << "safe_api: array length = " << N << '\n';
}

/*
 * std::span 是 C++20 提供的“指针 + 长度”视图
 *
 * 它不拥有数组内存，只是安全地描述一段连续内存：
 *
 *   span
 *   +---------+--------+
 *   | pointer | length |
 *   +---------+--------+
 *       |
 *       v
 *   +-----------+-----------+-----------+-----+-----------+
 *   | values[0] | values[1] | values[2] | ... | values[9] |
 *   +-----------+-----------+-----------+-----+-----------+
 *
 * 这样函数既能访问数组元素，也能知道数组长度
 */
#if COOKBOOK_CPP20
void safe_api_span(std::span<int> arr) {
    std::cout << "safe_api_span: array length = " << arr.size() << '\n';
}
#endif

int main() {
    int values[10]{};

    unsafe_api(values); // array decay: int[10] -> const int*
    safe_api(values);   // template reference: keeps size information

#if COOKBOOK_CPP20
    safe_api_span(values); // std::span: pointer + length
#else
    std::cout << "safe_api_span: requires C++20\n";
#endif
}
