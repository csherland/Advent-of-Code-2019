fn main() {
    let input_range = 172_930..683_082+1;
    let matches = input_range.fold(vec![], | mut acc, val | {
        let mut i = val;
        let mut digits = vec![];

        // Parse number into array of digits
        while i > 0 {
            let digit = i % 10;
            digits.push(digit);
            i /= 10;
        }
        digits.reverse();

        // What digits should look like if ascending
        let mut sorted = digits.clone();
        sorted.sort();

        // Use this to check that there are doubles
        let mut uniques = digits.clone();
        uniques.dedup();
        
        if sorted == digits && uniques.len() < digits.len() {
            acc.push(val);
        }
        acc
    });

    println!("{}", matches.len());
}

