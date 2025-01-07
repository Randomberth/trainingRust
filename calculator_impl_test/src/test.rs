#![cfg(test)]

#[test]
fn test_suma() {
    use super::*;
    let x: Numbers = operations::Numbers::new(3.0, 3.0);
    assert_eq!(x.suma(), 6.0);
}
#[test]
fn test_resta() {
    use super::*;
    let x: Numbers = operations::Numbers::new(9.0, 3.0);
    assert_eq!(x.resta(), 6.0);
}
#[test]
fn test_multiplo() {
    use super::*;
    let x: Numbers = operations::Numbers::new(3.0, 3.0);
    assert_eq!(x.multiplo(), 9.0);
}
#[test]
fn test_division() {
    use super::*;
    let x: Numbers = operations::Numbers::new(9.0, 3.0);
    assert_eq!(x.division(), 3.0);
}
#[test]
fn test_modulo() {
    use super::*;
    let x: Numbers = operations::Numbers::new(9.0, 3.0);
    assert_eq!(x.modulo(), 0.0);
}
