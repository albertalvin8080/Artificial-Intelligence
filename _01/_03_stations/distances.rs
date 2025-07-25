use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Station {
    E1 = 1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14,
}

impl Station {
    pub fn idx(self) -> usize {
        (self as usize) - 1
    }
}

impl std::str::FromStr for Station {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "E1" => Ok(Station::E1),
            "E2" => Ok(Station::E2),
            "E3" => Ok(Station::E3),
            "E4" => Ok(Station::E4),
            "E5" => Ok(Station::E5),
            "E6" => Ok(Station::E6),
            "E7" => Ok(Station::E7),
            "E8" => Ok(Station::E8),
            "E9" => Ok(Station::E9),
            "E10" => Ok(Station::E10),
            "E11" => Ok(Station::E11),
            "E12" => Ok(Station::E12),
            "E13" => Ok(Station::E13),
            "E14" => Ok(Station::E14),
            _ => Err(()),
        }
    }
}

pub const DISTANCES: [[u32; 14]; 14] = [
//  E1  E2  E3  E4  E5  E6  E7  E8  E9 E10 E11 E12 E13 E14
    [0, 11, 20, 27, 40, 43, 39, 28, 18, 10, 18, 30, 30, 32], // E1
    [11, 0,  9, 16, 29, 32, 28, 19, 11,  4, 17, 23, 21, 24], // E2
    [20, 9,  0,  7, 20, 22, 19, 15, 10, 11, 21, 21, 13, 18], // E3
    [27, 16, 7,  0, 13, 16, 12, 13, 13, 18, 26, 21, 11, 17], // E4
    [40, 29, 20, 13, 0,  3,  2, 21, 25, 31, 38, 27, 16, 20], // E5
    [43, 32, 22, 16, 3,  0,  4, 23, 28, 33, 41, 30, 17, 20], // E6
    [39, 28, 19, 12, 2,  4,  0, 22, 25, 29, 38, 28, 13, 17], // E7
    [28, 19, 15, 13, 21, 23, 22, 0,  9, 22, 18,  7, 25, 30], // E8
    [18, 11, 10, 13, 25, 28, 25, 9,  0, 13, 12, 12, 23, 28], // E9
    [10,  4, 11, 18, 31, 33, 29, 22, 13, 0, 20, 27, 20, 23], // E10
    [18, 17, 21, 26, 38, 41, 38, 18, 12, 20, 0, 15, 35, 39], // E11
    [30, 23, 21, 21, 27, 30, 28,  7, 12, 27, 15, 0, 31, 37], // E12
    [30, 21, 13, 11, 16, 17, 13, 25, 23, 20, 35, 31, 0,  5], // E13
    [32, 24, 18, 17, 20, 20, 17, 30, 28, 23, 39, 37, 5,  0], // E14
];

pub const REAL_DISTANCES: &[(Station, Station, u32)] = &[
    (Station::E1, Station::E2, 11),
    (Station::E2, Station::E3, 9),
    (Station::E2, Station::E9, 11),
    (Station::E2, Station::E10, 4),
    (Station::E3, Station::E4, 7),
    (Station::E3, Station::E9, 10),
    (Station::E3, Station::E13, 19),
    (Station::E4, Station::E5, 14),
    (Station::E4, Station::E8, 16),
    (Station::E4, Station::E13, 12),
    (Station::E5, Station::E6, 3),
    (Station::E5, Station::E7, 2),
    (Station::E5, Station::E8, 33),
    (Station::E8, Station::E9, 10),
    (Station::E8, Station::E12, 7),
    (Station::E9, Station::E11, 14),
    (Station::E13, Station::E14, 5),
];

#[derive(Debug)]
pub struct SubwayLine {
    pub name: &'static str,
    pub stations: Vec<Station>,
}

impl SubwayLine {
    pub fn new(name: &'static str, stations: Vec<Station>) -> Self {
        SubwayLine { name, stations }
    }
}

pub struct Edge {
    pub to: Station,
    pub line_name: &'static str,
    pub dist: u32,
}

