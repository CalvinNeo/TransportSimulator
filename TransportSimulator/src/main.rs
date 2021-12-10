
mod roadmap;
mod vehicle;
mod simulator;
mod trafficlight;
mod runner;

fn main() {
    let plain = roadmap::manhattan(3, 3);
    let reg = roadmap::regularize_bidirectional(&plain);
    let lookup = &reg.lookup.unwrap();
    for i in 0..3 {
        for j in 0..3 {
            let obj = reg.roads.get(lookup.get(&(i.to_owned(), j.to_owned())).unwrap()).unwrap();
            println!("Node {} {} {:?}", i, j, obj);
        }
    }

    for i in 1..=9 {
        for j in 1..=9 {
            let p = reg.shortest_path.get(&(i, j));
            match p {
                Some(x) => println!("{} {} {:?}", i, j, p),
                None => println!("{} {} NIL", i, j),
            };
        }
    }
}
