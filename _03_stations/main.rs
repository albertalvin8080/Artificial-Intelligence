mod distances;
use std::io;

use distances::*;

fn main() {
    // manual();
    matrix();
}

fn matrix() {
    let station_names = [
        "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "E10", "E11", "E12", "E13", "E14",
    ];

    let network = SubwayNetwork::new();

    println!();
    print!("{:>5} ", "");
    for col in station_names.iter() {
        print!("{:>6} ", col);
    }
    println!();

    let mut paths: Vec<Vec<(Station, &'static str)>> = Vec::new();

    for start in station_names.iter() {
        print!("{:>5} ", start);

        for goal in station_names.iter() {
            if let Some((_path, time)) =
                network.astar(start.parse().unwrap(), goal.parse().unwrap())
            {
                print!("{:>6.1} ", time);
                paths.push(_path);
            } else {
                print!("{:>6} ", "N/A");
            }
        }

        println!("{:>2}minutes", "");
    }

    // for path in paths.iter() {
    //     for (st, ln) in path.iter() {
    //         if ln.is_empty() {
    //             print!("{:?} -> ", st);
    //         } else {
    //             print!("{:?} (linha {}) -> ", st, ln);
    //         }
    //     }
    //     println!();
    // }
}

fn manual() {
    let network = SubwayNetwork::new();

    let (start, goal) = ask_input();

    if let Some((path, time)) = network.astar(start, goal) {
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
