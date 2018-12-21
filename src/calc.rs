// Check string whether number or not.
fn is_number<'n>(v: &Vec<&'n str>) -> bool {
    let mut stack = vec![];
    for s in v.iter() {
        if s.parse::<i32>().is_ok() {
            stack.push(true);
        } else {
            stack.push(false);
        }
    }
    if stack.iter().all(|&r| r == true) {
        stack.clear();
        true
    } else {
        stack.clear();
        false
    }
}

pub fn sum(mut v: Vec<&str>) -> Result<i32, String> {
    let params: Vec<&str> = v.drain(2..).collect();

    if is_number(&params) {
        let params2: Vec<i32> = params.iter()
            .map(|p| p.parse::<i32>().unwrap())
            .collect();
        let params3 = params2.iter().fold(0, |a, b| a + b);
        Ok(params3)
    } else {
        Err(format!("Contain not-number."))
    }
}
