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

    #[derive(serde::Serialize, PartialEq, Eq, serde::Deserialize)]
    struct Foo {
        field0: usize,
        field1: String,
    }

    #[test]
    fn test_serialize_deserialize() {
        let data = Foo {
            field0: usize::MAX,
            field1: format!("FOO_{}", i128::MIN),
        };
        let data = ArcSwap::from_pointee(data);

        let data_str = serde_json::to_string(&data).unwrap();
        let data_new = serde_json::from_str::<ArcSwap<Foo>>(&data_str).unwrap();

        assert_eq!(data.load().as_ref(), data_new.load().as_ref());
    }

    #[test]
    fn test_serialize_deserialize_option() {
        let data = Foo {
            field0: usize::MAX,
            field1: format!("FOO_{}", i128::MIN),
        };
        let data = ArcSwapOption::from_pointee(data);

        let data_str = serde_json::to_string(&data).unwrap();
        let data_new = serde_json::from_str::<ArcSwapOption<Foo>>(&data_str).unwrap();

        assert_eq!(data.load().as_ref(), data_new.load().as_ref());
    }

    #[test]
    fn test_serialize_deserialize_option_none() {
        let data = ArcSwapOption::<Foo>::from_pointee(None);

        let data_str = serde_json::to_string(&data).unwrap();
        let data_new = serde_json::from_str::<ArcSwapOption<Foo>>(&data_str).unwrap();

        assert_eq!(data.load().as_ref(), None);
        assert_eq!(data_new.load().as_ref(), None);
        assert_eq!(data.load().as_ref(), data_new.load().as_ref());
    }
}
