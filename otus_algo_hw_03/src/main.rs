use std::time::{Instant};
use num_traits::pow as pown;

struct Matrix<T>
{
    size: i32,
    data: Vec<Vec<T>>,
}

pub(crate) struct Matrix2D<T>{
    a: T, b: T,
    c: T, d: T,
}
impl Mul<Matrix2D<T>> for Matrix2D<T> {
    type Output = Self;

    pub fn mul(self, rhs: Self) -> Self::Output {
        Self {
            self.a*rhs.a+self.b*rhs.c, self.a*rhs.b+self.b*rhs.d,
            self.c*rhs.a+self.d*rhs.c, self.c*rhs.b+self.d*rhs.d,
         
        }
    }   


}


impl<T> Matrix2D<T> {
    pub fn new(_a: T, _b: T, _c: T, _d: T) -> Self {
        Self {
            a : _a, b: _b,
            c: _c, d: _d,
        }
    }

    //
    // |a1 b1| x |a2 b2| = |a1*a2+b1*c2 a1*b2+b1*d2| 
    // |c1 d1|   |c2 d2|   |c1*a2+d1*c2 c1*b2+d1*d2|
    //
}

// impl Matrix<T> {
//     pub fn new(s: i32) -> Self {
//         Self {
//             size = s,
//             data: vec!(vec!(0.0; s); s),
//         }
//     }

//     pub fn get<T>(&self, row: i32, col: i32) -> T
//     {
//         return self.data[row][col];
//     }

//     pub fn mul(&self, m: Matrix<T>)
//     {
//         if self.size == 2 
//         {
//             return Matrix{ self.get(0,0)*get(1,0) + self.get();
//         }
//     }
// }

fn f(n: i64) -> i64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    if n == 2 { return 1; }
    return f(n - 1) + f(n - 2); 
}

fn fi(n: i128) -> i128 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    if n == 2 { return 1; }

    let mut f0: i128 = 0;
    let mut f1: i128 = 1;
    let mut f2: i128 = 1;
    for i in 2 ..=n {
        f2 = f0 + f1; 
        f0 = f1;
        f1 = f2; 
    }
    return f2;
}

fn fg(n: i64) -> i64 {
    let mut f2 = 0;

    return f2;    
}

fn pow(a: f64, n: i64) -> f64 {
    if n == 0 || a == 1.0 { return 1.0; }
    else if n == 1 { return a; }
    let mut answer = 1.0;
    for _i in 0..n
    {
        answer *= a;
    }
    return answer;
}

fn powr(a: f64, n: i64) -> f64 {
    if n == 0 || a == 1.0 { return 1.0; }
    else if n == 1 { return a; }
    else { return powr(a, n - 1) * a; }
}

fn powb(a: f64, n: i64) -> f64 {
    if n == 0 || a == 1.0 { return 1.0; }
    else if n == 1 { return a; }
    if n % 2 == 0 {
        let x = powb(a, n / 2);
        return x * x;
    } else {
        return a * powb(a, n - 1);
    }
}


fn main() {
    // let e: f64 = std::f64::consts::E;
    // println!("{}", e);
    // let _n: f64 = 1000.0;
    // let a: f64 = 1.0 + 1.0 / _n;
    // let n: i64 = _n as i64; 
    // let mut start = Instant::now();
    // println!("pow({},{}) - e = {}, duration {:?}",a,n,pow(a, n) - e, Instant::now().duration_since(start));
    // start = Instant::now();
    // println!("powr({},{}) = {}, duration {:?}",a,n,powr(a, n), Instant::now().duration_since(start));
    // start = Instant::now();
    // println!("powb({},{}) - e = {}, duration {:?}",a,n,powb(a, n) - e, Instant::now().duration_since(start));
    // start = Instant::now();
    // println!("{}.powi({}) - e = {}, duration {:?}",a,n,a.powi(n as i32) - e, Instant::now().duration_since(start));
    // start = Instant::now();
    // println!("{}.powf({}) - e = {}, duration {:?}",a,n,a.powf(n as f64) - e, Instant::now().duration_since(start));
    // start = Instant::now();
    // println!("pown({},{}) - e = {}, duration {:?}",a,n,pown(a, n as usize) - e, Instant::now().duration_since(start));

    //println!("f = {}", f(100));
    println!("fi = {}", fi(100));
    let mut v = vec!(0_i128; 100);
    v = v.iter().enumerate().map(|(i, &x)| fi(i as i128)).collect();
    println!("{:?}",v);   
}

#[cfg(test)]
mod test {
use std::time::{Instant};

    #[test]
    fn f_test() {

        let f_base: Vec<i128> = vec!(0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 
                        89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 
                        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 
                        1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 
                        165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903, 2971215073, 4807526976, 7778742049, 12586269025, 
                        20365011074, 32951280099, 53316291173, 86267571272, 139583862445, 225851433717, 365435296162, 591286729879, 956722026041, 1548008755920, 
                        2504730781961, 4052739537881, 6557470319842, 10610209857723, 17167680177565, 27777890035288, 44945570212853, 72723460248141, 117669030460994, 190392490709135, 
                        308061521170129, 498454011879264, 806515533049393, 1304969544928657, 2111485077978050, 3416454622906707, 5527939700884757, 8944394323791464, 14472334024676221, 23416728348467685, 
                        37889062373143906, 61305790721611591, 99194853094755497, 160500643816367088, 259695496911122585, 420196140727489673, 679891637638612258, 1100087778366101931, 1779979416004714189, 2880067194370816120, 
                        4660046610375530309, 7540113804746346429, 12200160415121876738, 19740274219868223167, 31940434634990099905, 51680708854858323072, 83621143489848422977, 135301852344706746049, 218922995834555169026);
        let mut start = Instant::now();
        // let _f_out = f_base.iter().enumerate().map(|(index, &value)| assert_eq!(value, super::f(index as i64)));
        // println!("f({:?}) duration {:?}", f_base, Instant::now().duration_since(start));
        // start = Instant::now();
        let _fi_out = f_base.iter().enumerate().map(|(index, &value)| assert_eq!(value as i128, super::fi(index as i128)));
        println!("fi({:?}) duration {:?}", f_base, Instant::now().duration_since(start));
    }

    #[test]
    fn pow_test() {
        let e: f64 = std::f64::consts::E;
        let _n: f64 = 10000.0;
        let a: f64 = 1.0 + 1.0 / _n;
        let n: i64 = _n as i64; 
        let eps = 1.0 / (_n - 1.0);
        let mut start = Instant::now();
        assert!(super::pow(a, n) - e  < eps);
        println!("pow({}, {}) duration {:?}", a, n, Instant::now().duration_since(start));
        start = Instant::now();
        assert!(super::powr(a, n) - e  < eps);
        println!("powr({}, {}) duration {:?}", a, n, Instant::now().duration_since(start));
        start = Instant::now();
        assert!(super::powb(a, n) - e  < eps);
        println!("powb({}, {}) duration {:?}", a, n, Instant::now().duration_since(start));
    }
}