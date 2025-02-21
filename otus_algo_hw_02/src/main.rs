mod tickets;

use tickets::brut_get_happy_count;
use tickets::fast_get_happy_count;


fn main() {
    for i in 1..=5
    {
        println!("Brut count: {} {}",i, brut_get_happy_count(i));    
    }

    for i in 1..=10
    {
        println!("Fast count: {} {}",i, fast_get_happy_count(i));    
    }

}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    //use std::num::ParseIntError;
    
    // #[test]
    // fn brut_tickets_test()
    // {
    //     let path: &str  = "1.Tickets/test";
    //     for i in 0..5
    //     {
    //         let input = fs::read_to_string(format!("{}.{}.in",path,i).as_str())
    //             .expect("Should have been able to read the file")
    //             .trim_end()
    //             .parse()
    //             .expect("Not a number");
    //         let output = fs::read_to_string(format!("{}.{}.out",path,i).as_str())
    //             .expect("Should have been able to read the file")
    //             .trim_end()
    //             .parse()
    //             .expect("Not a number");
    //         let count = brut_get_happy_count(input);
    //         println!("Brut count: {}  {}",input, count);    
    //         assert_eq!(count, output);
    //     }
    // }
    #[test]
    fn fast_tickets_test()
    {
        let path: &str  = "1.Tickets/test";
        for i in 0..10
        {
            let input = fs::read_to_string(format!("{}.{}.in",path,i).as_str())
                .expect("Should have been able to read the file")
                .trim_end()
                .parse()
                .expect("Not a number");
            let output = fs::read_to_string(format!("{}.{}.out",path,i).as_str())
                .expect("Should have been able to read the file")
                .trim_end()
                .parse()
                .expect("Not a number");
            let count = fast_get_happy_count(input);
            println!("rust fast count: {:<2} : {}",input, count);    
            assert_eq!(count, output);
        }
    }

}