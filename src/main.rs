use sequence_bean;

fn main() {
    let _first_sequence = run_example_1_set();
    let _second_sequence = run_example_2_set();
}

fn run_example_1_set() -> Vec<u8> {
    let sequence = vec![5, 3, 1];
    run_set_for_sequence(sequence)
}

fn run_example_2_set() -> Vec<u8> {
    let sequence = vec![4, 2, 2];
    run_set_for_sequence(sequence)
}

fn run_set_for_sequence(sequence: Vec<u8>) -> Vec<u8> {
    // Rules require a non increasing list. Sorting (dec) prevents such an example from flowing through
    let sorted_sequence = sort_sequence(&sequence);
    let sorted_bean_sequence = sequence_bean::get_bean_representation(&sorted_sequence);

    let conjugate = sequence_bean::conjugate_collection(&sorted_sequence);
    let conjugate_bean = sequence_bean::get_bean_representation(&conjugate);

    println!("(original, sorted, conjugate)");
    println!(
        "({:?}, {:?}, {:?})",
        &sequence, &sorted_sequence, &conjugate
    );
    sequence_bean::print_bean_representation(&sorted_bean_sequence);
    sequence_bean::print_bean_representation(&conjugate_bean);

    conjugate
}

fn sort_sequence(sequence: &Vec<u8>) -> Vec<u8> {
    let mut copy = sequence.clone(); // don't mutate the original sequence.
    copy.sort_by(|a, b| b.partial_cmp(a).unwrap());

    copy
}
