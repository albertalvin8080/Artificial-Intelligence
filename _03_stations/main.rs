mod distances;
use std::io;

use distances::*;

fn main() {
    let stations_names = [
        "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "E10", "E11", "E12", "E13", "E14",
    ];

    let network = SubwayNetwork::new();

    for start_name in stations_names.iter() {
        for goal_name in stations_names.iter() {
            println!("{}", "-".repeat(50));

            if let Some((path, time)) =
                network.astar(start_name.parse().unwrap(), goal_name.parse().unwrap())
            {
                println!("origem: {}", start_name);
                println!("destin: {}", goal_name);
                println!("Caminho mais rápido:");
                for (st, ln) in path.iter() {
                    if ln.is_empty() {
                        println!("{}", format!("{:?}", st));
                    } else {
                        println!("{:?} (linha {})", st, ln);
                    }
                }
                println!("Tempo total: {:.1} minutos", time);
            } else {
                println!("Caminho não encontrado");
            }
        }
    }
}

fn main1() {
    let network = SubwayNetwork::new();

    let (start, goal) = ask_input();

    if let Some((path, time)) = network.astar(start, goal) {
        // println!("origem: {}", start);
        // println!("destin: {}", goal_name);
        println!("Caminho mais rápido:");
        for (st, ln) in path.iter() {
            if ln.is_empty() {
                println!("{}", format!("{:?}", st));
            } else {
                println!("{:?} (linha {})", st, ln);
            }
        }
        println!("Tempo total: {:.1} minutos", time);
    } else {
        println!("Caminho não encontrado");
    }
}

fn ask_input() -> (Station, Station) {
    let mut buf = String::new();

    println!("Origem (e.g. E1):");
    io::stdin().read_line(&mut buf).unwrap();
    let start: Station = buf.trim().parse().expect("Estação inválida");
    buf.clear();

    println!("Destino (e.g. E5):");
    io::stdin().read_line(&mut buf).unwrap();
    let goal: Station = buf.trim().parse().expect("Estação inválida");

    (start, goal)
}
