pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut resultado = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                resultado.push((lista[i], lista[j]));
            }
        }
    }
    resultado
}

#[test]
fn test_pares_com_soma() {
    let res = pares_com_soma(&[1, 2, 3, 4], 5);
    assert!(res.contains(&(1, 4)) || res.contains(&(2, 3)));
}
