#[allow(clippy::approx_constant, clippy::suboptimal_flops)]
fn sinpi_remez(x: f32) -> f32 {
    let xx = x * x;

    let y = 0.078_222_424 * xx - 0.598_625_54;
    let y = y * xx + 2.550_144_2;
    let y = y * xx - 5.167_715;
    let y = y * xx + 3.141_592_7;

    x * y
}

#[allow(clippy::suboptimal_flops)]
fn sinpi_taylor_11(x: f32) -> f32 {
    let x = x * core::f32::consts::PI;
    let xx = x * x;

    let y = -2.505_210_8e-8 * xx + 2.755_731_9e-6;
    let y = y * xx - 0.000_198_412_7;
    let y = y * xx + 0.008_333_334;
    let y = y * xx - 0.166_666_67;
    let y = y * xx + 1.0;

    x * y
}

#[allow(clippy::suboptimal_flops)]
fn sinpi_taylor_13(x: f32) -> f32 {
    let x = x * core::f32::consts::PI;
    let xx = x * x;

    let y = 1.605_904_4e-10 * xx - 2.505_210_8e-8;
    let y = y * xx + 2.755_731_9e-6;
    let y = y * xx - 0.000_198_412_7;
    let y = y * xx + 0.008_333_334;
    let y = y * xx - 0.166_666_67;
    let y = y * xx + 1.0;

    x * y
}

/// Compute the maximum error of an estimator of sin(π`x`) in the range of
/// [`f32::MIN_POSITIVE`]..=0.5.
///
/// This function returns a tuple of the input value `x` and the maximum error
/// in `f32` ULPs.
fn compute_max_error(f: fn(f32) -> f32) -> (f32, f64) {
    let min = f32::MIN_POSITIVE.to_bits();
    let max = 0.5_f32.to_bits();

    (min..=max)
        .map(f32::from_bits)
        .map(|x| {
            const SHIFT: u32 = f64::MANTISSA_DIGITS - f32::MANTISSA_DIGITS;
            let y: f64 = f(x).into();
            let expected = core_math::sinpi(x.into());
            let ulps = y.to_bits().abs_diff(expected.to_bits());

            #[allow(clippy::cast_precision_loss)]
            (x, ulps as f64 * (-f64::from(SHIFT)).exp2())
        })
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .expect("The domain should not be empty")
}

fn main() {
    println!("Remez[9]: {:?}", compute_max_error(sinpi_remez));
    println!("Taylor[11]: {:?}", compute_max_error(sinpi_taylor_11));
    println!("Taylor[13]: {:?}", compute_max_error(sinpi_taylor_13));
}
