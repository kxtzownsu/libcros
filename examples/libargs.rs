use libcros::libargs::ArgCheck;

fn main() {
  let mut args: ArgCheck = ArgCheck::new();
  let verbose: bool = args.fbool("--verbose", "", "Enable debug messages");
  let foobar = args.fequals_str("--foobar", "-f", "Example flag");

  args.check_help();

  println!("Verbose logging enabled: {}", verbose);
  println!("Foobar flag value: {}",  foobar);
}