fn main()  {
	let p:f64 = 520000000.00;
	let t:f64 = 5.00;
	let r:f64 =10.00;

	let base:f64 = 1.00 + (r/100.00);
	let a = p*(base.powf(t));
	let ci:f64 = a-p;

	println!("The compound interest after 5 years is N{}",ci );
}