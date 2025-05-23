mod distances;
use std::io;

use distances::*;

fn main() {
    let lines = vec![
        SubwayLine::new(
            "yellow",
            vec![
                Station::E10,
                Station::E2,
                Station::E9,
                Station::E8,
                Station::E5,
                Station::E7,
            ],
        ),
        SubwayLine::new(
            "blue",
            vec![
                Station::E1,
                Station::E2,
                Station::E3,
                Station::E4,
                Station::E5,
                Station::E6,
            ],
        ),
        SubwayLine::new(
            "red",
            vec![Station::E11, Station::E9, Station::E3, Station::E13],
        ),
        SubwayLine::new(
            "green",
            vec![
                Station::E12,
                Station::E8,
                Station::E4,
                Station::E13,
                Station::E14,
            ],
        ),
    ];
    let network = SubwayNetwork::new(lines);

    let mut buf = String::new();

    println!("Origem (e.g. E1):");
    io::stdin().read_line(&mut buf).unwrap();
    let start: Station = buf.trim().parse().expect("Estação inválida");
    buf.clear();
    
    println!("Destino (e.g. E5):");
    io::stdin().read_line(&mut buf).unwrap();
    let goal: Station = buf.trim().parse().expect("Estação inválida");

    if let Some((path, time)) = network.astar(start, goal) {
        println!("Caminho mais rápido:");
        for (st, ln) in path.iter() 
        {
            if ln.is_empty() 
            {
                println!("{}", format!("{:?}", st));
            } 
            else 
            {
                println!("{:?} (linha {})", st, ln);
            }
        }
        println!("Tempo total: {:.1} minutos", time);
    } else {
        println!("Caminho não encontrado");
    }
}
