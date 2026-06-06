#include <chrono>
#include <functional>
#include <iostream>
#include <memory>
#include <thread>

/*
 * 不要让从shared_ptr获取的裸指针脱离其所有权控制
 *
 * 这里用更常见的 shared_ptr.get() 场景说明：
 *
 *   不要让从 shared_ptr 获取的裸指针，脱离 shared_ptr 的生命周期控制
 *
 * 裸指针不拥有对象，也不会增加引用计数
 * 如果裸指针被异步回调、线程、缓存等保存下来，而原 shared_ptr 已经释放对象，
 * 这个裸指针就会悬空
 */

using namespace std::chrono_literals;

struct Connection {
    explicit Connection(int id) : id(id) {
        std::cout << "创建连接 #" << id << '\n';
    }

    ~Connection() {
        std::cout << "销毁连接 #" << id << '\n';
    }

    void send(const char* message) const {
        std::cout << "连接 #" << id << " 发送: " << message << '\n';
    }

    int id;
};

void post_delayed(std::function<void()> callback) {
    std::this_thread::sleep_for(50ms);
    callback();
}

void bad_raw_pointer_escapes() {
    std::cout << "\nbad_raw_pointer_escapes\n";

    auto connection = std::make_shared<Connection>(1);
    std::weak_ptr<Connection> observer = connection;

    /*
     * raw 只是一个观察用裸指针
     * 它不会让 Connection 多活一秒
     */
    Connection* raw = connection.get();
    std::cout << "从 shared_ptr.get() 得到裸指针: " << raw << '\n';

    auto callback = [raw, observer] {
        std::cout << "异步回调开始\n";

        if (observer.expired()) {
            std::cout << "对象已经销毁，raw 已经悬空，不能再使用 raw->send()\n";
            return;
        }

        raw->send("如果对象还活着，裸指针才碰巧可用");
    };

    connection.reset();
    std::cout << "原 shared_ptr reset，对象生命周期结束\n";

    post_delayed(callback);
}

void good_capture_shared_ptr() {
    std::cout << "\ngood_capture_shared_ptr\n";

    auto connection = std::make_shared<Connection>(2);

    /*
     * 回调捕获 shared_ptr，表示回调需要延长对象生命周期
     * 即使外部 shared_ptr reset，对象也会活到回调执行结束
     */
    auto callback = [connection] {
        std::cout << "异步回调开始\n";
        connection->send("回调持有 shared_ptr，对象一定还活着");
    };

    connection.reset();
    std::cout << "外部 shared_ptr reset，但回调仍持有一份所有权\n";

    post_delayed(callback);
}

void good_capture_weak_ptr() {
    std::cout << "\ngood_capture_weak_ptr\n";

    auto connection = std::make_shared<Connection>(3);
    std::weak_ptr<Connection> weak_connection = connection;

    /*
     * 回调捕获 weak_ptr，表示回调不延长对象生命周期
     * 使用前通过 lock() 检查对象是否仍然存在
     */
    auto callback = [weak_connection] {
        std::cout << "异步回调开始\n";

        if (auto connection = weak_connection.lock()) {
            connection->send("weak_ptr lock 成功，对象还活着");
        } else {
            std::cout << "weak_ptr lock 失败，对象已经销毁，安全放弃操作\n";
        }
    };

    connection.reset();
    std::cout << "外部 shared_ptr reset，weak_ptr 不延长生命周期\n";

    post_delayed(callback);
}

int main() {
    bad_raw_pointer_escapes();
    good_capture_shared_ptr();
    good_capture_weak_ptr();
}
