#include <iostream>
#include <memory>
#include "demo1.h"

/*
 * 正例：
 * next 使用 shared_ptr，prev 使用 weak_ptr
 *
 * 为什么要这样做：
 * next 表示拥有后继节点，所以可以使用 shared_ptr
 * prev 只是回看前驱节点，不应该拥有前驱节点，所以应该使用 weak_ptr
 *
 * weak_ptr 不会增加引用计数，因此可以打破 shared_ptr 形成的循环引用
 */
class GoodNode : public Demo1NodeBase {
public:
    explicit GoodNode(int value) : Demo1NodeBase(value) {
        std::cout << "正例节点 " << value << " 已创建\n";
    }

    ~GoodNode() override {
        std::cout << "正例节点 " << value << " 已销毁\n";
    }

    // 关键点：next 表示拥有后继节点
    // 为什么要这样做：当前节点负责维护后继节点的生命周期
    std::shared_ptr<GoodNode> next;

    // 关键点：prev 只是观察前驱节点，不拥有前驱节点
    // 为什么要这样做：weak_ptr 不增加引用计数，可以打破循环引用
    std::weak_ptr<GoodNode> prev;
};

int main() {
    std::cout << "============================\n";
    std::cout << "正例：使用 weak_ptr 打破循环引用\n";
    std::cout << "============================\n";

    {
        auto node1 = std::make_shared<GoodNode>(1);
        auto node2 = std::make_shared<GoodNode>(2);

        // node1 拥有 node2
        node1->next = node2;

        // node2 只是观察 node1，不拥有 node1
        // 关键点：这里使用 weak_ptr，不会增加 node1 的引用计数
        node2->prev = node1;

        std::cout << "node1 的引用计数：" << node1.use_count() << "\n";
        std::cout << "node2 的引用计数：" << node2.use_count() << "\n";

        /*
         * 关键点：
         * weak_ptr 不能直接使用 -> 访问对象
         *
         * 为什么要这样做：
         * weak_ptr 指向的对象可能已经被销毁
         * lock() 会尝试把 weak_ptr 转成 shared_ptr
         * 如果对象还活着，就能安全访问
         * 如果对象已经销毁，lock() 会返回空 shared_ptr
         */
        if (auto prevNode = node2->prev.lock()) {
            std::cout << "node2 的前驱节点值：" << prevNode->getValue() << "\n";
        } else {
            std::cout << "node2 的前驱节点已经被销毁\n";
        }
    }

    std::cout << "作用域已结束\n";
    std::cout << "如果看到“正例节点已销毁”，说明 weak_ptr 成功打破了循环引用\n";

    return 0;
}