const BIAS: i32 = 127;
const RADIX: f32 = 2.;

fn to_parts(a: f32) -> [u32; 3] {
    let bits = a.to_bits();
    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = bits & 0x7fffff;

    [sign, exponent, mantissa]
}

fn decode(parts: [u32; 3]) -> (f32, f32, f32) {
    // book has let sign = (-1.0_f32).powf(sign as f32);
    let sign: f32 = if parts[0] == 0 { 1. } else { -1. };
    let exponent = (parts[1] as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    let fraction = parts[2];
    let mut mantissa = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf( i_ - 23. );
            mantissa += weight;
        }
    }

    (sign, exponent, mantissa)
}

fn from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

fn main() {
    let a: f32 = 42.42;
    let parts = to_parts(a);

    let (sign, exponent, mantissa) = decode(parts);
    let b: f32 = from_parts(sign, exponent, mantissa);

    println!("{} -> {}", a, b);
    println!("f32 as bits {:032b}", a.to_bits());
    println!("sign as bits {:032b}", parts[0]);
    println!("exponent as bits {:032b}", parts[1]);
    println!("mantissa as bits {:032b}", parts[2]);
    println!("{:8} | {:8} | {}", "field", "as bits", "as real number");
    println!("{:8} | {:8b} | {}", "sign", parts[0], sign);
    println!("{:8} | {:08b} | {}", "exponent", parts[1], exponent);
    println!("{:8} | {:023b} | {}", "mantissa", parts[2], mantissa);
}
