#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

/*
 * 避免显式调用 new 和 delete
 *
 * 原因：
 *   显式 new/delete 会把“释放资源”的责任分散到代码路径里
 *   每个 new 都必须刚好对应一次 delete，而且异常、提前 return、多个分支都不能漏
 *
 * C++ 里优先使用：
 *   1. 局部对象：不需要动态分配时，直接放在栈上
 *   2. std::make_unique：需要独占动态对象时，表达所有权
 *   3. std::vector：需要动态数组时，交给容器管理
 */

struct Connection {
    explicit Connection(std::string name) : name(std::move(name)) {
        std::cout << "打开连接: " << this->name << '\n';
    }

    ~Connection() {
        std::cout << "关闭连接: " << name << '\n';
    }

    void send(const std::string& message) const {
        std::cout << name << " 发送消息: " << message << '\n';
    }

    std::string name;
};

void bad_manual_new_delete() {
    std::cout << "\nbad_manual_new_delete\n";

    /*
     * 问题：
     *   1. 中途 throw 时 delete 不会执行
     *   2. 多个 return 分支都必须记得 delete
     *   3. 指针传给别的函数后，谁负责 delete 不清楚
     */
    auto* connection = new Connection("manual-new");
    connection->send("hello");
    throw std::runtime_error("something failed before delete");
    delete connection;

    std::cout << "显式 new/delete 要求每条路径都正确释放，太容易出错\n";
}

void good_local_object() {
    std::cout << "\ngood_local_object\n";

    Connection connection("local object");
    connection.send("不需要动态分配时，直接使用局部对象");

    std::cout << "离开函数时，局部对象自动析构\n";
}

void good_unique_ptr_with_exception() {
    std::cout << "\ngood_unique_ptr_with_exception\n";

    try {
        auto connection = std::make_unique<Connection>("unique_ptr");
        connection->send("unique_ptr 独占所有权");

        throw std::runtime_error("模拟异常：离开作用域前失败");
    } catch (const std::exception& e) {
        std::cout << "捕获异常: " << e.what() << '\n';
        std::cout << "异常发生时，unique_ptr 已经自动释放对象\n";
    }
}

void good_transfer_ownership() {
    std::cout << "\ngood_transfer_ownership\n";

    auto connection = std::make_unique<Connection>("owned connection");
    connection->send("所有权现在在 connection 变量里");

    auto another_owner = std::move(connection);
    another_owner->send("所有权已经转移到 another_owner");

    if (connection == nullptr) {
        std::cout << "原 unique_ptr 已经为空，所有权关系清晰\n";
    }
}

void good_vector_instead_of_new_array() {
    std::cout << "\ngood_vector_instead_of_new_array\n";

    /*
     * 不推荐：
     *   auto* values = new int[3]{1, 2, 3};
     *   delete[] values;
     *
     * 推荐：
     *   vector 自动管理动态数组内存
     */
    std::vector<int> values{1, 2, 3};

    for (int value : values) {
        std::cout << value << ' ';
    }

    std::cout << "\n离开函数时，vector 自动释放数组内存\n";
}

int main() {
    bad_manual_new_delete();
    good_local_object();
    good_unique_ptr_with_exception();
    good_transfer_ownership();
    good_vector_instead_of_new_array();
}
