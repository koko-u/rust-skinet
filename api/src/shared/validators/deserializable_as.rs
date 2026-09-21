pub fn deserializable_as<T>(value: &str, _: &()) -> garde::Result
where
    T: serde::de::DeserializeOwned,
{
    serde_plain::from_str::<T>(value)
        .map(|_| ())
        .map_err(|_| garde::Error::new("invalid value"))
}
