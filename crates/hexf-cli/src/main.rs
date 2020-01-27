use clap::{App, Arg, SubCommand};
use hexf::{parse_hexf32, parse_hexf64};

fn main() {
    let matches = App::new("hexf-cli")
        .version("0.1.0")
        .author("Augustin CHERON <cheron.augustin@gmail.com>")
        .about("Print literal hex float as decimal")
        .arg(
            Arg::with_name("f")
                .short("f")
                .help("Try to parse 32 bit float"),
        )
        .arg(
            Arg::with_name("d")
                .short("d")
                .help("Try to parse 64 bit float"),
        )
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    if matches.is_present("d") {
        let d = parse_hexf64(&input, false).expect("Unvalid 64 btis float");
        println!("{}", d);
        println!("{:#x}", d.to_bits());
        return;
    }

    let f = parse_hexf32(&input, false).expect("Invalid 32 bits float");
    println!("{}", f);
    println!("{:#x}", f.to_bits());
}
