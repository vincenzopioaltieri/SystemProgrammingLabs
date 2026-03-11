mod cli;
mod io;         

//std::env::args()

fn main() {
    println!("Hello, world!");
    println!("Progetto impostato");

    // Leggo da stdin
    match cli::parse_input() {
        Ok(config) => {
            io::process_file(config.nome_file, config.head);
        },
        Err(errore) => {
            eprintln!("{}", errore);
            std::process::exit(1);
        }
    }
}
