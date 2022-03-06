use crate::utils::ZeroGarbler;
use garble::Garble;

#[derive(Garble, Clone, Debug, PartialEq)]
struct MyStruct {
    a: u32,
    b: String,
}

#[derive(Garble, Clone, Debug, PartialEq)]
enum MyEnum {
    V1,
    V2(u32),
}

#[test]
fn test_struct() {
    let input = MyStruct {
        a: 1,
        b: "hello".to_string(),
    };
    let expected = MyStruct {
        a: 0,
        b: String::new(),
    };

    let output = input.garble(&mut ZeroGarbler);
    assert_eq!(output, expected);
}

#[test]
fn test_enum_v1() {
    let input = MyEnum::V1;
    let expected = MyEnum::V1;

    let output = input.garble(&mut ZeroGarbler);
    assert_eq!(output, expected);
}

#[test]
fn test_enum_v2() {
    let input = MyEnum::V2(128);
    let expected = MyEnum::V2(0);

    let output = input.garble(&mut ZeroGarbler);
    assert_eq!(output, expected);
}
