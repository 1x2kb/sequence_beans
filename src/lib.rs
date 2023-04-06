pub fn conjugate_collection(sequence: &Vec<u8>) -> Vec<u8> {
    let mut vec = vec![];
    for i in 0..sequence.first().unwrap().clone() {
        let num: u8 = sequence.iter().fold(0, |accumulator, next| {
            if next >= &(i + 1) {
                return accumulator + 1;
            }

            accumulator
        });
        vec.push(num);
    }

    vec.into_iter().collect()
}

pub fn get_bean_representation(sequence: &Vec<u8>) -> Vec<String> {
    sequence
        .into_iter()
        .map(|row_item| {
            let mut beans = String::new();
            for _ in 0..row_item.clone() {
                beans.push('o');
            }

            beans
        })
        .collect()
}

pub fn print_bean_representation(bean_sequence: &Vec<String>) {
    bean_sequence
        .into_iter()
        .for_each(|bean_row| println!("{}", bean_row));
}

#[cfg(test)]
mod get_bean_representation_should {
    use super::*;

    #[test]
    fn return_correct_orientation_example_1() {
        let vec = vec![3, 2, 2, 1, 1];
        let result = get_bean_representation(&vec);
        let expected = vec!["ooo", "oo", "oo", "o", "o"];
        assert_eq!(result, expected);
    }

    #[test]
    fn return_correct_orientation_example_2() {
        let vec = vec![3, 3, 1, 1];
        let result = get_bean_representation(&vec);
        let expected = vec!["ooo", "ooo", "o", "o"];
        assert_eq!(result, expected);
    }

    #[test]
    fn return_correct_orientation_num_gapped_example() {
        let vec = vec![8, 7, 4, 1];
        let result = get_bean_representation(&vec);
        let expected = vec!["oooo", "ooo", "ooo", "ooo", "oo", "oo", "oo", "o"];
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod conjugate_collection_should {
    use super::*;

    #[test]
    fn conjugate_first_example() {
        let vec = vec![5, 3, 1];
        let result = conjugate_collection(&vec);
        assert_eq!(result, vec![3, 2, 2, 1, 1]);
    }

    #[test]
    fn conjugate_second_example() {
        let vec = vec![4, 2, 2];
        let result = conjugate_collection(&vec);
        assert_eq!(result, vec![3, 3, 1, 1]);
    }

    #[test]
    fn conjugate_num_gapped_example() {
        let vec = vec![8, 7, 4, 1];
        let result = conjugate_collection(&vec);
        assert_eq!(result, vec![4, 3, 3, 3, 2, 2, 2, 1]);
    }
}
