use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = vec![
        "Ajibogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akambi",
        "Osazuwa Faith Etiyeye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South East",
        "South East",
    ];

    let mut file = File::create("efcc_output.txt").expect("Cannot create file");

    writeln!(file, "EFCC MERGED MINISTERIAL DATA\n").unwrap();

    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{} | {} | {}",
            commissioners[i], ministries[i], zones[i]
        )
        .unwrap();
    }

    println!("File created: efcc_output.txt");
}
