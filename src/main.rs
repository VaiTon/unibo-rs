use chrono::{Locale, NaiveDate};
use clap::{Parser, Subcommand};
use comfy_table::Cell;
use owo_colors::OwoColorize;
use unibo::models::Aula;
use unibo::Client;

fn main() {
    let args = Args::parse();
    let client = Client::new();

    match args.cmd {
        Commands::Cerca { aula } => cerca(&client, aula),
        Commands::Impegni { aula } => impegni(&client, aula),
        Commands::Lezioni { corso, data, anno } => lezioni(&client, corso, data, anno),
    }
}

fn lezioni(client: &unibo::Client, corso: String, data: Option<String>, anno: u8) {
    let data: NaiveDate = match data {
        Some(d) => chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").unwrap(),

        None => chrono::Local::now().date_naive(),
    };

    println!(
        "Lezioni di {} di {}:",
        corso,
        data.format_localized("%A %-d %B %Y", Locale::it_IT)
    );

    let lezioni = client.get_lezioni(&corso, anno, &data, &data).unwrap();

    if lezioni.is_empty() {
        eprintln!("{}", "Nessuna lezione trovata".red());
        return;
    }

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::UTF8_BORDERS_ONLY);
    table.set_header(vec!["Nome", "Docente", "Orario", "Aula"]);

    for lezione in lezioni {
        let aula = &lezione.aule[0].des_risorsa;
        table.add_row(vec![
            Cell::new(lezione.title).add_attribute(comfy_table::Attribute::Bold),
            Cell::new(lezione.docente),
            Cell::new(lezione.time),
            Cell::new(aula),
        ]);
    }
    println!("{table}")
}

fn impegni(client: &unibo::Client, nome: String) {
    let aule = client.get_aule().unwrap();
    let aule: Vec<&Aula> = aule
        .iter()
        .filter(|a| a.descrizione.eq_ignore_ascii_case(&nome))
        .collect();

    if aule.len() == 0 {
        eprintln!("Nessuna aula trovata");
        return;
    }

    let aula = aule.first().unwrap();

    let resp = client.get_impegni_calendario(vec![&aula.id]).unwrap();

    println!("{}", aula.descrizione.green());
    for impegno in resp {
        println!(
            "\t{}: {} - {}",
            impegno.nome, impegno.data_inizio, impegno.data_fine
        );
    }
}

fn cerca(client: &unibo::Client, nome: String) {
    let aule = client.get_aule().unwrap();

    let mut aule_ok = Vec::<Aula>::new();
    let mut exact_match: Option<Aula> = None;

    for aula in aule {
        let desc = &aula.descrizione.to_lowercase();
        if desc == &nome.to_lowercase() {
            exact_match = Some(aula);
        } else if (&desc).contains(&nome.to_lowercase()) {
            aule_ok.push(aula);
        }
    }

    if let Some(aula) = exact_match {
        print_aula(&aula);
        return;
    } else if aule_ok.len() == 1 {
        print_aula(&aule_ok[0]);
        return;
    } else if aule_ok.len() != 0 {
        print_aule_found(nome, &aule_ok);
        return;
    } else {
        eprintln!("Nessuna aula trovata");
        return;
    }
}

fn print_aula(aula: &Aula) {
    println!("{}", aula.descrizione.green());
    println!("\tCapienza:\t{}", aula.capienza.unwrap_or(0));
    println!("\tPostazioni:\t{}", aula.numero_postazioni.unwrap_or(0));
    println!("\tMetri quadri:\t{} mq", aula.metri_quadri);
    println!("\tPiano:\t\t{}", aula.piano.descrizione);
}

fn print_aule_found(search: String, aule: &Vec<Aula>) {
    let search_lower = search.to_lowercase();

    println!("{}", "Aule trovate:".bold());
    for aula in aule {
        let desc_lower = aula.descrizione.to_lowercase();

        let match_indexes = desc_lower.match_indices(&search_lower);
        let mut desc = aula.descrizione.to_string();

        for indexes in match_indexes {
            let start = indexes.0;
            let end = start + indexes.1.len();

            let replacement = &aula.descrizione[start..end].to_string().green().to_string();
            desc.replace_range(start..end, &replacement);
        }

        println!("\t{}", desc);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Cerca un'aula
    #[clap(visible_alias = "c")]
    Cerca {
        /// Nome dell'aula
        aula: String,
    },

    /// Mostra gli impegni di un'aula
    #[clap(visible_alias = "i")]
    Impegni {
        /// Nome dell'aula
        aula: String,
    },

    /// Mostra le lezioni di un corso
    #[clap(visible_alias = "l")]
    Lezioni {
        /// Nome del corso
        corso: String,
        // Anno del corso
        anno: u8,
        /// Data del giorno da cercare
        data: Option<String>,
    },
}
