fn double(values: &mut [i32]) {
    // &mut [T] 只要求获得一段连续元素的独占访问权
    for value in values {
        *value *= 2;    // 只修改已有元素，不需要知道元素来自数组还是 Vec
    }
}

fn append(values: &mut Vec<i32>, value: i32) {
    // 切片只有固定的元素范围，没有 push、pop 或 reserve 等方法
    // 需要 Vec 提供的容器管理能力，因此参数必须是 &mut Vec<i32>
    values.push(value);
}

fn main() {
    let mut array = [1, 2, 3];
    double(&mut array);
    assert_eq!(array, [2, 4, 6]);

    let mut vector = vec![1, 2, 3, 4];
    double(&mut vector); // &mut Vec<i32> 自动转换为 &mut [i32]
    assert_eq!(vector, [2, 4, 6, 8]);

    double(&mut vector[1..3]); // 也可以只修改 Vec 中的一段连续元素
    assert_eq!(vector, [2, 8, 12, 8]);

    append(&mut vector, 10);
    assert_eq!(vector, [2, 8, 12, 8, 10]);
}
