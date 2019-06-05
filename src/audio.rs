use libm::F32Ext;

pub const AUDIO_HZ: usize = 44_100;

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

// http://www.sengpielaudio.com/calculator-notenames.htm
pub const A3: f32 = 220.000;
pub const B3: f32 = 246.942;
pub const C4: f32 = 261.626;
pub const D4: f32 = 293.665;
pub const E4: f32 = 329.628;
pub const F4: f32 = 349.228;
pub const G4: f32 = 391.995;

pub fn write_song(wav_data: &mut [u16]) {
    for i in 0..wav_data.len() {
        let t: f32 = i as f32 / AUDIO_HZ as f32;
        let tone = match (i / AUDIO_HZ) % 7 {
            0 => tone(A3, 0., t) + tone(B3, 0., t),
            1 => tone(B3, 0., t) + tone(C4, 0., t),
            2 => tone(C4, 0., t) + tone(D4, 0., t),
            3 => tone(D4, 0., t) + tone(E4, 0., t),
            4 => tone(E4, 0., t) + tone(F4, 0., t),
            5 => tone(F4, 0., t) + tone(G4, 0., t),
            6 => tone(G4, 0., t) + tone(A3, 0., t),
            _ => {
                abort!("math is wrong");
                0.
            },
        };

        const SCALE: f32 = (1 << 12) as f32;
        wav_data[i] = (tone * SCALE + 0.5) as u16;
    }
}
