fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	let base:f64 = 1.00 - (r/100.00);
	let a:f64 = p*base.powf(n);

	println!("the value of the TV after 3 years is n{}", a );
}