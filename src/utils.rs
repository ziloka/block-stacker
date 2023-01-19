use macroquad::rand::gen_range;
use crate::consts::Tetrimino;

// https://www.geeksforgeeks.org/shuffle-a-given-array-using-fisher-yates-shuffle-algorithm/
pub fn shuffle(arr: &mut [Tetrimino; 7]) {
    // Start from the last element and swap one by one. We don't
    // need to run for the first element that's why i > 0
    for i in (1..arr.len()).rev() {
        // Pick a random index from 0 to i
        let j = gen_range(0, i + 1);
 
        // Swap arr[i] with the element at random index
        arr.swap(i, j);
    }
}