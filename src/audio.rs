// Thank you John Cook
//      https://www.johndcook.com/blog/2017/05/31/listening-to-golden-angles

pub fn tone(freq: f32, decay: f32, t: f32) -> f32 {
    let y = libm::sinf(freq * 2.0 * 3.14159 * t);
    if decay > 0. {
        abort!("Not yet");
        // k = np.log(2) / decay
        // y *= np.exp(-k*t)
    }
    return y;
}

// # Scale signal to 16-bit integers,
// # values between -2^15 and 2^15 - 1.
// def to_integer(signal):
//     signal /= max(abs(signal))
//     return np.int16(signal*(2**15 - 1))

// C = 262 # middle C frequency in Hz
