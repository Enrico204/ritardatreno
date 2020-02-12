use colored::*;
use trenitalia::Trenitalia;

pub fn by_name(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let ti = Trenitalia::new();
    let stations = ti.find_train_station_online(&name);

    if stations.is_none() {
        println!("Nessuna stazione trovata con il nome indicato!")
    } else {
        println!("Stazione trovata!\nCodice: {}\nNomi: {}", stations.unwrap().id, stations.unwrap().aliases.join(", ").bold())
    }
    Ok(())
}

pub fn by_position(latitude: f32, longitude: f32) -> Result<(), Box<dyn std::error::Error>> {
    let ti = Trenitalia::new();
    let stations = ti.nearest_station((latitude as f64, longitude as f64));

    println!("Stazione più vicina:\nCodice: {}\nNomi: {}", stations.id, stations.aliases.join(", "));
    Ok(())
}
