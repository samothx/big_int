use crate::BigInt;

#[test]
fn test_from_i128() {
    let bi = BigInt::from_i128(0x1);
    assert_eq!(bi.to_i128(), Some(0x1));

    let bi = BigInt::from_i128(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    assert_eq!(bi.to_i128(), Some(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF));
}
