pub fn decimal_places(max: u32) -> impl FnOnce(&'_ rust_decimal::Decimal, &'_ ()) -> garde::Result {
    move |value, _| {
        if value.normalize().scale() > max {
            return Err(garde::Error::new(format!(
                "must have at most {max} decimal places"
            )));
        }

        Ok(())
    }
}

pub fn opt_decimal_places(
    max: u32,
) -> impl FnOnce(&'_ Option<rust_decimal::Decimal>, &'_ ()) -> garde::Result {
    move |value, _| match value {
        Some(value) if value.normalize().scale() > max => {
            return Err(garde::Error::new(format!(
                "must have at most {max} decimal places"
            )));
        }
        _ => Ok(()),
    }
}
