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

/// Convert to uppercase
///
/// Convert the characters in a string to uppercase.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  OUT        O   Output string, all uppercase.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is the input string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the output string. This is the input string
///           with all lowercase letters converted to uppercase.
///           Non-letters are not affected.
///
///           OUT may overwrite IN.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the output string length is less than the input string
///      length, the result will be truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  Convert each lowercase character in IN to uppercase.
/// ```
///
/// # Examples
///
/// ```text
///  'This is an example'   becomes   'THIS IS AN EXAMPLE'
///  '12345 +-=? > * $ &'             '12345 +-=? > * $ &'
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 05-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Added entry #1 in $Exceptions section.
///
/// -    SPICELIB Version 1.1.0, 13-MAR-1996 (KRG)
///
///         Removed the calls to the string lexicographic functions.
///
///         Modified the algorithm to use the ICHAR() intrinsic function
///         and some local integer storage for the bases of the lower and
///         upper case letters.
///
///         Added a "FIRST" clause to the code so that the lower and
///         upper case bases and the separation between them are only
///         initialized the first time the subroutine is called rather
///         than every time.
///
///         These changes were made to improve the execution speed of
///         the subroutine
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn ucase(ctx: &mut SpiceContext, in_: &str, out: &mut str) {
    UCASE(
        in_.as_bytes(),
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure UCASE ( Convert to uppercase )
pub fn UCASE(IN: &[u8], OUT: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut ICH: i32 = 0;

    //
    // Local variables
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
    // Move the string from IN to OUT. Step through OUT one character
    // at a time, translating letters between 'a' and 'z' to uppercase.
    //
    fstr::assign(OUT, IN);

    for I in 1..=intrinsics::LEN(OUT) {
        ICH = intrinsics::ICHAR(fstr::substr(OUT, I..=I));

        if ((ICH >= save.LOWA) && (ICH <= save.LOWZ)) {
            fstr::assign(
                fstr::substr_mut(OUT, I..=I),
                &intrinsics::CHAR((ICH + save.SHIFT)),
            );
        }
    }
}
