fn main()
{
	let input = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
	match saikoro::evaluate(&input)
	{
		Ok(result) => println!("{result}"),
		Err(_) => eprintln!("Could not parse input expression \"{input}\"!"),
	};

	match saikoro::evaluate("8d6")
	{
		Ok(roll) => println!("Fireball deals {} fire damage", roll.value),
		Err(_) => println!("An error occured while parsing the input string!"),
	}
}
