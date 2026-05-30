#include <iostream>
#include <memory>
#include <string>
#include <utility>

struct Widget {
    explicit Widget(std::string name) : name(std::move(name)) {
        std::cout << "构造 Widget：" << this->name << '\n';
    }

    ~Widget() {
        std::cout << "析构 Widget：" << name << '\n';
    }

    std::string name;
};

/*
 * std::unique_ptr 用 RAII 管理动态对象
 *
 * make_unique 创建对象，unique_ptr 独占对象所有权
 * 离开作用域时 unique_ptr 自动 delete，不需要手动释放
 */
int main() {
    {
        auto widget = std::make_unique<Widget>("独占资源");
        std::cout << "正在使用 Widget：" << widget->name << '\n';
    }

    std::cout << "离开作用域后，unique_ptr 已经自动释放对象\n";
}
