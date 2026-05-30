#include <iostream>

/*
 * RAII: Resource Acquisition Is Initialization(资源获取即初始化)
 *
 * 精髓：
 *   资源不要裸露在外面手动管理，而是交给对象管理
 *
 *   构造函数获取资源
 *   析构函数释放资源
 *   对象的生命周期就是资源的生命周期
 *
 * 这样正常返回、提前返回、异常退出时，局部对象都会自动析构，资源也会自动释放
 */

class DemoResource {
public:
    explicit DemoResource(const char* name) : name_(name) {
        std::cout << "构造：获取资源 -> " << name_ << '\n';
    }

    ~DemoResource() {
        std::cout << "析构：释放资源 -> " << name_ << '\n';
    }

private:
    const char* name_;
};

int main() {
    std::cout << "进入 main\n";

    {
        DemoResource resource("演示资源");
        std::cout << "资源正在作用域内被使用\n";
    }

    std::cout << "离开作用域后，资源已经自动释放\n";
}
