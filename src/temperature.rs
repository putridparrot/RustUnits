// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

//! # Temperature based converters

/// Celsius conversion functions
pub mod celsius {
	/// Converts the supplied Celsius value to Fahrenheit
	/// # Arguments
	/// * `value` - The Celsius input value
	pub fn to_fahrenheit(value: f64) -> f64 {
		return value * 1.8 + 32.0;
	}
	/// Converts the supplied Celsius value to Kelvin
	/// # Arguments
	/// * `value` - The Celsius input value
	pub fn to_kelvin(value: f64) -> f64 {
		return value + 273.15;
	}
	/// Converts the supplied Celsius value to Rankine
	/// # Arguments
	/// * `value` - The Celsius input value
	pub fn to_rankine(value: f64) -> f64 {
		return value * 9.0/5.0 + 491.67;
	}
}
/// Fahrenheit conversion functions
pub mod fahrenheit {
	/// Converts the supplied Fahrenheit value to Celsius
	/// # Arguments
	/// * `value` - The Fahrenheit input value
	pub fn to_celsius(value: f64) -> f64 {
		return ((value - 32.0) * 5.0) / 9.0;
	}
	/// Converts the supplied Fahrenheit value to Kelvin
	/// # Arguments
	/// * `value` - The Fahrenheit input value
	pub fn to_kelvin(value: f64) -> f64 {
		return (((value - 32.0) * 5.0) / 9.0) + 273.15;
	}
	/// Converts the supplied Fahrenheit value to Rankine
	/// # Arguments
	/// * `value` - The Fahrenheit input value
	pub fn to_rankine(value: f64) -> f64 {
		return value + 459.67;
	}
}
/// Kelvin conversion functions
pub mod kelvin {
	/// Converts the supplied Kelvin value to Celsius
	/// # Arguments
	/// * `value` - The Kelvin input value
	pub fn to_celsius(value: f64) -> f64 {
		return value - 273.15;
	}
	/// Converts the supplied Kelvin value to Fahrenheit
	/// # Arguments
	/// * `value` - The Kelvin input value
	pub fn to_fahrenheit(value: f64) -> f64 {
		return ((value - 273.15) * 1.8) + 32.0;
	}
	/// Converts the supplied Kelvin value to Rankine
	/// # Arguments
	/// * `value` - The Kelvin input value
	pub fn to_rankine(value: f64) -> f64 {
		return value * 1.8;
	}
}
/// Rankine conversion functions
pub mod rankine {
	/// Converts the supplied Rankine value to Celsius
	/// # Arguments
	/// * `value` - The Rankine input value
	pub fn to_celsius(value: f64) -> f64 {
		return (value - 491.67) * 5.0/9.0;
	}
	/// Converts the supplied Rankine value to Fahrenheit
	/// # Arguments
	/// * `value` - The Rankine input value
	pub fn to_fahrenheit(value: f64) -> f64 {
		return value - 459.67;
	}
	/// Converts the supplied Rankine value to Kelvin
	/// # Arguments
	/// * `value` - The Rankine input value
	pub fn to_kelvin(value: f64) -> f64 {
		return value / 1.8;
	}
}
