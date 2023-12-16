use std::{collections::HashSet, vec, net::ToSocketAddrs, usize};
use distance::hamming;
use itertools::Itertools;
use rand::{thread_rng, Rng};

use crate::distances::{dmax_rel, dmin_rel};


pub fn to_vector_space(set1:&Vec<String>, space: &Vec<String>) -> Vec<usize> {
    let mut x = space.clone();
    let mut rvector:Vec<usize> = Vec::new();
    x.sort();

    x.iter().for_each(|x| {
        let mut min_d = 10000;
        set1.iter().for_each(|y|{
            let distance = hamming(x, y).unwrap();
            if min_d > distance{
                min_d = distance
            }
        });
        rvector.push(min_d);
    });
    return rvector;
}

pub fn generate_val_combinations(length: usize) -> Vec<String> {
    (0..2usize.pow(length as u32))
        .map(|i| format!("{:0length$b}", i, length = length))
        .collect()
}


pub fn set_functions(set1:&Vec<String>, set2:&Vec<String>) -> Vec<Vec<(String, String)>> {
    // println!("X:{:?}, Y:{:?}", set1,set2);

    let mut result: Vec<Vec<(String, String)>> = Vec::new();
    let mut permuts: Vec<Vec<&String>> = Vec::new();
    let mut small: Vec<String> = Vec::new();
    let mut big:Vec<String> = Vec::new();

    if set1.len()<=set2.len(){
        small = set1.clone();
        big = set2.clone();
    }else {
        small = set2.clone();
        big = set1.clone();
    }

    for permut in big.iter().permutations(big.len()){
        permuts.push(permut);
    }

    permuts.iter().for_each(|x| {
        let mut function: Vec<(String, String)> = Vec::new();
        for (a, b) in small.iter().zip(x) {
            function.push((a.to_string(), b.to_string()));
        }
        result.push(function);
    });

    is_injective(&result[0]);
    get_remain_set(set1, set2, &result[0]);
    return result;
}

pub fn is_injective(function:&Vec<(String, String)>) -> bool {
    // let domain:Vec<String> = function.iter().map(|x| x.to_owned().0).collect();
    let image:Vec<String> = function.iter().map(|x| x.to_owned().1).collect();

    // Hashset.insert(x) return true if it is not in seen and false otherwise.
    let mut seen = HashSet::new();
    let is_injective = image.iter().all(|x| seen.insert(x)); 

    // DEBUG COMMENT
    // println!("Function: {:?} ", function);
    // print!("Domain: {:?}", domain);
    // println!(" Image: {:?}", image);
    // println!("Is injective? {}", is_injective);
    return is_injective;
}

pub fn get_remain_set(set1:&Vec<String>,set2:&Vec<String>, inj_function:&Vec<(String, String)>) -> Option<Vec<String>>{
    if !is_injective(inj_function){
        return None;
    }else {
        let mut big:Vec<String> = Vec::new();
        let image:Vec<String> = inj_function.iter().map(|x| x.to_owned().1).collect();

        if set1.len()<=set2.len(){
            big = set2.clone();
        }else {
            big = set1.clone();
        }

        // let mut seen: HashSet<String> = HashSet::from_iter(image);
        // big.retain(|k| !seen.insert(k.to_string()));
        // println!("Function: {:?} ", inj_function);
        let big_hashset:HashSet<String> = big.iter().cloned().collect();
        let image_hashset:HashSet<String> = image.iter().cloned().collect();
        return Some((&big_hashset - &image_hashset).iter().cloned().collect());
    }
}

pub fn get_opt_fun(set1:&Vec<String>,set2:&Vec<String>) -> Vec<(String, String)>{
    let set_inj_function = set_functions(set1, set2);
    let mut opt_inj_function:Vec<(String,String)> = Vec::new();
    opt_inj_function = set_inj_function.get(0).unwrap().to_vec();
    set_inj_function.iter().for_each(|x|{
        // println!("Func. {:?}, D {:?}", x, inj_distance(x) );
        if inj_distance(x) < inj_distance(&opt_inj_function){
            opt_inj_function = x.to_vec();
        }
    }
    );

    return opt_inj_function;
}

pub fn inj_distance(inj_function:&Vec<(String, String)>) -> i32 {
    let mut dis = 0;
    inj_function.iter().for_each(|x|{
        dis = dis + hamming(&x.0, &x.1).unwrap();
    });
    return dis as i32;
}

pub fn is_far(set1:&Vec<String>,set2:&Vec<String>, set3:&Vec<String>) -> bool{
    let max = dmax_rel(set1, set2);
    let mut union = set1.clone();
    let y = set2.clone();
    union.extend(y);
    
    // println!("{}",max);
    // println!("{}",dmin_rel(&union, set3));
    if  dmin_rel(&union, set3) > max{
        // println!("=");
        // println!("Dmax {}, Min {:?}-{:?} = {}",max, union,set3,dmin_rel(&union, set3) );
        return true;
    }
    return false;
}

pub fn remove_duplicates<T: Eq + std::hash::Hash>(vec: &mut Vec<T>){
  let set: HashSet<_> = vec.drain(..).collect();
  vec.extend(set.into_iter());
}

pub fn are_disjoint(set1:&Vec<String>,set2:&Vec<String>) -> bool{
    for x in set1.iter() {
        for y in set2.iter(){
            if x == y{
                return false;
            }
        }
    }
    return true;
}

pub fn cover(set1:&Vec<String>, point:String) -> f64  {
    let mut result = 0;
    match set1.len() {
       0 => return result as f64,
       _ => {
           set1.iter().for_each(|point_x| {
               let ham = hamming(&point_x.to_string(), &point.to_string()).unwrap();
               result = result + ham;
           });
       }
    }
    return result as f64;
}


pub fn difference(set1:&Vec<String>, set2:&Vec<String>) -> Vec<String>{
    let mut difference: Vec<String> = Vec::new();
    
    for num in set1 {
        if !set2.contains(&num) {
            difference.push(num.to_string());
        }
    }
    
    return difference;
}


fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn factorial(num: i32) -> i32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}


//We denote by Cop(X) a certain number of copies of X, i.e., Cop(X) = { (x, i) : x in X and i in |U|!/|X| }.
pub fn copy(set1:&Vec<String>) -> Vec<String> {
    let set_card = set1.len() as i32;
    let alpha_dom = set1.get(0).unwrap().len();
    let base:i32 = 2;
    let dom_card = base.pow(alpha_dom as u32);

    let fact_dom = factorial(dom_card);
    let n_copies = fact_dom / set_card;

    let mut copy_x = set1.clone();
    (1..n_copies).for_each(|_|{
        let mut clone = set1.clone();
        copy_x.append(&mut clone);
    });

    println!("set_card = {:?}", set_card);
    println!("dom_card = {:?}", dom_card);
    println!("fact_dom = {:?}", fact_dom);
    println!("n_copies = {:?}", n_copies);
    println!("set1 = {:?}", set1);
    println!("copy_x = {:?}", copy_x);

    return copy_x;
}
