// use libm::F32Ext;
use libm;

use core::mem;

pub const AUDIO_HZ: usize = 44_100;
// pub const AUDIO_HZ: usize = 11_025;

pub static mut WAV_SCRATCH: [i16; 10 * 2 * AUDIO_HZ] = [0; 10 * 2 * AUDIO_HZ];

pub static mut HEADER_BUFFER: [u8; 100] = [0; 100];

// Thank you John Cook
//      https://www.johndcook.com/blog/2017/05/31/listening-to-golden-angles
pub fn tone(freq: f32, decay: f32, t: f32) -> f32 {
    let y = libm::sinf(freq * 2.0 * 3.14159 * t);
    if decay > 0. {
        abort!("Not yet");
        // k = np.log(2) / decay
        // y *= np.exp(-k*t)
    }
    y
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

pub fn write_song(wav_data: &mut [i16]) {
    use libm::sinf;

    // t in [0, 1]
    fn envelope_sin(t: f32) -> f32 {
        sinf(3.1415 * t)
    }
    fn envelope_into(t: f32) -> f32 {
        const INTRO: f32 = 0.05;
        if t < INTRO {
            t
        } else if t > (1. - INTRO) {
            // point-slope form
            // ...still pops
            let e = -1. / INTRO * (t - (1. - INTRO)) + 1.;
            e.max(0.).min(1.)
        } else {
            1.
        }
    }

    for t in 0..=100 {
        println!("t = {}, e = {}", t, envelope_into(t as f32 / 100.));
    }

    let n_samples = wav_data.len() / 2;
    for i in 0..n_samples {
        let j = 2 * i;
        let t: f32 = j as f32 / AUDIO_HZ as f32;
        let tone = match (j / AUDIO_HZ) % 7 {
            0 => tone(A3, 0., t),
            1 => tone(B3, 0., t),
            2 => tone(C4, 0., t),
            3 => tone(D4, 0., t),
            4 => tone(E4, 0., t),
            5 => tone(F4, 0., t),
            6 => tone(G4, 0., t),
            _ => {
                abort!("math is wrong");
            },
        };

        const SCALE: f32 = (1 << 15) as f32;
        let scaled = (envelope_into(t % 1.0) * tone * SCALE) as i16;
        wav_data[j + 0] = scaled;
        wav_data[j + 1] = scaled;
    }
}

pub fn play() {
    unsafe {
        println!("{} audio samples generating...", WAV_SCRATCH.len());
        write_song(&mut WAV_SCRATCH);
        println!("Done!");
    }

    unsafe {
        use winapi::shared::mmreg;
        use winapi::um::mmeapi;
        use winapi::um::mmsystem;

        println!("Found {} WaveOut device(s)", mmeapi::waveOutGetNumDevs());

        let mut h_wave = mem::zeroed();
        let mut mm_res;

        let samples_per_sec: u32 = AUDIO_HZ as u32 * 3 / 4;
        let bits_per_sample: u32 = 16;
        let block_align: u32 = bits_per_sample / 8;
        let mut format = mmreg::WAVEFORMATEX {
            wFormatTag:      mmreg::WAVE_FORMAT_PCM,
            nChannels:       1,
            nSamplesPerSec:  samples_per_sec,
            nAvgBytesPerSec: samples_per_sec * block_align,
            nBlockAlign:     block_align as u16,
            wBitsPerSample:  bits_per_sample as u16,
            cbSize:          0,
        };

        mm_res = mmeapi::waveOutOpen(
            &mut h_wave,
            mmsystem::WAVE_MAPPER,
            &mut format,
            0,
            0,
            mmsystem::CALLBACK_NULL,
        );
        println!("h_wave = 0x{:x}", h_wave as usize);
        if mm_res != 0 {
            println!("mm_res = 0x{}", mm_res);
        }

        let p_header: *mut mmsystem::WAVEHDR =
            HEADER_BUFFER.as_mut().as_ptr() as *mut _;

        *p_header = mmsystem::WAVEHDR {
            dwBufferLength: WAV_SCRATCH.len() as u32,
            lpData: WAV_SCRATCH.as_ptr() as *mut _,
            ..mem::zeroed()
        };

        mm_res = mmeapi::waveOutPrepareHeader(
            h_wave,
            p_header,
            mem::size_of_val(&*p_header) as u32,
        );
        if mm_res != 0 {
            println!("mm_res = 0x{}", mm_res);
        }

        mm_res = mmeapi::waveOutWrite(
            h_wave,
            p_header,
            mem::size_of_val(&*p_header) as u32,
        );
        if mm_res != 0 {
            println!("mm_res = 0x{}", mm_res);
        }
    }
}
