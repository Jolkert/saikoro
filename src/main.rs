fn main()
{
	let input = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
	match saikoro::evaluate(&input)
	{
		Ok(result) => println!("{result}"),
		Err(_) => eprintln!("Could not parse input expression \"{input}\"!"),
	}
}
