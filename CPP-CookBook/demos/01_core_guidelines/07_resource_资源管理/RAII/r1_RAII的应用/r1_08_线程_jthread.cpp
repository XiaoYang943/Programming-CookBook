#include <chrono>
#include <iostream>
#include <thread>

/*
 * std::jthread 是线程的 RAII 封装
 *
 * std::thread 析构前必须手动 join 或 detach
 * std::jthread 析构时会自动请求停止并 join，减少忘记 join 的风险
 */
int main() {
    {
        std::jthread worker([] {
            std::cout << "子线程开始工作\n";
            std::this_thread::sleep_for(std::chrono::milliseconds(30));
            std::cout << "子线程完成工作\n";
        });

        std::cout << "主线程继续执行\n";
    }

    std::cout << "离开作用域：jthread 析构并自动 join\n";
}
