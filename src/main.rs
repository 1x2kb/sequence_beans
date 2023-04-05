use sequence_bean;

fn main() {
    let vec: Vec<u8> = vec![4, 1, 7, 8];
    println!("Recieved: {:?}", &vec);
    let sorted_vec = sort_sequence(vec); // Does not assume sorted form.
    println!("Sorted to form: {:?}", &sorted_vec);
    println!("Bean sequence:");
    let original_bean_sequence = sequence_bean::get_bean_representation(&sorted_vec);

    sequence_bean::print_bean_representation(&original_bean_sequence);

    let conjugate = sequence_bean::conjugate_collection(&sorted_vec);
    println!("Conjugated form: {:?}", &conjugate);
}

fn sort_sequence(sequence: Vec<u8>) -> Vec<u8> {
    let mut copy = sequence.clone();
    copy.sort_by(|a, b| b.partial_cmp(a).unwrap());

    copy
}
