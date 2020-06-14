use std::f32::consts;

const TWO_PI: f32 = consts::PI * 2f32;
const THREE_HALVES_PI: f32 = consts::PI * 1.5f32;

/// Calculates the arctangent, given a quotient in the range [-1, 1].
///
/// Obtained from [IEEE Signal Processing Magazine](http://www-labs.iro.umontreal.ca/~mignotte/IFT2425/Documents/EfficientApproximationArctgFunction.pdf).
///
/// # Parameters
///
/// * `quotient` - the minimum of `cos / sin` and `sin / cos`.
pub fn atan_approx(quotient: f32) -> f32 {
    (consts::FRAC_PI_4 + 0.273f32 * (1f32 - quotient.abs())) * quotient
}

/// Calculates the arctangent from the cosine and sine.
///
/// # Parameters
///
/// * `cos` - the cosine/x term.
/// * `sin` - the sine/y term.
pub fn atan2_approx(cos: f32, sin: f32) -> f32 {
    if cos.abs() > sin.abs() {
        if cos < 0f32 {
            atan_approx(sin / cos) + consts::PI
        } else if sin < 0f32 {
            atan_approx(sin / cos) + TWO_PI
        } else {
            atan_approx(sin / cos)
        }
    } else if sin < 0f32 {
        -atan_approx(cos / sin) + THREE_HALVES_PI
    } else {
        -atan_approx(cos / sin) + consts::FRAC_PI_2
    }
}
