pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod chess;

const number_possible_move: [[usize; 8]; 8] = [
    [23,24,25,25,25,25,24,23],
    [24,27,30,31,31,30,27,25],
    [25,30,33,34,34,33,30,25],
    [25,31,34,36,36,34,31,25],
    [25,31,34,36,36,34,31,25],
    [25,30,33,34,34,33,30,25],
    [24,27,30,31,31,30,27,24],
    [23,24,25,25,25,25,24,23]
];
