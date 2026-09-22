pub fn positive_u32(value: &Option<String>, _: &()) -> garde::Result {
    let Some(value) = value else {
        return Ok(());
    };

    if value.is_empty() {
        return Ok(());
    }

    let value = serde_plain::from_str::<u32>(value)
        .map_err(|_| garde::Error::new("Cannot parse value as integer"))?;
    if value < 1 {
        return Err(garde::Error::new("Value must be positive"));
    }

    Ok(())
}
