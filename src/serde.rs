use crate::{ArcSwapAny, RefCnt, Strategy};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T, S> Serialize for ArcSwapAny<T, S>
where
    T: RefCnt + Serialize,
    S: Strategy<T>,
{
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
        self.load().serialize(serializer)
    }
}

impl<'de, T, S> Deserialize<'de> for ArcSwapAny<T, S>
where
    T: RefCnt + Deserialize<'de>,
    S: Strategy<T> + Default,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(T::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArcSwap, ArcSwapAny, ArcSwapOption, RefCnt};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_derive::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use std::sync::Arc;

    #[derive(Debug)]
    struct ArcSwapAnyEq<T: RefCnt>(ArcSwapAny<T>);
    impl<T: RefCnt + PartialEq> PartialEq for ArcSwapAnyEq<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.load().eq(&other.0.load())
        }
    }
    impl<T: RefCnt + PartialEq> Eq for ArcSwapAnyEq<T> {}
    impl<T: RefCnt + Serialize> Serialize for ArcSwapAnyEq<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.0.load().serialize(serializer)
        }
    }
    impl<'de, T: RefCnt + Deserialize<'de>> Deserialize<'de> for ArcSwapAnyEq<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let inner = T::deserialize(deserializer)?;
            Ok(Self(ArcSwapAny::new(inner)))
        }
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Foo {
        field0: usize,
        field1: String,
    }

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Bar {
        field0: ArcSwapAnyEq<Arc<usize>>,
        field1: ArcSwapAnyEq<Option<Arc<String>>>,
    }

    #[test]
    fn test_serialize_deserialize() {
        let field0 = usize::MAX;
        let field1 = "FOO_-0123456789";

        let data_orig = Foo {
            field0,
            field1: field1.to_string(),
        };
        let data = ArcSwapAnyEq(ArcSwap::from_pointee(data_orig));
        assert_tokens(
            &data,
            &[
                Token::Struct {
                    name: "Foo",
                    len: 2,
                },
                Token::Str("field0"),
                #[cfg(target_pointer_width = "16")]
                Token::U16(u16::MAX),
                #[cfg(target_pointer_width = "32")]
                Token::U32(u32::MAX),
                #[cfg(target_pointer_width = "64")]
                Token::U64(u64::MAX),
                Token::Str("field1"),
                Token::String(field1),
                Token::StructEnd,
            ],
        );

        let data = Bar {
            field0: ArcSwapAnyEq(ArcSwap::from_pointee(field0)),
            field1: ArcSwapAnyEq(ArcSwapOption::from_pointee(field1.to_string())),
        };
        assert_tokens(
            &data,
            &[
                Token::Struct {
                    name: "Bar",
                    len: 2,
                },
                Token::Str("field0"),
                #[cfg(target_pointer_width = "16")]
                Token::U16(u16::MAX),
                #[cfg(target_pointer_width = "32")]
                Token::U32(u32::MAX),
                #[cfg(target_pointer_width = "64")]
                Token::U64(u64::MAX),
                Token::Str("field1"),
                Token::Some,
                Token::String(field1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_deserialize_option() {
        let field0 = usize::MAX;
        let field1 = "FOO_-0123456789";

        let data_orig = Foo {
            field0,
            field1: field1.to_string(),
        };
        let data = ArcSwapAnyEq(ArcSwapOption::from_pointee(data_orig));
        assert_tokens(
            &data,
            &[
                Token::Some,
                Token::Struct {
                    name: "Foo",
                    len: 2,
                },
                Token::Str("field0"),
                #[cfg(target_pointer_width = "16")]
                Token::U16(u16::MAX),
                #[cfg(target_pointer_width = "32")]
                Token::U32(u32::MAX),
                #[cfg(target_pointer_width = "64")]
                Token::U64(u64::MAX),
                Token::Str("field1"),
                Token::String(field1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_serialize_deserialize_option_none() {
        let data = ArcSwapAnyEq(ArcSwapOption::<Foo>::from_pointee(None));

        assert_tokens(&data, &[Token::None]);
    }
}
