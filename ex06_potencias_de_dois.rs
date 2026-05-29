pub fn potencias_de_dois(n: u64) {
    let mut i: u64 = 1;
    while i < n {
        println!("{}", i);
        i *= 2;
    }
}

#[test]
fn test_potencias_de_dois() {
    potencias_de_dois(10);
}
