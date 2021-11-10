use serde::{Serialize, Serializer, Deserialize, Deserializer};
use crate::{RefCnt, ArcSwapAny, Strategy};

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
    use crate::{ArcSwapOption, ArcSwap};
    use serde_derive::{Serialize, Deserialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,)]
    struct Foo {
        field0: usize,
        field1: String,
    }

    #[test]
    fn test_serialize() {
        let data_orig = Foo {
            field0: usize::MAX,
            field1: format!("FOO_{}", i128::MIN),
        };
        let data = ArcSwap::from_pointee(data_orig.clone());

        let data_str = serde_json::to_string(&data).unwrap();
        let data_deser = serde_json::from_str::<Foo>(&data_str).unwrap();

        assert_eq!(data_deser, data_orig);
    }

    #[test]
    fn test_deserialize() {
        let field0 = usize::MAX;
        let field1 = format!("FOO_{}", usize::MIN);

        let str = format!(r#"{{"field0":{},"field1":"{}"}}"#, field0, field1);
        let data = serde_json::from_str::<ArcSwap<Foo>>(&str).unwrap();

        assert_eq!(data.load().field0, field0);
        assert_eq!(data.load().field1, field1);
    }

    #[test]
    fn test_serialize_option() {
        let data_orig = Foo {
            field0: usize::MAX,
            field1: format!("FOO_{}", i128::MIN),
        };
        let data = ArcSwapOption::from_pointee(data_orig.clone());

        let data_str = serde_json::to_string(&data).unwrap();
        let data_deser = serde_json::from_str::<Foo>(&data_str).unwrap();

        assert_eq!(data_deser, data_orig);
    }

    #[test]
    fn test_deserialize_option() {
        let field0 = usize::MAX;
        let field1 = format!("FOO_{}", usize::MIN);

        let str = format!(r#"{{"field0":{},"field1":"{}"}}"#, field0, field1);
        let data = serde_json::from_str::<ArcSwapOption<Foo>>(&str).unwrap();

        assert_eq!(data.load_full().unwrap().field0, field0);
        assert_eq!(data.load_full().unwrap().field1, field1);
    }

    #[test]
    fn test_serialize_option_none() {
        let data = ArcSwapOption::<Foo>::from_pointee(None);

        let data_str = serde_json::to_string(&data).unwrap();
        let data_deser = serde_json::from_str::<Option<Foo>>(&data_str).unwrap();

        assert_eq!(data_deser, None);
    }

    #[test]
    fn test_deserialize_option_none() {
        let str = "null";
        let data = serde_json::from_str::<ArcSwapOption<Foo>>(&str).unwrap();

        assert_eq!(data.load_full(), None);
    }
}
