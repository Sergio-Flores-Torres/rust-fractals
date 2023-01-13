extern crate num_complex;
extern crate image;

use num_complex::Complex;

use std::{
	str::FromStr,
	fs::File,
	io::{
		Error, ErrorKind, Result,
	},
};

use image::{
	ColorType,
	codecs::png::PngEncoder
};


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

/// Splits coordinates params into number component parts, like so 100x40, or 1.0,5000
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

/// Given a string representing a complex, creates a complex from it
fn parse_complex(s: &str) -> Option<Complex<f64>> {

	match parse_coordinates::<f64>(s, ',') {
		None => None, 
		Some((l, r)) => Some(Complex {re: l , im: r}),
	}

}

/// Converts a pixel from image space to a point in number space
///
/// pixel is a point in the image
/// bounds is the width and height of the image
/// upper_left and lower_right are the end points of the number space
fn pixel_to_point(	pixel: (f64, f64),
					bounds: (f64, f64), 
					upper_left: Complex<f64>,
					lower_right: Complex<f64>) -> Complex<f64> {

	let (width, height) = ( lower_right.re - upper_left.re, upper_left.im - lower_right.im);


	Complex {
		re: ((pixel.0 / bounds.0) * width) + upper_left.re,
		im: ((pixel.1 / bounds.1) * height) - upper_left.im,
	}
}

/// Given the image space params and number space params, determines the color of a pixel in image space
fn render(pixels: &mut [u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {

	for y in 0..bounds.1 {
		for x in 0..bounds.0 {
			let point = pixel_to_point(	(x as f64, y as f64), 
										(bounds.0 as f64, bounds.1 as f64),
										upper_left, lower_right);

			pixels[(y * bounds.0) + x] = match outside_two_radius_iterations_count(point, 255 as u64) {
				None => 0,
				Some(count) => 255 - count as u8,
			};
		}
	}
}

/// Saves image to file
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<()> {
	let output = File::create(filename)?;
	let encoder = PngEncoder::new(output);

	match encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8) {
		Ok(v) => v,
		Err(_v) => return Err(Error::new(ErrorKind::Other, "oh no!")),
	}

	Ok(())
}

fn main() {
    println!("Rust Fractals Demo - Jan 12th, 2022 by sergio@saft.industries");
	println!("Mandelbrot set");

	let args: Vec<String> = std::env::args().collect();

	if args.len() != 5 {
		eprintln!("Usage: rust-fractals FILENAME IMAGESIZE UPPERLEFT LOWERRIGHT");
		eprintln!("Example: rust-fractals mandel.png 1000x750 -1.20,0.35 -1,0.20");
		std::process::exit(1);
	}

	let bounds = parse_coordinates(&args[2], 'x').expect("error with imagesize");
	let upper_left = parse_complex(&args[3]).expect("error with upperleft");
	let lower_right = parse_complex(&args[4]).expect("error with lowerright");

	let mut pixels = vec![0; bounds.0 * bounds.1];

	render(&mut pixels, bounds, upper_left, lower_right);

	write_image(&args[1], &pixels, bounds).expect("error with PNG file");
}
