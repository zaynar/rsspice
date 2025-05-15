//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Compress a character string
///
/// Compress a character string by removing occurrences of
/// more than N consecutive occurrences of a specified
/// character.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  DELIM      I   Delimiter to be compressed.
///  N          I   Maximum consecutive occurrences of DELIM.
///  INPUT      I   Input string.
///  OUTPUT     O   Compressed string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DELIM    is the delimiter to be compressed out of the string.
///           This may be any ASCII character.
///
///  N        is the maximum number of consecutive occurrences
///           of DELIM that will be allowed to remain in the
///           output string.
///
///  INPUT    is the input string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTPUT   is the output string. This is the input string
///           with all occurrences of more than N consecutive
///           delimiters removed.
///
///           If OUTPUT is not large enough to hold the
///           compressed string, it is truncated on the right.
///
///           OUTPUT may overwrite INPUT.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the output string length is too short to contain the result
///      of compressing the input string, the result is truncated on
///      the right.
/// ```
///
/// # Particulars
///
/// ```text
///  Occurrences of more than N consecutive delimiters are removed
///  from the input string as it is copied to the output string.
///  If the output string is not large enough to hold the compressed
///  string, it is truncated on the right.
/// ```
///
/// # Examples
///
/// ```text
///  Let DELIM = '.', and N = 2. Then
///
///     'ABC...DE.F...',           becomes   'ABC..DE.F..'
///     ' ...........'                       ' ..'
///     '.. ..AB....CD'                      '.. ..AB..CD'
///
///  Let DELIM = ' ', and N = 0. Then
///
///     ' DISK:[USER.  SUB  ]'     becomes   'DISK:[USER.SUB]'
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 09-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Added $Exceptions entry #1.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (IMU)
/// ```
pub fn cmprss(delim: char, n: i32, input: &str, output: &mut str) {
    CMPRSS(
        &[u8::try_from(delim).unwrap()],
        n,
        input.as_bytes(),
        fstr::StrBytes::new(output).as_mut(),
    );
}

//$Procedure CMPRSS ( Compress a character string )
pub fn CMPRSS(DELIM: &[u8], N: i32, INPUT: &[u8], OUTPUT: &mut [u8]) {
    let DELIM = &DELIM[..1];
    let mut J: i32 = 0;
    let mut COUNT: i32 = 0;
    let mut INLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;

    //
    //
    // Local Variables
    //

    //
    // Find out how much space there is in the INPUT and OUTPUT strings
    // and initialize the delimiter counter and output place holder.
    //
    INLEN = intrinsics::LEN(INPUT);
    OUTLEN = intrinsics::LEN(OUTPUT);

    COUNT = 0;
    J = 0;

    for I in 1..=INLEN {
        //
        // Check each character to see if it is a delimiter or not.
        //
        if fstr::eq(fstr::substr(INPUT, I..=I), DELIM) {
            COUNT = (COUNT + 1);

            //
            // Copy delimiters until enough consecutive delimiters
            // have been accumulated.  When enough consecutive delimiters
            // have accumulated, we no longer copy them.
            //
            if (COUNT <= N) {
                J = (J + 1);
                fstr::assign(fstr::substr_mut(OUTPUT, J..=J), fstr::substr(INPUT, I..=I));
            }
        } else {
            //
            // We don't have a delimiter here so we just copy the
            // character and set the delimiter counter to zero.
            //
            COUNT = 0;
            J = (J + 1);
            fstr::assign(fstr::substr_mut(OUTPUT, J..=J), fstr::substr(INPUT, I..=I));
        }

        if (J == OUTLEN) {
            return;
        }
    }

    //
    // Pad any left over space in the output string with blanks.
    //
    if (J < OUTLEN) {
        fstr::assign(fstr::substr_mut(OUTPUT, (J + 1)..), b" ");
    }
}
