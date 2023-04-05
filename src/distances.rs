use std::{thread, time};

use distance::hamming;
use itertools::Itertools;
use crate::utils::{set_functions, is_injective, inj_distance, self};

//absolute
pub fn dmax_abs(set:&Vec<String>) -> i32{
    let val = set.get(0).unwrap().len();
    return val as i32;
}

//relative
pub fn dmax_rel(set1:&Vec<String>,set2:&Vec<String>) -> i32{
    let mut max = 0;
    set1.iter().for_each(|x_val|{
        set2.iter().for_each(|y_val|{
            if hamming(x_val, y_val).unwrap() > max{
                max = hamming(x_val, y_val).unwrap();
            }
        });
    });

    return max as i32;
}

pub fn dmin_rel(set1:&Vec<String>,set2:&Vec<String>) ->i32{
    let mut min = dmax_rel(set1, set2) as usize;
    set1.iter().for_each(|x_val|{
        set2.iter().for_each(|y_val|{
            if hamming(x_val, y_val).unwrap() <= min{
                min = hamming(x_val, y_val).unwrap();
            }
        });
    });

    return min as i32;
}

pub fn min_distance_point_set(point: &String, set:&Vec<String>) -> i32{
    let mut ham = 0;
    if set.is_empty(){
        ham = point.len();
    }else{
        ham = hamming(&point, &set.get(0).unwrap()).unwrap();
    }
    set.iter().for_each(|x| {
        let curr = hamming(&point, &x).unwrap();
        // println!("Distance between {} and {}: {}", &point, &x, ham);
        if ham > curr {
            ham = curr
        }
    });
    return ham as i32;
}

pub fn sum_min_distance(set1:&Vec<String>,set2:&Vec<String>) -> i32{
    let mut distance = 0;
    set1.iter().for_each(|valuation|{
        distance = distance + min_distance_point_set(&valuation, &set2);
    });

    set2.iter().for_each(|valuation|{
        distance = distance + min_distance_point_set(&valuation, &set1);
    });

    return distance as i32;
}

pub fn idis_simple(set1:&Vec<String>,set2:&Vec<String>) -> i32 {
    let opt_inj_function = utils::get_opt_fun(set1, set2);
    return inj_distance(&opt_inj_function);
}

pub fn idis(set1:&Vec<String>,set2:&Vec<String>) -> i32 {
    let mut dis = 0;
    let mut union = set1.clone();
    let y = set2.clone();
    union.extend(y);

    let opt_inj_function = utils::get_opt_fun(set1, set2);
    let rem = utils::get_remain_set(set1, set2, &opt_inj_function);
    
    // in this case dmax is over the domain of set1\cup set2
    match rem {
        None => {
            dis = idis_simple(set1, set2);
        },
        Some(..) => {
            let n_rem = rem.as_ref().unwrap().len() as i32;
            let penalty = dmax_abs(&union) * n_rem;
            dis = idis_simple(set1, set2) + penalty;
        }
    };
    return dis;
}

pub fn idis_rec(set1:&Vec<String>,set2:&Vec<String>) -> i32 {
    let mut dis = 0;
    let mut union = set1.clone();
    let y = set2.clone();
    let mut small:Vec<String> = Vec::new();
    let mut big:Vec<String> = Vec::new();
    union.extend(y);

    let opt_inj_function = utils::get_opt_fun(set1, set2);
    let rem = utils::get_remain_set(set1, set2, &opt_inj_function);

    if set1.len()<=set2.len(){
        small = set1.clone();
        big = set2.clone();
    }else {
        small = set2.clone();
        big = set1.clone();
    }
    
    // in this case dmax is over the domain of set1\cup set2
    match set1.len()==set2.len() {
        true => {
            // println!("Iguales");
            dis = idis_simple(set1, set2);
        },
        false => {
            let n_rem = rem.as_ref().unwrap().len() as i32;
            let penalty = dmax_abs(&union) * n_rem;
            dis = idis_simple(set1, set2) + penalty;
            dis = dis+ idis_rec(&rem.as_ref().unwrap(), &small);
        }
    };
    return dis;
}
