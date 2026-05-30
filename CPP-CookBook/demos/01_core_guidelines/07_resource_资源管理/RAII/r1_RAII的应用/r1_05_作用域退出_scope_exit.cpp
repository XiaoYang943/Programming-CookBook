#include <iostream>
#include <utility>

/*
 * scope_exit 是一种通用 RAII 技巧
 *
 * 构造时登记一个清理动作，析构时自动执行
 * 适合临时资源、状态回滚、日志收尾等场景
 */
template <typename Function>
class ScopeExit {
public:
    explicit ScopeExit(Function function) : function_(std::move(function)) {}

    ScopeExit(const ScopeExit&) = delete;
    ScopeExit& operator=(const ScopeExit&) = delete;

    ~ScopeExit() {
        function_();
    }

private:
    Function function_;
};

template <typename Function>
ScopeExit<Function> make_scope_exit(Function function) {
    return ScopeExit<Function>(std::move(function));
}

int main() {
    std::cout << "开始处理任务\n";

    auto cleanup = make_scope_exit([] {
        std::cout << "离开作用域：自动执行收尾动作\n";
    });

    std::cout << "任务处理中\n";
    std::cout << "main 即将结束\n";
}
