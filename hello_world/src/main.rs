use std::io::{ Write, Read };

fn main() {
    let a_vector = gimmee_a_vector();
    write_vector(a_vector, &mut std::io::stdout()).unwrap();
}

fn gimmee_a_vector() -> Vec<i32> {
    let mut my_vector = Vec::new();
    my_vector.push(1);
    my_vector.push(2);
    my_vector
}

fn write_vector<W: Write>(v: Vec<i32>, w: &mut W) -> std::io::Result<()> {
    let body = v.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    write!(w, "[{}]", body)
}

#[test]
fn test_gimmee_a_vector() {
    assert_eq!(gimmee_a_vector(), vec![1, 2])
}

#[test]
fn test_write_vec() {
    let mut read_writer = std::io::Cursor::new(Vec::new());
    let v = vec![5, 6];
    write_vector(v, &mut read_writer).unwrap();

    read_writer.set_position(0);

    let mut output = String::new();
    read_writer.read_to_string(&mut output).unwrap();

    assert_eq!(output, "[5, 6]".to_string());
}
