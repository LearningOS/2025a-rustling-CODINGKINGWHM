fn sort<T: PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let pivot_idx = partition(array);
    let (left, right) = array.split_at_mut(pivot_idx);
    sort(left);
    sort(&mut right[1..]); // 排除基准值
}

// 快速排序分区函数
fn partition<T: PartialOrd>(array: &mut [T]) -> usize {
    let pivot_idx = array.len() - 1;
    let mut i = 0;
    for j in 0..pivot_idx {
        if array[j] <= array[pivot_idx] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, pivot_idx);
    i
}