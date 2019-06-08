// use libm::F32Ext;
use libm;

use core::mem;

pub const AUDIO_HZ: usize = 44_100;
// pub const AUDIO_HZ: usize = 11_025;

pub static mut WAV_SCRATCH: [u16; 90 * AUDIO_HZ] = [0; 90 * AUDIO_HZ];

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
pub const A4: f32 = 440.000;

fn sawtooth(t: f32) -> f32 {
    t % 1.0
}

fn f2i(f: f32) -> u16 {
    (f * (1 << 12) as f32) as u16
}

pub fn write_song(wav_data: &mut [u16]) {
    use libm::{
        cosf,
        powf,
        sinf,
    };

    // t in [0, 1]
    fn envelope_sin(t: f32) -> f32 {
        sinf(3.1415 * t)
    }

    let n_samples = wav_data.len() / 2;
    for i in 0..n_samples {
        let t: f32 = i as f32 / (AUDIO_HZ as f32 * 4. / 3.);
        let e: f32 = if t < 1. {
            sinf(0.5 * 3.14159 * (t / 1.))
        } else if 40. < t && t > 41. {
            cosf((t - 40.) * 3.14159 / 2.)
        } else {
            1.
        };

        // Pick voices
        let mut amp = 0.;

        let octave = powf(2.0, -18. / 12.);
        const OFFSET: f32 = 0.31415;
        // Every time
        amp += 0.5 * sawtooth(octave * C4 * t);
        amp += 0.5 * sawtooth(octave * E4 * t);

        // Every other
        if (t as u32) % 2 == 1 {
            amp += 0.5 * sawtooth(octave * C4 * t + OFFSET);
            amp += 0.5 * sawtooth(octave * E4 * t + OFFSET);
        }

        if ((t as u32) % 4 == 3) && (t < 35.) {
            amp += envelope_sin(t) * (sawtooth(octave / 2. * C4 * t - OFFSET));
            amp += envelope_sin(t) * (sawtooth(octave / 2. * E4 * t - OFFSET));
        }

        if t > 35. {
            if (t as u32) % 4 == 3 {
                amp +=
                    envelope_sin(t) * (sawtooth(octave / 2. * C4 * t + OFFSET));
                amp +=
                    envelope_sin(t) * (sawtooth(octave / 2. * E4 * t + OFFSET));
            }
        }

        wav_data[i] = f2i(e * amp);
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

        let samples_per_sec: u32 = AUDIO_HZ as u32;
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
