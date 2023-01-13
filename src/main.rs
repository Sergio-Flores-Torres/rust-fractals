extern crate num_complex;

use num_complex::Complex;
use std::str::FromStr;

/// Given a complex number c, determine if z remains within the 2.0 radius limit or not in max limit iterations
fn outside_two_radius_iterations_count(c: Complex<f64>, iterations_limit: u64) -> Option<u64> {
	let mut z = Complex {re: 0.0 , im: 0.0 };

	for i in 0..iterations_limit { // Some values coalesce very fast and so the limit need not be very large
		z = (z * z) + c;
		//println!("{}", z);

		if z.norm_sqr() > 4.0 {	// No need to test further in this case
			return Some(i);
		}
	}

	None
}

// Splits coordinates params into number component parts, like so 100x40, or 1.0,5000
fn parse_coordinates<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {

	match s.find(sep) {
		None => None, 
		Some(index) => {

			match (T::from_str(&s[..index]), T::from_str(&s[(index + 1)..])) {
				(Ok(l), Ok(r)) => Some((l, r)),
				(Err(_l), Err(_r)) => None,
				(Ok(_l), Err(_r)) => None,
				(Err(_l), Ok(_r)) => None,
			}

		}
	}

}

// Given a string representing a complex, creates a complex from it
fn parse_complex(s: &str) -> Option<Complex<f64>> {

	match parse_coordinates::<f64>(s, ',') {
		None => None, 
		Some((l, r)) => Some(Complex {re: l , im: r}),
	}

}

fn main() {
    println!("Rust Fractals Demo - Jan 12th, 2022 by sergio@saft.industries");
	println!("Mandelbrot set");

	outside_two_radius_iterations_count(Complex {re: 0.5 , im: 0.5 }, 1000 as u64);
}
