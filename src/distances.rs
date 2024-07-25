use std::{thread, time, cmp};
use std::collections::{HashMap, BTreeMap, HashSet};

use distance::hamming;
use itertools::Itertools;
use crate::{utils::{set_functions, is_injective, inj_distance, self, cover, difference}, MetricSpace};

//absolute
pub fn dmax_abs(set:&Vec<String>) -> i32{
    let val = set.get(0).unwrap().len();
    return val as i32;
}

pub fn symmetric_difference(set1:&Vec<String>,set2:&Vec<String>) -> f64{
    let set_a: HashSet<_> = set1.iter().cloned().collect();
    let set_b: HashSet<_> = set2.iter().cloned().collect();

    let diff_a = set_a.difference(&set_b);
    let diff_b = set_b.difference(&set_a);

    let symmetric_difference: HashSet<_> = diff_a.chain(diff_b).cloned().collect();
    return symmetric_difference.len() as f64;
}

//relative: Return dmax between two poitns of set1 and set2
pub fn dmax_rel(set1:&Vec<String>,set2:&Vec<String>) -> f64{
    let mut max = 0;
    set1.iter().for_each(|x_val|{
        set2.iter().for_each(|y_val|{
            if hamming(x_val, y_val).unwrap() > max{
                max = hamming(x_val, y_val).unwrap();
            }
        });
    });

    return max as f64;
}

//relative: Return dmin between two poitns of set1 and set2
pub fn dmin_rel(set1:&Vec<String>,set2:&Vec<String>) ->f64{
    let mut min = dmax_rel(set1, set2) as usize;
    set1.iter().for_each(|x_val|{
        set2.iter().for_each(|y_val|{
            if hamming(x_val, y_val).unwrap() <= min{
                min = hamming(x_val, y_val).unwrap();
            }
        });
    });

    return min as f64;
}

