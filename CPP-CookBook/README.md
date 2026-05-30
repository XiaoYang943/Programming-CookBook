# CookBook-CPP

一个用于学习 C++ 语法和 C++ Core Guidelines 的 demo 工程

## 依赖子仓库

本工程使用 Git submodule 引入 Microsoft GSL：

```text
third_party/microsoft-gsl -> https://github.com/microsoft/GSL.git
```

GSL 当前固定在：

```text
v4.2.0
```

## 克隆工程

推荐直接递归克隆：

```powershell
git clone --recurse-submodules https://github.com/XiaoYang943/CookBook-CPP.git
```

如果已经普通 clone 了，需要再初始化子仓库：

```powershell
git submodule update --init --recursive
```

## 检查子仓库状态

查看当前子仓库指向的提交：

```powershell
git submodule status
```

查看 GSL 当前版本：

```powershell
git -C third_party/microsoft-gsl describe --tags --exact-match
```

## 更新 GSL 版本

进入子仓库：

```powershell
cd third_party/microsoft-gsl
```

拉取远端 tag：

```powershell
git fetch --tags
```

切换到指定版本，例如：

```powershell
git checkout v4.2.0
```

回到主工程并提交 submodule 指针变化：

```powershell
cd ../..
git add third_party/microsoft-gsl
git commit -m "Update Microsoft GSL submodule"
```

## CMake 集成方式

根目录的 `CMakeLists.txt` 会优先检测：

```text
third_party/microsoft-gsl/CMakeLists.txt
```

如果存在，就通过：

```cmake
add_subdirectory(third_party/microsoft-gsl EXCLUDE_FROM_ALL)
```

把 GSL 作为子工程引入

所有 demo target 都会链接：

```cmake
Microsoft.GSL::GSL
```

所以 demo 中可以直接使用：

```cpp
#include <gsl/gsl>
```

## CLion 使用

打开工程后，等待 CLion 自动 Reload CMake

如果 GSL 相关头文件无法解析，先确认子仓库已经初始化：

```powershell
git submodule update --init --recursive
```

然后在 CLion 中执行：

```text
Reload CMake Project
```
