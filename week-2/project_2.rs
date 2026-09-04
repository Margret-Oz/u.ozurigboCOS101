fn main() {
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;

	let sum:f64 = t*2.00 + m + h*3.00 + d*3.00 + a; // sum of all sales capital made
	let average:f64 = sum/10.00; // average of all sales capital made

	println!(" the sum and average respectively of sales made are N{} and N{}", sum, average );
    

}