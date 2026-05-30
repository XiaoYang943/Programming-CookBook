#include <chrono>
#include <iostream>
#include <thread>

/*
 * 作用域计时器也是 RAII
 *
 * 构造时记录开始时间
 * 析构时计算耗时并输出
 * 这样函数有多个出口时，也不用在每个出口都手写统计代码
 */
class ScopeTimer {
public:
    explicit ScopeTimer(const char* name)
        : name_(name), start_(std::chrono::steady_clock::now()) {
        std::cout << "开始计时：" << name_ << '\n';
    }

    ~ScopeTimer() {
        const auto end = std::chrono::steady_clock::now();
        const auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start_);
        std::cout << "结束计时：" << name_ << "，耗时 " << elapsed.count() << " ms\n";
    }

private:
    const char* name_;
    std::chrono::steady_clock::time_point start_;
};

int main() {
    ScopeTimer timer("模拟任务");

    std::this_thread::sleep_for(std::chrono::milliseconds(50));
    std::cout << "任务执行中\n";
}
