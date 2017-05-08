fn main() {
    let a_vector = gimmee_a_vector();
    let a_vector = print_vector(a_vector);
    print_vector(a_vector);
}

fn gimmee_a_vector() -> Vec<i32> {
    let mut my_vector = Vec::new();
    my_vector.push(1);
    my_vector.push(2);
    my_vector
}

fn print_vector(vector : Vec<i32>) -> Vec<i32> {
    for n in vector.clone() {
        println!("{}", n);
    }
    vector
}

#[test]
fn test_gimmee_a_vector() {
    assert_eq!(gimmee_a_vector(), vec![1, 2])
}
