use colored::*;
use trenitalia::Trenitalia;

use chrono::{TimeZone, Datelike};

pub fn calc(from: String, to: String, when: String) {
    let ti = Trenitalia::new();

    let from_station = ti.find_train_station_online(&from);
    let to_station = ti.find_train_station_online(&to);

    if from_station.is_none() {
        println!("{}", "Stazione di partenza non trovata".red());
        return;
    } else if to_station.is_none() {
        println!("{}", "Stazione di arrivo non trovata".red());
        return;
    }

    let mut when_as_dt = chrono::Local::now();
    if when != "" {
        let mut datefmt = String::from("%Y-%m-%d %H:%M");
        if !when.contains(" ") {
            datefmt = format!("{}-{}-{} %H:%M", when_as_dt.year(), when_as_dt.month(), when_as_dt.day())
        }
        let when_parsed = chrono::Local.datetime_from_str(&when, &datefmt);
        if when_parsed.is_err() {
            println!("{}", "Formato della data/ora non corretto".red());
            return;
        }
        when_as_dt = when_parsed.unwrap()
    }

    let trip = ti.find_trips(from_station.unwrap(), to_station.unwrap(), &when_as_dt);

    if trip.len() == 0 {
        println!("Nessun percorso trovato!")
    } else {
        for t in trip {
            for i in t {
                let (departure, departure_datetime) = i.departure;
                let (arrival, arrival_datetime) = i.arrival;
                println!("{:10} - {} ({}) -> {} ({})",
                    i.train_number.to_string(),
                    departure.aliases.join(", "),
                    departure_datetime.format("%H:%M"),
                    arrival.aliases.join(", "),
                    arrival_datetime.format("%H:%M")
                )
            }
        }
    }
}
