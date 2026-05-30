#pragma once

#include <iostream>

class Demo1NodeBase {
public:
    explicit Demo1NodeBase(int value) : value(value) {
        std::cout << "节点基类 " << value << " 已创建\n";
    }

    virtual ~Demo1NodeBase() {
        std::cout << "节点基类 " << value << " 已销毁\n";
    }

    int getValue() const {
        return value;
    }

protected:
    int value;
};