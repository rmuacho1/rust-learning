fn main() {
	let x = 12.0;
	let y = 14.0;

	let sum = add_number(x, y);
	let subtraction = subtract_number(x,y);
	let multiplication = multiply_number(x,y);
	let divide = divide_number(x,y);

	println!("{x} + {y} = {sum}");
	println!("{x} - {y} = {subtraction}");
	println!("{x} x {y} = {multiplication}");
	println!("{x} / {y} = {divide}");

}

fn add_number(a: f32, b: f32) -> f32 {
	a + b
}

fn subtract_number(a: f32, b: f32) -> f32 {
	a - b
}

fn multiply_number(a: f32, b: f32) -> f32 {
	a * b
}

fn divide_number(a: f32, b: f32) -> f32 {
	a / b
}