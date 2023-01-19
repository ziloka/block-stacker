use macroquad::rand::gen_range;

// https://www.geeksforgeeks.org/shuffle-a-given-array-using-fisher-yates-shuffle-algorithm/
pub fn shuffle<T>(arr: &mut [T; 7]) {
    for i in (1..arr.len()).rev() {
        let j = gen_range(0, i + 1);
        arr.swap(i, j);
    }
}
