use std::{thread, time};

use distance::hamming;
use itertools::Itertools;
use crate::{utils::{set_functions, is_injective, inj_distance, self, cover, difference}, MetricSpace};

//absolute
pub fn dmax_abs(set:&Vec<String>) -> i32{
    let val = set.get(0).unwrap().len();
    return val as i32;
}

//relative: Return dmax between two poitns of set1 and set2
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

//relative: Return dmin between two poitns of set1 and set2
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

pub fn andreas_distance(set1:&Vec<String>,set2:&Vec<String>) -> i32{
    let mut sum = 0;
    set1.iter().for_each(|x_val|{
        let min = min_distance_point_set(&x_val, set2);
        sum = sum+min;
    });
    set2.iter().for_each(|x_val|{
        let min = min_distance_point_set(&x_val, set1);
        sum = sum+min;
    });

    return sum;
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

    let mut small:Vec<String> = Vec::new();
    let mut big:Vec<String> = Vec::new();
    if set1.len()<=set2.len(){
        small = set1.clone();
        big = set2.clone();
    }else {
        small = set2.clone();
        big = set1.clone();
    }

    let opt_inj_function = utils::get_opt_fun(set1, set2);
    let rem = utils::get_remain_set(set1, set2, &opt_inj_function);
    
    let n_rem = rem.as_ref().unwrap().len() as i32;
    let penalty_complex = dmax_abs(set1) * n_rem;
    dis = idis_simple(set1, set2) + penalty_complex;
    return dis;
}

pub fn double_i_dis(set1:&Vec<String>,set2:&Vec<String>) -> i32 {
    //DOUBLE INJECTION OVER THE REMAINING SET
    let mut dis = 0;
    let mut union = set1.clone();
    let y = set2.clone();
    union.extend(y);
    let mut small:Vec<String> = Vec::new();
    let mut big:Vec<String> = Vec::new();

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
            dis = idis_simple(set1, set2);
        },
        false => {
            let opt_inj_function = utils::get_opt_fun(rem.as_ref().unwrap(), &big);
            let mut penalty = 0;
            opt_inj_function.iter().for_each(|x|{
                let current_pen = dmax_abs(&union) as usize + hamming(&x.0, &x.1).unwrap() as usize;
                penalty = penalty + current_pen;
            });
            dis = idis_simple(&small, &big) +  penalty as i32;
        }
    };
    return dis;
}

pub fn idis_rec(set1:&Vec<String>,set2:&Vec<String>) -> i32 {
    let mut dis = 0;
    let mut union = set1.clone();
    let y = set2.clone();
    union.extend(y);
    let mut small:Vec<String> = Vec::new();
    let mut big:Vec<String> = Vec::new();

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
            // let penalty = 0 * n_rem;
            dis = idis_simple(set1, set2) + penalty;
            dis = dis+ idis_rec(&rem.as_ref().unwrap(), &small);
        }
    };
    return dis;
}

pub fn rep_dis(set1:&Vec<String>,set2:&Vec<String>) -> i32 {
   let mut dis = 0; 
   let mut small:Vec<String> = Vec::new();
   let mut big:Vec<String> = Vec::new();
   let mut union = set1.clone();
   let y = set2.clone();
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
   match set1.len()==set2.len() {
       true => {
           // println!("Iguales");
           dis = idis_simple(set1, set2);
       },
       false => {
           let n_rem = rem.as_ref().unwrap().len() as i32;
           let penalty = n_rem * idis_simple(&rem.unwrap(), &small);
           dis = idis_simple(set1, set2) + penalty;
           println!("IDIS = {}; Pen= {}",idis_simple(set1, set2),penalty );
       }
   };
   return dis;
   
}

pub fn g_average(set1:&Vec<String>, set2:&Vec<String>) -> f64  {
    let mut dis = 0 as f64;

    let mut setx = set1.clone();
    let mut sety = set2.clone();

    setx.retain(|x| !set2.contains(x));
    sety.retain(|x| !set1.contains(x));

    // g(X,Y)
    set1.iter().for_each(|x|{
        set2.iter().for_each(|y| {
            let h = hamming(x, y).unwrap();
            dis = dis + h as f64;
            println!("Distance {} - {}={}", x,y,h);
        })
    });

    let sup = dis as f64;
    let inf = (set1.len() * set2.len()) as f64;
    dis = sup/inf;
    return dis as f64;
}

pub fn average_dis(set1:&Vec<String>, set2:&Vec<String>) -> f64  {
    let dis;

    let mut setx = set1.clone();
    let mut sety = set2.clone();

    // X\Y
    setx.retain(|x| !set2.contains(x));
    sety.retain(|x| !set1.contains(x));

    let cardinalunion = (set1.len() + set2.len()) as f64;
    let shortsetx = setx.len() as f64;
    let shortsety = sety.len() as f64;


    let disfirst = (shortsety/cardinalunion) * g_average(set1, &sety);
    let dissecond = (shortsetx/cardinalunion) * g_average(set2, &setx);
    dis = disfirst + dissecond;
    return dis as f64;
}


// ZONE
// Cover(X, x) = the sum of the distances between x and every element of X
// Proxy(X, Y) = Min{ d(x,y) + Cover(X\Y, x) + Cover(Y\X, y) : x in X, y in Y }.
pub fn proxy(set1:&Vec<String>, set2:&Vec<String>) -> i32 {
    let mut result = 100 as i32;
    let mut d;
    let mut x = "0";
    let mut y = "0";

    for point_x in set1 {
        for point_y in set2 {
            let cover_x = cover(&difference(&set1, &set2), point_x.to_string()) as i32;
            let cover_y = cover(&difference(&set2, &set1), point_y.to_string()) as i32;
            d = cover_x + cover_y + hamming(point_x, point_y).unwrap() as i32;
            if d < result {
                result = d;
                x = point_x;
                y = point_y;
            }
        }
    }
    // println!("SetX {:?}", set1);
    // println!("SetY {:?}", set2);
    // println!("Result {}", result);
    // println!("x={:?} y={:?}", x , y);
    return result as i32;
}

pub fn copy_dis(set1:&Vec<String>, set2:&Vec<String>) -> i32 {
    let copy_x = utils::copy(set1);
    let copy_y = utils::copy(set2);

    match set1.len() ==0 && set2.len() ==0{
        true => return 0,
        false => {
            let opt_inj_function = utils::get_opt_fun(&copy_x, &copy_y);
            return inj_distance(&opt_inj_function);
        }
    }
}

pub fn realspace(set1:&Vec<String>, set2:&Vec<String>) -> i32{
    let space = utils::generate_val_combinations(set1.get(0).unwrap().len());
    let set1_vector = utils::to_vector_space(set1, &space);
    let set2_vector = utils::to_vector_space(set2, &space);
    let mut result = 0;
    let base: i32 = 2;
    set1_vector.iter().for_each(|x|{
        set2_vector.iter().for_each(|y|{
            let mut coordinates = 0;
            if y > x {
                coordinates = (y - x) as u32;
            } else{
                coordinates = (x - y) as u32;
            }
            result = base.pow(coordinates);
        });
    });

    return (result as f64).sqrt() as i32;
}

