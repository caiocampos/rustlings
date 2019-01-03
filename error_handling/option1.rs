// option1.rs

fn main() {
    let mut list = vec![3];

    let last = list.pop();
    if let Some(_) = last {
        println!("The last item in the list is {:?}", last.unwrap());
    }
    
    let second_to_last = list.pop();
    if let Some(_) = second_to_last {
        println!("The second-to-last item in the list is {:?}", second_to_last.unwrap());
    }
}