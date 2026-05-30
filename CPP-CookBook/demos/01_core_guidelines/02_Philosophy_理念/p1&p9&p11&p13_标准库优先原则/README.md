# 标准库优先原则：常用 STL 速查

## 容器

- `std::array`：固定长度数组，大小在编译期确定
- `std::vector`：动态连续数组，适合随机访问和尾部追加
- `std::deque`：双端队列，适合头尾两端高效插入和删除
- `std::list`：双向链表，适合已知位置的高效插入和删除
- `std::forward_list`：单向链表，比 `std::list` 更轻量
- `std::set`：有序唯一集合，自动按键排序
- `std::multiset`：有序集合，允许重复元素
- `std::map`：有序键值表，按 key 自动排序
- `std::multimap`：有序键值表，允许多个相同 key
- `std::unordered_set`：哈希唯一集合，平均 O(1) 查找
- `std::unordered_multiset`：哈希集合，允许重复元素
- `std::unordered_map`：哈希键值表，平均 O(1) 按 key 查找
- `std::unordered_multimap`：哈希键值表，允许多个相同 key
- `std::stack`：后进先出容器适配器
- `std::queue`：先进先出容器适配器
- `std::priority_queue`：优先队列，默认每次取出最大元素

## 字符串与视图

- `std::string`：拥有字符内存的动态字符串
- `std::string_view`：不拥有字符内存的只读字符串视图
- `std::to_string`：把数值转换成字符串
- `std::stoi` / `std::stol` / `std::stod`：把字符串转换成数值
- `std::getline`：按行读取输入
- `std::format`：C++20 格式化字符串

## 算法

- `std::find`：在线性范围中查找指定值
- `std::find_if`：在线性范围中查找满足条件的元素
- `std::count`：统计指定值出现次数
- `std::count_if`：统计满足条件的元素数量
- `std::all_of`：判断范围内所有元素是否都满足条件
- `std::any_of`：判断范围内是否至少有一个元素满足条件
- `std::none_of`：判断范围内是否没有元素满足条件
- `std::for_each`：对范围内每个元素执行操作
- `std::transform`：把一个范围转换到另一个范围
- `std::copy`：复制范围内的元素
- `std::copy_if`：复制满足条件的元素
- `std::remove`：把要移除的元素移动到末尾并返回新逻辑尾
- `std::remove_if`：把满足移除条件的元素移动到末尾并返回新逻辑尾
- `std::erase`：C++20 从容器中删除指定值
- `std::erase_if`：C++20 从容器中删除满足条件的元素
- `std::sort`：对范围排序
- `std::stable_sort`：稳定排序，相等元素保持原有相对顺序
- `std::partial_sort`：只排序前 N 个元素
- `std::nth_element`：把第 N 个元素放到排序后应在的位置
- `std::binary_search`：在有序范围中判断值是否存在
- `std::lower_bound`：在有序范围中找到第一个不小于目标值的位置
- `std::upper_bound`：在有序范围中找到第一个大于目标值的位置
- `std::equal_range`：在有序范围中找到等于目标值的区间
- `std::merge`：合并两个有序范围
- `std::unique`：把连续重复元素移动到末尾并返回新逻辑尾
- `std::reverse`：反转范围内元素顺序
- `std::rotate`：循环移动范围内元素
- `std::shuffle`：随机打乱范围内元素
- `std::min_element`：找到最小元素
- `std::max_element`：找到最大元素
- `std::minmax_element`：同时找到最小和最大元素

## 迭代器与范围

- `std::begin` / `std::end`：获取范围的起始和结束迭代器
- `std::next`：向前移动迭代器
- `std::prev`：向后移动迭代器
- `std::advance`：原地移动迭代器
- `std::distance`：计算两个迭代器之间的距离
- `std::back_inserter`：把算法输出追加到容器尾部
- `std::front_inserter`：把算法输出插入到容器头部
- `std::inserter`：把算法输出插入到指定位置
- `std::ranges::find`：C++20 范围版查找算法
- `std::ranges::sort`：C++20 范围版排序算法
- `std::ranges::views::filter`：C++20 惰性过滤视图
- `std::ranges::views::transform`：C++20 惰性转换视图

