fn get_sum_number(mut number: u64 ) -> usize {
	let mut sum: u64 = 0;
	//let mut number: u64 = _number;
	while number > 0
	{
		sum += number % 10;
		number /= 10;
	}

	return sum as usize;
}
pub fn brut_get_happy_count(n: u32) -> u64
{
        let mut count: u64 = 0;
        let max_num: u64 = u64::pow(10,n);
        for left in 0..max_num
        {
            for right in 0..max_num
            {
                if get_sum_number(left) == get_sum_number(right)  
                {
                    count += 1;
                }
            }
        }
    return count;
}

pub fn fast_get_happy_count(n: u32) -> u64
{
    let mut count: u64 = 0;
    let max_num: u64 = u64::pow(10,n);
    let sums_number: usize = (9*n + 1) as usize;
    let mut sums: Vec<u64> = vec![0;sums_number];
	(0..max_num).for_each(|number| {
		sums[get_sum_number(number)] += 1;
	});
	sums.iter().for_each(|x| count += x*x);
    
	return count;
}
