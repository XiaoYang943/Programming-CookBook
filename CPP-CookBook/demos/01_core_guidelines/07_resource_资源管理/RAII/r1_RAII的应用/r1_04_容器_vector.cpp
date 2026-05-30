#include <iostream>
#include <string>
#include <vector>

/*
 * 标准库容器是日常最常用的 RAII
 *
 * std::vector 会自动管理动态内存
 * 元素增加时它负责分配和扩容，离开作用域时它负责析构元素并释放内存
 */
int main() {
    {
        std::vector<std::string> names;
        names.push_back("std::vector");
        names.push_back("std::string");
        names.push_back("std::map");

        std::cout << "vector 自动管理了 " << names.size() << " 个字符串\n";
    }

    std::cout << "离开作用域后，vector 自动释放元素和动态内存\n";
}