## 数值

- `std::accumulate`：累加范围内元素
- `std::reduce`：可并行化的归约计算
- `std::inner_product`：计算两个范围的内积
- `std::partial_sum`：计算前缀和
- `std::exclusive_scan`：计算不包含当前元素的前缀扫描
- `std::inclusive_scan`：计算包含当前元素的前缀扫描
- `std::iota`：用连续递增值填充范围
- `std::gcd`：计算最大公约数
- `std::lcm`：计算最小公倍数
- `std::midpoint`：安全计算两个数的中点
- `std::lerp`：线性插值

## 内存与资源管理

- `std::unique_ptr`：独占所有权智能指针
- `std::shared_ptr`：共享所有权智能指针
- `std::weak_ptr`：观察 `shared_ptr` 管理对象但不延长生命周期
- `std::make_unique`：安全创建 `unique_ptr`
- `std::make_shared`：安全创建 `shared_ptr`
- `std::move`：把对象转换成右值以支持移动语义
- `std::forward`：在模板中保持参数的值类别
- `std::swap`：交换两个对象的值

## 类型工具

- `std::optional`：表示可能没有值的对象
- `std::variant`：类型安全的联合体
- `std::any`：保存任意可复制类型的值
- `std::tuple`：固定数量、可不同类型的值组合
- `std::pair`：两个值的简单组合
- `std::type_traits`：在编译期查询和变换类型属性
- `std::is_same_v`：判断两个类型是否相同
- `std::is_integral_v`：判断类型是否为整数类型
- `std::is_floating_point_v`：判断类型是否为浮点类型
- `std::underlying_type_t`：获取枚举类型的底层整数类型

## 错误处理

- `std::exception`：标准异常基类
- `std::runtime_error`：表示运行时错误
- `std::logic_error`：表示逻辑错误
- `std::out_of_range`：表示访问越界
- `std::invalid_argument`：表示参数无效
- `std::error_code`：表示可携带平台错误码的错误对象
- `std::expected`：C++23 表示“值或错误”的返回结果

## 时间

- `std::chrono::duration`：表示时间长度
- `std::chrono::time_point`：表示时间点
- `std::chrono::steady_clock`：单调时钟，适合测量耗时
- `std::chrono::system_clock`：系统时钟，适合表示现实时间
- `std::chrono::high_resolution_clock`：高精度时钟别名，具体精度依实现而定

## 文件系统

- `std::filesystem::path`：跨平台表示文件路径
- `std::filesystem::exists`：判断路径是否存在
- `std::filesystem::is_regular_file`：判断路径是否为普通文件
- `std::filesystem::is_directory`：判断路径是否为目录
- `std::filesystem::create_directory`：创建单级目录
- `std::filesystem::create_directories`：创建多级目录
- `std::filesystem::directory_iterator`：遍历目录下的直接子项
- `std::filesystem::recursive_directory_iterator`：递归遍历目录
- `std::filesystem::file_size`：获取文件大小
- `std::filesystem::copy`：复制文件或目录
- `std::filesystem::rename`：移动或重命名文件

## 并发

- `std::thread`：创建和管理线程
- `std::jthread`：C++20 自动 join 的线程
- `std::mutex`：互斥锁
- `std::lock_guard`：作用域互斥锁封装
- `std::unique_lock`：可延迟加锁和手动解锁的互斥锁封装
- `std::scoped_lock`：同时锁定多个互斥量
- `std::condition_variable`：线程间等待和通知
- `std::atomic`：原子变量
- `std::async`：异步启动任务
- `std::future`：接收异步结果
- `std::promise`：手动设置异步结果

## 输入输出

- `std::cin`：标准输入流
- `std::cout`：标准输出流
- `std::cerr`：标准错误输出流
- `std::ifstream`：文件输入流
- `std::ofstream`：文件输出流
- `std::fstream`：文件输入输出流
- `std::stringstream`：字符串输入输出流
- `std::setw`：设置输出宽度
- `std::setprecision`：设置浮点输出精度
- `std::fixed`：使用定点格式输出浮点数
