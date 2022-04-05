use crate::BigInt;

#[test]
fn test_add() {
    let bi1: BigInt = 10.into();
    let bi2: BigInt = 20.into();
    assert_eq!(bi1 + bi2, 30.into());

    let bi1: BigInt = (-10).into();
    let bi2: BigInt = (-20).into();
    assert_eq!(bi1 + bi2, (-30).into());

    let bi1: BigInt = 10.into();
    let bi2: BigInt = (-20).into();
    assert_eq!(bi1 + bi2, (-10).into());

    let bi1: BigInt = 10.into();
    let bi2: BigInt = (-10).into();
    assert_eq!(bi1 + bi2, 0.into());

    let bi1: BigInt = 0x7FFFFFFFFFFFFFFFi64.into();
    let bi2: BigInt = 0x7FFFFFFFFFFFFFFFi64.into();
    let bi3 = bi1 + bi2;
    assert_eq!(bi3, 0xFFFFFFFFFFFFFFFEi128.into());

    let bi4 = bi3.add_to( &bi3);
    assert_eq!(bi4, 0x1FFFFFFFFFFFFFFFCi128.into());
}

#[test]
fn test_add_to_self() {
    let mut bi1: BigInt = 10.into();
    let bi2: BigInt = 20.into();
    bi1 += bi2;
    assert_eq!(bi1, 30.into());

    let mut bi1: BigInt = (-10).into();
    let bi2: BigInt = (-20).into();
    bi1 += bi2;
    assert_eq!(bi1, (-30).into());

    let mut bi1: BigInt = 10.into();
    let bi2: BigInt = (-20).into();
    bi1 += bi2;
    assert_eq!(bi1, (-10).into());

    let mut bi1: BigInt = 10.into();
    let bi2: BigInt = (-10).into();
    bi1 += bi2;
    assert_eq!(bi1, 0.into());

    let mut bi1: BigInt = 0x7FFFFFFFFFFFFFFFi64.into();
    let bi2: BigInt = 0x7FFFFFFFFFFFFFFFi64.into();
    bi1 += bi2;
    assert_eq!(bi1, 0xFFFFFFFFFFFFFFFEi128.into());

    bi1 += bi1.clone();
    assert_eq!(bi1, 0x1FFFFFFFFFFFFFFFCi128.into());
    eprintln!("{:?}", bi1);
}

#[test]
fn test_subtract() {
    let bi1: BigInt = 20.into();
    let bi2: BigInt = 10.into();
    assert_eq!(bi1 - bi2, 10.into());

    let bi1: BigInt = (-20).into();
    let bi2: BigInt = (-10).into();
    assert_eq!(bi1 - bi2, (-10).into());

    let bi1: BigInt = (-10).into();
    let bi2: BigInt = (-20).into();
    assert_eq!(bi1 - bi2, 10.into());

    let bi1: BigInt = 20.into();
    let bi2: BigInt = (-10).into();
    assert_eq!(bi1 - bi2, 30.into());

    let mut bi1: BigInt = 0x1FFFFFFFFFFFFFFFCi128.into();
    let bi2: BigInt = 0xFFFFFFFFFFFFFFFEi128.into();

    bi1 -= bi2;
    assert_eq!(bi1, 0xFFFFFFFFFFFFFFFEi128.into());

    let bi2: BigInt = 0x7FFFFFFFFFFFFFFFi64.into();
    bi1 -= bi2;
    assert_eq!(bi1, 0x7FFFFFFFFFFFFFFFi64.into());
}

#[test]
fn test_subtract_from_self() {
    let mut bi1: BigInt = 20.into();
    let bi2: BigInt = 10.into();
    bi1 -= bi2;
    assert_eq!(bi1, 10.into());

    let mut bi1: BigInt = (-20).into();
    let bi2: BigInt = (-10).into();
    bi1 -= bi2;
    assert_eq!(bi1, (-10).into());

    let mut bi1: BigInt = (-10).into();
    let bi2: BigInt = (-20).into();
    bi1 -= bi2;
    assert_eq!(bi1, 10.into());

    let mut bi1: BigInt = 20.into();
    let bi2: BigInt = (-10).into();
    bi1 -= bi2;
    assert_eq!(bi1, 30.into());

}


#[test]
fn test_from_i128() {
    let bi = BigInt::from_i128(0x1);
    assert_eq!(bi.to_i128(), Some(0x1));

    let bi = BigInt::from_i128(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    assert_eq!(bi.to_i128(), Some(0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF));
}
