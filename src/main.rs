use distances::idis;

use itertools::Itertools;
use utils::{is_far, remove_duplicates};
use rand::{Rng, seq::SliceRandom, thread_rng};
use visualitation::view;

use crate::distances::{proxy, andreas_distance, copy_dis};
mod axioms;
mod distances;
mod utils;
mod strict_implication;
mod visualitation;

struct MetricSpace<T> {
    domain: Vec<T>,
    distance: fn(Vec<T>, Vec<T>) -> i32,
}

fn powerset<T: Clone>(set: &Vec<T>) -> Vec<Vec<T>> {
    let power_set = (0..2usize.pow(set.len() as u32))
        .map(|i| {
            set.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect();

    return power_set;
}

fn debug(valutions: usize) {
    let valuations = generate_val_combinations(valutions);
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
    println!(
        "D({:?},{:?})={} ",
        set1,
        set2,
        distances::idis(&set1, &set2)
    );
    println!(
        "D({:?},{:?})={} ",
        set2,
        set3,
        distances::idis(&set2, &set3)
    );
    println!(
        "D({:?},{:?})={} ",
        set1,
        set3,
        distances::idis(&set1, &set3)
    );
    println!(
        "{:?}U{:?} and {:?} are far? {:?}",
        set1,
        set2,
        set3,
        is_far(&set1, &set2, &set3)
    );

    // idis(&set1,&set2);
}

fn powerset_limit<T: Clone>(set: &Vec<T>, limit: usize) -> Vec<Vec<T>> {
    let mut powerset: Vec<Vec<T>> = Vec::new();
    powerset.push(Vec::new());
    for elem in set.iter() {
        let mut new_subsets: Vec<Vec<T>> = Vec::new();
        for subset in powerset.iter() {
            let mut new_subset = subset.clone();
            new_subset.push(elem.clone());
            new_subsets.push(new_subset);
        }
        powerset.extend(new_subsets);
        if powerset.len() >= limit {
            break;
        }
    }
    return powerset;
}

fn generate_random_subsets(vec: &Vec<String>, n: usize) -> Vec<Vec<String>> {
    // NOT OPTIMIZE. Can repeat subsets
    // counter increases. It also avoids to return the emptyset.
    let mut rng = thread_rng();
    let mut random_subsets: Vec<Vec<String>> = Vec::new();
    let mut indices: Vec<usize> = (0..vec.len()).collect();
    for _ in 0..n {
        indices.shuffle(&mut rng);
        let subset: Vec<String> = indices
            .iter()
            .take(rng.gen_range(0..vec.len() + 1))
            .map(|&i| vec[i].clone())
            .collect();
        // if !random_subsets.contains(&subset){
        random_subsets.push(subset);
        // }
    }
    return random_subsets;
}

fn generate_val_combinations(length: usize) -> Vec<String> {
    //Limit cases
    // return (0..limit)
    //     .map(|i| format!("{:0length$b}", i, length = length))
    //     .collect();
    // All cases
    (0..2usize.pow(length as u32))
        .map(|i| format!("{:0length$b}", i, length = length))
        .collect()
}

fn counter_example() -> bool {
    let dmax = 20;
    let valuations = generate_val_combinations(dmax);
    let set_x: Vec<String> = valuations
        .choose_multiple(&mut rand::thread_rng(), 3)
        .map(|x| x.to_string())
        .collect();

    let mut set_y: Vec<String> = valuations
        .choose_multiple(&mut rand::thread_rng(), 6)
        .map(|x| x.to_string())
        .collect();

    let mut set_z: Vec<String> = Vec::new();
    set_z.push(
        valuations
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string(),
    );

    println!("{:?}", set_x);
    println!("{:?}", set_y);
    println!("{:?}", set_z);

    if set_x.iter().any(|e| set_y.contains(e)) {
        println!(" X is contained in Y");
        return false;
    } else if set_z.iter().any(|e| set_y.contains(e)) {
        println!(" Z is contained in Y");
        return false;
    } else {
        // Just penalty bc X=Z
        let result_xy = set_y.len() * dmax;

        //Join Y and z.
        set_y.append(&mut set_z);
        let result_xz = idis(&set_y, &set_x);
        println!("R ==================");
        println!("Result D(X,YUZ)={}", result_xy);
        println!("Result D(X,YUz)={}", result_xz);
        if result_xz >= result_xy as i32 {
            return true;
        } else {
            return false;
        }
    }
}

fn test_ax(n_val: usize, max: usize) {
    let valuations = generate_val_combinations(n_val);
    let mut pow = generate_random_subsets(&valuations, max);
    pow.retain(|x| !x.is_empty());
    let limit = pow.len();
    let mut rng = rand::thread_rng();
    let mut n = 0;
    println!("Size of Valuations: {}", n_val);
    println!("Number of valuations: {}", valuations.len());
    println!("N. Powerset: {}", limit);
    println!("Starting checking...");
    loop {
        n = n + 1;
        let set1 = pow.get(rng.gen_range(0..limit - 1)).unwrap();
        let set2 = pow.get(rng.gen_range(0..limit - 1)).unwrap();
        let set3 = pow.get(rng.gen_range(0..limit - 1)).unwrap();
        let set4 = pow.get(rng.gen_range(0..limit - 1)).unwrap();
        match axioms::ax7(&set1, &set2, &set3, &set4, distances::idis) {
            Some(_) => {
                println!(
                    "CASE {}: X={:?}, Y={:?}, X'={:?},X''={:?},",
                    n, set1, set2, set3, set4
                );
                break;
            }
            None => {}
        }
    }
}

fn is_metric(valuations: Vec<String>, limit: usize, dis: fn(&Vec<String>, &Vec<String>) -> i32) {
    let mut pow = generate_random_subsets(&valuations, limit);
    pow.retain(|x| !x.is_empty());

    println!("finding a counterexample..");
    // let mut id = None;
    // let mut sym = None;
    let mut tri = None;
    //
    // //Test identity
    // for x in pow.clone() {
    //     match axioms::id(&x, dis) {
    //         Some(x) =>{
    //             id = Some(x);
    //             break;
    //         },
    //         None => {},
    //     }
    // }
    // println!("Id finished");
    //
    // for x in pow.iter().combinations(2){
    //     let set1 = x.get(0).unwrap();
    //     let set2 = x.get(1).unwrap();
    //     match axioms::sym(set1, set2, dis) {
    //         Some(x) =>{
    //             sym = Some(x);
    //             break;
    //         },
    //         None => {},
    //     }
    // }
    //
    // println!("Sym finished");
    for x in pow.iter().combinations(3) {
        let set1 = x.get(0).unwrap();
        let set2 = x.get(1).unwrap();
        let set3 = x.get(2).unwrap();
        match axioms::triangle_inequality(set1, set2, set3, dis) {
            Some(x) => {
                tri = Some(x);
                break;
            }
            None => {
                println!("NO {:?}{:?}{:?}",set1,set2,set3 );
            }
        }
    }
    println!("RESULT=====================");
    println!("Tri counterexample: {:?}", tri);
}

fn test_dis(){
    let combinations = generate_val_combinations(2);
    let mut pow = generate_random_subsets(&combinations, 10);
    remove_duplicates(&mut pow);
    let metric_set:Vec<_> =  pow.iter().filter(|x| x.len() > 0).collect();
    let mut counter = 0;
    for x in metric_set.iter().combinations(3) {
        println!("========= {}", counter);
        counter +=1;
        let set1 = x.get(0).unwrap();
        let set2 = x.get(1).unwrap();
        let set3 = x.get(2).unwrap();
        println!("set1 = {:?}", set1);
        println!("set2 = {:?}", set2);
        println!("set3 = {:?}", set3);
        // let result = axioms::ax6_2(&set1, &set2, &set3, &set4, idis);
        let result = axioms::triangle_inequality(&set1, &set2, &set3, copy_dis);
        match result {
            Some(_) => {
                println!("counterexample: {:?}", result);
                return
            }, 
            None => continue
        };
    }
}

fn main() {
    // test_dis();
    let combinations = generate_val_combinations(5);
    let mut pow = generate_random_subsets(&combinations, 100000);
    remove_duplicates(&mut pow);
    let metric_set:Vec<_> =  pow.iter().filter(|x| x.len() > 0).collect();
    let mut counter = 0;
    for x in metric_set.iter().combinations(3) {
        println!("Case {}", counter);
        counter = counter + 1;
        let set1 = x.get(0).unwrap();
        let set2 = x.get(1).unwrap();
        let set3 = x.get(2).unwrap();
        println!("Set1 {:?}", set1);
        println!("Set2 {:?}", set2);
        println!("Set3 {:?}", set3);
        let result = axioms::triangle_inequality(&set1, &set2, &set3, distances::realspace);
        match result {
            Some(_) => {
                println!("counterexample: {:?}", result);
                return
            }, 
            None => continue
        };
    }
}
