use structopt::StructOpt;
use colored::*;
use std::num::NonZeroU32;

mod lookup_by_train;
mod utils;
mod search_station;
mod trip;

/// Un semplice client per il crate trenitalia.
#[derive(Debug, StructOpt)]
#[structopt(name = "ritardatreno")]
struct Cli {
    /// Numero del treno (per la ricerca tramite codice del treno)
    #[structopt(short, long)]
    number: Option<NonZeroU32>,

    /// Stazione di partenza (necessaria se il codice del treno è condiviso tra più classi)
    #[structopt(short = "u", long, default_value = "")]
    number_from: String,

    /// Latitudine della posizione (per la ricerca della stazione più vicina)
    #[structopt(short = "l", long)]
    latitude: Option<f32>,

    /// Longitudine della posizione (per la ricerca della stazione più vicina)
    #[structopt(short = "o", long)]
    longitude: Option<f32>,

    /// Cerca per nome della stazione
    #[structopt(short = "s", long)]
    search_station: Option<String>,

    /// Cerca tragitto: punto di partenza (specificare un nome della stazione completo, vedere -s)
    #[structopt(short = "f", long)]
    from: Option<String>,

    /// Cerca tragitto: punto di arrivo (specificare un nome della stazione completo, vedere -s)
    #[structopt(short = "t", long)]
    to: Option<String>,

    /// Cerca tragitto: orario di partenza (specificare come YYYY-mm-DD HH:ii)
    #[structopt(short = "w", long)]
    when: Option<String>,
}

fn main() {
    let args: Cli = Cli::from_args();

    if let Some(number) = args.number {
        lookup_by_train::lookup(number.get(), args.number_from);
    } else if let Some(search_station) = args.search_station {
        search_station::by_name(search_station);
    } else if let (Some(latitude), Some(longitude)) = (args.latitude, args.longitude) {
        search_station::by_position(latitude, longitude);
    } else if let (Some(from), Some(to), Some(when)) = (args.from, args.to, args.when) {
        trip::calc(from, to, when);
    } else {
        println!("Nessuna azione richiesta. Digitare {} {} per ottenere l'help", "", "-h".bold())
    }
}
