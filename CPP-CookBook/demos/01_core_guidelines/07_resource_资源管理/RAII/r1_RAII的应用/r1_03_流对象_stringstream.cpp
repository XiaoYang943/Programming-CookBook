#include <iostream>
#include <sstream>
#include <string>

/*
 * 流对象也是 RAII
 *
 * 文件流会在构造/打开时获取文件句柄，在析构时关闭文件句柄
 * 这里为了避免 demo 运行后产生额外文件，使用 std::ostringstream 演示同样的思想：
 *   流对象管理自己的内部缓冲区，离开作用域时自动释放
 */
int main() {
    std::string content;

    {
        std::ostringstream stream;
        stream << "name=CookBook-CPP\n";
        stream << "topic=RAII\n";

        content = stream.str();
        std::cout << "作用域内：流对象正在管理内部缓冲区\n";
    }

    std::cout << "离开作用域：流对象析构，内部缓冲区自动释放\n";
    std::cout << "生成的字符串内容：\n" << content;
}
