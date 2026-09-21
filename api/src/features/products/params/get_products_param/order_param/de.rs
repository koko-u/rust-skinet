use super::direction;
use super::sort;

pub fn deserialize_sort<'de, D>(deserializer: D) -> Result<Vec<sort::ProductSort>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize as _;
    let value = String::deserialize(deserializer)?;

    if value.is_empty() {
        return Ok(Vec::new());
    }

    let mut sorts = Vec::new();

    // split by , (ex. "name:asc,id:desc" -> ["name:asc", "id:desc"])
    let terms = value.split(',').map(|s| s.trim().to_string());
    for term in terms {
        // split by : upto 2 elements (ex. "name:asc" -> ["name", "asc"] )
        let mut it = term.splitn(2, ':');
        // field
        let field = it
            .next()
            .ok_or_else(|| serde::de::Error::custom("missing field"))?;
        // direction (optional)
        let direction = match it.next() {
            Some(dir) => dir.to_string(),
            None => direction::SortDirection::default().to_string(),
        };

        // pub sort data
        sorts.push(sort::ProductSort {
            field: field.to_string(),
            direction,
        });
    }

    Ok(sorts)
}
