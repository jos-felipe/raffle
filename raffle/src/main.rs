use anyhow::{Context, Result};
use csv::Reader;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		eprintln!("Uso: {} <arquivo_csv> <num_ganhadores>", args[0]);
		std::process::exit(1);
	}

	let csv_file = &args[1];
	let num_winners: usize = args[2].parse().context("Falha ao analisar o número de ganhadores")?;

	let participants = read_participants(csv_file)?;

	if participants.len() < num_winners {
		eprintln!("Não há participantes suficientes ({}), para selecionar {} ganhadores.", participants.len(), num_winners);
		std::process::exit(1);
	}

	let winners = select_winners(&participants, num_winners)?;

	display_winners(&winners);

	Ok(())
}

fn read_participants(csv_file: &str) -> Result<Vec<String>> {
	let file = File::open(Path::new(csv_file))
		.context(format!("Falha ao abrir o arquivo {}", csv_file))?;

	let mut reader = Reader::from_reader(file);

	let mut participants = Vec::new();

	for result in reader.records() {
		let record = result.context("Falha ao ler registro CSV")?;

		if let Some(name) = record.get(0) {
			participants.push(name.to_string());
		}
	}

	Ok(participants)
}

fn select_winners(participants: &[String], num_winners: usize) -> Result<Vec<String>> {
	let mut participants_copy = participants.to_vec();

	let mut rng = thread_rng();

	participants_copy.shuffle(&mut rng);

	let winners = participants_copy.into_iter().take(num_winners).collect();

	Ok(winners)
}

fn display_winners(winners: &[String]) {
	for (i, winner) in winners.iter().enumerate() {
		println!("{} - {}", i + 1, winner);
	}
}
