#[snippet = "matrix_swap"]
#[allow(dead_code)]
pub fn matrix_swap<T>(v: &mut Vec<Vec<T>>, i1: usize, j1: usize, i2: usize, j2: usize) {
    if i1 == i2 {
        v[i1].swap(j1, j2);
        return;
    }
    let n = v[i1].len();
    let mut e1 = v[i1].swap_remove(j1);
    std::mem::swap(&mut v[i2][j2], &mut e1);
    v[i1].push(e1);
    v[i1].swap(j1, n - 1);
}

#[test]
fn test_matrix_swap() {
    let mut v: Vec<Vec<i64>> = vec![vec![1, 2], vec![3, 4]];
    matrix_swap(&mut v, 0, 0, 1, 1);
    assert_eq!(v[0][0], 4);
    assert_eq!(v[1][1], 1);
    assert_eq!(v[0][1], 2);
    assert_eq!(v[1][0], 3);
}

