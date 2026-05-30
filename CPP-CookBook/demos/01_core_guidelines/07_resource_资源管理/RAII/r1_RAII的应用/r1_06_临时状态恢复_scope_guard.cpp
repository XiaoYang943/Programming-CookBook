#include <iostream>
#include <utility>

bool g_verbose_log = false;

/*
 * RAII 不只用于“释放资源”，也常用于“恢复状态”
 *
 * 构造时保存旧状态并设置新状态
 * 析构时自动恢复旧状态
 */
class VerboseLogGuard {
public:
    explicit VerboseLogGuard(bool enabled) : old_value_(g_verbose_log) {
        g_verbose_log = enabled;
        std::cout << "进入作用域：临时设置 verbose_log = " << std::boolalpha << g_verbose_log << '\n';
    }

    ~VerboseLogGuard() {
        g_verbose_log = old_value_;
        std::cout << "离开作用域：自动恢复 verbose_log = " << std::boolalpha << g_verbose_log << '\n';
    }

private:
    bool old_value_;
};

int main() {
    std::cout << "初始 verbose_log = " << std::boolalpha << g_verbose_log << '\n';

    {
        VerboseLogGuard guard(true);
        std::cout << "作用域内使用临时配置\n";
    }

    std::cout << "作用域外配置已经恢复\n";
}
