use itertools::Itertools;
fn main() {
    let mut count = 0;

    for perm in (1..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = muxa * a;

        if slon < 1000 || slon > 9999 {
            continue;
        }

        let slon_digits: Vec<u32> = slon
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let slon_check = [s, l, o, n];
        if slon_digits.iter().all(|d| slon_check.contains(&(*d as usize))) &&
           slon_digits.iter().copied().collect::<std::collections::HashSet<_>>().len() == 4
        {
            count += 1;
            println!("{:04}", muxa);
            println!("x   {}", a);
            println!("-----");
            println!("{:04}", slon);
            println!();
        }
    }

    println!("Total solutions found: {}", count);
}
