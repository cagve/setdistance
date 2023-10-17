use std::{collections::HashSet, vec};
use distance::hamming;
use itertools::Itertools;
use rand::{thread_rng, Rng};

use crate::distances::{dmax_rel, dmin_rel};



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
        println!("Is not an injective function");
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

