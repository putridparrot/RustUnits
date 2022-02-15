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
	fn it_convert_knowngigahertz_to_hertz_1() {
		let result: f64 = frequency::gigahertz::to_hertz(0.009);
		assert_approx_eq!(9000000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_hertz_2() {
		let result: f64 = frequency::gigahertz::to_hertz(0.000123);
		assert_approx_eq!(123000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_hertz_3() {
		let result: f64 = frequency::gigahertz::to_hertz(0.0000456);
		assert_approx_eq!(45600.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_kilohertz_1() {
		let result: f64 = frequency::gigahertz::to_kilohertz(0.009);
		assert_approx_eq!(9000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_kilohertz_2() {
		let result: f64 = frequency::gigahertz::to_kilohertz(1e-5);
		assert_approx_eq!(10.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_kilohertz_3() {
		let result: f64 = frequency::gigahertz::to_kilohertz(0.000065);
		assert_approx_eq!(65.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_megahertz_1() {
		let result: f64 = frequency::gigahertz::to_megahertz(0.9);
		assert_approx_eq!(900.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_megahertz_2() {
		let result: f64 = frequency::gigahertz::to_megahertz(0.0123);
		assert_approx_eq!(12.3, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knowngigahertz_to_megahertz_3() {
		let result: f64 = frequency::gigahertz::to_megahertz(5.0);
		assert_approx_eq!(5000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_kilohertz_1() {
		let result: f64 = frequency::hertz::to_kilohertz(800.0);
		assert_approx_eq!(0.8, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_kilohertz_2() {
		let result: f64 = frequency::hertz::to_kilohertz(1506.9);
		assert_approx_eq!(1.5069, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_kilohertz_3() {
		let result: f64 = frequency::hertz::to_kilohertz(5000.0);
		assert_approx_eq!(5.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_megahertz_1() {
		let result: f64 = frequency::hertz::to_megahertz(900009.0);
		assert_approx_eq!(0.900009, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_megahertz_2() {
		let result: f64 = frequency::hertz::to_megahertz(160000.0);
		assert_approx_eq!(0.16, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_megahertz_3() {
		let result: f64 = frequency::hertz::to_megahertz(888888.0);
		assert_approx_eq!(0.888888, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_gigahertz_1() {
		let result: f64 = frequency::hertz::to_gigahertz(100900900.0);
		assert_approx_eq!(0.1009009, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_gigahertz_2() {
		let result: f64 = frequency::hertz::to_gigahertz(9.0);
		assert_approx_eq!(9e-9, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownhertz_to_gigahertz_3() {
		let result: f64 = frequency::hertz::to_gigahertz(90909090.0);
		assert_approx_eq!(0.09090909, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_hertz_1() {
		let result: f64 = frequency::kilohertz::to_hertz(90.90909);
		assert_approx_eq!(90909.09, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_hertz_2() {
		let result: f64 = frequency::kilohertz::to_hertz(0.12345);
		assert_approx_eq!(123.45, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_hertz_3() {
		let result: f64 = frequency::kilohertz::to_hertz(500.0);
		assert_approx_eq!(500000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_megahertz_1() {
		let result: f64 = frequency::kilohertz::to_megahertz(909.0);
		assert_approx_eq!(0.909, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_megahertz_2() {
		let result: f64 = frequency::kilohertz::to_megahertz(123456.0);
		assert_approx_eq!(123.456, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_megahertz_3() {
		let result: f64 = frequency::kilohertz::to_megahertz(900.0);
		assert_approx_eq!(0.9, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_gigahertz_1() {
		let result: f64 = frequency::kilohertz::to_gigahertz(987654.0);
		assert_approx_eq!(0.987654, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_gigahertz_2() {
		let result: f64 = frequency::kilohertz::to_gigahertz(10000.0);
		assert_approx_eq!(0.01, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownkilohertz_to_gigahertz_3() {
		let result: f64 = frequency::kilohertz::to_gigahertz(90090.0);
		assert_approx_eq!(0.09009, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_hertz_1() {
		let result: f64 = frequency::megahertz::to_hertz(0.9);
		assert_approx_eq!(900000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_hertz_2() {
		let result: f64 = frequency::megahertz::to_hertz(0.001);
		assert_approx_eq!(1000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_hertz_3() {
		let result: f64 = frequency::megahertz::to_hertz(0.091);
		assert_approx_eq!(91000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_kilohertz_1() {
		let result: f64 = frequency::megahertz::to_kilohertz(0.87);
		assert_approx_eq!(870.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_kilohertz_2() {
		let result: f64 = frequency::megahertz::to_kilohertz(12.0);
		assert_approx_eq!(12000.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_kilohertz_3() {
		let result: f64 = frequency::megahertz::to_kilohertz(88.1);
		assert_approx_eq!(88100.0, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_gigahertz_1() {
		let result: f64 = frequency::megahertz::to_gigahertz(798.0);
		assert_approx_eq!(0.798, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_gigahertz_2() {
		let result: f64 = frequency::megahertz::to_gigahertz(900.0);
		assert_approx_eq!(0.9, result, 0.01);
	}

	/// Need to convert to parameterized tests
	#[test]
	fn it_convert_knownmegahertz_to_gigahertz_3() {
		let result: f64 = frequency::megahertz::to_gigahertz(579.1);
		assert_approx_eq!(0.5791, result, 0.01);
	}

}
