pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut dados = [5, 3, 8, 1, 2];
    ordenacao_bolha(&mut dados);
    assert_eq!(dados, [1, 2, 3, 5, 8]);
}
