// https://www.hackerrank.com/challenges/sock-merchant/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=warmup

use std::{collections::HashMap, usize};
use std::{env, fs, io::stdin};

pub fn sock_merchant(socks_array: &[u32], _array_size: usize) -> u32 {
    // Time complexity O(n)
    let mut count_socks: HashMap<u32, u32> = HashMap::new();
    for sock in socks_array {
        let new_count = count_socks.get(sock).unwrap_or(&0) + 1;
        count_socks.insert(*sock, new_count);
    }
    eprintln!("count_socks = {:#?}", count_socks);
    let mut num_pairs = 0;
    for (_, count) in count_socks {
        num_pairs += count / 2;
    }
    eprintln!("num_pairs = {:#?}", num_pairs);
    num_pairs
}

pub fn main() {
    //! Main function to run the code challenge on hacker rank
    let output_path = env::var("OUTPUT_PATH")
        .expect("Environment variable: OUTPUT_PATH with output path wasn't found!");
    let mut n = String::new();
    println!("Enter the size of the array");
    stdin()
        .read_line(&mut n)
        .expect("Error reading the size of the array");
    let n = n
        .trim()
        .parse()
        .expect("Didn't enter the correct size of the array. The value needs to be an integer!");

    let mut ar = String::new();
    println!("Enter each elements of the array using space as a separator");
    stdin().read_line(&mut ar).expect("Error reading the array");
    let ar = ar
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse::<u32>().expect(&format!(
                "Error Reading one element of the array {}. All values should be integers",
                x
            ))
        })
        .collect::<Vec<_>>();

    let result = sock_merchant(ar.as_slice(), n);
    fs::write(output_path, format!("{}\n", result))
        .expect("Couldn't save the result in the output file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_num_pairs() {
        let socks_ar1 = &[10, 20, 20, 10, 10, 30, 50, 10, 20];
        let socks_ar2 = &[1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sock_merchant(socks_ar1, socks_ar1.len()), 3);
        assert_eq!(sock_merchant(socks_ar2, socks_ar1.len()), 2);
    }
}
