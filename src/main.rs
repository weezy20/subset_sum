use rand::{prelude::SliceRandom, thread_rng, Rng};

fn main() {
    // greedy_subset_square returns the largest count of square ints
    // that could be squared and sum to `n`. This uses a greedy algorithm
    // to achieve that.

    greedy_subset_square(625);
    greedy_subset_square(50);
    greedy_subset_square(11);
}

fn greedy_subset_square(n: i32) {
    let mut results: Vec<Vec<i32>> = vec![];
    let mut rng = thread_rng();
    let square_items = (1..n).map(|i| i.pow(2)).collect::<Vec<i32>>();
    for _ in 0..1000 {
        let mut vec = square_items.to_vec();
        vec.shuffle(&mut rng);
        // become greedy
        let mut possible_result: Vec<i32> = vec![];
        let mut sum: i32 = vec.iter().fold(0, |mut acc, &x| {
            if x + acc <= n.pow(2) {
                acc += x;
                possible_result.push(x);
                acc
            } else {
                acc
            }
        });
        if sum == n.pow(2) {
            // println!("n_square = {}, calculated sum = {}", n.pow(2), sum);
            // results.push(vec)

            results.push(
                possible_result
                    .into_iter()
                    .map(|i| {
                        let ii: f64 = i as f64;
                        let ii: i32 = ii.sqrt() as i32;
                        ii
                    })
                    .collect::<Vec<i32>>(),
            );
        }
    }
    println!("**********Vectors tested for {}**********", n);
    results.into_iter().for_each(|x| println!("{:?}", x));
}
