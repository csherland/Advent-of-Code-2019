fn main() {
    let input_range = 172_930..683_082+1;
    let matches = input_range.fold(vec![], | mut acc, val | {
        let mut i = val;
        let mut digits = vec![];

        while i > 0 {
            let digit = i % 10;
            digits.push(digit);
            i /= 10;
        }
        digits.reverse();
        let mut clone = digits.clone();
        clone.sort();

        let mut uniques = clone.clone();
        uniques.dedup();
        
        if clone == digits && uniques.len() < digits.len() {
            acc.push(val);
        }
        acc
    });

    println!("{}", matches.len());
}

