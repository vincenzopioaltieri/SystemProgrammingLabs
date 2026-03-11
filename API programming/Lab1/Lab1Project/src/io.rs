use std::fs::File;	// Per vedere file(?)
use std::io::{BufRead, BufReader};	// Per gestire lettura
use std::process::exit;		// Per gestire errore in lettura

// File in cui vado a leggere tutte le righe del file in input e,
// dopo aver controllato lunghezza, stampo

pub fn process_file(_file_path: String, head: usize) { // Puntatore punta a main (?) e head mi dà la grandezza (di cosa?)

	
	let file = match File::open("data.csv"){ // Funzione per aprire il file in sicurezza
		Ok(file) => file,			// Gestione lettura corretta
		Err(why) => { eprintln!("Couldn't open file: {}", why); exit(1);}, // Gestione errore in lettura
	};

	// Creo vettore vuoto e mutabile per poter inserire le righe che leggo
	let mut saved_lines: Vec<String> = Vec::new();

	// for per inserire tutte le righe
	for (line_number, result) in BufReader::new(file).lines().enumerate(){ // Lettura di tutte le righe nel file
		match result{		// Gestione di lettura per ogni riga o errore generato
			Ok(line) => {
				//println!("{:4}: {}", line_number+1, line); // Stampo la linea (?)
				saved_lines.push(line);
			},
			Err(why) => { eprintln!("Error in line {}: {}", line_number+1, why);
			exit(2);}
		}
	}

	// Stampo il numero totali delle righe salvate e head (le N righe che voglio stampare inizialmente)
	let total_rows = saved_lines.len();
	println!("rows: {}", total_rows);
	println!("head ({}):", head);

	// Trovo il limite da imporre per non andare in errore
	let limit = std::cmp::min(total_rows, head);

	// Stampiamo tutte le righe
	for i in 0..limit{
		println!("{}", saved_lines[i]);
	}
}
