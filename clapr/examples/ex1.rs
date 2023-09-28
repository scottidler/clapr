use derive::Parser;

#[derive(Parser)]
#[command(name = "aka", about = "[a]lso [k]nown [a]s: an aliasing program")]
#[command(author = "Scott A. Idler <scott.a.idler@gmail.com>")]
#[command(config = "~/.config/ex1/ex1.yml")]
struct CLI {
    #[arg(short, long, help = "greeting")]
    greeting: String,

    #[arg(short, long, help = "name")]
    name: String,
}

fn main() {
    println!("Hello, world!");
}
