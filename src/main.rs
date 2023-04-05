use distances::idis;
use rand::{seq::SliceRandom, thread_rng};
use itertools::Itertools;
use utils::is_far;
use rand::Rng;
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

fn debug(valutions:usize) {
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
    println!("D({:?},{:?})={} ",set1,set2,distances::idis(&set1,&set2));
    println!("D({:?},{:?})={} ",set2,set3,distances::idis(&set2,&set3));
    println!("D({:?},{:?})={} ",set1,set3,distances::idis(&set1,&set3));
    println!("{:?}U{:?} and {:?} are far? {:?}", set1,set2,set3, is_far(&set1, &set2, &set3));


    // idis(&set1,&set2);

}

fn powerset_limit<T: Clone>(set: &Vec<T>, limit:usize) -> Vec<Vec<T>> {
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



fn test_ax(n_val:usize, max:usize) {
    let valuations = generate_val_combinations(n_val);
    let mut pow = generate_random_subsets(&valuations, max);
    pow.retain(|x| !x.is_empty());
    let limit = pow.len();
    let mut rng = rand::thread_rng();
    let mut n = 0;
    println!("Size of Valuations: {}",n_val);
    println!("Number of valuations: {}", valuations.len());
    println!("N. Powerset: {}", limit);
    println!("Starting checking...");
    loop{
        n = n+1;
        let set1 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        let set2 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        let set3 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        let set4 = pow.get(rng.gen_range(0..limit-1)).unwrap();
        match axioms::ax7(&set1, &set2, &set3, &set4, distances::idis) {
           Some(_)  => {
               println!("CASE {}: X={:?}, Y={:?}, X'={:?},X''={:?},", n, set1,set2,set3,set4);
               break;
           },
           None => {
           }
        }
    }
}

fn is_metric(valuations:Vec<String>, limit:usize, dis:fn(&Vec<String>,&Vec<String>) -> i32) {
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
    println!("Sym finished");
    for x in pow.iter().combinations(3){
        let set1 = x.get(0).unwrap();
        let set2 = x.get(1).unwrap();
        let set3 = x.get(2).unwrap();
        match axioms::triangle_inequality(set1, set2,set3, dis) {
            Some(x) =>{
                tri = Some(x);
                break;
            },
            None => {
                // println!("NO {:?}{:?}{:?}",set1,set2,set3 );
            }
        }
    }
    println!("RESULT=====================");
    println!("Tri counterexample: {:?}", tri);
}

fn main() {
    // let combinations = generate_val_combinations(3);
    // is_metric(combinations, 50, idis);
    // let mut pow = generate_random_subsets(&combinations, 100);
    // pow.retain(|x| !x.is_empty());
    // println!("{:?}", combinations);
    // println!("Comb size = {}, Pow size={}", combinations.len(), pow.len());
    
    // let val1 = vec!["100000011111".to_string()];
    // let val2 = vec!["000000000000".to_string(), "100010011111".to_string(), "100001011111".to_string(),"100000111111".to_string()];
    // let val3 = vec!["110000000000".to_string(), "111000000000".to_string(),"111100000000".to_string()];
    // let val4 = vec!["010000000000".to_string(), "011000000000".to_string(),"011100000000".to_string()];
    // println!("D(X',Y)")
    //
    // let val1 = vec!["1000000111111111111".to_string(), "1000100111111111111".to_string(), "1000010111111111111".to_string(),"1000001111111111111".to_string()];
    // let val2 = vec!["0000000000001111111".to_string(), "1000100111111111111".to_string(), "1000010111111111111".to_string(),"1000001111111111111".to_string()];
    // let val3 = vec!["1100000000000000000".to_string(), "1110000000000000000".to_string(),"1111000000000000000".to_string()];
    // let val4 = vec!["0100000000000000000".to_string(), "0110000000000000000".to_string(),"0111000000000000000".to_string()];

    // let sat = axioms::ax7(&val1, &val2, &val3,&val4, distances::idis_rec);
    // println!("{:?}", sat);

    // let sat = axioms::ax7(&val1, &val2, &val3,&val4, distances::idis);
    // println!("{:?}", sat);

    // let dis = distances::idis_rec(&val1, &val3);

    // for x in pow.iter().combinations(4){
    //     let set1 = x.get(0).unwrap();
    //     let set2 = x.get(1).unwrap();
    //     let set3 = x.get(2).unwrap();
    //     let set4 = x.get(3).unwrap();
    //     axioms::ax1(set1, set2, set3, set4,idis);
    // }
    // sat_axiom(&pow, distances::idis);
    // debug(3);
    test_ax(15,500);
}
