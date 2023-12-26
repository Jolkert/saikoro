use saikoro::evaluation;
fn main()
{
	let input = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

	match evaluation::eval_with_random(&input, &mut rand::thread_rng())
	{
		Ok(result) => println!("{result}"),
		Err(_) => eprintln!("Could not parse input expression \"{input}\"!"),
	}
}
