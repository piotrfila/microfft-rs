//! FFT on complex inputs (CFFT)

use crate::{cfft::*, Complex32};

/// Perform an in-place 2-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_2};
///
/// let input = [Complex32::default(); 2];
/// let result = cfft_2(input);
/// ```
#[inline]
#[must_use]
pub fn cfft_2(input: [Complex32; 2]) -> [Complex32; 2] {
    CFftN2::transform(input)
}

/// Perform an in-place 4-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_4};
///
/// let input = [Complex32::default(); 4];
/// let result = cfft_4(input);
/// ```
#[inline]
#[must_use]
pub fn cfft_4(input: [Complex32; 4]) -> [Complex32; 4] {
    CFftN4::transform(input)
}

/// Perform an in-place 8-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_8};
///
/// let input = [Complex32::default(); 8];
/// let result = cfft_8(input);
/// ```
#[cfg(any(
    feature = "maxn-8",
    feature = "maxn-16",
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_8(input: [Complex32; 8]) -> [Complex32; 8] {
    CFftN8::transform(input)
}

/// Perform an in-place 16-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_16};
///
/// let input = [Complex32::default(); 16];
/// let result = cfft_16(input);
/// ```
#[cfg(any(
    feature = "maxn-16",
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_16(input: [Complex32; 16]) -> [Complex32; 16] {
    CFftN16::transform(input)
}

/// Perform an in-place 32-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_32};
///
/// let input = [Complex32::default(); 32];
/// let result = cfft_32(input);
/// ```
#[cfg(any(
    feature = "maxn-32",
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_32(input: [Complex32; 32]) -> [Complex32; 32] {
    CFftN32::transform(input)
}

/// Perform an in-place 64-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_64};
///
/// let input = [Complex32::default(); 64];
/// let result = cfft_64(input);
/// ```
#[cfg(any(
    feature = "maxn-64",
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_64(input: [Complex32; 64]) -> [Complex32; 64] {
    CFftN64::transform(input)
}

/// Perform an in-place 128-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_128};
///
/// let input = [Complex32::default(); 128];
/// let result = cfft_128(input);
/// ```
#[cfg(any(
    feature = "maxn-128",
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_128(input: [Complex32; 128]) -> [Complex32; 128] {
    CFftN128::transform(input)
}

/// Perform an in-place 256-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_256};
///
/// let input = [Complex32::default(); 256];
/// let result = cfft_256(input);
/// ```
#[cfg(any(
    feature = "maxn-256",
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_256(input: [Complex32; 256]) -> [Complex32; 256] {
    CFftN256::transform(input)
}

/// Perform an in-place 512-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_512};
///
/// let input = [Complex32::default(); 512];
/// let result = cfft_512(input);
/// ```
#[cfg(any(
    feature = "maxn-512",
    feature = "maxn-1024",
    feature = "maxn-2048",
    feature = "maxn-4096",
))]
#[inline]
#[must_use]
pub fn cfft_512(input: [Complex32; 512]) -> [Complex32; 512] {
    CFftN512::transform(input)
}

/// Perform an in-place 1024-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_1024};
///
/// let input = [Complex32::default(); 1024];
/// let result = cfft_1024(input);
/// ```
#[cfg(any(feature = "maxn-1024", feature = "maxn-2048", feature = "maxn-4096"))]
#[inline]
#[must_use]
pub fn cfft_1024(input: [Complex32; 1024]) -> [Complex32; 1024] {
    CFftN1024::transform(input)
}

/// Perform an in-place 2048-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_2048};
///
/// let input = [Complex32::default(); 2048];
/// let result = cfft_2048(input);
/// ```
#[cfg(any(feature = "maxn-2048", feature = "maxn-4096"))]
#[inline]
#[must_use]
pub fn cfft_2048(input: [Complex32; 2048]) -> [Complex32; 2048] {
    CFftN2048::transform(input)
}

/// Perform an in-place 4096-point CFFT.
///
/// # Example
///
/// ```
/// use microfft::{Complex32, complex::cfft_4096};
///
/// let input = [Complex32::default(); 4096];
/// let result = cfft_4096(input);
/// ```
#[cfg(any(feature = "maxn-4096"))]
#[inline]
#[must_use]
pub fn cfft_4096(input: [Complex32; 4096]) -> [Complex32; 4096] {
    CFftN4096::transform(input)
}
