use structopt::StructOpt;
use colored::*;

mod lookup_by_train;
mod utils;
mod search_station;
mod trip;

#[derive(Debug, StructOpt)]
#[structopt(name = "ritardatreno", about = "Un semplice client per il crate trenitalia.")]
struct Cli {
    /// Numero del treno (per la ricerca tramite codice del treno)
    #[structopt(short, long, default_value = "0")]
    number: u32,

    /// Stazione di partenza (necessaria se il codice del treno è condiviso tra più classi)
    #[structopt(short = "u", long, default_value = "")]
    number_from: String,

    /// Latitudine della posizione (per la ricerca della stazione più vicina)
    #[structopt(short = "l", long, default_value = "0.0")]
    latitude: f32,

    /// Longitudine della posizione (per la ricerca della stazione più vicina)
    #[structopt(short = "o", long, default_value = "0.0")]
    longitude: f32,

    /// Cerca per nome della stazione
    #[structopt(short = "s", long, default_value = "")]
    search_station: String,

    /// Cerca tragitto: punto di partenza (specificare un nome della stazione completo, vedere -s)
    #[structopt(short = "f", long, default_value = "")]
    from: String,

    /// Cerca tragitto: punto di arrivo (specificare un nome della stazione completo, vedere -s)
    #[structopt(short = "t", long, default_value = "")]
    to: String,

    /// Cerca tragitto: orario di partenza (specificare come YYYY-mm-DD HH:ii)
    #[structopt(short = "w", long, default_value = "")]
    when: String,
}

fn main() {
    let args: Cli = Cli::from_args();

    if args.number > 0 {
        lookup_by_train::lookup(args.number, args.from);
    } else if args.search_station != "" {
        search_station::by_name(args.search_station);
    } else if args.latitude > 0.0 {
        search_station::by_position(args.latitude, args.longitude);
    } else if args.from != "" {
        trip::calc(args.from, args.to, args.when);
    } else {
        println!("Nessuna azione richiesta. Digitare {} {} per ottenere l'help", "", "-h".bold())
    }
}
