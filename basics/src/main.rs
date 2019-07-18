use std::env;
use std::io;
use std::fs::File;
use std::io::Read;

// following along with https://stevedonovan.github.io/rust-gentle-intro/1-basics.html
fn main() {
	hello_world();
	loops_and_ifs();
	add_it_up();
	explicit_funcs();
	array_and_slices();
	slicing_and_dicing();
	vectors();
	iterators();
	strings();
	command_line_args();
	matching();
	read_from_file();
	good_or_bad_call();
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

fn  iterators() {
	basic_iterators();
	array_iterate();
	idiomatic_sum();
	window_slices_and_chunks();
	more_vectors();
}

fn basic_iterators() {
	let mut iter = 0..3;
	println!("Making assertions.");
	assert_eq!(iter.next(), Some(0));
	assert_eq!(iter.next(), Some(1));
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), None);
	println!("Done with assertions!");
}

fn array_iterate() {
	// below fails because array are not iterators
	// let arr = [10,20,30];
	// for i in arr {
	//	println!("{}", i)
	//}

	// but slices are!
	let arr_sliced = &[10,20,30];
	for i in arr_sliced {
		println!("{}", i);
	}
}

fn idiomatic_sum() {
	let sum: i32 = (0..5).sum();
	println!("Sum is {}", sum);
	let sum: i64 = [0, 10, 15].iter().sum();
	println!("Sum is {}", sum);
}

fn window_slices_and_chunks() {
	let slice = &[1,2,3,4,5];

	for s in slice.windows(2) {
		println!("window {:?}", s);
	}

	for c in slice.chunks(2) {
		println!("chunk {:?}", c);
	}
}

fn more_vectors() {
	vector_bang();
}

fn vector_bang() {
	let mut v = vec![1,2,3,4,5];
	v.pop(); // remove the 5

	let mut v2 = Vec::new();
	v2.push(1);
	v2.push(2);
	v2.push(3);
	v2.push(4);

	assert_eq!(v, v2);

	let v3 = vec![1,2,3];
	let mut v4 = vec![];
	v4.extend([1].iter());
	v4.extend([2].iter());
	v4.extend(3..4);

	assert_eq!(v3, v4);

	let mut v5 = vec![1,2,3,4,1,2,3,4,1,2,3,4,1,2,3,4];
	v5.sort();
	assert_eq!(v5, &[1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4]);
	v5.dedup();
	assert_eq!(v5, &[1,2,3,4]);


}

fn strings() {
	basic_strings();
	push_pop_strings();
	array_to_string();
	string_slicing();
	an_exercise_in_utf8();
	string_collection();
}

fn basic_strings() {
	let text = "hello world"; // a string slice
	let s = text.to_string(); // allocated strings
	println!("str '{}'", text);
	println!("str '{}'", s);
}

fn push_pop_strings() {
	let mut s = String::new();
	s.push('H');
	s.push_str("ello");
	s.push(' ');
	s += "World!"; // short hand of push_str
	s.pop();
	assert_eq!(s, "Hello World");
}

fn array_to_string() {
	let input = [1,2,3,4];
	let result = string_to_array(&input);
	println!("Array was {:?}, String is {}", input, result);
}

fn string_to_array(arr: &[i32]) -> String {
	let mut s = '['.to_string(); // allocated string

	for i in arr {
		s += &i.to_string();
		s.push(',');
	}
	s.pop(); // remove trailing comma
	s.push(']');
	s
}

fn string_slicing() {
	let text = "static";
	let string = "dynamic!".to_string();

	let text_slice = &text[1..];
	let string_slice = &string[2..4];
	println!("Text Slice: {} | String Slice : {}", text_slice, string_slice);
}

fn an_exercise_in_utf8() {
	let multi = "Hi! ¡Hola! привет!";
	for ch in multi.chars() {
		print!("'{}'", ch);
	}

	println!("");
	println!("len {}", multi.len()); // count length of string
	println!("count {}", multi.chars().count()); // count length of characters

	let maybe = multi.find('п');
	if maybe.is_some() {
		let hi = &multi[maybe.unwrap()..];
		println!("Russian hi {}", hi);
	}
}

