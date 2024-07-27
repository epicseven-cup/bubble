use std::env;
use std::path;
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

fn _parse_path_yml() -> Vec<String> {
	let mut paths:Vec<String> = Vec::new();
	let contents = fs::read_to_string("config/paths.yml").expect("Fail to read path.yml");
	
	return paths; 
}

fn main() {
	let args: Vec<String> = env::args().collect();
	// The fix size of the args are: ###
	let config:Config = Config::build(&args).unwrap_or_else(|err| {
		println!("ERROR: Fail to parse the arguments -> {err}");
		process::exit(1);
	});
	println!("Hello, world!");

	// This is a test selection where I will check the inputs
	let url:String = config.url;
	println!("The url is {url}");

	// Parse path.yml for health check
	let paths:Vec<String> = Vec::new();
	loop {
		// Do stuff
		println!("Running");

	}
}


