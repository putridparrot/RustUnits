// <auto-generated>
// This code was generated by the UnitCodeGenerator tool
//
// Changes to this file will be lost if the code is regenerated
// </auto-generated>


#[cfg(test)]
mod tests {
	use unit_conversions::*;
	use assert_approx_eq::assert_approx_eq;

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_fahrenheit_1() {
		let result: f64 = temperature::celsius::to_fahrenheit(32.0);
		assert_approx_eq!(89.6, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_fahrenheit_2() {
		let result: f64 = temperature::celsius::to_fahrenheit(0.9);
		assert_approx_eq!(33.62, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_fahrenheit_3() {
		let result: f64 = temperature::celsius::to_fahrenheit(0.0);
		assert_approx_eq!(32.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_kelvin_1() {
		let result: f64 = temperature::celsius::to_kelvin(1.0);
		assert_approx_eq!(274.15, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_kelvin_2() {
		let result: f64 = temperature::celsius::to_kelvin(9.9);
		assert_approx_eq!(283.05, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_kelvin_3() {
		let result: f64 = temperature::celsius::to_kelvin(100.0);
		assert_approx_eq!(373.15, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_rankine_1() {
		let result: f64 = temperature::celsius::to_rankine(900.0);
		assert_approx_eq!(2111.67, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_rankine_2() {
		let result: f64 = temperature::celsius::to_rankine(12.0);
		assert_approx_eq!(513.27, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowncelsius_to_rankine_3() {
		let result: f64 = temperature::celsius::to_rankine(-3.0);
		assert_approx_eq!(486.27, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_celsius_1() {
		let result: f64 = temperature::fahrenheit::to_celsius(109.0);
		assert_approx_eq!(42.7778, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_celsius_2() {
		let result: f64 = temperature::fahrenheit::to_celsius(56.9);
		assert_approx_eq!(13.83333, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_celsius_3() {
		let result: f64 = temperature::fahrenheit::to_celsius(102.0);
		assert_approx_eq!(38.8889, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_kelvin_1() {
		let result: f64 = temperature::fahrenheit::to_kelvin(34.5);
		assert_approx_eq!(274.5389, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_kelvin_2() {
		let result: f64 = temperature::fahrenheit::to_kelvin(901.0);
		assert_approx_eq!(755.928, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_kelvin_3() {
		let result: f64 = temperature::fahrenheit::to_kelvin(23.9);
		assert_approx_eq!(268.65, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_rankine_1() {
		let result: f64 = temperature::fahrenheit::to_rankine(123.0);
		assert_approx_eq!(582.67, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_rankine_2() {
		let result: f64 = temperature::fahrenheit::to_rankine(9.2);
		assert_approx_eq!(468.87, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownfahrenheit_to_rankine_3() {
		let result: f64 = temperature::fahrenheit::to_rankine(0.2);
		assert_approx_eq!(459.87, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_celsius_1() {
		let result: f64 = temperature::kelvin::to_celsius(80.0);
		assert_approx_eq!(-193.15, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_celsius_2() {
		let result: f64 = temperature::kelvin::to_celsius(0.9);
		assert_approx_eq!(-272.25, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_celsius_3() {
		let result: f64 = temperature::kelvin::to_celsius(305.15);
		assert_approx_eq!(32.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_fahrenheit_1() {
		let result: f64 = temperature::kelvin::to_fahrenheit(123.4);
		assert_approx_eq!(-237.55, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_fahrenheit_2() {
		let result: f64 = temperature::kelvin::to_fahrenheit(800.0);
		assert_approx_eq!(980.33, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_fahrenheit_3() {
		let result: f64 = temperature::kelvin::to_fahrenheit(99.999);
		assert_approx_eq!(-279.6718, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_rankine_1() {
		let result: f64 = temperature::kelvin::to_rankine(156.0);
		assert_approx_eq!(280.8, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_rankine_2() {
		let result: f64 = temperature::kelvin::to_rankine(8.2);
		assert_approx_eq!(14.76, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkelvin_to_rankine_3() {
		let result: f64 = temperature::kelvin::to_rankine(0.8);
		assert_approx_eq!(1.44, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_celsius_1() {
		let result: f64 = temperature::rankine::to_celsius(190.0);
		assert_approx_eq!(-167.59444444, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_celsius_2() {
		let result: f64 = temperature::rankine::to_celsius(0.7);
		assert_approx_eq!(-272.76111111, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_celsius_3() {
		let result: f64 = temperature::rankine::to_celsius(900.0);
		assert_approx_eq!(226.85, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_fahrenheit_1() {
		let result: f64 = temperature::rankine::to_fahrenheit(109.0);
		assert_approx_eq!(-350.67, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_fahrenheit_2() {
		let result: f64 = temperature::rankine::to_fahrenheit(3567.0);
		assert_approx_eq!(3107.33, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_fahrenheit_3() {
		let result: f64 = temperature::rankine::to_fahrenheit(9.0);
		assert_approx_eq!(-450.67, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_kelvin_1() {
		let result: f64 = temperature::rankine::to_kelvin(123.0);
		assert_approx_eq!(68.333333333, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_kelvin_2() {
		let result: f64 = temperature::rankine::to_kelvin(0.9);
		assert_approx_eq!(0.5, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownrankine_to_kelvin_3() {
		let result: f64 = temperature::rankine::to_kelvin(23.0);
		assert_approx_eq!(12.777777778, result, 0.01);
	}

}
