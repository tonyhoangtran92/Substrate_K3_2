//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main(){
    let mut a = vec![1,2,3,4,5];
    let (a, c) = test(&mut a);
    println!("{}",c);
    println!("{:?}",a);
}

pub fn test(a: &mut Vec<i32>) -> (Vec<i32>, i32) {
    let mut b:Vec<i32>  = Vec::new();
    let mut c:i32 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c)
}