use distance::hamming;
use itertools::Itertools;

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

pub fn set_functions(set1:&Vec<String>, set2:&Vec<String>){
    println!("X:{:?}, Y:{:?}", set1,set2);

    let mut result: Vec<(String, String)> = Vec::new();
    for permut in set2.iter().permutations(set2.len()){
        for (a, b) in set1.iter().zip(permut) {
            result.push((a.to_string(), b.to_string()));
        }
    }
    println!("{:?}", result);
    
    
}
