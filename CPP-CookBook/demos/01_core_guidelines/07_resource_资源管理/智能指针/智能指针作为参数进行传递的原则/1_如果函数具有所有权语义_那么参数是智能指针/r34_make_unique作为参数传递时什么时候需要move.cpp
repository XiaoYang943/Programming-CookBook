//
// Created by Codex on 2026/5/31.
//

#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Job {
    explicit Job(std::string name) : name(std::move(name)) {
        std::cout << "Job [" << this->name << "] created\n";
    }

    ~Job() {
        std::cout << "Job [" << name << "] destroyed\n";
    }

    void run() const {
        std::cout << "Job [" << name << "] running\n";
    }

    std::string name;
};

class JobQueue {
public:
    /*
     * 生产中很常见的设计:
     * 调用者创建 Job, 队列接管 Job 的所有权, 之后由队列决定什么时候执行/销毁。
     *
     * 参数是 std::unique_ptr<Job>:
     * - 明确表达 submit() 会接管所有权
     * - 调用后, 调用者不应该再使用原来的 unique_ptr 指向的对象
     */
    void submit(std::unique_ptr<Job> job) {
        std::cout << "JobQueue::submit() owns [" << job->name << "] now\n";

        /*
         * 标准库成熟例子:
         * std::vector<std::unique_ptr<T>>::push_back(...)
         *
         * job 是一个有名字的 unique_ptr, 即使它的类型支持移动, 它本身也是左值。
         * 所以这里必须 std::move(job), 把所有权继续转移进 vector。
         */
        jobs_.push_back(std::move(job));
    }

    void run_all() const {
        for (const auto& job : jobs_) {
            job->run();
        }
    }

private:
    std::vector<std::unique_ptr<Job>> jobs_;
};

/*
 * 只是观察/使用对象, 不接管所有权。
 * 这种函数不应该接收 unique_ptr, 更不应该要求调用者 std::move。
 */
void log_job(const Job& job) {
    std::cout << "log only: [" << job.name << "]\n";
}

/*
 * 可能重新绑定调用者手里的 unique_ptr, 但不接管最终所有权。
 * 这对应 Core Guidelines R.33: 参数使用 std::unique_ptr<T>&。
 */
void replace_if_empty_name(std::unique_ptr<Job>& job) {
    if (job && job->name.empty()) {
        job = std::make_unique<Job>("default-name");
    }
}

int main() {
    JobQueue queue;

    std::cout << "\n=== 1. make_unique() 临时对象: 不需要 std::move ===\n";

    /*
     * std::make_unique<Job>(...) 返回的是临时 unique_ptr, 临时对象本来就是右值。
     * submit() 按值接收 unique_ptr, 会直接从这个临时对象移动构造参数。
     *
     * 所以下面这样写即可:
     */
    queue.submit(std::make_unique<Job>("from temporary make_unique"));

    /*
     * 下面这种写法也能编译, 但 std::move 是多余的:
     *
     * queue.submit(std::move(std::make_unique<Job>("redundant move")));
     *
     * 对 make_unique() 的返回值再 std::move, 没有带来额外价值。
     */

    std::cout << "\n=== 2. 有名字的 unique_ptr: 转移所有权时需要 std::move ===\n";

    auto batch_job = std::make_unique<Job>("from named variable");

    log_job(*batch_job);        // 只是借用对象, 不需要 std::move

    /*
     * batch_job 是一个有名字的变量, 因此它是左值。
     * unique_ptr 禁止拷贝, 所以下面这行不能编译:
     *
     * queue.submit(batch_job);
     *
     * 如果确实要把所有权交给 queue, 必须显式 std::move。
     */
    queue.submit(std::move(batch_job));

    if (!batch_job) {
        std::cout << "batch_job is empty after std::move\n";
    }

    std::cout << "\n=== 3. 函数只借用对象: 不需要 std::move ===\n";

    auto report_job = std::make_unique<Job>("report");

    /*
     * log_job() 接收 const Job&, 只临时使用对象。
     * 这里传 *report_job, 不转移所有权。
     */
    log_job(*report_job);
    report_job->run();

    std::cout << "\n=== 4. 函数重绑 unique_ptr: 也不需要 std::move ===\n";

    auto nameless_job = std::make_unique<Job>("");

    /*
     * replace_if_empty_name() 接收 std::unique_ptr<Job>&。
     * 它可能修改 nameless_job 指向谁, 但最终所有权仍在 main() 手里。
     */
    replace_if_empty_name(nameless_job);
    log_job(*nameless_job);

    std::cout << "\n=== 5. 标准库 vector<unique_ptr<T>>: 同样的规则 ===\n";

    std::vector<std::unique_ptr<Job>> jobs;

    /*
     * make_unique() 返回临时对象, push_back 可以直接移动进去。
     */
    jobs.push_back(std::make_unique<Job>("vector temporary"));

    auto vector_job = std::make_unique<Job>("vector named variable");

    /*
     * vector_job 是有名字的 unique_ptr, push_back 需要 std::move。
     */
    jobs.push_back(std::move(vector_job));

    if (!vector_job) {
        std::cout << "vector_job is empty after push_back(std::move(...))\n";
    }

    std::cout << "\n=== queue runs owned jobs ===\n";
    queue.run_all();

    std::cout << "\nEnd of main\n";
}
