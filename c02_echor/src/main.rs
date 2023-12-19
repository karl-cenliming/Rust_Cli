use clap::{App,Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Karl Cen <karl.cenliming@gmail.com>")
        .about("Rust version echo")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .short("n")
            .help("do not print newline")
            .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", matches);
}
