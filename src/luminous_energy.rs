// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

//! # Luminous Energy based converters

/// LumenHour conversion functions
pub mod lumen_hour {
	/// Converts the supplied LumenHour value to Talbot
	/// # Arguments
	/// * `value` - The LumenHour input value
	pub fn to_talbot(value: f64) -> f64 {
		return value * 3600.0;
	}
	/// Converts the supplied LumenHour value to LumenMinute
	/// # Arguments
	/// * `value` - The LumenHour input value
	pub fn to_lumen_minute(value: f64) -> f64 {
		return value * 60.0;
	}
	/// Converts the supplied LumenHour value to LumenSecond
	/// # Arguments
	/// * `value` - The LumenHour input value
	pub fn to_lumen_second(value: f64) -> f64 {
		return value * 3600.0;
	}
}
/// LumenMinute conversion functions
pub mod lumen_minute {
	/// Converts the supplied LumenMinute value to Talbot
	/// # Arguments
	/// * `value` - The LumenMinute input value
	pub fn to_talbot(value: f64) -> f64 {
		return value * 60.0;
	}
	/// Converts the supplied LumenMinute value to LumenHour
	/// # Arguments
	/// * `value` - The LumenMinute input value
	pub fn to_lumen_hour(value: f64) -> f64 {
		return value / 60.0;
	}
	/// Converts the supplied LumenMinute value to LumenSecond
	/// # Arguments
	/// * `value` - The LumenMinute input value
	pub fn to_lumen_second(value: f64) -> f64 {
		return value * 60.0;
	}
}
/// LumenSecond conversion functions
pub mod lumen_second {
	/// Converts the supplied LumenSecond value to Talbot
	/// # Arguments
	/// * `value` - The LumenSecond input value
	pub fn to_talbot(value: f64) -> f64 {
		return value;
	}
	/// Converts the supplied LumenSecond value to LumenHour
	/// # Arguments
	/// * `value` - The LumenSecond input value
	pub fn to_lumen_hour(value: f64) -> f64 {
		return value / 3600.0;
	}
	/// Converts the supplied LumenSecond value to LumenMinute
	/// # Arguments
	/// * `value` - The LumenSecond input value
	pub fn to_lumen_minute(value: f64) -> f64 {
		return value / 60.0;
	}
}
/// Talbot conversion functions
pub mod talbot {
	/// Converts the supplied Talbot value to LumenSecond
	/// # Arguments
	/// * `value` - The Talbot input value
	pub fn to_lumen_second(value: f64) -> f64 {
		return value;
	}
	/// Converts the supplied Talbot value to LumenHour
	/// # Arguments
	/// * `value` - The Talbot input value
	pub fn to_lumen_hour(value: f64) -> f64 {
		return value / 3600.0;
	}
	/// Converts the supplied Talbot value to LumenMinute
	/// # Arguments
	/// * `value` - The Talbot input value
	pub fn to_lumen_minute(value: f64) -> f64 {
		return value / 60.0;
	}
}
