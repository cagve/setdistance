use crate::utils::is_far;

pub fn id (set1: &Vec<String>, set2: &Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    let distance = dis(set1,  set2);
    if set1==set2 && distance > 0{
        return Some(vec![set1.to_vec(), set2.to_vec()]);
    }else{
        return None
    }
}

pub fn sym (set1: &Vec<String>, set2: &Vec<String>, dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
    if dis(set1,set2) == dis(set2,set1){
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
            println!("COUNTEREXAPMLE");
            return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec(),set4.to_vec()]);
        }else {
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
// pub fn ax7(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> Option<Vec<Vec<String>>> {
pub fn ax7(set1: &Vec<String>,set2: &Vec<String>,set3:&Vec<String>,set4:&Vec<String>,dis: fn(&Vec<String>,&Vec<String>) -> i32) -> bool{
    if is_far(set1,set2,set3) && is_far(set1,set2,set4) && set3.len() == set4.len() && dis(set2,set3)<dis(set3,set4) {
        println!("Se puede aplicar axioma 1 en el siguiente caso:");
        println!("X={:?}, Y={:?}, X'={:?}, Y'={:?}", set1,set2,set3,set4);
        return true
    }

    return false
}
