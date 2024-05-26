fn mock_rand_naive(n: u8) -> f32 {
    (n as f32) / 255.
}

fn mock_rand(n: u8) -> f32 {
    println!("n {n:39b}");
    let base: u32 = 0b0_01111110_00000000000000000000000;
    println!("base {base:36b}");
    let large_n = (n as u32) << 15;
    println!("large_n {large_n:33b}");
    let f32_bits = base | large_n;
    println!("f32_bits {f32_bits:32b}");
    let m = f32::from_bits(f32_bits);
    println!("m {m}");
    2.0 * ( m - 0.5 )
}

fn main() {
    println!("naive {} rand {}", mock_rand_naive(5), mock_rand(5));
    println!("naive {} rand {}", mock_rand_naive(215), mock_rand(215));

}
