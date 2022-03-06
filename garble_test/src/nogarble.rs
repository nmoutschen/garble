use crate::utils::ZeroGarbler;
use garble::Garble;

#[test]
fn test_named() {
    #[derive(Clone, Debug, Garble, PartialEq)]
    struct Named {
        a: u32,
        #[nogarble]
        b: u32,
    }

    let input = Named { a: 1, b: 2 };
    let expected = Named { a: 0, b: 2 };

    let output = input.garble(&mut ZeroGarbler);
    assert_eq!(output, expected);
}

#[test]
fn test_inline() {
    #[derive(Clone, Debug, Garble, PartialEq)]
    struct Inline(u32, #[nogarble] u32);

    let input = Inline(1, 2);
    let expected = Inline(0, 2);

    let output = input.garble(&mut ZeroGarbler);
    assert_eq!(output, expected);
}
