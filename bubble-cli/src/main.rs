use std::env;
use std::process;


const ARG_SIZE:usize = 2;

struct Config {
	url: String
}


impl Config {
	fn build(args: &[String]) -> Result<Config, &'static str>{
		if args.len() < ARG_SIZE {
			return Err("The amount of arguments are incorrect");
		}
		let url: String = args[1].clone();
		Ok(Config {url})
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	// The fix size of the args are: ###
	let config:Config = Config::build(&args).unwrap_or_else(|err| {
		println!("ERROR: Fail to parse the arguments -> {err}");
		process::exit(1);
	});
	println!("Hello, world!");
}


