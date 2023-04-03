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
    let distance1 = dis(set1,  set2);
    let distance2 = dis(set1,  set3);
    let distance3 = dis(set2,  set3);

    if distance1 <= distance2 + distance3{
        return Some(vec![set1.to_vec(),set2.to_vec(),set3.to_vec()]);
    }else {
        return None
    }


}
