fn main() {
    // Part 1 solution
    let input_range = 172_930..683_082+1;
    let matches = input_range.clone().fold(vec![], | mut acc, val | {
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

    // Part 2 solution
    let matches_2 = input_range.clone().fold(vec![], | mut acc, val | {
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

        let mut double = false;
        for unique in uniques {
            let mut count = 0;
            for digit in digits.clone() {
                if digit == unique {
                    count += 1;
                }
            }

            if count == 2 {
                double = true;
            }
        }
        
        if clone == digits && double {
            acc.push(val);
        }
        acc
    });

    println!("{}", matches_2.len());
}

