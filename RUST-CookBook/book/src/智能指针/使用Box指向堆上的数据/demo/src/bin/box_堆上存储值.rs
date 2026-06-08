fn main() {
    // 如果一个变量拥有一个数值 let value = 42，那变量 value 必然是存储在栈上的
    // 如果想要 value 的值存储在堆上就需要使用 Box<T>
    let value = Box::new(42);   // value 持有了该指针
    assert_eq!(*value, 42); // * 操作符对这个指针显式地解引用
}   // value 持有的智能指针，在作用域结束时被自动释放(因为 Box<T> 实现了 Drop Trait)
