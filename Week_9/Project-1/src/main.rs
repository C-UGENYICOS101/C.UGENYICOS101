use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    let mut file = File::create("nigerian_breweries.txt").expect("Cannot create file");

    writeln!(file, "NIGERIAN BREWERIES PRODUCT CATEGORIES\n").unwrap();
    writeln!(file, "LAGER:").unwrap();
    for item in &lager {
        writeln!(file, "- {}", item).unwrap();
    }

    writeln!(file, "\nSTOUT:").unwrap();
    for item in &stout {
        writeln!(file, "- {}", item).unwrap();
    }

    writeln!(file, "\nNON-ALCOHOLIC:").unwrap();
    for item in &non_alcoholic {
        writeln!(file, "- {}", item).unwrap();
    }

    println!("File created: nigerian_breweries.txt");
}
