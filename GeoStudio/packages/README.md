# Source Packages

该目录用于保存 GeoStudio 后续需要阅读、调试和测试的开源源码包。

GeoStudio 的业务代码仍然通过 npm 依赖调用正式发布版本；这里的源码副本用于源码分析、中文注释、断点调试和差分测试，不直接替代生产依赖。

| 包 | 上游版本 | 用途 |
| --- | --- | --- |
| [`kdbush`](./kdbush/) | `v4.1.0` | 静态二维 KD 树索引的构建、矩形查询和半径查询源码分析 |

## 上游源码管理

`kdbush` 通过 Git subtree 引入，上游仓库为：

```text
https://github.com/mourner/kdbush.git
```

Git remote 只保存在本地仓库配置中。首次克隆本仓库后，需要配置一次：

```powershell
git remote add kdbush-upstream https://github.com/mourner/kdbush.git
```

拉取上游信息并查看更新：

```powershell
git fetch --tags kdbush-upstream
git log v4.1.0..kdbush-upstream/main --oneline
```

确认目标版本并保持工作区干净后，优先按稳定标签更新：

```powershell
git subtree pull `
  --prefix=GeoStudio/packages/kdbush `
  kdbush-upstream vX.Y.Z `
  --squash
```

如需直接跟进上游主分支，可将 `vX.Y.Z` 替换为 `main`。本地源码注释与上游修改同一代码区域时，Git 会要求解决合并冲突。
