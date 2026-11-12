pub mod age_experience;
pub mod base_price;
pub mod kbm;
pub mod limitation;
pub mod place;
pub mod power;
pub mod season;

use crate::error::ApiError;
pub trait Validate {
    fn validate(&self) -> Result<(), ApiError>;
}
fn positive(value: f64) -> Result<(), ApiError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(ApiError::invalid(
            "Prices and coefficients must be finite and positive",
        ));
    }
    Ok(())
}
macro_rules! validate_models {
    ($module:ident, [$($model:ident),+], $value:ident, $body:block) => { $(
        impl Validate for $module::$model {
            fn validate(&self) -> Result<(), ApiError> { let $value = self; $body }
        }
    )+ };
}
validate_models!(place, [Place, CreatePlace], v, {
    positive(v.coefficient)?;
    if v.name.trim().is_empty() {
        return Err(ApiError::invalid("Place name is required"));
    }
    Ok(())
});
validate_models!(power, [Power, CreatePower], v, {
    positive(v.coefficient)?;
    if v.min_power < 0 || v.max_power < v.min_power {
        return Err(ApiError::invalid("Invalid power range"));
    }
    Ok(())
});
validate_models!(kbm, [Kbm, CreateKbm], v, {
    positive(v.coefficient)?;
    if !(-1..=13).contains(&v.class) {
        return Err(ApiError::invalid("Invalid KBM class"));
    }
    Ok(())
});
validate_models!(age_experience, [AgeExperience, CreateAgeExperience], v, {
    positive(v.coefficient)?;
    if v.age < 16 || v.experience > v.age.saturating_sub(16) {
        return Err(ApiError::invalid("Invalid age or experience"));
    }
    Ok(())
});
validate_models!(season, [Season, CreateSeason], v, {
    positive(v.coefficient)?;
    if !(1..=12).contains(&v.months) {
        return Err(ApiError::invalid("Months must be between 1 and 12"));
    }
    Ok(())
});
validate_models!(limitation, [Limitation, CreateLimitation], v, {
    positive(v.coefficient)
});
validate_models!(base_price, [BasePrice, CreateBasePrice], v, {
    positive(v.min_base_price)?;
    positive(v.max_base_price)?;
    if v.min_base_price > v.max_base_price {
        return Err(ApiError::invalid("Invalid base price range"));
    }
    Ok(())
});
