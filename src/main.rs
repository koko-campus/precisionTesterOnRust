use f128::f128;
use rug::*;


fn main() {
    // f32 prec
    let f32_prec = 10 as f32 / 3 as f32;
    println!("f32 -> \"{}\"", f32_prec); // 3.3333333

    // f64 prec
    let f64_prec = 10 as f64 / 3 as f64;
    println!("f64 -> \"{}\"", f64_prec); // 3.3333333333333335

    // f128 prec
    let f128_prec = f128::from(10) / f128::from(3);
    println!("f128 -> \"{}\"", f128_prec); // 3.33333

    // 次にRugを使用して任意精度で♪

    // f16 prec of rug
    let prec16 = 16;
    let f16_prec_rug = Float::with_val(prec16, 10) / Float::with_val(prec16, 3);
    println!("f16 of rug -> \"{}\"", f16_prec_rug); // 3.33331

    // f128 prec of rug
    let prec128 = 128;
    let f128_prec_rug = Float::with_val(prec128, 10) / Float::with_val(prec128, 3);
    println!("f128 of rug -> \"{}\"", f128_prec_rug); // 3.333333333333333333333333333333333333329

    // f256 prec of rug
    let prec256 = 256;
    let f256_prec_rug = Float::with_val(prec256, 10) / Float::with_val(prec256, 3);
    println!("f256 of rug -> \"{}\"", f256_prec_rug); // 3.333333333333333333333333333333333333333333333333333333333333333333333333333322

    // f512 prec of rug
    let prec512 = 512;
    let f512_prec_rug = Float::with_val(prec512, 10) / Float::with_val(prec512, 3);
    println!("f512 of rug -> \"{}\"", f512_prec_rug); // 3.33333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333333323
}
