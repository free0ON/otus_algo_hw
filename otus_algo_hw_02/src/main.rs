mod tickets;

use tickets::fast_get_happy_count;
use std::time::SystemTime;
use std::fs;
fn main() {
    let now = SystemTime::now();
    let path: &str  = "1.Tickets/test";

    for i in 0..= 9
    {
        let input: u32 = fs::read_to_string(format!("{}.{}.in",path,i).as_str())
            .expect("Should have been able to read the file")
            .trim_end()
            .parse() 
            .expect("Not a number");
        let output: u64 = fs::read_to_string(format!("{}.{}.out",path,i).as_str())
            .expect("Should have been able to read the file")
            .trim_end()
            .parse()
            .expect("Not a number");
        let count = fast_get_happy_count(input);
        let res=
            if output == count {
                "..... pass"
            }
            else {
            "..... fail"
        };
        println!("rust fast count: {} {} {}", input, output, res);    
    }

    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("Time : {} sec", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }

}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
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
