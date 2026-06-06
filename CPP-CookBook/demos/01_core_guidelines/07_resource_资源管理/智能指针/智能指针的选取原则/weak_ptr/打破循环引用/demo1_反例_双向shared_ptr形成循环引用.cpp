#include <iostream>
#include <memory>
#include "demo1.h"

/*
 * 反例：
 * next 和 prev 都使用 shared_ptr
 *
 * 为什么不要这样做：
 * shared_ptr 会增加引用计数
 * 如果两个对象互相通过 shared_ptr 持有对方，就会形成循环引用
 * 循环引用会导致引用计数永远无法归零，对象不会被销毁，最终造成内存泄漏
 */
class BadNode : public NodeBase {
public:
    explicit BadNode(int value) : NodeBase(value) {
        std::cout << "反例节点 " << value << " 已创建\n";
    }

    ~BadNode() override {
        std::cout << "反例节点 " << value << " 已销毁\n";
    }

    // next 表示当前节点拥有后继节点，这里使用 shared_ptr 本身是合理的
    std::shared_ptr<BadNode> next;

    // 关键问题：prev 只是表示前驱关系，但这里也用了 shared_ptr
    // 为什么不要这样做：prev 会增加前驱节点的引用计数，从而和 next 一起形成循环引用
    std::shared_ptr<BadNode> prev;
};

int main() {
    std::cout << "============================\n";
    std::cout << "反例：双向 shared_ptr 形成循环引用\n";
    std::cout << "============================\n";

    {
        auto node1 = std::make_shared<BadNode>(1);
        auto node2 = std::make_shared<BadNode>(2);

        // node1 通过 next 持有 node2
        node1->next = node2;

        // node2 通过 prev 又持有 node1
        // 关键问题：两个对象互相拥有对方，形成 shared_ptr 引用环
        node2->prev = node1;

        std::cout << "node1 的引用计数：" << node1.use_count() << "\n";
        std::cout << "node2 的引用计数：" << node2.use_count() << "\n";
    }

    std::cout << "作用域已结束\n";
    
    std::cout << "如果没有看到“已销毁”，说明双向 shared_ptr 形成循环引用，导致对象没有释放\n";

    return 0;
}