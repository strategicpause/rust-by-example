use byte_unit::Byte;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    required_val: u64,
    #[arg(short, long)]
    optional_val: Option<u64>,
    #[arg(short, long)]
    with_bool: bool,
    #[arg(short, long)]
    size_val: Option<String>
}

fn main() {
    let args = Args::parse();

    println!("Required: {}", args.required_val);

    if let Some(val) = args.optional_val {
        println!("Optional: {}", val);
    }

    println!("Bool val: {}", args.with_bool);

    if let Some(val) = args.size_val {
        let bytes = Byte::parse_str(val, true).unwrap();
        println!("Bytes: {}", bytes.as_u64());
    }
}
