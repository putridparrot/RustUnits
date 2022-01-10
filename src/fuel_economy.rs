// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

///Fuel Economy conversion functions
pub mod fuel_economy {
	/// Kilometre Per Litre conversion functions
	pub mod kilometre_per_litre {
		/// Converts the supplied Kilometre Per Litre value to US Miles Per Gallon
		/// # Arguments
		/// * `value` - The Kilometre Per Litre input value
		pub fn to_u_s_miles_per_gallon(value: f64) -> f64 {
			return value * 2.35215;
		}
		/// Converts the supplied Kilometre Per Litre value to Miles Per Gallon
		/// # Arguments
		/// * `value` - The Kilometre Per Litre input value
		pub fn to_miles_per_gallon(value: f64) -> f64 {
			return value * 2.82481;
		}
	}
	/// Miles Per Gallon conversion functions
	pub mod miles_per_gallon {
		/// Converts the supplied Miles Per Gallon value to Kilometre Per Litre
		/// # Arguments
		/// * `value` - The Miles Per Gallon input value
		pub fn to_kilometre_per_litre(value: f64) -> f64 {
			return value / 2.82481;
		}
		/// Converts the supplied Miles Per Gallon value to US Miles Per Gallon
		/// # Arguments
		/// * `value` - The Miles Per Gallon input value
		pub fn to_u_s_miles_per_gallon(value: f64) -> f64 {
			return value / 1.20095;
		}
	}
	/// US Miles Per Gallon conversion functions
	pub mod u_s_miles_per_gallon {
		/// Converts the supplied US Miles Per Gallon value to Kilometre Per Litre
		/// # Arguments
		/// * `value` - The US Miles Per Gallon input value
		pub fn to_kilometre_per_litre(value: f64) -> f64 {
			return value / 2.35215;
		}
		/// Converts the supplied US Miles Per Gallon value to Miles Per Gallon
		/// # Arguments
		/// * `value` - The US Miles Per Gallon input value
		pub fn to_miles_per_gallon(value: f64) -> f64 {
			return value * 1.20095;
		}
	}
}
