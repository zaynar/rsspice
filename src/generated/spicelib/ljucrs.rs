//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

struct SaveVars {
    SHIFT: i32,
    LOWA: i32,
    LOWZ: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SHIFT: i32 = 0;
        let mut LOWA: i32 = 0;
        let mut LOWZ: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            SHIFT,
            LOWA,
            LOWZ,
            FIRST,
        }
    }
}

/// Left-justify, Uppercase, Compress
///
/// Left-justify, uppercase, and space-compress a character string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  N          I      Maximum consecutive occurrences of space.
///  INPUT      I      Input string.
///  OUTPUT     O      Output string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  N        is the maximum number of consecutive occurrences
///           of space that will be allowed to remain in the
///           output string.
///
///  INPUT    is the input string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTPUT   is the output string. This is the input string that
///           left-justified and with all occurrences of more than
///           N consecutive spaces removed.
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
/// ```
///
/// # Particulars
///
/// ```text
///  The string is left-justified and uppercased. Occurrences of more
///  than N consecutive spaces are removed from the input string as it
///  is copied to the output string. If the output string is not large
///  enough to hold the compressed string, it is truncated on the
///  right.
/// ```
///
/// # Examples
///
/// ```text
///  Let N = 1. Then
///
///      ' Abc  DE F  ',           becomes    'ABC DE F',
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 29-JUL-2013 (BVS)
/// ```
pub fn ljucrs(ctx: &mut SpiceContext, n: i32, input: &str, output: &mut str) {
    LJUCRS(
        n,
        input.as_bytes(),
        fstr::StrBytes::new(output).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure LJUCRS ( Left-justify, Uppercase, Compress )
pub fn LJUCRS(N: i32, INPUT: &[u8], OUTPUT: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut J: i32 = 0;
    let mut COUNT: i32 = 0;
    let mut INLEN: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut ICH: i32 = 0;

    //
    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial Data
    //

    //
    // Do some set up stuff the first time through so that we do not
    // need to reinitialize the boundary values used for comparisons
    // and the shift on each call.
    //
    if save.FIRST {
        save.FIRST = false;
        save.LOWA = intrinsics::ICHAR(b"a");
        save.LOWZ = intrinsics::ICHAR(b"z");
        save.SHIFT = (intrinsics::ICHAR(b"A") - save.LOWA);
    }

    //
    // Find out how much space there is in the INPUT and OUTPUT strings
    // and initialize the space counter and output place holder.
    //
    INLEN = intrinsics::LEN(INPUT);
    OUTLEN = intrinsics::LEN(OUTPUT);

    COUNT = 0;
    J = 0;

    for I in 1..=INLEN {
        //
        // Skip leading spaces.
        //
        if ((J == 0) && fstr::eq(fstr::substr(INPUT, I..=I), b" ")) {

            //
            // Another leading space. Skip it.
            //
        } else {
            //
            // Check this character to see if it is a space or not.
            //
            if fstr::eq(fstr::substr(INPUT, I..=I), b" ") {
                COUNT = (COUNT + 1);

                //
                // Copy spaces until enough consecutive spaces
                // have been accumulated. When enough consecutive spaces
                // have accumulated, we no longer copy them.
                //
                if (COUNT <= N) {
                    J = (J + 1);
                    fstr::assign(fstr::substr_mut(OUTPUT, J..=J), fstr::substr(INPUT, I..=I));
                }
            } else {
                //
                // We don't have a space here. Set the space counter to
                // zero.
                //
                COUNT = 0;

                //
                // Copy this character while swapping lowercase with upper
                // case along the way.
                //
                J = (J + 1);

                ICH = intrinsics::ICHAR(fstr::substr(INPUT, I..=I));

                if ((ICH >= save.LOWA) && (ICH <= save.LOWZ)) {
                    fstr::assign(
                        fstr::substr_mut(OUTPUT, J..=J),
                        &intrinsics::CHAR((ICH + save.SHIFT)),
                    );
                } else {
                    fstr::assign(fstr::substr_mut(OUTPUT, J..=J), fstr::substr(INPUT, I..=I));
                }
            }

            if (J == OUTLEN) {
                return;
            }
        }
    }

    //
    // Pad any left over space in the output string with blanks. Note
    // that if the input string was blank, J will be zero at this
    // point and the case below will set the whole output string to
    // blank.
    //
    if (J < OUTLEN) {
        fstr::assign(fstr::substr_mut(OUTPUT, (J + 1)..), b" ");
    }
}
