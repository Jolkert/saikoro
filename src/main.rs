use saikoro::evaluation;
fn main()
{
	loop
	{
		println!("Enter string to parse:");
		let input = read_line();

		let result = evaluation::eval_with_random(&input, &mut rand::thread_rng())
			.expect("could not parse input!");
		println!(
			"Result: {} {:?}",
			result.value,
			result
				.rolls
				.iter()
				.flat_map(|it| it.rolls.iter().map(|r| format!(
					"{}{}",
					r.original_value,
					r.is_removed().then_some("*").unwrap_or_default()
				)))
				.collect::<Vec<String>>(),
		);

		println!("continue? (y/n)");
		if read_line().to_lowercase().starts_with('n')
		{
			break;
		}
		println!();
	}
}

fn read_line() -> String
{
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("Invalid user input!");
	input
}
