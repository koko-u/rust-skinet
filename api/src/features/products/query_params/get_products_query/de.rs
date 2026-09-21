/// comma separated `String` into `Vec<String>` deserializer
pub fn comma_separated<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize as _;

    let Some(value) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None);
    };
    let values = value
        .split(',')
        .map(|term| term.trim().to_string())
        .collect::<Vec<_>>();

    let opt_values = if values.is_empty() { None } else { Some(values) };

    Ok(opt_values)
}