// The closest point of B to the given a\in A
pub fn inf_point_to_set(point:&str,set2:&Vec<String>) ->i32{
    let mut min = 1000000000;
    set2.iter().for_each(|y_val|{
        if hamming(point, y_val).unwrap() <= min{
            min = hamming(point, y_val).unwrap();
        }
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

pub fn idis(set1:&Vec<String>,set2:&Vec<String>) -> f64 {
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
    return dis as f64;
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

pub fn realspace(set1:&Vec<String>, set2:&Vec<String>) -> f64{
    let set1_vector = utils::to_vector_space(set1);
    let set2_vector = utils::to_vector_space(set2);
    let mut result = 0;
    let base: i32 = 2;
    //// let mut sum_vec = Vec::new(); //for debugging
    let mut index = 0;
    set1_vector.iter().for_each(|x|{
        let mut coordinates = 0;
        let y = set2_vector.get(index).unwrap();
        if y > x {
            coordinates = (y - x) as u32;
        } else{
            coordinates = (x - y) as u32;
        }
        result = result + base.pow(coordinates);
        //// sum_vec.push(coordinates); //for debugging
        index = index + 1
    });
    //// println!("sum_vec = {:?}", sum_vec); //for debugginf
    return (result as f64).sqrt();
}

pub fn pivot_distance(set1:&Vec<String>,set2:&Vec<String>) -> BTreeMap<String, usize>{
    let length = set1.clone().get(0).unwrap().len();
    let full_points:Vec<String> = (0..2usize.pow(length as u32))
        .map(|i| format!("{:0length$b}", i, length = length))
        .collect();
    let u_size = full_points.len();
    let mut result = BTreeMap::new();
    let mut distance = 0;
    if !utils::are_equals(set1, set2){
        for z in full_points{
            //Iter set1
            distance = 0;
            set1.iter().for_each(|x| {
                let ham =  hamming(x, &z).unwrap();
                // println!("d({},{}) = {}",x,z.clone(),ham);
                distance = distance + ham;
            });
            set2.iter().for_each(|x| {
                let ham =  hamming(x, &z).unwrap();
                // println!("d({},{}) = {}",x,z.clone(),ham);
                distance = distance + ham;
            });
            result.insert(z,distance);
        }
    }

    return result;
}

pub fn weight(set1:&Vec<String>) -> f64 {
    let length = set1.clone().get(0).unwrap().len();
    let full_points:Vec<String> = (0..2usize.pow(length as u32))
        .map(|i| format!("{:0length$b}", i, length = length))
        .collect();
    let u_size = full_points.len();
    let mut weight =  0; //DIstance from A to full metric space
    // println!("Weight of {:?}", set1); //debug
    set1.iter().for_each(|x| {
        full_points.iter().for_each(|y| {
            let ham =  hamming(x, y).unwrap();
            // println!("point {}, d={}", y, ham); //debug
            weight = weight + ham;
        })
    });
    return (weight as f64)/(u_size as f64) as f64;
}

pub fn full_pivot_distance(set1:&Vec<String>,set2:&Vec<String>) -> f64{
    let mut distance = 0 as f64;
    if !utils::are_equals(set1, set2){
        distance = weight(&set1)+weight(&set2);
    }
    return distance as f64;
}

pub fn mean_dis(set1:&Vec<String>,set2:&Vec<String>) -> f64{
    let mut distance = 0;
    let card1 = set1.len();
    let card2 = set2.len();

    set1.iter().for_each(|x| {
        set2.iter().for_each(|y|{
            distance = distance + hamming(x, y).unwrap();
        })
    });

    let card12 = card1*card2;
    let result:f64 = (distance as f64)/(card12 as f64);
    //println!("CASE X={:?} y={:?}", set1,set2); //debug
    //println!("Cardinality both {:?}", card12); //debug
    //println!("Link distance {:?}", distance); //debug
    //println!("Total distance {:?}/{:?}={:?}", distance,card12,result); //debug
    //println!(" "); //debug

    return result; 
}

pub fn fujita(set1:&Vec<String>,set2:&Vec<String>) -> f64 {
    let mut d1 = 0.0;
    let mut d2 = 0.0;
    let a = set1.clone();
    let b = set2.clone();
    let mut ab =  a.clone();
    ab.extend(b.clone());
    ab.sort();
    ab.dedup();

    // First part
    let b_diff = utils::vec_difference(&b,&a);
    let div1 = ab.len()*a.len(); // |A U B| * |A|
    a.iter().for_each(|x| {
        if b_diff.len() as f64 != 0.0 {
            b_diff.iter().for_each(|y|{
                // println!("{:?}-{:?}={:?}", x,y,hamming(x,y).unwrap()); //debug
                d1 = d1 + (hamming(x, y).unwrap() as f64);
            })
        }
    });
    d1 = d1 as f64/div1 as f64;
    
    // Second part
    let a_diff = utils::vec_difference(&a,&b);
    let div2 = ab.len()*b.len(); // |A U B| * |B|
    b.iter().for_each(|x| {
        if a_diff.len() as f64 != 0.0 {
            a_diff.iter().for_each(|y|{
                d2 = d2 + (hamming(x, y).unwrap() as f64);
                //println!("{:?}-{:?}={:?}", x,y,hamming(x,y)); //debug
            })
        }
    });
    d2 = d2 as f64/div2 as f64;
    //println!("d1 = {:?}", d1); //debug
    //println!("d2 = {:?}", d2); //debug
    
    return d1+d2 as f64;
}

pub fn hausdorff(set1:&Vec<String>, set2:&Vec<String>)-> i32{
    let mut sup_set1 = 0;
    set1.iter().for_each(|x|{
        let inf = inf_point_to_set(x,set2);
        if inf > sup_set1{
            sup_set1 = inf;
        }
    });

    let mut sup_set2 = 0;
    set2.iter().for_each(|y|{
        let inf = inf_point_to_set(y,set1);
        if inf > sup_set2{
            sup_set2 = inf;
        }
    });

    if sup_set2 > sup_set1{
        return sup_set2;
    }else{
        return sup_set1;
    }
}
