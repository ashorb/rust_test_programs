
fn main() {
    let mut v1 = vec![1,2,3];
    let mut v2: Vec<u8> = Vec::new();
    let mut v3: Vec<u8> = Vec::with_capacity(10);

    v1.push(7);
    v1.push(9);
    v1.pop();
    
    println!("v1 Length: {}", v1.len());
    println!("v1 Capacity: {}", v1.capacity());
    println!("v1 {:?}", v1);

    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(7);
    v2.push(9);
    v2.pop();

    println!("v2 Length: {}", v2.len());
    println!("v2 Capacity: {}", v2.capacity());
    println!("v2 {:?}", v2);

    v3.push(1);
    v3.push(2);
    v3.push(3);
    v3.push(7);
    v3.push(9);
    v3.pop();

    println!("v3 Length: {}", v3.len());
    println!("v3 Capacity: {}", v3.capacity());
    println!("v3 {:?}", v3);

}