pub struct SubwayNetwork {
    pub stations_with_adj: HashMap<Station, Vec<Edge>>,
}

#[derive(Clone)]
struct State {
    station: Station,
    line_name: Option<&'static str>,
    g: f64,
    f: f64,
}
impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SubwayNetwork {
    fn create_lines() -> Vec<SubwayLine> {
        // NOTE: The stations must be put in a direct line formation.
        vec![
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
        ]
    }

    pub fn new() -> Self {
        let mut adj: HashMap<Station, Vec<Edge>> = HashMap::new();
        let lines: Vec<SubwayLine> = Self::create_lines();

        for line in &lines {
            // windows(2) basically combines the previous station with the next inside 'line'
            for w in line.stations.windows(2) {
                let station_a: Station = w[0];
                let station_b: Station = w[1];

                // You use & or && depending on how deeply you destructure
                let distance = REAL_DISTANCES
                    .iter()
                    .find(|&&(x, y, _)| {
                        (x == station_a && y == station_b) || (x == station_b && y == station_a)
                    })
                    .map(|&(_, _, d)| d)
                    .unwrap();

                // or_default() for in case the entry doesn't yet exist
                adj.entry(station_a).or_default().push(Edge {
                    to: station_b,
                    line_name: line.name,
                    dist: distance,
                });
                adj.entry(station_b).or_default().push(Edge {
                    to: station_a,
                    line_name: line.name,
                    dist: distance,
                });
            }
        }
        SubwayNetwork {
            stations_with_adj: adj,
        }
    }

    fn heuristic(&self, a: Station, b: Station) -> f64 {
        let km = DISTANCES[a.idx()][b.idx()] as f64;
        km / 30.0 * 60.0
    }

    pub fn astar(
        &self,
        start: Station,
        goal: Station,
    ) -> Option<(Vec<(Station, &'static str)>, f64)> {
        let mut heap = BinaryHeap::new();

        type StationAndLineName = (Station, Option<&'static str>);
        let mut backtracking_map: HashMap<StationAndLineName, StationAndLineName> = HashMap::new();

        // NOTE: This guy here holds the best known cost (in MINUTES) from the start up to some state
        let mut gscores: HashMap<StationAndLineName, f64> = HashMap::new();

        let start_state = State {
            station: start,
            line_name: None,
            g: 0.0,
            f: self.heuristic(start, goal),
        };

        heap.push(start_state.clone());
        gscores.insert((start, None), 0.0);

        while let Some(cur_state) = heap.pop() {
            if cur_state.station == goal {
                let mut path: Vec<(Station, &'static str)> = Vec::new();
                let mut key: StationAndLineName = (cur_state.station, cur_state.line_name);

                while let Some(&prev) = backtracking_map.get(&key) {
                    path.push((key.0, key.1.unwrap_or("")));
                    key = prev;
                }

                path.push((start, ""));
                path.reverse();
                return Some((path, cur_state.g));
            }

            for edge in self
                .stations_with_adj
                .get(&cur_state.station)
                .unwrap_or(&Vec::new())
            {
                let travel = edge.dist as f64 / 30.0 * 60.0;
                let change = if cur_state.line_name.map_or(false, |l| l != edge.line_name)
                    && cur_state.line_name.is_some()
                {
                    5.0
                } else {
                    0.0
                };

                let tentative = cur_state.g + travel + change;

                let next_key: StationAndLineName = (edge.to, Some(edge.line_name));

                // poda de subótimos
                if tentative < *gscores.get(&next_key).unwrap_or(&f64::INFINITY) {
                    let prev_key: StationAndLineName = (cur_state.station, cur_state.line_name);
                    backtracking_map.insert(next_key, prev_key);
                    gscores.insert(next_key, tentative);

                    heap.push(State {
                        station: edge.to,
                        line_name: Some(edge.line_name),
                        g: tentative,
                        f: tentative + self.heuristic(edge.to, goal),
                    });
                }
            }
        }
        None
    }
}
