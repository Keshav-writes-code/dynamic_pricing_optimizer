use clap::Parser;

#[derive(Parser)]
#[command( version, about, long_about = None )]
struct Args {
    #[arg(short, long, default_value_t = 10000)]
    count: usize,
}

fn main() {
    let args = Args::parse();
    data_gen::gen_data(args.count);
    training::training();
}
