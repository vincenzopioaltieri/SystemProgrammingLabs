//use std::env;

// "Contenitore" dove andiamo a leggere le scritture in stdinS
pub struct Config{
	pub nome_file: String, 
	pub head: usize,
}

// Obiettivo: leggere ciò che l'utente scrive da terminale
pub fn parse_input() -> Result<Config, String> {

	// Dichiaro il vettore di Stringhe in cui vado a leggere l'input da linea di comando
	let args: Vec<String> = std::env::args().collect();

	// Verifico se ci sono meno di 2 argomenti
	if args.len() < 2 {
		return Err("Errore: file mancante".to_string());
	}

	// Leggo il nome del file in una nuova variabile
	let nome_file = args[1].clone();

	// Dichiaro la variabile head (che può essere aggiornata in seguito=)
	let mut head = 10;

	// Verifico se ho più di 2 argomenti
	if args.len() > 2{
		if args[2] == "--head"{
			if args.len() < 4{
				return Err("Missing argument".to_string());
			}
			// args3 esiste, verifico sia un numero
			match args[3].parse::<usize>(){
				Ok(numero) => {
					head = numero;
				}
				Err(_) => {
					return Err("Format of the head not compatible".to_string());
				}
			}
		}
	}
	Ok(Config{
		nome_file,
		head,
	})
}