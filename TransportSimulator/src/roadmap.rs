use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use itertools::izip;


pub type Distance = i64;

#[derive(Default,Debug)]
pub struct PlainRoadMap {
    roads: Vec<((i32, i32), (i32, i32))>,
    weights: Vec<Distance>,
}

impl PlainRoadMap {
    pub fn new() -> PlainRoadMap {
        PlainRoadMap {
            roads: vec![],
            weights: vec![],
        }
    }
}

#[derive(Default,Debug)]
pub struct RegulatedRoad {
    pub id: i32, // index in RegulatedRoadMap
    pub weight: Distance,
    pub outbounds: HashSet<i32>, // An edge from a to every Road in `outbounds` has weight `weight`
    pub inbounds: HashSet<i32>,
}

#[derive(Default,Debug)]
pub struct RegulatedRoadMap {
    pub roads: HashMap<i32, RegulatedRoad>,
    pub lookup: Option<HashMap<(i32, i32), i32>>,
    pub shortest_path: HashMap<(i32, i32), ShortestPath>,
}

#[derive(Debug)]
pub struct ShortestPath {
    pub next: HashSet<i32>,
    pub dist: Distance,
}

impl RegulatedRoadMap {
    pub fn shortest_path(&mut self) {
        for (_, e) in self.roads.iter() {
            for n in e.outbounds.iter() {
                self.shortest_path.insert((e.id, n.to_owned()), ShortestPath {
                    next: [n.to_owned()].iter().cloned().collect(),
                    dist: e.weight.to_owned(),
                });
            }
        }
        for (k, _) in self.roads.iter() {
            for(i, _) in self.roads.iter() {
                for(j, _) in self.roads.iter() {
                    let a = self.shortest_path.get_key_value(&(i.to_owned(), k.to_owned()));
                    let b = self.shortest_path.get_key_value(&(k.to_owned(), j.to_owned()));
                    if a.is_none() || b.is_none() {
                        continue;
                    }
                    let new_dist = a.unwrap().1.dist + b.unwrap().1.dist;
                    let updated = match self.shortest_path.entry((i.to_owned(), j.to_owned())) {
                        Vacant(entry) => Some(ShortestPath {
                            next: self.shortest_path.get(&(i.to_owned(), k.to_owned())).unwrap().next.clone(),
                            dist: new_dist,
                        }),
                        Occupied(e) =>
                            match new_dist.cmp(&e.get().dist) {
                                std::cmp::Ordering::Less => Some(ShortestPath {
                                    next: self.shortest_path.get(&(i.to_owned(), k.to_owned())).unwrap().next.clone(),
                                    dist: new_dist,
                                }),
                                std::cmp::Ordering::Greater => Some(e.remove()),
                                std::cmp::Ordering::Equal => {
                                    let mut e = e.remove();
                                    e.next.extend(self.shortest_path.get(&(i.to_owned(), k.to_owned())).unwrap().next.iter());
                                    Some(e)
                                },
                            },
                    };
                    if updated.is_some() {
                        self.shortest_path.insert((i.to_owned(), j.to_owned()), updated.unwrap());
                    }
                }
            }
        }
    }
}


pub fn manhattan(m: i32, n: i32) -> PlainRoadMap {
    let mut res: PlainRoadMap = PlainRoadMap::new();
    for i in 0..m {
        for j in 0..n {
            if j - 1 > 0 {
                res.roads.push(((i, j), (i, j - 1)));
                res.weights.push(100);
            }
            if j + 1 < n {
                res.roads.push(((i, j), (i, j + 1)));
                res.weights.push(100);
            }
            if i - 1 > 0 {
                res.roads.push(((i, j), (i - 1, j)));
                res.weights.push(100);
            }
            if i + 1 < m {
                res.roads.push(((i, j), (i + 1, j)));
                res.weights.push(100);
            }
        }
    }
    res
}

pub fn regularize_bidirectional(m: &PlainRoadMap) -> RegulatedRoadMap {
    let mut res: RegulatedRoadMap = Default::default();
    let mut gen = 1..;
    let mut lookup: HashMap<(i32, i32), i32> = Default::default();
    for (((x1, y1), (x2, y2)), w) in izip!(m.roads.iter(), m.weights.iter()) {
        if !lookup.contains_key(&(x1.to_owned(), y1.to_owned())) {
            let index = gen.next().unwrap();
            res.roads.insert(index.to_owned(), RegulatedRoad{
                id: index.to_owned(),
                weight: w.to_owned(),
                outbounds: Default::default(),
                inbounds: Default::default(),
            });
            lookup.insert((x1.to_owned(), y1.to_owned()),index);
        }
        if !lookup.contains_key(&(x2.to_owned(), y2.to_owned())) {
            let index = gen.next().unwrap();
            println!("generate {}", index.to_owned());
            res.roads.insert(index.to_owned(), RegulatedRoad{
                id: index.to_owned(),
                weight: w.to_owned(),
                outbounds: Default::default(),
                inbounds: Default::default(),
            });
            lookup.insert((x2.to_owned(), y2.to_owned()),index);
        }
    }
    for ((x, y), index) in lookup.iter() {
        println!("{} {} is {}", x, y, index)
    }
    for ((x1, y1), (x2, y2)) in m.roads.iter() {
        let index1 = lookup.get(&(x1.to_owned(), y1.to_owned())).unwrap();
        let index2 = lookup.get(&(x2.to_owned(), y2.to_owned())).unwrap();
        res.roads.get_mut(index1).unwrap().outbounds.insert(index2.to_owned());
        res.roads.get_mut(index2).unwrap().inbounds.insert(index1.to_owned());
        res.roads.get_mut(index2).unwrap().outbounds.insert(index1.to_owned());
        res.roads.get_mut(index1).unwrap().inbounds.insert(index2.to_owned());
    }

    res.lookup = Some(lookup);
    res.shortest_path();
    res
}


