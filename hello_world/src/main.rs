mod value_tour;
mod types;

fn main() {
    let mut arr = vec![1,2,3];
    arr.push(4);

    let mut arr1 = vec![1,2,3];
    let ir1 = &arr;
    let ir2 = &arr1;

    println!("ir1: {:?} ir2: {:?}", ir1, ir2)
}
