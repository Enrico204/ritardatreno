use colored::*;
use trenitalia::Trenitalia;

use crate::utils;

pub fn lookup(train_number: u32, start_from: String) -> Result<(), Box<dyn std::error::Error>> {
    let ti = Trenitalia::new();
    let train_info = ti.train_info(train_number, start_from)?;

    if train_info.is_at_station {
        let stazione = train_info.current_station.aliases.join(", ");
        println!("Treno in stazione a {} con {} minuti di ritardo", stazione.bold(), train_info.current_delay)
    } else {
        println!("Treno in movimento, ultimo ritardo rilevato: {}", train_info.current_delay)
    }

    println!("Tragitto: ");
    for trip_stop in train_info.stops {
        println!("* {} ({})",
                 trip_stop.station.aliases.join(", ").bold(),
                 trip_stop.platform.cyan());
        print!("|    ");
        if !trip_stop.expected_arrival.is_none() {
            print!("{}", utils::format_real_time(trip_stop.expected_arrival, trip_stop.arrival));
        }
        if !trip_stop.expected_departure.is_none() {
            print!(" -> {}", utils::format_real_time(trip_stop.expected_departure, trip_stop.departure))
        }

        println!()
    }
    Ok(())
}
