//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const PICLEN: i32 = 11;

//
// The utility routine T_ITHSYM creates the name of the ith
// symbol.  The names created by this routine have the form
//
//    XXX...XXXnnn
//
// where nnn is the input number N, printed with a sufficient
// number of leading zeros so that all numbers in the range
// 1:NMAX have the same width.  The number is padded with
// leading 'X' characters up to the length of the string NAME.
//
pub fn T_ITHSYM(N: i32, NMAX: i32, NAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut PICTUR = [b' '; PICLEN as usize];
    let mut NPAD: i32 = 0;
    let mut NPICT: i32 = 0;
    let mut SLEN: i32 = 0;
    let mut NDIGIT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Get the length of the output name.
    //
    SLEN = intrinsics::LEN(NAME);

    //
    // Determine the number of digits required.
    //
    NDIGIT = ((f64::log10(((NMAX as f64) + 0.5)) as i32) + 1);

    //
    // Create the format picture for the names.
    //
    fstr::assign(&mut PICTUR, b"0");
    NPICT = intrinsics::MIN0(&[SLEN, NDIGIT]);

    for I in 2..=NPICT {
        fstr::assign(fstr::substr_mut(&mut PICTUR, I..=I), b"X");
    }

    spicelib::DPFMT((N as f64), &PICTUR, NAME, ctx)?;

    //
    // Add non-blank leading padding to the name.
    //
    NPAD = (SLEN - NPICT);

    if (NPAD > 0) {
        spicelib::RJUST(&NAME.to_vec(), NAME);

        for I in 1..=NPAD {
            fstr::assign(fstr::substr_mut(NAME, I..=I), b"X");
        }
    }

    Ok(())
}
