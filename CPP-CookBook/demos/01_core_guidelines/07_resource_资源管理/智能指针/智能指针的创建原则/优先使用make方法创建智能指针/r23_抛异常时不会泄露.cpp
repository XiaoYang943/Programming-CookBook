//
// Created by Administrator on 2026/2/18.
//

#include <iostream>
#include <memory>
#include <stdexcept>

struct Foo {
    Foo(int x) {
        std::cout << "Foo constructed\n";
        if (x == -1) {
            std::cout << "Throwing in constructor!\n";
            throw std::runtime_error("construction failed");
        }
    }

    ~Foo() {
        std::cout << "Foo destructed\n";
    }
};

void process(std::unique_ptr<Foo> p) {
    std::cout << "process() called\n";
}

void bad_case() {
    std::cout << "=== BAD: constructing unique_ptr from new ===\n";

    try {
        // ❌ 不推荐写法
        std::unique_ptr<Foo> p(new Foo(1));
        process(std::move(p));
    }
    catch (const std::exception& e) {
        std::cout << "Caught: " << e.what() << "\n";
    }
}

void good_case() {
    std::cout << "\n=== GOOD: using make_unique ===\n";

    try {
        // ✅ 推荐写法
        auto p = std::make_unique<Foo>(1);
        process(std::move(p));
    }
    catch (const std::exception& e) {
        std::cout << "Caught: " << e.what() << "\n";
    }
}

void exception_case() {
    std::cout << "\n=== Exception case ===\n";

    try {
        // 构造函数抛异常时不会泄漏
        auto p = std::make_unique<Foo>(-1);
    }
    catch (const std::exception& e) {
        std::cout << "Caught: " << e.what() << "\n";
    }
}

int main() {
    bad_case();

    good_case();

    exception_case();
}
