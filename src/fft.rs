use libc;
use libc::c_int;

// Thanks bro
// https://www.nayuki.io/page/free-small-fft-in-multiple-languages
extern "C" {
    // Computes the discrete Fourier transform (DFT) of the given complex
    // vector, storing the result back into the vector. The vector can have
    // any length. This is a wrapper function. Returns true if successful, false
    // otherwise (out of memory).
    #[link_name = "Fft_transform"]
    pub fn transform(real: *mut f64, imag: *mut f64, n: isize) -> c_int;


    // Computes the inverse discrete Fourier transform (IDFT) of the given
    // complex vector, storing the result back into the vector. The vector
    // can have any length. This is a wrapper function. This transform does not
    // perform scaling, so the inverse is not a true inverse. Returns true
    // if successful, false otherwise (out of memory).
    #[link_name = "Fft_inverseTransform"]
    pub fn inverse_transform(real: *mut f64, imag: *mut f64, n: isize)
        -> c_int;


    // Computes the discrete Fourier transform (DFT) of the given complex
    // vector, storing the result back into the vector. The vector's length
    // must be a power of 2. Uses the Cooley-Tukey decimation-in-time radix-2
    // algorithm. Returns true if successful, false otherwise (n is not a
    // power of 2, or out of memory).
    #[link_name = "Fft_transformRadix2"]
    pub fn transform_radix2(real: *mut f64, imag: *mut f64, n: isize) -> c_int;


    // Computes the discrete Fourier transform (DFT) of the given complex
    // vector, storing the result back into the vector. The vector can have
    // any length. This requires the convolution function, which in turn
    // requires the radix-2 FFT function. Uses Bluestein's chirp z-transform
    // algorithm. Returns true if successful, false otherwise (out of memory).
    #[link_name = "Fft_transformBluestein"]
    pub fn transform_bluestein(
        real: *mut f64,
        imag: *mut f64,
        n: isize,
    ) -> c_int;


    // Computes the circular convolution of the given real vectors. Each
    // vector's length must be the same. Returns true if successful, false
    // otherwise (out of memory).
    #[link_name = "Fft_convolveReal"]
    pub fn convolve_real(
        x: *const f64,
        y: *const f64,
        out: *mut f64,
        n: isize,
    ) -> c_int;


    // Computes the circular convolution of the given complex vectors. Each
    // vector's length must be the same. Returns true if successful, false
    // otherwise (out of memory).
    #[link_name = "Fft_convolveComplex"]
    pub fn convolve_complex(
        xreal: *const f64,
        ximag: *const f64,
        yreal: *const f64,
        yimag: *const f64,
        outreal: *mut f64,
        outimag: *mut f64,
        n: isize,
    ) -> c_int;

}
