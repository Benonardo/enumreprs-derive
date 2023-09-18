use enumreprs::{FromRepr, FromReprError, IntoRepr};
use enumreprs_derive::{FromRepr, IntoRepr};

#[repr(u8)]
#[derive(IntoRepr, FromRepr, PartialEq, Debug)]
pub enum FieldlessTest {
    A = 1,
    B = 2,
    C = 3,
}

#[repr(i64)]
#[derive(IntoRepr)]
pub enum FieldTest {
    X(u64) = i64::MAX,
    Y(String, String) = i64::MIN,
    Z { a: Vec<FieldTest>, b: FieldlessTest } = i64::BITS as i64,
    W = 0,
}

#[test]
fn fieldless_into() {
    assert_eq!(FieldlessTest::A.into_repr(), 1);
    assert_eq!(FieldlessTest::B.into_repr(), 2);
    assert_eq!(FieldlessTest::C.into_repr(), 3);
}

#[test]
fn fieldless_from() {
    assert_eq!(FieldlessTest::from_repr(1).unwrap(), FieldlessTest::A);
    assert_eq!(FieldlessTest::from_repr(2).unwrap(), FieldlessTest::B);
    assert_eq!(FieldlessTest::from_repr(3).unwrap(), FieldlessTest::C);
    assert_eq!(
        FieldlessTest::from_repr(4).unwrap_err(),
        FromReprError::InvalidVariant(4)
    );
}

#[test]
fn field_into() {
    assert_eq!(FieldTest::X(0).into_repr(), i64::MAX);
    assert_eq!(
        FieldTest::Y(String::new(), String::new()).into_repr(),
        i64::MIN
    );
    assert_eq!(
        FieldTest::Z {
            a: Vec::new(),
            b: FieldlessTest::A
        }
        .into_repr(),
        i64::BITS as i64
    );
    assert_eq!(FieldTest::W.into_repr(), 0);
}
