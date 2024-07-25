use std::vec;

use distance::hamming;
use egui::util;

use crate::{distances::{self, average_dis, dmax_rel, dmin_rel, idis, inf_point_to_set, pivot_distance, realspace}, utils::{self, are_disjoint, difference, get_opt_fun, get_remain_set, is_far}};


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

pub fn p1(set1: &Vec<String>, set2: &Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> bool{
    println!("=====================NEW CASE: ");
    println!("set1 = {:?}", set1);
    println!("set2 = {:?}", set2);
    let d1 = dis(set1,set2);
    let diff = difference(set1, set2);
    for x in diff{
        println!("======== point {}", &x);
        let mut uniony = set2.clone();
        uniony.push(x.to_string());
        let d2 = dis(set1, &uniony);
        let min = inf_point_to_set(&x, set2);
        if d1 < d2 + (min as f64){
            // println!("Set1 {:?}", set1);
            // println!("Set2 {:?}", set2);
            println!("Counter");
            println!("point {}", x);
            println!("&d1 = {}", &d1);
            println!("D(X,YUx) = {}", &d2);
            println!("D(x,Y) = {}", &min);
            println!("{}>={}", &d1, &d2+(min as f64));
            return false;
        }
    };

    return true;
}

pub fn p2(set1: &Vec<String>, universe:Vec<String>, dis: fn(&Vec<String>, &Vec<String>)->f64) -> bool{
    println!("=====================NEW CASE: ");
    println!("set1 = {:?}", set1);

    for x in universe{
        let mut union = set1.clone();
        union.push(x.clone());
        let d2 = dis(set1, &union);
        let min = inf_point_to_set(&x, set1);
        if d2 != (min as f64){
            println!("Counter");
            println!("point {}", &x);
            println!("D(X,XUx) = {}", &d2);
            println!("D(x,Y) = {}", &min);
            return false;
        }
    }
    return true
}

// pub fn triangle_inequality(set1: &Vec<String>, set2: &Vec<String>, set3:&Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> { FOR NORMAL CASES
pub fn triangle_inequality(set1: &Vec<String>, set2: &Vec<String>, set3:&Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> { //REALSPACE DEFsqrt
    let distance1 = dis(set1,  set3);
    let distance2 = dis(set1,  set2);
    let distance3 = dis(set2,  set3);
    println!("D(X,Y)={:?} + D(Y,Z)={:?} >= D(X,Z)={:?}", distance2, distance3, distance1); // Debug
    if distance1 > distance2 + distance3{
        return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
    }else {
        return None
    }
}


pub fn ax_singlenton(x: String, y:String,u: String, v:String, dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    if hamming(&x.to_string(), &y.to_string()).unwrap() <= hamming(&u.to_string(), &v.to_string()).unwrap() {
        let setx = vec![x.to_string()];
        let sety = vec![y.to_string()];
        let setu = vec![u.to_string()];
        let setv = vec![v.to_string()];

        if dis(&setx,&sety) > dis(&setu, &setv){
            return Some(vec![setx.to_vec(), sety.to_vec(), setu.to_vec(), setv.to_vec()])
        }

    }

    return None
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

pub fn lemmaDecomposition(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut union_yz = set2.clone();
    union_yz.extend(set3.clone());
    if are_disjoint(&set2,&set1) && are_disjoint(&set1,&set3)&&are_disjoint(&set2,&set3) {
        if dis(&set1,&set2)+dis(&set1, &set3) < dis(&set1,&union_yz) {
            println!("Counterexaple:");
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }
    return None;
}

//set1 = X
//set2 = Y
//set3 = Z
pub fn ax7(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut union_yz = set2.clone();
    union_yz.extend(set3.clone());
    println!("set1 = {:?}", set1);
    println!("set2 = {:?}", set2);
    println!("set3 = {:?}", set3);
    println!("set4 = {:?}", union_yz);
    println!("dis(X,Z) = {:?}", dis(&set1,&set3));
    println!("dis(X,ZUY) = {:?}", dis(&set1,&union_yz));

    if set1.len() <= set2.len() && are_disjoint(&set2,&set3) && dis(&set1,&set3)>0.0{
    // if are_disjoint(&set2,&set3) &&&set1,&set3) && set1.len() <= set2.len() && are_disjoint(&set2,&set3) && dis(&set1,&set3)>0.0{
        if dis(&set1,&set2) >= dis(&set1,&union_yz) {
            println!("Counterexaple:");
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }
    return None;
}

pub fn ax7_1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut union_yz = set2.clone();
    union_yz.extend(set3.clone());
    let dxz = dis(&set1, &set3);
    let dxy = dis(&set1, &set2);

    // if set1.len() <= set2.len() && are_disjoint(&set2,&set3) && dis(&set1,&set3)>0{
    // println!("{:?}", set1);
    // println!("{:?}", set2);
    // println!("{:?}", set3);
    // AXIOM 7' disjoint ocndition VVV
    // if  set1.len() <= set2.len() && are_disjoint(set1,set2) && are_disjoint(set1, set3) && are_disjoint(&set2,&set3) && dxz<dxy{
    // if  set1.len() <= set2.len() && are_disjoint(set1,set2) && are_disjoint(set1, set3) && are_disjoint(&set2,&set3) && dmax_rel(set1, set3) < dmin_rel(set1, set2){
    if  set1.len() <= set2.len() && are_disjoint(set2,set3) && dxz< dxy{
        if dis(&set1,&union_yz) >= dis(&set1,&set2) {
            println!("Counterexaple:");
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    } else if set1.len() > set2.len(){
        println!("Condition len not working ");
        
    } else if !are_disjoint(set2, set3){
        println!("Condition disjoint not working ");

    } else if dxz > dxy{
        println!("Condition DXY not working");
    }
    return None;
}

pub fn ax8(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut union_xz = set1.clone();
    union_xz.extend(set3.clone());

    let dxy = dis(&set1,&set2);
    let dyz = dis(&set2,&set3);
    let dxuzy = dis(&union_xz,&set2);
    //Pairwise disjoint
    // if union_xz.len() <= set2.len() && are_disjoint(&set1,&set3) && are_disjoint(&set1, &set2) && are_disjoint(&set2, &set3) {
    if union_xz.len() <= set2.len() && are_disjoint(&set1,&set3) && dyz < dxy{
        if dxy < dxuzy {
            println!("Counterexaple:");
            println!("dis(X,Y) = {:?}", dxy);
            println!("dis(XUZ,Y) = {:?}", dxuzy);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }
    return None;
}
pub fn ax8_1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>, set4:&Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut union_xz = set1.clone();
    union_xz.extend(set3.clone());
    let mut union_yz2 = set2.clone();
    union_yz2.extend(set4.clone());

    let dxy = dis(&set1,&set2);
    let dxz1 = dis(&set1,&set3);
    let dxz2 = dis(&set1,&set4);
    let dyz1 = dis(&set2,&set3);
    let dxuzyuz2 = dis(&union_xz,&union_yz2);

    // Axiom 5.1
    // if set1.len() == set2.len() && are_disjoint(&set1,&set3) && are_disjoint(&set2, &set4) && dxy < dxz2 && dxy < dyz1 {
    // Axiom 5.1'
    if set1.len() == set2.len() && are_disjoint(&set1,&set3) && are_disjoint(&set2, &set4) && dmax_rel(set1, set2) < dmin_rel(set1, set4) && dmax_rel(set1, set2) < dmin_rel(set2, set3) {
        if dxy >= dxuzyuz2 {
            println!("Counterexaple:");
            println!("dis(X,Y) = {:?}", dxy);
            println!("dis(XUZ,Y) = {:?}", dxuzyuz2);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(), set4.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }
    return None;
}
pub fn ax81_1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut union_xz = set1.clone();
    union_xz.extend(set3.clone());
    let mut union_yz2 = set2.clone();
    union_yz2.extend(set4.clone());

    let dxy = dis(&set1,&set2);
    let dxz2 = dis(&set1,&set4);
    let dyz = dis(&set2,&set3);
    let dxuzyuz2 = dis(&union_xz,&union_yz2);

    if set1.len() == set2.len() && dxy<dxz2 && dxy < dyz  {
        if dxuzyuz2 >= dxy {
            println!("Counterexaple:");
            println!("dis(X,Y) = {:?}", dxy);
            println!("dis(XUZ,Y) = {:?}", dxuzyuz2);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(), set4.to_vec()]);
        }else {
            println!("Applied but no counterexaple:");
            return None;
        }
    }
    return None;
}

pub fn ax5(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let dmax_x1y1 = dmax_rel(set1, set2);
    let dmax_x2z2 = dmin_rel(set3, set4);
    let dx1y1 = dis(set1,set2);
    let dx2y2 = dis(set3, set4);

    if dmax_x1y1 < dmax_x2z2 {
        if dx2y2 >= dx1y1 { //counterexample
            println!("Counterexaple:");
            println!("{:?}", vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            println!("Applied but not a counterexample");
            return None;
        }
    }else {
        println!("Not applied");
        return None;
    }
}

pub fn ax6(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
    let mut set23 = set2.clone();
    let mut set24 = set2.clone();

    set23.extend(set3.clone());
    set24.extend(set4.clone());

    //remove duplicates
    set23.sort();
    set23.dedup();
    set24.sort();
    set24.dedup();

    let mut counter = 0;
    //DISJOINT OPTION between Y and X
    if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) && utils::are_disjoint(set2, set4) && utils::are_disjoint(set2,set3) {
    //DISJOINT OPTION
    // if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) && are_disjoint(set1, set3) && are_disjoint(set1, set4) && are_disjoint(set2, set3) && are_disjoint(set2, set4) {
    // DISJOINT OPTION + 6': D(X,Z) < D(X,Y)
    // if set3.len() == set4.len() && dis(set1, set3) < dis(set1,set4) && dis(set1,set3) < dis(set1,set2) && are_disjoint(set1, set3) && are_disjoint(set1, set4) && are_disjoint(set2, set3) && are_disjoint(set2, set4) {
    println!("=======CASO NUEVO ==========");
    println!("Set1:, {:?}", set1);
    println!("Set2:, {:?}", set2);
    println!("Set3:, {:?}", set3);
    println!("Set4:, {:?}", set4);

    // let dxy  = distances::average(set1, set2); //debug
    // let dxz1 = distances::average(set1, set3); //debug
    // let dxz2 = distances::average(set1, set4); //debug
    // let dxyz1  = distances::average(set1, &set23); //debug
    // let dxyz2  = distances::average(set1, &set24); //debug
    // println!("D(X,Y)    = {:?}", dxy); //debug
    // println!("D(X,Z)   = {:?}", dxz1); //debug
    // println!("D(X,Z')   = {:?}", dxz2); //debug
    // println!("D(X,YUZ)  = {:?}", dxyz1); //debug
    // println!("D(X,YUZ') = {:?}", dxyz2); //debug

    counter = counter+ 1;
    println!("+Satisfy conditions n{}+", counter);
        if dis(set1,&set23) > dis(set1,&set24) { //counterexample
            print!("Counterexaple:");
            println!("{:?}", vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
            return None; 
        };
    }
    return None;
}

pub fn ax6_1(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<Vec<String>>> {
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
    if set3.len() == set4.len() && dis(set1,set3) < dis(set1,set2) && dmax_xz < dmin_xy && are_disjoint(set1,set2) && are_disjoint(set2, set3) && are_disjoint(set2, set4) && are_disjoint(set1, set3) && are_disjoint(set1, set4){
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

// d(x,y) < d(x','y), then D({x}, {y})<D({x'}, {y'})
pub fn ax4(point1: &String,point2: &String, point3: &String, point4:&String, dis: fn(&Vec<String>,&Vec<String>) -> f64) -> Option<Vec<String>> {
    let set1 = vec![point1.clone(), point3.clone(), point2.clone()]; //x
    let set2 = vec![point2.clone(), point4.clone(), point1.clone()]; //y
    let set3 = vec![point3.clone()]; //x'
    let set4 = vec![point4.clone()]; //y'

    let hamx1y1 = hamming(&point1, &point2).unwrap();
    let hamx2y2 = hamming(&point3, &point4).unwrap();

    println!("Stoped bc d(x,y) <= d(x',y') BUT D(x,y) > D(x','y)"); //debug
    println!("d({}, {})={}    D({:?}, {:?})= {}",point1, point2, hamx1y1, set1,set2,dis(&set1,&set2)); // debug
    // println!("d({}, {})={}    D({:?}, {:?})= {}",point3, point4, hamx2y2, set3,set4,dis(&set3,&set4)); // debug
    // if hamx1y1 <= hamx2y2 {
    //     if dis(&set1,&set2) > dis(&set3,&set4) {
    //         // println!("Stoped bc d(x,y) <= d(x',y') BUT D(x,y) > D(x','y)"); //debug
    //         // println!("d({}, {})={}    D({:?}, {:?})= {}",point1, point2, hamx1y1, set1,set2,dis(&set1,&set2)); // debug
    //         // println!("d({}, {})={}    D({:?}, {:?})= {}",point3, point4, hamx2y2, set3,set4,dis(&set3,&set4)); // debug
    //         println!("Counterexaple:");
    //         return Some(vec![point1.to_string(),point2.to_string(),point3.to_string(),point4.to_string()]);
    //     }else {
    //         return None;
    //     }
    // }
    return None;
}