fn string_collection() {
	let text1 = "the red fox jumps over the lazy dog";
	let words1: Vec<&str> = text1.split_whitespace().collect();
	println!("Words 1 ... {:?}", words1);

	let text2 = "the grumpy man shouts at the youngsters";
	let mut words2 = Vec::new();
	words2.extend(text2.split_whitespace());
	println!("Words 2 ... {:?}", words2);

	let stripped: String = text2.chars()
		.filter(|ch| ! ch.is_whitespace())
		.collect();
	println!("Stripped ... {:?}", stripped);
}

fn command_line_args() {
	command_line_args_string();
	command_line_args_vector();
	command_line_args_rusty();
}

fn command_line_args_string() {
	println!("---------------------------------");
	println!("Command line Arguments as Strings");
	println!("---------------------------------");
	let args = env::args();
	for arg in args {
		println!("'{}'", arg);
	}
	println!("----------------------");
}

fn command_line_args_vector() {
	println!("---------------------------------");
	println!("Command line Arguments as Vector");
	println!("---------------------------------");
	let args: Vec<String> = env::args().skip(1).collect();
	if args.len() > 1 {
		for arg in args {
			println!("'{}'", arg);
		}
	}
	println!("----------------------");
}

fn command_line_args_rusty() {
	println!("---------------------------------");
	println!("Command line Arguments Rusty-like");
	println!("---------------------------------");
	let first = env::args().nth(1).expect("Please supply an argument");
	let force_int: i32 = first.parse().expect("Not an integer!?");
	println!("{}", force_int);
	println!("----------------------");
}

fn matching() {
	multilingual_with_match();
}

fn multilingual_with_match() {
	let multi = "Hi! ¡Hola! привет!";
	match multi.find('r') {
		Some(idx) => {
			let hi = &multi[idx..];
			println!("Russian hi {}", hi);
		},
		None => println!("Couldn't find nuthin'")
	}

	// or a one liner
	if let Some(idx) = multi.find('п') {
		println!("Russian hi is {}", &multi[idx..]);
	}

	// match as switch
	let n = 3;
	let text = match n {
		0 => "zero",
		1 => "one",
		2 => "two",
		3...10 => "between 3 and 10",
		_ => "none",
	};
	println!("Match as a switch returns - {}", text);
}

fn read_from_file() {
	let fname = "text.txt";
	read_from_file_throw_away_errors(&fname);
	read_from_files_handle_all_errors(&fname);
	read_from_files_magic_question(&fname);
}

fn read_from_file_throw_away_errors(fname: &str) { // bad!
	// this function throws away errors and causes the programme to panic
	// *this is not ideal*
	let mut file = File::open(&fname).expect("Can't open the file");
	let mut text = String::new();
	file.read_to_string(&mut text).expect("Can't read the file");
	println!("File had {} bytes", text.len());
}

fn read_to_string(filename: &str) -> Result<String, io::Error> {
	let mut file = match File::open(&filename) {
		Ok(f) => f,
		Err(e) => return Err(e),
	};

	let mut text = String::new();
	match file.read_to_string(&mut text) {
		Ok(_) => Ok(text.to_string()),
		Err(e) => Err(e),
	}
}

fn read_to_string_question_return(filename: &str) -> io::Result<String> {
	let mut file = File::open(&filename)?; // will return this error
	let mut text = String::new();
	file.read_to_string(&mut text)?; // will return this error
	Ok(text)
}

fn read_from_files_handle_all_errors(file: &str) { // better
	let text = read_to_string(&file).expect("Bad file");
	println!("File had {} bytes", text.len());
}

fn read_from_files_magic_question(filename: &str) { // rust-y level!
	let text = read_to_string_question_return(&filename).expect("Bad file");
	println!("File had {} bytes", text.len());
}

fn good_or_bad(good: bool) -> Result<i32, String> {
	if good {
		Ok(42)
	} else {
		Err("Is not good".to_string())
	}
}

fn good_or_bad_call() {
	match good_or_bad(true) {
		Ok(e) => println!("Yay - the answer is {}", e),
		Err(t) => println!("No - got {}", t),
	};

	match good_or_bad(false) {
		Ok(e) => println!("Yay - the answer is {}", e),
		Err(t) => println!("No - got {}", t),
	};
}
