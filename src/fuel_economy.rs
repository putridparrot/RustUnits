// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>

/// Fuel Economy conversion functions
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
		/// Converts the supplied Kilometre Per Litre value to Litres per 100 Kilometres
		/// # Arguments
		/// * `value` - The Kilometre Per Litre input value
		pub fn to_litresper100_kilometres(value: f64) -> f64 {
			return 100.0 / value;
		}
	}
	/// Litres per 100 Kilometres conversion functions
	pub mod litresper100_kilometres {
		/// Converts the supplied Litres per 100 Kilometres value to Kilometre Per Litre
		/// # Arguments
		/// * `value` - The Litres per 100 Kilometres input value
		pub fn to_kilometre_per_litre(value: f64) -> f64 {
			return 100.0 / value;
		}
		/// Converts the supplied Litres per 100 Kilometres value to Miles Per Gallon
		/// # Arguments
		/// * `value` - The Litres per 100 Kilometres input value
		pub fn to_miles_per_gallon(value: f64) -> f64 {
			return 282.481 / value;
		}
		/// Converts the supplied Litres per 100 Kilometres value to US Miles Per Gallon
		/// # Arguments
		/// * `value` - The Litres per 100 Kilometres input value
		pub fn to_u_s_miles_per_gallon(value: f64) -> f64 {
			return 235.215 / value;
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
		/// Converts the supplied Miles Per Gallon value to Litres per 100 Kilometres
		/// # Arguments
		/// * `value` - The Miles Per Gallon input value
		pub fn to_litresper100_kilometres(value: f64) -> f64 {
			return 282.481 / value;
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
		/// Converts the supplied US Miles Per Gallon value to Litres per 100 Kilometres
		/// # Arguments
		/// * `value` - The US Miles Per Gallon input value
		pub fn to_litresper100_kilometres(value: f64) -> f64 {
			return 235.215 / value;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use assert_approx_eq::assert_approx_eq;

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_u_s_miles_per_gallon_1() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_u_s_miles_per_gallon(109.0);
		assert_approx_eq!(256.384, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_u_s_miles_per_gallon_2() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_u_s_miles_per_gallon(9.4);
		assert_approx_eq!(22.1102, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_u_s_miles_per_gallon_3() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_u_s_miles_per_gallon(1.3);
		assert_approx_eq!(3.05779, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_miles_per_gallon_1() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_miles_per_gallon(5.0);
		assert_approx_eq!(14.124, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_miles_per_gallon_2() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_miles_per_gallon(180.0);
		assert_approx_eq!(508.466, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_miles_per_gallon_3() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_miles_per_gallon(5.4);
		assert_approx_eq!(15.254, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_litresper100_kilometres_1() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_litresper100_kilometres(109.0);
		assert_approx_eq!(0.917431, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_litresper100_kilometres_2() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_litresper100_kilometres(0.8);
		assert_approx_eq!(125.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilometre_per_litre_to_litresper100_kilometres_3() {
		let result: f64 = fuel_economy::kilometre_per_litre::to_litresper100_kilometres(2.3);
		assert_approx_eq!(43.4783, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_kilometre_per_litre_1() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_kilometre_per_litre(0.7);
		assert_approx_eq!(142.857, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_kilometre_per_litre_2() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_kilometre_per_litre(109.0);
		assert_approx_eq!(0.917431, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_kilometre_per_litre_3() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_kilometre_per_litre(0.012);
		assert_approx_eq!(8333.3333, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_miles_per_gallon_1() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_miles_per_gallon(107.0);
		assert_approx_eq!(2.64001, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_miles_per_gallon_2() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_miles_per_gallon(0.8);
		assert_approx_eq!(353.101, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_miles_per_gallon_3() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_miles_per_gallon(0.02);
		assert_approx_eq!(14124.05, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_u_s_miles_per_gallon_1() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_u_s_miles_per_gallon(12.0);
		assert_approx_eq!(19.6012, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_u_s_miles_per_gallon_2() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_u_s_miles_per_gallon(0.3);
		assert_approx_eq!(784.049, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownlitresper100_kilometres_to_u_s_miles_per_gallon_3() {
		let result: f64 = fuel_economy::litresper100_kilometres::to_u_s_miles_per_gallon(1.5);
		assert_approx_eq!(156.81, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_kilometre_per_litre_1() {
		let result: f64 = fuel_economy::miles_per_gallon::to_kilometre_per_litre(12.0);
		assert_approx_eq!(4.24807, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_kilometre_per_litre_2() {
		let result: f64 = fuel_economy::miles_per_gallon::to_kilometre_per_litre(8.2);
		assert_approx_eq!(2.90285, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_kilometre_per_litre_3() {
		let result: f64 = fuel_economy::miles_per_gallon::to_kilometre_per_litre(0.5);
		assert_approx_eq!(0.177003, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_u_s_miles_per_gallon_1() {
		let result: f64 = fuel_economy::miles_per_gallon::to_u_s_miles_per_gallon(1.4);
		assert_approx_eq!(1.16574, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_u_s_miles_per_gallon_2() {
		let result: f64 = fuel_economy::miles_per_gallon::to_u_s_miles_per_gallon(1008.0);
		assert_approx_eq!(839.3356, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_u_s_miles_per_gallon_3() {
		let result: f64 = fuel_economy::miles_per_gallon::to_u_s_miles_per_gallon(0.8);
		assert_approx_eq!(0.666139, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_litresper100_kilometres_1() {
		let result: f64 = fuel_economy::miles_per_gallon::to_litresper100_kilometres(22.3);
		assert_approx_eq!(12.66731, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_litresper100_kilometres_2() {
		let result: f64 = fuel_economy::miles_per_gallon::to_litresper100_kilometres(0.4);
		assert_approx_eq!(706.202, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmiles_per_gallon_to_litresper100_kilometres_3() {
		let result: f64 = fuel_economy::miles_per_gallon::to_litresper100_kilometres(5.2);
		assert_approx_eq!(54.3233, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_kilometre_per_litre_1() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_kilometre_per_litre(23.0);
		assert_approx_eq!(9.77831, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_kilometre_per_litre_2() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_kilometre_per_litre(7.0);
		assert_approx_eq!(2.97601, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_kilometre_per_litre_3() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_kilometre_per_litre(0.9);
		assert_approx_eq!(0.382629, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_miles_per_gallon_1() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_miles_per_gallon(109.0);
		assert_approx_eq!(130.9036, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_miles_per_gallon_2() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_miles_per_gallon(78.2);
		assert_approx_eq!(93.91432, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_miles_per_gallon_3() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_miles_per_gallon(0.9);
		assert_approx_eq!(1.08086, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_litresper100_kilometres_1() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_litresper100_kilometres(111.0);
		assert_approx_eq!(2.11905, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_litresper100_kilometres_2() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_litresper100_kilometres(0.4);
		assert_approx_eq!(588.036, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownu_s_miles_per_gallon_to_litresper100_kilometres_3() {
		let result: f64 = fuel_economy::u_s_miles_per_gallon::to_litresper100_kilometres(5.2);
		assert_approx_eq!(45.2336, result, 0.01);
	}

}
