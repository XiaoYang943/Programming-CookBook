#include <iostream>
#include <mutex>

std::mutex g_mutex;
int g_counter = 0;

/*
 * 锁是 RAII 最经典的例子之一
 *
 * std::lock_guard 构造时加锁，析构时解锁
 * 这样就不用手动写 lock() / unlock()，也不会因为异常或提前 return 忘记解锁
 */
void increment_counter() {
    std::lock_guard<std::mutex> lock(g_mutex);

    ++g_counter;
    std::cout << "锁已持有，安全修改共享计数器：" << g_counter << '\n';
}

int main() {
    increment_counter();

    std::cout << "函数结束后 lock_guard 析构，锁已经自动释放\n";
}
