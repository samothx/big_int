use crate::Rational;

#[test]
fn test_from() {
    let rat = Rational::from((1,3));
    assert_eq!(format!("{}",rat).as_str(),"1/3");
    assert_eq!(format!("{:?}",rat).as_str(),"((L:1,0x1)/(L:2,0x3))");
}
