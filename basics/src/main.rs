// following along with https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
fn main() {
	hello_world();
	loops_and_ifs();
	add_it_up();
	explicit_funcs();
	array_and_slices();
	slicing_and_dicing();
	vectors();
}

fn hello_world() {
	let answer = 42;
	assert_eq!(answer, 42);
	println!("Hello World! {}", answer);
}

fn loops_and_ifs() {
	for i in 0..5 {
		if i % 2 == 0 {
			println!("{} is even", i);
		} else {
			println!("{} is odd", i);
		}
	}

	// lets get wild
	for x in 0..5 {
		let result = if x % 2 == 0 {"even"} else {"odd"};
		println!("{} is {}", x, result);
	}
}

fn add_it_up() {
	let mut sum = 0.0;
	for i in 0..5 {
		// because i don't like += yet :)
		sum = sum + i as f64;
	}

	println!("Sum is : {}", sum);
}

fn explicit_funcs() {
	let squared = sqrt(42);
	println!("Square of 42 is {}", squared);

	let absolute = abs(42);
	println!("Absolute value of 42 is {}", absolute);

	let clamped = clamp(42, 41, 40);
	println!("Absolute value of [42, 41, 40] is {}", clamped);

	let factorialed = factorial(5);
	println!("Factorial of 5 is {}", factorialed);

	let i = 10;
	let res1 =by_ref(&i);
	let res2 =by_ref(&42);
	println!("1: {} | 2: {} | og: {}", res1, res2, i);

	let mut y = 2;
	modifies(&mut y);
	println!("Res is now {}", y);
}

fn sqrt(initial: i64) -> i64 {
	initial * initial
}

fn abs(x: i64) -> i64 {
	if x > 0 {
		x
	} else {
		-x
	}
}

fn clamp(x: i64, x1: i64, x2: i64) -> i64 {
	if x < x1 {
		x1
	} else if x > x2 {
		x2
	} else {
		x
	}
}

fn factorial(n: u64) -> u64 {
	if n == 0 {
		1
	} else {
		n * factorial(n-1)
	}
}

fn by_ref(x: &i32) -> i32 {
	*x + 1
}

fn modifies(x: &mut i64) {
	*x = 1;
}

fn array_and_slices() {
	basic_arrays();
	sum_with_slices();
}

fn basic_arrays() {
	let arr = [10, 20, 30, 40];

	println!("The first item is: {}", arr[0]);

	for i in 1..4 {
		println!("Key {}: | Value: {}", i, arr[i]);
	}

	println!("Length of array is: {}", arr.len());
}

fn sum_with_slices() {
	let arr = [10, 20, 30, 40];
	let res = slices_sum(&arr);
	println!("Sum With Slices Output is: {}", res);
}

fn slices_sum(values: &[i32]) -> i32 {
	let mut total = 0;
	for i in 0..values.len() {
		total += values[i];
	}

	total
}

fn slicing_and_dicing() {
	print_em();
	slice_em();
	slice_and_get();
	slice_and_get_with_options();
}

fn print_em() {
	let ints = [1,2,3];
	let floats = [1.1, 2.1, 3.1];
	let strings = ["hello", "gary"];
	let ints_ints = [[1,2], [3,4]];

	println!("Ints {:?}", ints);
	println!("Floats {:?}", floats);
	println!("Strings {:?}", strings);
	println!("Ints and Ints {:?}", ints_ints);
}

fn slice_em() {
	let ints = [1,2,3,4,5];
	let slice1 = &ints[0..2];
	let slice2 = &ints[2..];

	println!("Ints - {:?}", ints);
	println!("Slice1 - {:?}", slice1);
	println!("Slice2 - {:?}", slice2);
}

fn slice_and_get() {
	let ints = [11, 22, 33, 44, 55];
	let slice = &ints;
	let first = slice.get(0);
	let last = slice.get(5);
	println!("First {:?}", first);
	println!("Last {:?}", last);
}

fn slice_and_get_with_options() {
	// the docs - https://doc.rust-lang.org/std/option/enum.Option.html
	let ints = [11, 22, 33, 44, 55];
	let slice = &ints;
	let first = slice.get(0);
	let last = slice.get(5);
	println!("First {} {}", first.is_some(), first.is_none());
	println!("Last {} {}", last.is_some(), last.is_none());
	println!("first value {}", first.unwrap());

	// because last.unwrap() will panic we can do...
	let maybe_last = slice.get(5);
	let last_without_panic = if maybe_last.is_some() {
		// `*` is used to deference the Some -> &i32 turning it into i32
		*maybe_last.unwrap()
	} else {
		-1
	};

	println!("Don't panic- last value is {}", last_without_panic);

	// or, to save on postage
	// remember to keep types the same. slice.get will return &i32 so the
	// other side (unwrap_or) needs to return a &i32
	let short_maybe_last = slice.get(5).unwrap_or(&-1);
	println!("A one liner for no-panic {}", short_maybe_last);
}

fn vectors() {
	basic_vectors();
	vectors_and_slices();
}

fn basic_vectors() {
	let mut v = Vec::new();
	v.push(10);
	v.push(20);
	v.push(30);
	v.push(40);

	let first = v[0];
	let maybe_first = v.get(0);

	println!("v is {:?}", v);
	println!("First is {}", first);
	println!("Maybe first is {:?}", maybe_first);
}

fn vectors_and_slices() {
	let mut v = Vec::new();
	v.push(10);
	v.push(11);
	v.push(12);

	println!("v is {:?}", v);

	let slice = &v[1..];
	println!("Slice is {:?}", slice);
}
