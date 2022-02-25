use garble::{Garble, SimpleGarbler};

#[derive(Garble, Clone, Debug)]
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
fn test_struct_0pc() {
    let mut garbler = SimpleGarbler::new(0.0);

    let s = MyStruct { a: 128, b: "hello".to_string() };
    let s_garbled = s.clone().garble(&mut garbler);

    assert_eq!(s.a, s_garbled.a);
    assert_eq!(s.b, s_garbled.b);
}

#[test]
fn test_struct_100pc() {
    let mut garbler = SimpleGarbler::new(1.0);

    let s = MyStruct { a: 128, b: "hello".to_string() };
    let s_garbled = s.clone().garble(&mut garbler);

    assert_ne!(s.a, s_garbled.a);
    assert_ne!(s.b, s_garbled.b);
}

#[test]
fn test_enum_v1() {
    let mut garbler = SimpleGarbler::new(0.5);

    let e = MyEnum::V1;
    let e_garbled = e.clone().garble(&mut garbler);

    assert_eq!(e, e_garbled);
}

#[test]
fn test_enum_v2_0pc() {
    let mut garbler = SimpleGarbler::new(0.0);

    let e = MyEnum::V2(128);
    let e_garbled = e.clone().garble(&mut garbler);

    assert_eq!(e, e_garbled);
}

#[test]
fn test_enum_v2_100pc() {
    let mut garbler = SimpleGarbler::new(1.0);

    let e = MyEnum::V2(128);
    let e_garbled = e.clone().garble(&mut garbler);

    assert_ne!(e, e_garbled);
}