// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

///Pressure conversion functions
pub mod pressure {
	/// Atmospheres conversion functions
	pub mod atmospheres {
		/// Converts the supplied Atmospheres value to Bars
		/// # Arguments
		/// * `value` - The Atmospheres input value
		pub fn to_bars(value: f64) -> f64 {
			return value * 1.01325;
		}
		/// Converts the supplied Atmospheres value to Pascals
		/// # Arguments
		/// * `value` - The Atmospheres input value
		pub fn to_pascals(value: f64) -> f64 {
			return value * 101325.0;
		}
		/// Converts the supplied Atmospheres value to Torrs
		/// # Arguments
		/// * `value` - The Atmospheres input value
		pub fn to_torrs(value: f64) -> f64 {
			return value * 760.0;
		}
		/// Converts the supplied Atmospheres value to Psi
		/// # Arguments
		/// * `value` - The Atmospheres input value
		pub fn to_psi(value: f64) -> f64 {
			return value * 14.69596432068;
		}
	}
	/// Bars conversion functions
	pub mod bars {
		/// Converts the supplied Bars value to Atmospheres
		/// # Arguments
		/// * `value` - The Bars input value
		pub fn to_atmospheres(value: f64) -> f64 {
			return value / 1.01325;
		}
		/// Converts the supplied Bars value to Pascals
		/// # Arguments
		/// * `value` - The Bars input value
		pub fn to_pascals(value: f64) -> f64 {
			return value / 0.00001;
		}
		/// Converts the supplied Bars value to Torrs
		/// # Arguments
		/// * `value` - The Bars input value
		pub fn to_torrs(value: f64) -> f64 {
			return value * 750.0616827042;
		}
		/// Converts the supplied Bars value to Psi
		/// # Arguments
		/// * `value` - The Bars input value
		pub fn to_psi(value: f64) -> f64 {
			return value * 14.50378911491;
		}
	}
	/// Pascals conversion functions
	pub mod pascals {
		/// Converts the supplied Pascals value to Atmospheres
		/// # Arguments
		/// * `value` - The Pascals input value
		pub fn to_atmospheres(value: f64) -> f64 {
			return value / 101325.0;
		}
		/// Converts the supplied Pascals value to Bars
		/// # Arguments
		/// * `value` - The Pascals input value
		pub fn to_bars(value: f64) -> f64 {
			return value * 0.00001;
		}
		/// Converts the supplied Pascals value to Torrs
		/// # Arguments
		/// * `value` - The Pascals input value
		pub fn to_torrs(value: f64) -> f64 {
			return value * 0.007500616827042;
		}
		/// Converts the supplied Pascals value to Psi
		/// # Arguments
		/// * `value` - The Pascals input value
		pub fn to_psi(value: f64) -> f64 {
			return value * 0.0001450378911491;
		}
	}
	/// Psi conversion functions
	pub mod psi {
		/// Converts the supplied Psi value to Bars
		/// # Arguments
		/// * `value` - The Psi input value
		pub fn to_bars(value: f64) -> f64 {
			return value / 14.50378911491;
		}
		/// Converts the supplied Psi value to Pascals
		/// # Arguments
		/// * `value` - The Psi input value
		pub fn to_pascals(value: f64) -> f64 {
			return value / 0.0001450378911491;
		}
		/// Converts the supplied Psi value to Atmospheres
		/// # Arguments
		/// * `value` - The Psi input value
		pub fn to_atmospheres(value: f64) -> f64 {
			return value / 14.69596432068;
		}
		/// Converts the supplied Psi value to Torrs
		/// # Arguments
		/// * `value` - The Psi input value
		pub fn to_torrs(value: f64) -> f64 {
			return value / 0.01933679515879;
		}
	}
	/// Torrs conversion functions
	pub mod torrs {
		/// Converts the supplied Torrs value to Bars
		/// # Arguments
		/// * `value` - The Torrs input value
		pub fn to_bars(value: f64) -> f64 {
			return value / 750.0616827042;
		}
		/// Converts the supplied Torrs value to Pascals
		/// # Arguments
		/// * `value` - The Torrs input value
		pub fn to_pascals(value: f64) -> f64 {
			return value / 0.007500616827042;
		}
		/// Converts the supplied Torrs value to Atmospheres
		/// # Arguments
		/// * `value` - The Torrs input value
		pub fn to_atmospheres(value: f64) -> f64 {
			return value / 760.0;
		}
		/// Converts the supplied Torrs value to Psi
		/// # Arguments
		/// * `value` - The Torrs input value
		pub fn to_psi(value: f64) -> f64 {
			return value * 0.01933679515879;
		}
	}
}
