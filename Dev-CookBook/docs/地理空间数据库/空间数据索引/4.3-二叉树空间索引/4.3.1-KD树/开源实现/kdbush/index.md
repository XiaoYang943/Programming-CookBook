**KDBush**

[mourner/kdbush v4.1.0](https://github.com/mourner/kdbush)

> 一种基于扁平 KD 树的二维点静态空间索引算法。

## Feature

```js
import KDBush from 'kdbush';

const points = [
    {name: 'A', x: 2, y: 3},
    {name: 'B', x: 8, y: 5},
    {name: 'C', x: 4, y: 7}
];

// 1. 初始化空索引
const index = new KDBush(points.length);

// 2. 录入数据
for (const point of points) {
    index.add(point.x, point.y);
}

// 3. 构建索引
index.finish();
```

### 矩形 BBox范围查询
```js
// 查询矩形 [0, 0]—[10, 10] 内的点
const rangeIds = index.range(0, 0, 10, 10);
const rangePoints = rangeIds.map(id => points[id]);
```
### 圆形半径查询
```js
// 查询圆心 (10, 10)、半径 10 内的点
const neighborIds = index.within(10, 10, 10);
const neighborPoints = neighborIds.map(id => points[id]);
```

## 源码分析

### 初始化空索引

```js
new KDBush(numItems, nodeSize, ArrayType, ArrayBufferType)
```

- `numItems`：必填，准备录入的准确点数。实际调用 `add()` 的次数必须与它相等。
- `nodeSize`：可选，默认值为 64，控制递归划分何时停止并改用线性扫描。
- `ArrayType`：可选，坐标的类型化数组类型，默认为 `Float64Array`。
- `ArrayBufferType`：可选，底层缓冲区类型，默认为 `ArrayBuffer`。


### 录入数据

```js
class KDBush {
    add(x, y) {
        // 一个点占用两个连续坐标位置，除以 2 得到该点的原始 ID
        const index = this._pos >> 1;

        // ID 与点的录入顺序一致：0、1、2……
        this.ids[index] = index;

        // 将 x、y 连续写入坐标数组，并把写入位置向后移动
        this.coords[this._pos++] = x;
        this.coords[this._pos++] = y;

        // 返回原始 ID，查询结果将使用同一种 ID
        return index;
    }
}
```

`add()` 的输入是一个点的 `x`、`y` 坐标，输出是该点从 0 开始递增的原始 ID。它不会寻找中位点，也不会调整已有数据，因此执行完全部 `add()` 后，点仍然保持录入顺序，索引还不能查询。

### 构建索引

- 将顺序点数组”转变为“扁平 KD 树索引”

```js
class KDBush {
    // ... 

    finish() {
        // ...

        // 对 ids 和 coords 进行原地 KD 排列
        // 处理完整区间 [0, numItems - 1]，根层从 x 轴（axis = 0）开始
        sort(
            this.ids,
            this.coords,
            this.nodeSize,
            0,
            this.numItems - 1,
            0
        );

        // 标记构建完成
        this._finished = true;
        
        return this;
    }

    // ... 
}
```

#### `sort()`：递归构建扁平 KD 树

```js
function sort(ids, coords, nodeSize, left, right, axis) {
    // 当前区间已经足够小：停止划分，查询时直接线性扫描
    if (right - left <= nodeSize) return;

    // 当前区间的中间下标将用于存放分割点
    const m = (left + right) >> 1;

    // 按当前轴原地重排区间：
    // 中位点放到 m，较小点放在左侧，较大点放在右侧
    select(ids, coords, m, left, right, axis);

    // 递归处理左右两个连续区间，并切换 x、y 分割轴
    sort(ids, coords, nodeSize, left, m - 1, 1 - axis);
    sort(ids, coords, nodeSize, m + 1, right, 1 - axis);
}
```

`sort()` 的目标是：**把 `[left, right]` 这段无空间结构的点数组，原地转换成一个由数组下标隐式表示的 KD 子树。**

它接收六个参数：

- `ids`：点的原始 ID 数组，必须与坐标同步重排。
- `coords`：按 `[x₀, y₀, x₁, y₁, ...]` 布局的坐标数组。
- `nodeSize`：停止继续划分的阈值。
- `left`、`right`：当前负责处理的闭区间。
- `axis`：当前分割轴；`0` 表示 $x$ 轴，`1` 表示 $y$ 轴。

第一次调用来自 `finish()`：

```js
sort(ids, coords, nodeSize, 0, numItems - 1, 0);
```

这表示从包含全部点的区间开始，并首先按 $x$ 轴划分。一次递归处理包含以下步骤：

1. **判断是否继续划分。** 当区间足够小时直接返回，把它保留为查询阶段线性扫描的叶子块。
2. **确定分割点位置。** 取中间下标 $m=\lfloor(left+right)/2\rfloor$。
3. **按当前轴划分。** `select()` 把当前轴的中位点移动到 `m`，并把较小点、较大点分别放到两侧。
4. **递归构建左右子树。** `[left, m-1]` 是左子树，`[m+1, right]` 是右子树；下一层通过 `1 - axis` 切换分割轴。

`sort()` 返回时不会产生 `Node` 对象。它的输出是已经被原地重排的 `ids` 和 `coords`：当前区间的根位于 `m`，左右子树分别占据 `m` 两侧的连续区间。整棵树因此被编码在数组位置中。

需要特别注意，`sort()` 负责控制递归结构，但每一层真正执行“选出中位点并划分左右两侧”的是 `select()`。二者的关系是：

$$
\texttt{finish()}\longrightarrow
\texttt{sort()}\longrightarrow
\texttt{select()}
$$

- `finish()`：校验输入并启动构建。
- `sort()`：确定递归区间、分割位置和交替轴。
- `select()`：在当前区间内完成一次原地中位点划分。

#### `select()`：原地完成中位点划分

`sort()` 已经确定当前分割点应当放在中间下标 `m`，随后调用：

```js
select(ids, coords, m, left, right, axis);
```

`select()` 的目标不是把整个区间排好序，而是找到当前轴上第 `k` 小的点，把它放到下标 `k`，同时保证较小点在左侧、较大点在右侧。这里 `k` 就是 `sort()` 传入的中间下标 `m`。

下面保留算法的核心分区代码。原源码对超过 600 个元素的区间使用 Floyd–Rivest 采样来缩小搜索范围，该优化不改变分区结果，此处用 `...` 省略其计算细节。

```js
function select(ids, coords, k, left, right, axis) {
    // 只要目标下标 k 所在的候选区间还未缩小为一个位置，就继续分区
    while (right > left) {
        if (right - left > 600) {
            // ... 使用 Floyd–Rivest 采样估计更小的候选区间
            select(ids, coords, k, newLeft, newRight, axis);
        }

        // 取下标 k 当前点在分割轴上的坐标作为枢轴值
        const t = coords[2 * k + axis];
        let i = left;
        let j = right;

        // 把枢轴点移到区间边界，便于执行双指针分区
        swapItem(ids, coords, left, k);
        if (coords[2 * right + axis] > t) {
            swapItem(ids, coords, left, right);
        }

        // i 从左向右、j 从右向左移动
        // 把小于 t 的点放到左侧，把大于 t 的点放到右侧
        while (i < j) {
            swapItem(ids, coords, i, j);
            i++;
            j--;

            while (coords[2 * i + axis] < t) i++;
            while (coords[2 * j + axis] > t) j--;
        }

        // 把枢轴点放回本轮分区得到的最终位置 j
        if (coords[2 * left + axis] === t) {
            swapItem(ids, coords, left, j);
        } else {
            j++;
            swapItem(ids, coords, j, right);
        }

        // 只保留仍然包含目标下标 k 的一侧，继续下一轮分区
        if (j <= k) left = j + 1;
        if (k <= j) right = j - 1;
    }
}
```

`select()` 的输入包括：

- `ids`、`coords`：需要同步原地重排的点数据。
- `k`：中位点最终必须占据的目标下标。
- `left`、`right`：本次选择操作的闭区间。
- `axis`：比较坐标的轴，`0` 为 $x$，`1` 为 $y$。

算法处理过程可以概括为：

1. **选择枢轴。** 读取下标 `k` 当前点在指定轴上的坐标，记为 `t`。
2. **执行双指针分区。** 指针 `i` 从左向右寻找不小于 `t` 的点，`j` 从右向左寻找不大于 `t` 的点，并通过交换把它们放到正确一侧。
3. **确定枢轴位置。** 一轮分区结束后，把枢轴点移动到本轮确定的位置 `j`。
4. **缩小候选区间。** 如果 `j` 不等于目标下标 `k`，下一轮只处理仍然包含 `k` 的一侧，另一侧不再检查。
5. **到达目标位置。** 候选区间收缩完成后，第 `k` 小的轴坐标位于下标 `k`。

分区结束时，数组满足：

$$
\begin{aligned}
coords[2i+axis]&\leq coords[2k+axis], &&left\leq i<k\\
coords[2j+axis]&\geq coords[2k+axis], &&k<j\leq right
\end{aligned}
$$

这个结果已经足够让 `sort()` 把 `[left, k-1]` 和 `[k+1, right]` 当作左右子树继续递归。两侧内部是否完全有序并不重要，因此 `select()` 避免了完整排序的额外工作。

#### `swapItem()`：保证 ID 与坐标不分离

分区过程中不能只交换当前轴的坐标。一个点的 ID、$x$ 和 $y$ 必须作为整体一起移动：

```js
function swapItem(ids, coords, i, j) {
    // 交换原始 ID
    swap(ids, i, j);

    // 交换同一个点的 x、y 坐标
    swap(coords, 2 * i, 2 * j);
    swap(coords, 2 * i + 1, 2 * j + 1);
}
```

`select()` 的输出不是一个新的中位点对象，也没有显式返回值。它直接修改 `ids` 和 `coords`，最终使下标 `k` 成为当前 KD 子树的分割点。`sort()` 随后以 `k` 为边界递归处理左右区间。

### 矩形范围查询

```js
const ids = index.range(minX, minY, maxX, maxY);
```

`range()` 返回闭合矩形内所有点的 ID，边界上的点也会命中。参数分别表示矩形的最小 $x$、最小 $y$、最大 $x$ 和最大 $y$。返回顺序由索引排列及遍历顺序决定，不保证与录入顺序一致。

### 圆形半径查询

```js
const ids = index.within(x, y, radius);
```

`within()` 返回以 $(x,y)$ 为圆心、`radius` 为半径的闭合圆内所有点的 ID，圆周上的点也会命中。它只按距离筛选结果，不会按距离从近到远排序。

需要反复查询并控制结果容器分配时，可以使用：

```js
const out = new Uint32Array(points.length);
const count = index.withinInto(x, y, radius, out);
const ids = out.subarray(0, count);
```

`withinInto()` 不创建结果数组，而是把命中的 ID 写入调用者提供的 `out`，并返回实际写入数量。`out` 既可以是预先分配的类型化数组，也可以是普通数组。

从 API 可以先得到一个整体认识：`add()` 负责准备输入，`finish()` 负责构建扁平 KD 树，`range()` 和 `within()` 负责利用索引剪枝查询。下面的数据结构和核心算法正是围绕这四类操作展开的。

## 数据结构
kdbush 使用扁平 KD 树：树的结点和子树不表示为相互引用的对象，而是隐含在经过 KD 排列的一维数组区间中。每个区间的中间位置是当前分割点，中间位置左侧和右侧的连续区间分别表示两棵子树。这样的布局减少了对象和指针开销，也使查询可以直接在数组下标区间上进行。

## 构建索引

构建索引的核心不是创建一批树结点，而是把输入点**原地重排成具有 KD 树性质的数组**

### 1. 初始化：确定输入规模
- 索引的本质是数组，构建索引的本质是构建数组，那么构建索引之前需要知道数据体量

- 为输入点一次性准备容量准确的连续存储空间，并确定后续建树使用的叶子块阈值

KDBush 是静态索引，构建完成后不会通过扩容来接收新点。因此，它要求调用者在录入点之前提供准确的 `numItems`。有了点数 $n$，构造器才能确定 ID 数组需要 $n$ 个位置、坐标数组需要 $2n$ 个位置，并把它们一次性分配在底层连续缓冲区中。这样做避免了录入过程中的动态扩容和内存搬移，也为后面的原地重排提供了固定范围 `[0, n - 1]`。

创建 KDBush 时必须预先给出点的总数：

```js
constructor(numItems, nodeSize = 64, ArrayType = Float64Array,
    ArrayBufferType = ArrayBuffer, data) {
    this.numItems = +numItems;
    this.nodeSize = Math.min(Math.max(+nodeSize, 2), 65535);

    // 省略 IndexArrayType、idsByteSize、padCoords 和 data 的初始化代码
    this.ids = new this.IndexArrayType(data, HEADER_SIZE, numItems);
    this.coords = new ArrayType(
        data,
        HEADER_SIZE + idsByteSize + padCoords,
        numItems * 2
    );
    this._pos = 0;
    this._finished = false;
}
```

对于 $n$ 个二维点，`ids` 有 $n$ 个位置，`coords` 有 $2n$ 个位置。第 $i$ 个点在两个数组中的对应关系为：

$$
ids[i]\longleftrightarrow(coords[2i],coords[2i+1])
$$

其中 `ids[i]` 用于找回原始点，`coords[2 * i]` 和 `coords[2 * i + 1]` 分别是该点的 $x$、$y$ 坐标。构建过程会改变 $i$，但绝不能破坏这组对应关系。

### 2. `add()`：把输入点写成顺序数组

**本步目标：收集所有待索引的二维点，并建立“原始 ID—坐标”对应关系，为下一步原地重排准备完整输入。** 此时只录入数据，不进行空间划分。

```js
add(x, y) {
    const index = this._pos >> 1;
    this.ids[index] = index;
    this.coords[this._pos++] = x;
    this.coords[this._pos++] = y;
    return index;
}
```

`add()` 的输入是一个二维点 `(x, y)`，输出是该点的原始 ID `index`。`_pos` 指向下一个待写入的坐标位置；因为每个点占两个坐标位置，所以 `_pos >> 1` 等价于整除 2，可得到当前点编号。

假设依次输入三个点 $(8,2)$、$(1,7)$、$(5,4)$，录入完成但尚未构建时，数组为：

```text
ids:    [0, 1, 2]
coords: [8, 2, 1, 7, 5, 4]
          └─0─┘ └─1─┘ └─2─┘
```

此时数组只记录了加入顺序，还没有 KD 树的空间划分性质，因此不能执行查询。

### 3. `finish()`：从顺序数组启动 KD 构建

**本步目标：确认输入已经完整，然后以全部点为范围启动 KD 排列；构建成功后把索引切换为可查询状态。** 它是数据录入阶段与构建索引阶段的边界。

```js
finish() {
    const numAdded = this._pos >> 1;
    if (numAdded !== this.numItems) {
        throw new Error(`Added ${numAdded} items when expected ${this.numItems}.`);
    }

    sort(this.ids, this.coords, this.nodeSize,
        0, this.numItems - 1, 0);

    this._finished = true;
    return this;
}
```

`finish()` 没有接收新的点。它以已经写满的 `ids`、`coords` 为输入，先确认实际点数与声明点数一致，然后让 `sort()` 处理闭区间 `[0, numItems - 1]`。最后一个参数 `0` 表示根层首先按 $x$ 轴划分。

这里的 `sort()` 不是普通的全排序。它的目标是逐层确定中位点，让数组满足 KD 树的分区不变量。

### 4. `sort()`：递归确定树的每个分割点

**本步目标：把一个无空间结构的点数组递归划分成若干连续区间，使每个区间的中间点成为分割点，左右区间分别隐式表示左右子树。** 这一步决定整棵扁平 KD 树的结构。

```js
function sort(ids, coords, nodeSize, left, right, axis) {
    if (right - left <= nodeSize) return;

    const m = (left + right) >> 1;

    select(ids, coords, m, left, right, axis);

    sort(ids, coords, nodeSize, left, m - 1, 1 - axis);
    sort(ids, coords, nodeSize, m + 1, right, 1 - axis);
}
```

一次 `sort()` 调用的输入含义如下：

- `ids`、`coords`：正在被原地重排的全部点。
- `[left, right]`：本次调用负责的连续数组区间。
- `axis`：当前分割轴；`0` 表示 $x$，`1` 表示 $y$。
- `nodeSize`：停止递归划分的阈值。

处理过程分为四步：

1. 如果 `right - left <= nodeSize`，该区间不再细分，查询时直接线性扫描。按代码的闭区间计数方式，这个条件对应区间内最多有 `nodeSize + 1` 个点。
2. 计算中间下标 $m=\lfloor(left+right)/2\rfloor$。下标 $m$ 将成为当前区间的分割点。
3. `select()` 按当前轴重排 `[left, right]`，把该轴上的中位点放到 $m$。
4. 左右区间分别递归，并用 `1 - axis` 切换分割轴。

执行 `select()` 后只保证：

$$
\begin{aligned}
coords[2i+axis]&\leq coords[2m+axis], &&left\leq i<m\\
coords[2j+axis]&\geq coords[2m+axis], &&m<j\leq right
\end{aligned}
$$

左右两侧内部不要求有序。比如根层 `axis = 0`，只需让 $x$ 坐标较小的点位于 $m$ 左侧、较大的点位于右侧；下一层再分别按 $y$ 坐标处理两侧区间。这正是“只选择中位点而不完整排序”能够工作的原因。

### 5. `select()`：原地选出当前轴的中位点

**本步目标：在不完整排序整个区间的前提下，把当前分割轴上的中位点放到指定下标 `k`，并把较小点与较大点分置两侧。** 它为 `sort()` 提供一次空间划分所需的核心操作。

`select(ids, coords, k, left, right, axis)` 的输入是目标下标 `k`、待处理区间和比较轴。下面是其核心分区代码；源码中针对超过 600 个元素的区间还有 Floyd–Rivest 采样缩小范围的优化，此处省略该优化分支：

```js
function select(ids, coords, k, left, right, axis) {
    while (right > left) {
        const t = coords[2 * k + axis];
        let i = left;
        let j = right;

        swapItem(ids, coords, left, k);
        if (coords[2 * right + axis] > t) {
            swapItem(ids, coords, left, right);
        }

        while (i < j) {
            swapItem(ids, coords, i, j);
            i++;
            j--;
            while (coords[2 * i + axis] < t) i++;
            while (coords[2 * j + axis] > t) j--;
        }

        if (coords[2 * left + axis] === t) {
            swapItem(ids, coords, left, j);
        } else {
            j++;
            swapItem(ids, coords, j, right);
        }

        if (j <= k) left = j + 1;
        if (k <= j) right = j - 1;
    }
}
```

`t` 是目标位置 `k` 当前点在分割轴上的值。双指针 `i`、`j` 从区间两端向中间移动，把小于 `t` 的点交换到左侧，把大于 `t` 的点交换到右侧。一次分区完成后，如果枢轴最终位置 `j` 还不是 `k`，就把下一轮处理范围缩小到仍包含 `k` 的一侧。循环结束时，第 `k` 小的轴坐标位于 `k`，但整个区间没有被完整排序。

点的三个组成部分必须一起移动，因此交换操作是：

```js
function swapItem(ids, coords, i, j) {
    swap(ids, i, j);
    swap(coords, 2 * i, 2 * j);
    swap(coords, 2 * i + 1, 2 * j + 1);
}
```

如果只交换坐标而不交换 ID，查询返回的 ID 就会指向错误的原始点。`swapItem()` 同步交换 `ids[i]`、$x_i$ 和 $y_i$，保证一个点在重排过程中始终是不可拆分的整体。

### 6. 构建输出：数组就是索引

**本步目标：明确构建最终产物以及后续查询如何使用它。** 构建不会产生一组带指针的树结点；最终产物是同步重排后的 `ids`、`coords` 以及由数组下标隐式表达的树结构。

`finish()` 的输出仍是当前 KDBush 实例，但实例内部的 `ids` 和 `coords` 已由输入顺序变成 KD 排列，`_finished` 也被置为 `true`。以任意递归区间 `[left, right]` 为例，其结构都可以从下标直接推出：

```text
区间根（分割点）: m = (left + right) >> 1
左子树:           [left, m - 1]
右子树:           [m + 1, right]
下一层分割轴:     1 - axis
```

因此，构建过程没有输出传统的 `Node {point, left, right}` 对象，也没有保存子结点指针。它输出的是两个同步重排后的连续数组；中间下标、左右区间和交替轴共同隐式编码了整棵 KD 树。后续 `range()` 和 `within()` 正是根据这套下标规则恢复树的遍历关系并进行剪枝。

## 矩形范围查询

`range(minX, minY, maxX, maxY)` 查找闭合矩形范围内的点，即同时满足：

$$
minX\leq x\leq maxX,\qquad minY\leq y\leq maxY
$$

查询遍历的不是传统的 `Node.left`、`Node.right` 对象，而是扁平数组中的连续下标区间。对于当前区间 `[left, right]`：

```text
当前分割点：m = (left + right) >> 1
左子树：    [left, m - 1]
右子树：    [m + 1, right]
```

`range()` 不使用递归调用，而用显式栈保存待搜索的 `(left, right, axis)` 三元组。查询过程如下：

1. 将完整索引区间 `[0, numItems - 1]` 和初始分割轴 $x$ 压入栈。
2. 每次从栈顶取出一个区间及其分割轴。
3. 如果 $right-left\leq nodeSize$，说明已经到达叶子块，逐点检查区间内的坐标，将落入查询矩形的点 ID 加入结果。默认 `nodeSize` 为 64，按闭区间计数时一个叶子块最多包含 65 个点。
4. 如果当前区间不是叶子块，计算中间下标 $m=\lfloor(left+right)/2\rfloor$，读取下标 `m` 对应的分割点 $(x,y)$。
5. 检查该分割点是否同时满足 $x$ 和 $y$ 方向的闭区间条件；如果满足，将 `ids[m]` 加入结果。
6. 根据当前分割轴判断左右子树是否可能包含结果，只把可能相交的子区间压入栈，并将下一层分割轴切换为另一个轴。
7. 重复上述过程，直到栈为空，返回累计得到的原始点 ID。

设当前分割点为 $(x,y)$，子树搜索条件为：

- 当前按 $x$ 轴分割时，`minX <= x` 才搜索左子树，`maxX >= x` 才搜索右子树。
- 当前按 $y$ 轴分割时，`minY <= y` 才搜索左子树，`maxY >= y` 才搜索右子树。

如果条件不成立，对应分割线一侧不可能出现命中点，因此整个连续数组区间都可以被剪枝。显式栈实现的是深度优先搜索，只是把递归状态改为由数组保存。返回结果的顺序取决于 KD 排列和栈的遍历顺序，不表示点的录入顺序，也不表示距离顺序。

## 半径查询

`within(qx, qy, r)` 查找以 $(qx,qy)$ 为圆心、$r$ 为半径的闭合圆内的点。点 $(x,y)$ 是否命中通过平方距离判断：

$$
(x-qx)^2+(y-qy)^2\leq r^2
$$

实现比较平方距离而不计算平方根。它与矩形范围查询使用相同的显式栈和叶子块线性扫描策略，但剪枝条件改为判断查询圆在当前分割轴上的投影是否越过分割线：

- 在 $x$ 轴层，`qx - r <= x` 时搜索左侧，`qx + r >= x` 时搜索右侧。
- 在 $y$ 轴层，`qy - r <= y` 时搜索左侧，`qy + r >= y` 时搜索右侧。

`within()` 本身先创建普通数组，再调用 `withinInto()` 完成实际查询。`withinInto(qx, qy, r, out)` 把命中的 ID 依次写入调用者提供的 `out` 容器，并返回写入数量。`out` 可以是普通数组，也可以是预先分配的类型化数组；后者可在反复查询时复用输出空间，减少结果数组分配。两种接口使用相同的搜索和剪枝逻辑。

源码参见 [kdbush v4.1.0：index.js](https://github.com/mourner/kdbush/blob/v4.1.0/index.js)。

## 特点总结

- 二维：只支持二维坐标，分割轴在 $x$ 轴和 $y$ 轴之间交替
- 静态：当构建索引完成后，索引才能用于查询，且索引结果不支持动态插入或删除
- 查询方式：矩形范围查询、圆形半径查询
- 查询结果：点的ID
- `nodeSize` 默认为 64。它不是传统 KD 树单个结点的大小，而是停止继续划分后进行线性扫描的最大数组区间阈值
