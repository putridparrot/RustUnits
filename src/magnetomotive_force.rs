// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

/// Magnetomotive Force conversion functions
pub mod magnetomotive_force {
	/// Ampere-turns conversion functions
	pub mod ampereturns {
		/// Converts the supplied Ampere-turns value to Gilberts
		/// # Arguments
		/// * `value` - The Ampere-turns input value
		pub fn to_gilberts(value: f64) -> f64 {
			return value / 0.7957747154595;
		}
	}
	/// Gilberts conversion functions
	pub mod gilberts {
		/// Converts the supplied Gilberts value to Ampere-turns
		/// # Arguments
		/// * `value` - The Gilberts input value
		pub fn to_ampereturns(value: f64) -> f64 {
			return value * 0.7957747154595;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use assert_approx_eq::assert_approx_eq;

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownampereturns_to_gilberts_1() {
		let result: f64 = magnetomotive_force::ampereturns::to_gilberts(400.0);
		assert_approx_eq!(502.6548248, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownampereturns_to_gilberts_2() {
		let result: f64 = magnetomotive_force::ampereturns::to_gilberts(6.7);
		assert_approx_eq!(8.4194683154, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownampereturns_to_gilberts_3() {
		let result: f64 = magnetomotive_force::ampereturns::to_gilberts(0.8);
		assert_approx_eq!(1.0053096496, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngilberts_to_ampereturns_1() {
		let result: f64 = magnetomotive_force::gilberts::to_ampereturns(0.8);
		assert_approx_eq!(0.6366197721, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngilberts_to_ampereturns_2() {
		let result: f64 = magnetomotive_force::gilberts::to_ampereturns(67.0);
		assert_approx_eq!(53.316905912, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngilberts_to_ampereturns_3() {
		let result: f64 = magnetomotive_force::gilberts::to_ampereturns(2.3);
		assert_approx_eq!(1.8302818447, result, 0.01);
	}

}