fn F(n: i32) -> f32
{
    if n <= 1 n;
    else F(n - 1) + F(n - 2); 
}


fn main() {
    println!("F{}", F(10));
}
