use itertools::Itertools;
mod distances;
mod axioms;


struct MetricSpace<T> {
    domain: Vec<T>,
    distance: fn(Vec<T>,Vec<T>) -> i32
}

fn powerset<T: Clone>(set: &Vec<T>) -> Vec<Vec<T>> {
    let power_set = (0..2usize.pow(set.len() as u32)).map(|i| {
        set.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect();

    return power_set;
}

fn create_valuations(n:i32) -> Vec<String>{
    let characters = vec!["1", "0"];
   
    let combinations : Vec<_> = (2..n).fold(
        characters.iter().cartesian_product(characters.iter()).map(|(&a, &b)| a.to_owned() + b).collect(),
        |acc, _| acc.into_iter().cartesian_product(characters.iter()).map(|(a, b)| a.to_owned() + b).collect()
        );

    return combinations;
}

fn sat_axiom(powerset: &Vec<Vec<String>>, dis: fn(&Vec<String>,&Vec<String>) -> i32){
    // Identity 
    for x in powerset.iter().combinations(2){
        let set1 = x.get(0).unwrap();
        let set2 = x.get(1).unwrap();
        let id = axioms::id(set1, set2, dis);
        match id {
            Some(x)=> {
                println!("{:?}", x);
                break;
            },
            None => {} 
        }
    };
    // Triangle inequality
    for x in powerset.iter().combinations(3){
        let set1 = x.get(0).unwrap();
        let set2 = x.get(1).unwrap();
        let set3 = x.get(2).unwrap();
        let sat = axioms::triangle_inequality(set1, set2, set3, dis);
        match sat {
            Some(x)=> {
                println!("Triangle counterexample: {:?}", x);
                break;
            },
            None => {} 
        }
    }
}

fn main() {
    let valuations = create_valuations(2);
    let pow = powerset(&valuations);
    let set1 = pow.get(2).unwrap();
    let set2 = pow.get(5).unwrap();
    distances::set_functions(&set1,&set2);
    // let set3 = pow.get(200).unwrap();
    // let id = axioms::id(set1, set2, distances::sum_min_distance);
    // let sym = axioms::sym(set1, set2, distances::sum_min_distance);
    // let tri = axioms::triangle_inequality(set1, set2, set3, distances::sum_min_distance);

    // sat_axiom(&pow, distances::sum_min_distance);
    // println!("{}", id);
    // println!("{}", sym);
    // println!("{}", tri);
    // println!("1: {}, 2:{}, dis(1,2): {}", point1, point2, dis);
}
