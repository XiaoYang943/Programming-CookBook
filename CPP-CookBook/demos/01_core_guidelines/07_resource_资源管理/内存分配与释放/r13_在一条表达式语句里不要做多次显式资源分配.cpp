#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>

/*
 * 一条表达式语句里最多只做一次显式资源分配
 *
 * 重点不是“函数不能接收两个对象”，而是不要写这种代码：
 *
 *   use(new A, new B);
 *
 * 如果多个 new 混在同一条表达式里，求值过程中一旦抛异常，
 * 某个已经 new 出来的裸指针可能还没有被 RAII 对象接管，从而泄漏
 *
 * 最佳实践：
 *   1. 不写裸 new
 *   2. 使用 make_unique / make_shared
 *   3. 需要多次资源分配时，拆成多条语句，让每个资源尽快被 RAII 对象接管
 */

class Plugin {
public:
    explicit Plugin(std::string name, bool fail = false) : name_(std::move(name)) {
        std::cout << "构造插件: " << name_ << '\n';

        if (fail) {
            std::cout << "插件构造失败: " << name_ << '\n';
            throw std::runtime_error("plugin init failed: " + name_);
        }
    }

    ~Plugin() {
        std::cout << "析构插件: " << name_ << '\n';
    }

    void run() const {
        std::cout << "运行插件: " << name_ << '\n';
    }

private:
    std::string name_;
};

void install_plugins_raw(Plugin* left, Plugin* right) {
    std::cout << "安装插件 raw pointer 版本\n";

    left->run();
    right->run();

    delete right;
    delete left;
}

void install_plugins(std::unique_ptr<Plugin> left, std::unique_ptr<Plugin> right) {
    std::cout << "安装插件 unique_ptr 版本\n";

    left->run();
    right->run();
}

void bad_multiple_new_in_one_expression() {
    std::cout << "\nbad_multiple_new_in_one_expression\n";

    /*
     * 一条表达式里出现两个显式 new
     * 如果第二个 Plugin 构造失败，第一个 new 出来的裸指针可能还没被安全接管
     */
    install_plugins_raw(
        new Plugin("logger"),
        new Plugin("metrics", true)
    );

    std::cout << "不要在一条表达式里写多个 new\n";
}

void better_one_allocation_per_statement() {
    std::cout << "\nbetter_one_allocation_per_statement\n";

    try {
        /*
         * 每条语句最多一个显式资源分配
         * 第一个资源创建后立刻交给 unique_ptr 管理
         * 第二个资源创建失败时，logger 会自动析构，不会泄漏
         */
        std::unique_ptr<Plugin> logger(new Plugin("logger"));
        std::unique_ptr<Plugin> metrics(new Plugin("metrics", true));

        install_plugins(std::move(logger), std::move(metrics));
    } catch (const std::exception& e) {
        std::cout << "捕获异常: " << e.what() << '\n';
        std::cout << "logger 已经由 unique_ptr 自动清理\n";
    }
}

void best_use_make_unique() {
    std::cout << "\nbest_use_make_unique\n";

    auto logger = std::make_unique<Plugin>("logger");
    auto metrics = std::make_unique<Plugin>("metrics");

    install_plugins(std::move(logger), std::move(metrics));

    std::cout << "make_unique 避免裸 new，所有权表达清晰\n";
}

int main() {
    bad_multiple_new_in_one_expression();
    better_one_allocation_per_statement();
    best_use_make_unique();
}
