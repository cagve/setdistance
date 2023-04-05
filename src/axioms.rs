use std::vec;

use crate::{utils::is_far, distances::{idis, dmax_rel, dmin_rel}};


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

pub fn triangle_inequality(set1: &Vec<String>, set2: &Vec<String>, set3:&Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    let distance1 = dis(set1,  set3);
    let distance2 = dis(set1,  set2);
    let distance3 = dis(set2,  set3);

    if distance1 > distance2 + distance3{
        return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
    }else {
        return None
    }
}

pub fn ax1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    if is_far(set1, set2, set3) && is_far(set1, set2, set4) && set3.len()==set4.len() && dis(set3,set4)>0{
        // println!("Se puede aplicar axioma 1 en el siguiente caso:");
        // println!("X={:?}, Y={:?}, X'={:?}, Y'={:?}", set1,set2,set3,set4);
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
    // println!("DMAX(X,Y) = {:?} ", dmax_rel(set1, set2));
    // println!("DMAX(X',X') = {:?} ", dmax_rel(set3, set3));
    // println!("DMAX(X'', X'') = {:?} ", dmax_rel(set4, set4));
    //
    // println!("dmin(XY,X') = {:?} ", dmin_rel(&unionx1, set3));
    // println!("dmin(XY,X'') = {:?} ", dmin_rel(&unionx1, set4));
    //
    // println!("isfarXY X' = {:?} ", is_far(set1,set2, set3));
    // println!("isfarXY X'' = {:?} ", is_far(set1,set2, set4));
    //
    // println!("X'== X'' {:?} ", set3.len()==set4.len());
    // println!("isfarXY X'' = {:?} ", is_far(set1,set2, set4));
    //
    // println!("D(X,Y)={:?}", idis(&set1, &set2));
    // println!("D(X',Y)={:?}", idis(&set2,&set3));
    // println!("D(X'',Y)={:?}", idis(&set2,&set4));

    if is_far(set1,set2,set3) && is_far(set1,set2,set4) && set3.len() == set4.len() && dis(set2,set3)<dis(set2,set4) {
        let mut unionx1 = set1.clone();
        let mut unionx2 = set1.clone();

        unionx1.extend(set3.clone());
        unionx2.extend(set4.clone());
        
        // println!("D(X',Y)={:?} < D(X'',Y)={:?}", idis(set3,set2), idis(set4,set2));
        // println!("D(XX',Y)={:?} < D(XX'',Y)={:?}", idis(&unionx1,set2), idis(&unionx2,set2));

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
