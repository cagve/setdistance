use axioms::id;
use itertools::Itertools;
use utils::is_far;
use rand::Rng;
use crate::distances::{idis, dmax_rel};
mod distances;
mod axioms;
mod utils;


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

fn debug(valutions:i32) {
    let valuations = create_valuations(valutions);
    let mut pow = powerset(&valuations);
    //remove empty set
    pow.retain(|x| !x.is_empty());
    println!("{:?}", pow);
   
    // let set1 = pow.get(0).unwrap();
    // let set2 = pow.get(1).unwrap();
    // let set3 = pow.get(9).unwrap();
    let set1 = vec!["100".to_string()];
    let set2 = vec!["110".to_string()];
    let set3 = vec!["111".to_string()];
    println!("D({:?},{:?})={} ",set1,set2,distances::idis(&set1,&set2));
    println!("D({:?},{:?})={} ",set2,set3,distances::idis(&set2,&set3));
    println!("D({:?},{:?})={} ",set1,set3,distances::idis(&set1,&set3));
    println!("{:?}U{:?} and {:?} are far? {:?}", set1,set2,set3, is_far(&set1, &set2, &set3));


    // idis(&set1,&set2);

}

fn powerset_limit<T: Clone>(set: &Vec<T>) -> Vec<Vec<T>> {
    let power_set = (0..1000).map(|i| {
        set.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element.clone())
            .collect()
    }).collect();

    return power_set;
}


fn test_ax(n_val:i32, max:i32) {
    let valuations = create_valuations(n_val);
    let mut pow = powerset_limit(&valuations);
    pow.retain(|x| !x.is_empty());
    let limit = pow.len();

    let mut rng = rand::thread_rng();
    let mut n = 0;
    loop{
        n = n+1;
        let set1 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        let set2 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        let set3 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        let set4 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        println!("CASE {}: X={:?}, Y={:?}, X'={:?},X''={:?},", n, set1,set2,set3,set4);
        if axioms::ax7(set1, set2, set3, set4, distances::idis) || n==max {
            println!("FINISHED");
            break;
        }

    }
}

fn main() {
    let valuations = create_valuations(3);
    let mut pow = powerset(&valuations);
    //remove empty set
    pow.retain(|x| !x.is_empty());


    // for x in pow.iter().combinations(4){
    //     let set1 = x.get(0).unwrap();
    //     let set2 = x.get(1).unwrap();
    //     let set3 = x.get(2).unwrap();
    //     let set4 = x.get(3).unwrap();
    //     axioms::ax1(set1, set2, set3, set4,idis);
    // }
    // sat_axiom(&pow, distances::idis);
    // debug(3);
    test_ax(6,100);
}
