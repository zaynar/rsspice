//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Replace a word
///
/// Replace the Nth word in a string with a new word.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  INSTR      I   Input string.
///  NTH        I   Number of the word to be replaced.
///  NEW        I   Replacement word.
///  OUTSTR     O   Output string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  INSTR    is the input character string, possibly containing
///           one or more words, where a word is any string of
///           consecutive non-blank characters delimited by a
///           blank or by either end of the string.
///
///  NTH      is the number of the word to be replaced. Words
///           are numbered from one. If NTH is less than one,
///           or greater than the number of words in the string,
///           no replacement is made.
///
///  NEW      is the word which is to replace the specified word
///           in the input string. Leading and trailing blanks
///           are ignored. If the replacement word is blank,
///           the original word is simply removed.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUTSTR   is the output string. This is the input string
///           with the N'th word replaced by the word NEW.
///           Any blanks originally surrounding the replaced
///           word are retained.
///
///           OUTSTR may overwrite INSTR.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If NEW is blank, then the Nth word is replaced by a single
///      space.
/// ```
///
/// # Particulars
///
/// ```text
///  The effect of this routine is to remove the old word with
///  REMSUB, and insert the replacement word with INSSUB.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///        INSTR  = '  Woodsy is the Anti-Pollution  Owl.'
///
///  and
///        NEW    = '   an   '
///
///  then the following values of NTH yield the following strings.
///
///        NTH      OUTSTR
///        ---      ------------------------------------------
///         -1      '  Woodsy is the Anti-Pollution  Owl.'
///          0      '  Woodsy is the Anti-Pollution  Owl.'
///          1      '  an is the Anti-Pollution  Owl.'
///          3      '  Woodsy is an Anti-Pollution  Owl.'
///          4      '  Woodsy is the an  Owl.'
///          5      '  Woodsy is the Anti-Pollution  an'
///          6      '  Woodsy is the Anti-Pollution  Owl.'
///
///  Note that in the first, second, and last cases, the string
///  was not changed. Note also that in the next to last case,
///  the final period was treated as part of the fifth word in the
///  string.
///
///  If NEW is ' ', and NTH is 3, then
///
///        OUTSTR = '  Woodsy is Anti-Pollution  Owl.'
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (WLT) (HAN) (NJB)
/// ```
pub fn replwd(instr: &str, nth: i32, new: &str, outstr: &mut str) {
    REPLWD(
        instr.as_bytes(),
        nth,
        new.as_bytes(),
        fstr::StrBytes::new(outstr).as_mut(),
    );
}

//$Procedure REPLWD ( Replace a word )
pub fn REPLWD(INSTR: &[u8], NTH: i32, NEW: &[u8], OUTSTR: &mut [u8]) {
    let mut SHORT = [b' '; 2];
    let mut BEGIN: i32 = 0;
    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;
    let mut K: i32 = 0;
    let mut N: i32 = 0;
    let mut SHIFT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // First just shift the input string into the output string,
    // then do everything in place (for the case when the new
    // word is longer than the old one.  When its shorter we'll
    // need to change this scheme slightly.)
    //
    fstr::assign(OUTSTR, INSTR);
    //
    // Where does the word to be replaced begin? If there is none,
    // just return the original string.
    //
    NTHWD(OUTSTR, NTH, &mut SHORT, &mut BEGIN);

    if (BEGIN == 0) {
        return;
    }
    //
    // Otherwise, find out where it ends as well.
    //
    FNDNWD(INSTR, BEGIN, &mut I, &mut J);

    //
    // Now insert only the non-blank part of the replacement string.
    // If the replacement string is blank, don't insert anything.
    //
    if fstr::ne(NEW, b" ") {
        F = FRSTNB(NEW);
        L = LASTNB(NEW);

        //
        // Except in the lucky case that the word to insert is the
        // same length as the word it's replacing, we will have
        // to shift right or left by some amount.  Compute the
        // appropriate amount to shift right.
        //
        SHIFT = ((L - F) - (J - I));
    } else {
        F = 1;
        L = 1;
        SHIFT = (I - J);
    }

    if (SHIFT > 0) {
        //
        // To shift right in place start at the right most character
        // of the string and copy the character SHIFT spaces to the
        // left.
        //
        K = intrinsics::LEN(OUTSTR);
        N = (K - SHIFT);

        while (N > J) {
            let val = fstr::substr(OUTSTR, N..=N).to_vec();
            fstr::assign(fstr::substr_mut(OUTSTR, K..=K), &val);
            K = (K - 1);
            N = (N - 1);
        }

        //
        // Once the appropriate characters have been shifted out
        // of the way, replace the opened space with the new
        // word.
        //
        while ((F <= L) && (I <= intrinsics::LEN(OUTSTR))) {
            fstr::assign(fstr::substr_mut(OUTSTR, I..=I), fstr::substr(NEW, F..=F));
            F = (F + 1);
            I = (I + 1);
        }
    } else {
        //
        // We have a left shift. Fill in the first part of the word
        // we are replacing with the new one.
        //
        while ((F <= L) && (I <= intrinsics::LEN(OUTSTR))) {
            fstr::assign(fstr::substr_mut(OUTSTR, I..=I), fstr::substr(NEW, F..=F));
            F = (F + 1);
            I = (I + 1);
        }

        //
        // Now starting just past the end of the word we are replacing
        // shift the remainder of string left one character at a time.
        //
        if (SHIFT < 0) {
            J = (J + 1);

            while ((I <= intrinsics::LEN(OUTSTR)) && (J <= intrinsics::LEN(INSTR))) {
                fstr::assign(fstr::substr_mut(OUTSTR, I..=I), fstr::substr(INSTR, J..=J));
                I = (I + 1);
                J = (J + 1);
            }

            //
            // Finally pad the string with blanks.
            //
            if (I <= intrinsics::LEN(OUTSTR)) {
                fstr::assign(fstr::substr_mut(OUTSTR, I..), b" ");
            }
        }
    }
}
