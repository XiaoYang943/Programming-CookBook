#include <cstdlib>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

/*
 * 避免 malloc() and free()
 *
 * 原因：
 *   malloc 只分配一段原始内存，不会调用构造函数，对象如果没有被构造，访问其成员时会发生 UB
 *   free 只释放一段原始内存，不会调用析构函数
 *
 * 这和 C++ 对象模型不匹配
 *
 * C++ 里优先使用：
 *   1. 局部对象：自动构造、自动析构
 *   2. 标准库容器：自动管理动态内存和元素生命周期
 *   3. 智能指针：表达所有权并自动释放
 */

struct Record {
    explicit Record(std::string name) : name(std::move(name)) {
        std::cout << "构造 Record: " << this->name << '\n';
    }

    ~Record() {
        std::cout << "析构 Record: " << name << '\n';
    }

    std::string name;
};

void bad_malloc_for_cpp_object() {
    std::cout << "\nbad_malloc_for_cpp_object\n";

    /*
     * malloc(sizeof(Record)) 只得到一块足够大的字节空间，
     * 但这块空间里并没有一个已经构造完成的 Record 对象
     *
     * Record 里的 std::string name 也没有被构造
     * 如果访问 p->name，就是未定义行为
     */
    auto* p = static_cast<Record*>(std::malloc(sizeof(Record)));
    std::cout << p->name << '\n'; // UB: p 指向的不是一个有效 Record 对象
    std::free(p);                 // 也不会调用 Record::~Record()

    std::cout << "malloc 只分配原始内存，不会构造 Record\n";
}

void good_local_object() {
    std::cout << "\ngood_local_object\n";

    Record record("local object");
    std::cout << "正在使用: " << record.name << '\n';

    std::cout << "离开函数时，局部对象会自动析构\n";
}

void good_vector_for_many_objects() {
    std::cout << "\ngood_vector_for_many_objects\n";

    std::vector<Record> records;
    records.emplace_back("record-1");
    records.emplace_back("record-2");

    std::cout << "vector 中有 " << records.size() << " 个 Record\n";
    std::cout << "离开函数时，vector 会自动析构元素并释放内存\n";
}

void good_unique_ptr_for_dynamic_object() {
    std::cout << "\ngood_unique_ptr_for_dynamic_object\n";

    auto record = std::make_unique<Record>("unique_ptr object");
    std::cout << "正在使用: " << record->name << '\n';

    std::cout << "离开函数时，unique_ptr 会自动 delete\n";
}

void malloc_for_raw_bytes_only() {
    std::cout << "\nmalloc_for_raw_bytes_only\n";

    /*
     * malloc/free 更适合 C 风格的原始字节内存
     * 即使必须使用，也要避免用它直接管理 C++ 对象
     */
    void* bytes = std::malloc(16);

    if (bytes == nullptr) {
        std::cout << "malloc failed\n";
        return;
    }

    std::cout << "获得 16 字节原始内存\n";
    std::free(bytes);
    std::cout << "释放 16 字节原始内存\n";
}

int main() {
    bad_malloc_for_cpp_object();
    good_local_object();
    good_vector_for_many_objects();
    good_unique_ptr_for_dynamic_object();
    malloc_for_raw_bytes_only();
}
