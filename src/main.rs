use f128::f128;
use rug::*;
// use num::Complex;

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

    // ::::: ::::: ::::: ::::: ::::: ::::: ::::: ::::: :::::
    // ::::: ::::: ::::: ::::: 複素数型 ::::: ::::: ::::: :::::
    // ::::: ::::: ::::: ::::: ::::: ::::: ::::: ::::: :::::

    
    // f32 prec complex
    let comp32: num::Complex<f32> = num::Complex{re: 10.0, im: 15.0};
    let comp32_devider: num::Complex<f32> = num::Complex{re: 3.0, im: 3.0};
    println!("f32 prec complex -> {}", comp32 / comp32_devider); // 4.1666665+0.8333333i
    
    // f64 prec complex
    let comp64: num::Complex<f64> = num::Complex{re: 10.0, im: 15.0};
    let comp64_devider: num::Complex<f64> = num::Complex{re: 3.0, im: 3.0};
    println!("{}", comp64 / comp64_devider); // 4.166666666666667+0.8333333333333334i


    // f128 prec complex of rug
    let prec128_comp_rug = 128;
    let comp128_divider_rug = rug::Complex::with_val(prec128_comp_rug, (10.0, 15.0));
    let comp128_rug = rug::Complex::with_val(prec128_comp_rug, (3.0, 3.0));
    println!("f64 prec complex -> {}", comp128_rug / comp128_divider_rug); // (2.307692307692307692307692307692307692305e-1 -4.615384615384615384615384615384615384615e-2)

    // f256 prec complex of rug
    let prec256_comp_rug = 256;
    let comp256_divider_rug = rug::Complex::with_val(prec256_comp_rug, (10.0, 15.0));
    let comp256_rug = rug::Complex::with_val(prec256_comp_rug, (3.0, 3.0));
    println!("f256 prec complex of rug -> {}", comp256_rug / comp256_divider_rug); // (2.307692307692307692307692307692307692307692307692307692307692307692307692307697e-1 -4.615384615384615384615384615384615384615384615384615384615384615384615384615395e-2)

    // f512 prec complex of rug
    let prec512_comp_rug = 512;
    let comp512_divider_rug: rug::Complex = rug::Complex::with_val(prec512_comp_rug, (10.0, 15.0));
    let comp512_rug = rug::Complex::with_val(prec512_comp_rug, (3.0, 3.0));
    println!("f512 prec complex of rug -> {}", comp512_rug / comp512_divider_rug); // (2.30769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230769230763e-1 -4.61538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461538461536e-2)
}
