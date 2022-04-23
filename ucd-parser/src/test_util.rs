#[allow(unused)]
macro_rules! assert_panic {
	($message:literal in $code:expr) => {{
		let input = stringify!($code);
		let expected_message = $message;
		let prev_hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(|_| {}));
		let result = std::panic::catch_unwind(|| {
			$code;
		});
		std::panic::set_hook(prev_hook);
		let err = result
			.err()
			.expect(&format!("in `{}`: expected a panic", input));
		let actual_message = if err.is::<&str>() {
			err.downcast::<&str>().unwrap().to_string()
		} else if err.is::<String>() {
			err.downcast::<String>().unwrap().to_string()
		} else {
			panic!("in `{}`: panic is not a string", input);
		};
		if !actual_message.contains(expected_message) {
			panic!(
				"in `{}`: expected panic with `{}`, but it was `{}`",
				input, expected_message, actual_message
			);
		}
	}};
}
