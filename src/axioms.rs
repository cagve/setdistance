use std::vec;

use distance::hamming;

use crate::{utils::{is_far, get_opt_fun, get_remain_set, are_disjoint}, distances::{idis, dmax_rel, average_dis, dmin_rel, self}};


pub fn id (set1: &Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    println!("{:?}",set1);
    let distance = dis(set1,  set1);
    if distance > 0{
        return Some(vec![set1.to_vec()]);
    }else{
        return None
    }
}

pub fn sym (set1: &Vec<String>, set2: &Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    if dis(set1,set2) != dis(set2,set1){
        return Some(vec![set1.to_vec(),set2.to_vec()]);
    }else{
        return None;
    }
}

// pub fn triangle_inequality(set1: &Vec<String>, set2: &Vec<String>, set3:&Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> { FOR NORMAL CASES
pub fn triangle_inequality(set1: &Vec<String>, set2: &Vec<String>, set3:&Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> { //REALSPACE DEFsqrt
    let distance1 = dis(set1,  set3);
    let distance2 = dis(set1,  set2);
    let distance3 = dis(set2,  set3);
    // println!("D(X,Y)={:?} + D(Y,Z)={:?} >= D(X,Z)={:?}", distance2, distance3, distance1);

    if distance1 > distance2 + distance3{
        return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
    }else {
        return None
    }
}

pub fn ax1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    if is_far(set1, set2, set3) && is_far(set1, set2, set4) && set3.len()==set4.len() && dis(set3,set4)>0{
        let disxy = dis(set1,set2);

        let mut unionx = set1.clone();
        let mut uniony = set2.clone();

        unionx.extend(set3.clone());
        uniony.extend(set4.clone());

        let disunion = dis(&unionx,&uniony);
        if disxy >= disunion { //Return counterexample
            println!("Counterexaple:");
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None
        }

    }else {
        return None;
    }
}

//set1 = X
//set2 = Y
//set3 = X'
//set4 = X''
pub fn ax7(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    let mut unionx1 = set1.clone();
    unionx1.extend(set2.clone());


    // DEBUG
    println!("DMAX(X,Y) = {:?} ", dmax_rel(set1, set2));
    println!("DMAX(X',X') = {:?} ", dmax_rel(set3, set3));
    println!("DMAX(X'', X'') = {:?} ", dmax_rel(set4, set4));

    println!("dmin(XY,X') = {:?} ", dmin_rel(&unionx1, set3));
    println!("dmin(XY,X'') = {:?} ", dmin_rel(&unionx1, set4));

    println!("isfarXY X' = {:?} ", is_far(set1,set2, set3));
    println!("isfarXY X'' = {:?} ", is_far(set1,set2, set4));

    println!("X'== X'' {:?} ", set3.len()==set4.len());
    println!("isfarXY X'' = {:?} ", is_far(set1,set2, set4));

    println!("D(X,Y)={:?}", idis(&set1, &set2));
    println!("D(X',Y)={:?}", idis(&set2,&set3));
    println!("D(X'',Y)={:?}", idis(&set2,&set4));

    if is_far(set1,set2,set3) && is_far(set1,set2,set4) && set3.len() == set4.len() && dis(set2,set3)<dis(set2,set4) {
        let mut unionx1 = set1.clone();
        let mut unionx2 = set1.clone();

        unionx1.extend(set3.clone());
        unionx2.extend(set4.clone());
        
        // println!("Opt(XX0Y)={:?}", get_opt_fun(set1, set2));
        // println!("Opt(XX1Y)={:?}", get_opt_fun(&unionx1, set2));
        // println!("Opt(XX2Y)={:?}", get_opt_fun(&unionx2, set2));
        println!("Opt(X1Y)={:?}", get_opt_fun(set3, set2));
        get_opt_fun(set3, set2).iter().for_each(|x|{
            println!("{}", hamming(&x.0, &x.1).unwrap());
        });
        println!("Opt(X2Y)={:?}", get_opt_fun(set4, set2));
        get_opt_fun(set4, set2).iter().for_each(|x|{
            println!("{}", hamming(&x.0, &x.1).unwrap());
        });
        println!("D(X',Y)={:?} < D(X'',Y)={:?}", dis(set3,set2), dis(set4,set2));
        println!("D(XX',Y)={:?} < D(XX'',Y)={:?}", dis(&unionx1,set2), dis(&unionx2,set2));

        if dis(&unionx1,&set2) >= dis(&unionx2,&set2) {
            println!("Counterexaple:");
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }
    return None;
}


pub fn ax6(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    let mut set23 = set2.clone();
    let mut set24 = set2.clone();

    set23.extend(set3.clone());
    set24.extend(set4.clone());

    //NON DISJOINT OPTION
    // if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) {
    //DISJOINT OPTION
    if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) && are_disjoint(set2, set3) && are_disjoint(set2, set4) {
        if dis(set1,&set23) > dis(set1,&set24) { //counterexample
            print!("Counterexaple:");
            println!("{:?}", vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
            println!("OPTxz {:?}", get_opt_fun(set1, set3));
            println!("OPTxz' {:?}", get_opt_fun(set1, set4));
            println!("Condition 1");
            println!("D(X,Z) = {}", dis(set1, set3));
            println!("D(X,Z') = {}", dis(set1, set4));
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            return None; 
        };
    };

    return None;
}

pub fn ax6_1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    let mut set23 = set2.clone();
    let mut set24 = set2.clone();

    set23.extend(set3.clone());
    set24.extend(set4.clone());

    //NON DISJOINT OPTION
    // if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) && dis(set1,set3) < dis(set1,set2) {
    //DISJOINT OPTION
    if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) && dis(set1,set3) < dis(set1,set2) && are_disjoint(set2, set3) && are_disjoint(set2, set4) {
        if dis(set1,&set23) > dis(set1,&set24) { //counterexample
            println!("Counterexaple:");
            println!("{:?}", vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }else {
        return None;
    }
}

pub fn ax6_2(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    let mut set23 = set2.clone();
    let mut set24 = set2.clone();

    set23.extend(set3.clone());
    set24.extend(set4.clone());

    let dmax_xz = dmax_rel(set1, set3);
    let dmin_xy = dmin_rel(set1, set2);

    // if set3.len() == set4.len() && dis(set1,set3) < dis(set1,set2) && dmax_xz < dmin_xy{
    //DISJOINT CASE
    if set3.len() == set4.len() && dis(set1,set3) < dis(set1,set2) && dmax_xz < dmin_xy && are_disjoint(set2, set3) && are_disjoint(set2, set4){
        if dis(set1,&set23) > dis(set1,&set24) { //counterexample
            println!("Counterexaple:");
            println!("{:?}", vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            return None;
        }
    }else {
        return None;
    }
}


