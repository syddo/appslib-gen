use clap::{App, Arg};

#[derive(Clone, Debug)]
struct CliOpts {
    library_name: String
}

fn get_cli_options() -> CliOpts {
    let matches = App::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("name")
        .short("n")
        .long("name")
        .value_name("LIBRAY_NAME")
        .help("Unison Applications Library Name")
        .takes_value(true)
        .required(true))
    .get_matches();

    let library_name = matches.value_of("name").unwrap().to_owned();

    CliOpts { library_name }
}

fn main() {
    
    let user_cli_inputs = get_cli_options();
    
    println!("Unsion Library Name: {}", user_cli_inputs.library_name);
    
    println!("Done!");
}
