// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

//! # Power based converters

/// Mechanical Horse Power conversion functions
pub mod mechanical_horse_power {
	/// Converts the supplied Mechanical Horse Power value to Metric Horse Power
	/// # Arguments
	/// * `value` - The Mechanical Horse Power input value
	pub fn to_metric_horse_power(value: f64) -> f64 {
		return value * 1.013869665424;
	}
}
/// Metric Horse Power conversion functions
pub mod metric_horse_power {
	/// Converts the supplied Metric Horse Power value to Mechanical Horse Power
	/// # Arguments
	/// * `value` - The Metric Horse Power input value
	pub fn to_mechanical_horse_power(value: f64) -> f64 {
		return value / 1.013869665424;
	}
}