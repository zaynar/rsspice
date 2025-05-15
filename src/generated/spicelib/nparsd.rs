//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BASE: f64 = 10.0;
const DIGIT: i32 = 1;
const DPOINT: i32 = 2;
const EXPONT: i32 = 3;
const IGNORE: i32 = 4;
const BPISTR: i32 = 5;
const EPISTR: i32 = 6;
const SIGN: i32 = 7;
const IBASE: i32 = 10;
const NKNOWN: i32 = 128;
const TEST: i32 = 16;
const ERRSIZ: i32 = 160;

struct SaveVars {
    BLNKST: Vec<u8>,
    TOOBIG: Vec<u8>,
    UNRCST: Vec<u8>,
    UNXPCH: Vec<u8>,
    UNXPPT: Vec<u8>,
    UNXPSN: Vec<u8>,
    DECVAL: f64,
    DIVISR: f64,
    DPSIGN: StackArray<f64, 2>,
    ECOUNT: f64,
    EXPVAL: f64,
    FACTOR: f64,
    INTBND: f64,
    INTVAL: f64,
    LOOKUP: StackArray<f64, 11>,
    MAXEXP: f64,
    MINEXP: f64,
    NEXT: f64,
    SMLBND: f64,
    VALUE: f64,
    VALUES: StackArray<f64, 128>,
    CLASS: StackArray<i32, 129>,
    EXP: i32,
    ID: i32,
    SIGNDX: i32,
    B: i32,
    BLANK: i32,
    L: i32,
    M: i32,
    NL: i32,
    NEXTI: i32,
    THISI: i32,
    DODEC: bool,
    EXPOK: bool,
    DOEXP: bool,
    FIRST: bool,
    DOINT: bool,
    MANTSA: bool,
    BPIOK: bool,
    EPIOK: bool,
    PNTOK: bool,
    ROUNDI: bool,
    ROUNDD: bool,
    SIGCHR: bool,
    SIGNOK: bool,
    ZEROI: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut BLNKST = vec![b' '; ERRSIZ as usize];
        let mut TOOBIG = vec![b' '; ERRSIZ as usize];
        let mut UNRCST = vec![b' '; ERRSIZ as usize];
        let mut UNXPCH = vec![b' '; ERRSIZ as usize];
        let mut UNXPPT = vec![b' '; ERRSIZ as usize];
        let mut UNXPSN = vec![b' '; ERRSIZ as usize];
        let mut DECVAL: f64 = 0.0;
        let mut DIVISR: f64 = 0.0;
        let mut DPSIGN = StackArray::<f64, 2>::new(1..=2);
        let mut ECOUNT: f64 = 0.0;
        let mut EXPVAL: f64 = 0.0;
        let mut FACTOR: f64 = 0.0;
        let mut INTBND: f64 = 0.0;
        let mut INTVAL: f64 = 0.0;
        let mut LOOKUP = StackArray::<f64, 11>::new(0..=IBASE);
        let mut MAXEXP: f64 = 0.0;
        let mut MINEXP: f64 = 0.0;
        let mut NEXT: f64 = 0.0;
        let mut SMLBND: f64 = 0.0;
        let mut VALUE: f64 = 0.0;
        let mut VALUES = StackArray::<f64, 128>::new(1..=NKNOWN);
        let mut CLASS = StackArray::<i32, 129>::new(0..=NKNOWN);
        let mut EXP: i32 = 0;
        let mut ID: i32 = 0;
        let mut SIGNDX: i32 = 0;
        let mut B: i32 = 0;
        let mut BLANK: i32 = 0;
        let mut L: i32 = 0;
        let mut M: i32 = 0;
        let mut NL: i32 = 0;
        let mut NEXTI: i32 = 0;
        let mut THISI: i32 = 0;
        let mut DODEC: bool = false;
        let mut EXPOK: bool = false;
        let mut DOEXP: bool = false;
        let mut FIRST: bool = false;
        let mut DOINT: bool = false;
        let mut MANTSA: bool = false;
        let mut BPIOK: bool = false;
        let mut EPIOK: bool = false;
        let mut PNTOK: bool = false;
        let mut ROUNDI: bool = false;
        let mut ROUNDD: bool = false;
        let mut SIGCHR: bool = false;
        let mut SIGNOK: bool = false;
        let mut ZEROI: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(10.0),
                Val::D(100.0),
                Val::D(1000.0),
                Val::D(10000.0),
                Val::D(100000.0),
                Val::D(1000000.0),
                Val::D(10000000.0),
                Val::D(100000000.0),
                Val::D(1000000000.0),
                Val::D(10000000000.0),
            ]
            .into_iter();
            LOOKUP
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        FIRST = true;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), NKNOWN as usize))
                .chain([]);

            VALUES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(0)]
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), NKNOWN as usize))
                .chain([]);

            CLASS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            BLNKST,
            TOOBIG,
            UNRCST,
            UNXPCH,
            UNXPPT,
            UNXPSN,
            DECVAL,
            DIVISR,
            DPSIGN,
            ECOUNT,
            EXPVAL,
            FACTOR,
            INTBND,
            INTVAL,
            LOOKUP,
            MAXEXP,
            MINEXP,
            NEXT,
            SMLBND,
            VALUE,
            VALUES,
            CLASS,
            EXP,
            ID,
            SIGNDX,
            B,
            BLANK,
            L,
            M,
            NL,
            NEXTI,
            THISI,
            DODEC,
            EXPOK,
            DOEXP,
            FIRST,
            DOINT,
            MANTSA,
            BPIOK,
            EPIOK,
            PNTOK,
            ROUNDI,
            ROUNDD,
            SIGCHR,
            SIGNOK,
            ZEROI,
        }
    }
}

/// Double Precision parsing of a string
///
/// Parse a character string that represents a number and return
/// a double precision value.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  ---------------------------------------------------
///  STRING     I   Character string representing a numeric value.
///  X          O   Double precision value parsed from STRING.
///  ERROR      O   Message indicating whether errors have occurred.
///  PTR        O   Position in string where an error occurred.
/// ```
///
/// # Detailed Input
///
/// ```text
///  STRING   is a character string that represents a numeric value.
///           Commas and spaces may be used in this string for
///           ease of reading and writing the number. They
///           are treated as insignificant but non-error-producing
///           characters.
///
///           For exponential representation the characters
///           'E','D','e','d' may be used.
///
///           The following are legitimate numeric expressions
///
///            +12.2 e-1
///            -3. 1415 9276
///            1e12
///            E10
///
///           The program also recognizes the following  mnemonics
///           'PI', 'pi', 'Pi', 'pI'
///           '+PI', '+pi', '+Pi', '+pI'
///           '-PI', '-pi', '-Pi', '-pI'
///           and returns the value
///           ( + OR - ) 3.1415 9265 3589 7932 3846 2600 D0 as
///           appropriate.
/// ```
///
/// # Detailed Output
///
/// ```text
///  X        double precision parsed value of input string. If an
///           error is encountered, X is not changed.
///
///  ERROR    is a message indicating that the string could
///           not be parsed due to use of an unexpected or misplaced
///           character or due to a string representing a number
///           too large for double precision. If the number was
///           successfully parsed, ERROR will be returned as a blank.
///
///           In particular, blank strings, or strings that do not
///           contain either a digit or exponent character will
///           be regarded as errors.
///
///  PTR      this indicates which character was being used when
///           the error occurred. If no error occurs, PTR is
///           returned as 0.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the string is non-numeric, PTR indicates the location in
///      the string where the error occurred, and ERROR contains a
///      descriptive error message.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine parses an input character string that represents a
///  number, checks for overflow, unexpected or misplaced
///  characters. It returns the double precision number or an error
///  message.
/// ```
///
/// # Examples
///
/// ```text
///  Let   LINE = 'DELTA_T_A       =   32.184'
///
///  The following code fragment parses the line and obtains the
///  double precision value.
///
///
///     CALL NEXTWD ( LINE,  FIRST,  REST )
///     CALL NEXTWD ( REST, SECOND,  REST )
///     CALL NEXTWD ( REST,  THIRD,  REST )
///
///     CALL NPARSD (  THIRD,  VALUE, ERROR, PTR    )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  Due to rounding errors this routine may not be able to parse
///      the decimal character string representation of the largest
///      and smallest double precision numbers.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.6.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 3.5.0, 15-AUG-2002 (WLT)
///
///         Replaced the call to INSSUB with a call to ZZINSSUB so
///         that this routine can legitimately call itself Error Free
///
/// -    SPICELIB Version 3.4.0, 03-DEC-2001 (NJB)
///
///         Added an extra check to make sure that ICHAR of any character
///         of the input string is positive.
///
/// -    SPICELIB Version 3.3.0, 29-FEB-1996 (KRG)
///
///         The declaration for the SPICELIB function PI is now
///         preceded by an EXTERNAL statement declaring PI to be an
///         external function. This removes a conflict with any
///         compilers that have a PI intrinsic function.
///
///         Removed the error message and storage for the unexpected
///         comma error message. This variable was set but never used,
///         and according to the spec for this routine a comma is a valid
///         delimiter, treated like a space, within numbers.
///
/// -    SPICELIB Version 3.2.0, 10-JAN-1995 (WLT)
///
///         Changed error strings from parameters to assignments to
///         compensate for shortcomings of the Absoft FORTRAN compiler
///         on the NeXT.
///
/// -    SPICELIB Version 3.1.0, 12-JUL-1994 (WLT)
///
///         The previous version of the routine assumed that the range
///         of values of ICHAR was 0 to 128. That turns out not to be
///         true on some machines. If a character whose ICHAR value is
///         outside this range is detected, it is now handled properly
///         as an unexpected character.
///
/// -    SPICELIB Version 3.0.0, 24-FEB-1993 (WLT)
///
///         The previous version of the algorithm interpreted P or p as 1.
///         This was not the intent of the routine and was corrected.
///
/// -    SPICELIB Version 2.0.0, 28-AUG-1992 (WLT) (KRG)
///
///         The basic algorithm was completely re-written. As a result
///         the routine now runs an order of magnitude faster than
///         it did before. In addition, strings that do not contain
///         enough information to assign a value to the string are now
///         regarded as errors. These include blank strings or strings
///         that contain only a sign characters, blanks and commas.
///
///         In addition the error diagnosis and checking for overflow
///         was greatly enhanced.
///
///         Note: strings may now parse with slightly different values
///         from the previous version of NPARSD. The current
///         implementation is more accurate in converting strings to
///         double precision numbers.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 17-APR-1990 (WLT)
///
///         Bug fix. The subscript used to reference individual characters
///         of the input string could sometimes step out of bounds. This
///         went unnoticed until NAIF began compiling with the CHECK=BOUNDS
///         option of the DEC Fortran compiler.
///
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (HAN) (NJB)
/// ```
pub fn nparsd(ctx: &mut SpiceContext, string: &str, x: &mut f64, error: &mut str, ptr: &mut i32) {
    NPARSD(
        string.as_bytes(),
        x,
        fstr::StrBytes::new(error).as_mut(),
        ptr,
        ctx.raw_context(),
    );
}

//$Procedure NPARSD ( Double Precision parsing of a string )
pub fn NPARSD(STRING: &[u8], X: &mut f64, ERROR: &mut [u8], PTR: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // SPICELIB functions.
    //

    //
    // Local Parameters.
    //

    //
    // Save everything.  It's easier than tracking down every
    // little variable that might need to be saved.
    //

    if save.FIRST {
        save.FIRST = false;
        //
        // Set up the error messages
        //
        fstr::assign(&mut save.TOOBIG, b"The number represented by the input string is too large to be stored as a double precision number. ");

        fstr::assign(
            &mut save.UNXPCH,
            b"An unexpected character was found while attempting to parse the input string. ",
        );

        fstr::assign(
            &mut save.UNXPPT,
            b"An unexpected decimal point was found in the input string. ",
        );

        fstr::assign(
            &mut save.UNXPSN,
            b"An unexpected sign character was found in the input string. ",
        );

        fstr::assign(
            &mut save.BLNKST,
            b"The input string is blank. Blank strings are not considered to be numbers. ",
        );

        fstr::assign(
            &mut save.UNRCST,
            b"The input string could not be recognized as a number. ",
        );

        save.BLANK = intrinsics::ICHAR(b" ");

        save.VALUES[intrinsics::ICHAR(b"0")] = 0.0;
        save.VALUES[intrinsics::ICHAR(b"1")] = 1.0;
        save.VALUES[intrinsics::ICHAR(b"2")] = 2.0;
        save.VALUES[intrinsics::ICHAR(b"3")] = 3.0;
        save.VALUES[intrinsics::ICHAR(b"4")] = 4.0;
        save.VALUES[intrinsics::ICHAR(b"5")] = 5.0;
        save.VALUES[intrinsics::ICHAR(b"6")] = 6.0;
        save.VALUES[intrinsics::ICHAR(b"7")] = 7.0;
        save.VALUES[intrinsics::ICHAR(b"8")] = 8.0;
        save.VALUES[intrinsics::ICHAR(b"9")] = 9.0;
        save.VALUES[intrinsics::ICHAR(b"-")] = -1.0;
        save.VALUES[intrinsics::ICHAR(b"+")] = 1.0;

        save.CLASS[intrinsics::ICHAR(b" ")] = IGNORE;
        save.CLASS[intrinsics::ICHAR(b",")] = IGNORE;

        save.CLASS[intrinsics::ICHAR(b".")] = DPOINT;

        save.CLASS[intrinsics::ICHAR(b"E")] = EXPONT;
        save.CLASS[intrinsics::ICHAR(b"D")] = EXPONT;
        save.CLASS[intrinsics::ICHAR(b"e")] = EXPONT;
        save.CLASS[intrinsics::ICHAR(b"d")] = EXPONT;

        save.CLASS[intrinsics::ICHAR(b"+")] = SIGN;
        save.CLASS[intrinsics::ICHAR(b"-")] = SIGN;

        save.CLASS[intrinsics::ICHAR(b"1")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"2")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"3")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"4")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"5")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"6")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"7")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"8")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"9")] = DIGIT;
        save.CLASS[intrinsics::ICHAR(b"0")] = DIGIT;

        save.CLASS[intrinsics::ICHAR(b"p")] = BPISTR;
        save.CLASS[intrinsics::ICHAR(b"P")] = BPISTR;
        save.CLASS[intrinsics::ICHAR(b"i")] = EPISTR;
        save.CLASS[intrinsics::ICHAR(b"I")] = EPISTR;
        //
        // Finally create the numbers that will be used for checking
        // for floating point overflow.
        //
        // NOTE: The value for MINEXP may be too small by one, but it
        //       really doesn't make any difference, as you're going to
        //       underflow anyway, and dividing zero by a number (BASE)
        //       still gives you zero.
        //
        save.MAXEXP = f64::trunc(f64::log10(DPMAX()));
        save.MINEXP = -(save.MAXEXP + 1 as f64);
        save.SMLBND = (DPMAX() / save.LOOKUP[IBASE]);

        save.INTBND = BASE;
        save.NEXT = (save.INTBND + 1.0);

        while (save.INTBND != save.NEXT) {
            save.INTBND = (save.INTBND * BASE);
            save.NEXT = (save.INTBND + 1.0);
        }

        save.INTBND = (save.INTBND / BASE);
        //
        // That takes care of the first pass initializations.
        //
    }
    //
    // Here's what's true right now.
    //
    // There are no errors.
    // The error pointer doesn't need to point anywhere.
    // It's ok for the next token to be a decimal point.
    // It's ok for the next token to be a sign character.
    // It's ok for the next token to be an exponent marker.
    // It's ok for the next character to be the start of pi.
    //
    // We expect to be constructing the integer part of the
    // numeric string.
    //
    fstr::assign(ERROR, b" ");
    *PTR = 0;

    save.PNTOK = true;
    save.SIGNOK = true;
    save.EXPOK = true;
    save.BPIOK = true;
    save.DOINT = true;
    save.ROUNDD = true;
    save.ROUNDI = true;
    //
    // Here's some other facts.
    //
    // We are not parsing the decimal part of the string.
    // We are not parsing the exponent part of the string.
    // We have not encountered any digits in the mantissa.
    // We have not encountered any significant characters.
    // It's not ok for the next character to be the end of pi (i).
    //
    save.DODEC = false;
    save.DOEXP = false;
    save.MANTSA = false;
    save.SIGCHR = false;
    save.EPIOK = false;
    //
    // So far there is no integer, decimal or exponent part to this
    // string.
    //
    save.INTVAL = 0.0;
    save.DECVAL = 0.0;
    save.EXPVAL = 0.0;
    save.DIVISR = 1.0;
    save.FACTOR = 1.0;
    save.ECOUNT = 0.0;
    //
    // Right now if we encounter a sign, it's part of the mantissa.
    // And until we know better the sign of both the mantissa and
    // exponent are +1 (as opposed to -1).
    //
    save.SIGNDX = 1;
    save.DPSIGN[1] = 1.0;
    save.DPSIGN[2] = 1.0;
    //
    // Before doing anything else we determine whether or not
    // the input string is empty.
    //
    if fstr::eq(STRING, b" ") {
        fstr::assign(ERROR, &save.BLNKST);
        *PTR = 1;
        return;
    }

    //
    // We need to find the last non-blank character of the input
    // string.  We shall use the idea of binary searching to locate
    // this character.  At first this may appear to be a bit convoluted
    // when compared to the obvious thing to do (start at the end of
    // the string and step backward until a non-blank character is
    // located).  However, on every machine we've looked at this method
    // locates the last non-blank character much more quickly on average
    // than the obvious method.
    //
    // L and B denote the last and beginning characters
    // of the substring we are searching.  NL is the next to last
    // character that we are concerned with and M is the middle of
    // the current search interval ( from B to NL ).
    //
    save.L = intrinsics::LEN(STRING);
    save.B = 1;
    save.NL = (save.L - 1);

    //
    // We want M to be ( B + NL ) / 2   but right now that's L/2
    //
    save.M = (save.L / 2);

    while ((save.L - save.B) > TEST) {
        //
        // What is true right now?  The string from L+1 on out
        // is blank.  L > B; L-1 = NL >= B;  M = (B + NL) / 2;
        // and M >= B,  B is at least one and if greater than 1
        // there must be a non-blank character between B and the
        // end of the string.
        //
        if (intrinsics::ICHAR(fstr::substr(STRING, save.L..=save.L)) != save.BLANK) {
            save.B = save.L;
        } else if fstr::eq(fstr::substr(STRING, save.M..=save.NL), b" ") {
            //
            // If you got here, the STRING(L:L) is a blank.
            // The string from L+1 on out is blank.
            // The string from M to NL (=L-1) is blank.  Thus the
            // string from M out is blank.
            //
            // M is greater than or equal to B.
            // If M  is less than B + 2, then L will become
            // B or less and there will not be a
            // next pass through the loop.  That means that
            // we will never get to this point again and don't
            // have to worry about the reference STRING(M:NL)
            // giving us an access violation.
            //
            save.L = (save.M - 1);
        //
        // With the new value of L, we now know that STRING(L+1:)
        // is blank.
        //
        } else {
            //
            // If you get to this point all of the string from
            // L out is blank and L is greater than M.
            // There is a non-blank character between M and NL.
            // If L should get within 16 of B, then the loop
            // will not be executed again.  That means again that
            // we don't have to worry about STRING(M:NL) being
            // an ill formed string.
            //
            save.L = save.NL;
            save.B = save.M;
            //
            // With the new value of L, we now know that STRING(L+1:)
            // is blank.
            //
        }

        //
        // Finally compute NL,the index of the character that precedes
        // L and the new midpoint of the stuff from B to NL.
        //
        save.NL = (save.L - 1);
        save.M = ((save.B + save.NL) / 2);

        //
        // What's true now?  The string from L+1 on out is blank.
        //
    }

    //
    // L is now within 16 characters of the last non-blank character
    // of the input string.  We simply search backward from L to
    // locate this last non-blank.
    //
    while (intrinsics::ICHAR(fstr::substr(STRING, save.L..=save.L)) == save.BLANK) {
        save.L = (save.L - 1);
    }

    //
    // Begin to collect the number in its various parts: an integer
    // portion, a fractional portion, and an exponent.
    //
    for I in 1..=save.L {
        save.ID = intrinsics::ICHAR(fstr::substr(STRING, I..=I));

        if ((save.ID > NKNOWN) || (save.ID < 0)) {
            //
            // This is definitely not expected.  Set the error message
            // and return.
            //
            save.NEXTI = (I + 1);
            save.THISI = I;

            ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
            ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
            PREFIX(&save.UNXPCH, 1, ERROR);

            *PTR = I;
            return;

        //
        // The action taken depends upon the class of the token.
        //
        } else if (save.CLASS[save.ID] == DIGIT) {
            //
            // Once a digit has been encountered, we can no longer
            // allow the string 'PI' or a sign until an exponent
            // character is hit and resets the SIGNOK flag.
            //
            save.BPIOK = false;
            save.EPIOK = false;
            save.SIGNOK = false;
            save.SIGCHR = true;
            //
            // If we are constructing the integer part ...
            //
            if save.DOINT {
                save.MANTSA = true;
                //
                // Check the current value of the integer part to
                // make sure we don't overflow.
                //
                if (save.INTVAL < save.INTBND) {
                    save.INTVAL = ((save.INTVAL * BASE) + save.VALUES[save.ID]);
                } else {
                    //
                    // Once the integer exceeds a given bound,
                    // we add the rest on as fractional part and
                    // keep track of the factor we will need to
                    // multiply the decimal part by to scale things
                    // appropriately.  We also keep track of the number
                    // we will need to add to the exponent part.
                    //
                    save.ECOUNT = (save.ECOUNT + 1 as f64);
                    save.FACTOR = (save.FACTOR / BASE);

                    if save.ROUNDI {
                        save.ROUNDI = false;

                        if (save.VALUES[save.ID] > (0.5 * BASE)) {
                            save.INTVAL = (save.INTVAL + 1.0);
                        }
                    }
                }
            //
            // ... or the decimal part ...
            //
            } else if save.DODEC {
                save.MANTSA = true;
                //
                // There are two cases to consider.  The case in which
                // the integer portion of the string has value 0...
                //
                if save.ZEROI {
                    //
                    // We can just keep accumulating the decimal part
                    // as an integer.  But we keep track of how many
                    // places past the decimal point the first non-zero
                    // digit occurs.  Note that once the decimal part
                    // exceeds the integer bound, we don't need to do
                    // anything.  The remaining digits cannot contribute
                    // to the value of the decimal part.
                    //
                    if (save.DECVAL < save.INTBND) {
                        save.DECVAL = ((save.DECVAL * BASE) + save.VALUES[save.ID]);
                        save.ECOUNT = (save.ECOUNT - 1 as f64);
                    } else if save.ROUNDD {
                        save.ROUNDD = false;

                        if (save.VALUES[save.ID] >= (0.5 * BASE)) {
                            save.DECVAL = (save.DECVAL + 1.0);
                        }
                    }
                //
                // ...and the case in which the integer portion is not
                // zero.
                //
                } else {
                    //
                    // In this case, we know there is at least _something_
                    // to the integer part of this string.  We can
                    // stop accumulating the decimal part when the divisor
                    // portion exceeds the integer barrier.  After that
                    // the extra digits can't make any contribution to
                    // the double precision value given to the string.
                    //
                    if (save.DIVISR < save.INTBND) {
                        save.DECVAL = ((save.DECVAL * BASE) + save.VALUES[save.ID]);
                        save.DIVISR = (save.DIVISR * BASE);
                    }
                }
            //
            // ...or the exponent part of the string.
            //
            } else if save.DOEXP {
                if ((save.EXPVAL + save.ECOUNT) > save.MAXEXP) {
                    //
                    // This number is too big to put into a double
                    // precision number. The marginal case where
                    // EXPVAL + ECOUNT .EQ. MAXEXP will be dealt
                    // with when the integer and fractional parts
                    // of the double precision number are built
                    // at the end of this routine.
                    //
                    fstr::assign(ERROR, &save.TOOBIG);
                    *PTR = I;
                    return;
                } else if ((save.EXPVAL + save.ECOUNT) < save.MINEXP) {
                    //
                    // This number is going to underflow, we can
                    // just stop accumulating exponent. But we don't
                    // stop parsing the string yet. There might be
                    // a bad character lurking somewhere later in the
                    // string.
                    //
                    // NOTE: It is also possible to underflow when the
                    //       value of EXPVAL + ECOUNT is equal to MINEXP,
                    //       since an entire 'BASE' scale is not supported
                    //       for this particular exponent.
                    //
                } else {
                    //
                    // This is the case we expect.  Just add on the
                    // next part of the exponent.
                    //
                    save.EXPVAL = ((save.EXPVAL * BASE) + (save.DPSIGN[2] * save.VALUES[save.ID]));
                }
            //
            // Even though this character is a digit, its not expected
            // for some reason.  Set the error flag and return.
            //
            } else {
                save.NEXTI = (I + 1);
                save.THISI = I;

                ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
                PREFIX(&save.UNXPCH, 1, ERROR);

                *PTR = I;
                return;
            }
        } else if (save.CLASS[save.ID] == DPOINT) {
            if save.PNTOK {
                save.BPIOK = false;
                save.EPIOK = false;
                save.PNTOK = false;
                save.SIGNOK = false;
                save.DODEC = true;
                save.DOINT = false;
                save.DOEXP = false;
                save.ZEROI = (save.INTVAL == 0.0);
            } else {
                save.NEXTI = (I + 1);
                save.THISI = I;

                ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
                PREFIX(&save.UNXPPT, 1, ERROR);

                *PTR = I;
                return;
            }
        } else if (save.CLASS[save.ID] == EXPONT) {
            save.SIGCHR = true;

            if save.EXPOK {
                save.BPIOK = false;
                save.EPIOK = false;
                save.EXPOK = false;
                save.PNTOK = false;
                save.DODEC = false;
                save.DOINT = false;
                save.DOEXP = true;
                save.SIGNOK = true;
                save.SIGNDX = 2;
            } else {
                save.NEXTI = (I + 1);
                save.THISI = I;

                ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
                PREFIX(&save.UNXPCH, 1, ERROR);

                *PTR = I;
                return;
            }
        } else if (save.CLASS[save.ID] == SIGN) {
            if save.SIGNOK {
                save.DPSIGN[save.SIGNDX] = save.VALUES[save.ID];
                save.SIGNOK = false;
            } else {
                save.NEXTI = (I + 1);
                save.THISI = I;

                ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
                PREFIX(&save.UNXPSN, 1, ERROR);

                *PTR = I;
                return;
            }
        } else if (save.CLASS[save.ID] == BPISTR) {
            save.SIGCHR = true;

            if save.BPIOK {
                save.DOINT = false;
                save.DODEC = false;
                save.DOEXP = false;
                save.EXPOK = false;
                save.PNTOK = false;
                save.BPIOK = false;
                save.SIGNOK = false;
                save.EPIOK = true;
            } else {
                save.NEXTI = (I + 1);
                save.THISI = I;

                ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
                PREFIX(&save.UNXPCH, 1, ERROR);

                *PTR = I;
                return;
            }
        } else if (save.CLASS[save.ID] == EPISTR) {
            if save.EPIOK {
                save.DOINT = false;
                save.DODEC = false;
                save.DOEXP = false;
                save.EXPOK = false;
                save.PNTOK = false;
                save.BPIOK = false;
                save.SIGNOK = false;
                save.EPIOK = false;
                save.MANTSA = true;

                save.INTVAL = PI(ctx);
            } else {
                save.NEXTI = (I + 1);
                save.THISI = I;

                ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
                ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
                PREFIX(&save.UNXPCH, 1, ERROR);

                *PTR = I;
                return;
            }
        } else if (save.CLASS[save.ID] == IGNORE) {

            //
            // We don't do anything.
            //
        } else {
            //
            // This is definitely not expected.  Set the error message
            // and return.
            //
            save.NEXTI = (I + 1);
            save.THISI = I;

            ZZINSSUB(STRING, b"]", save.NEXTI, ERROR);
            ZZINSSUB(&ERROR.to_vec(), b"[", save.THISI, ERROR);
            PREFIX(&save.UNXPCH, 1, ERROR);

            *PTR = I;
            return;
        }
    }

    //
    // If we got through the loop and it's OK to end PI, then we started
    // it but never finished.  This is an error.
    //
    if save.EPIOK {
        fstr::assign(ERROR, &save.UNRCST);
        *PTR = save.L;
        return;
    }

    //
    // Put together the portion that does not involve an exponent.
    //
    // If
    //    (1) MANTSA = .TRUE., then we had some explicit part of a
    //        number, an  integer part, a fractional part, or both.
    //
    //    (2) SIGCHR = .TRUE, then we had either:
    //
    //        (a) MANTSA = .TRUE.
    //
    //     or
    //
    //        (b) there was an implicit value associated with the input
    //            string. For example, an exponent character followed
    //            by an optional exponent would produce a valid number:
    //            E+10 --> 1.0d+10. This is due to the fact that this
    //            routine emulates an RPN calculator of popular repute,
    //            not because it is inherently a good idea.
    //
    if save.MANTSA {
        //
        // We had an integer part of the number, a fractional part, or
        // both, so we need to put them together in an appropriate
        // fashion.
        //
        save.VALUE = (save.INTVAL + ((save.DECVAL / save.DIVISR) * save.FACTOR));
    } else if save.SIGCHR {
        //
        // We do not have a mantissa, so we had an  implicit mantissa,
        // see above, so we need to set the value to one.
        //
        save.VALUE = 1.0;
    } else {
        //
        // We have an error. There were no significant characters in the
        // input character string, and hence we could not parse it into
        // a number. An example of such a string would be: '+  ,,.,,'.
        // So, we will set an appropriate error message and return.
        //
        fstr::assign(ERROR, &save.UNRCST);
        *PTR = (intrinsics::LEN(STRING) + 1);
        return;
    }
    //
    // Adjust the entered part of the exponent by the amount
    // we "shifted" the decimal point when we were computing
    // the integer and decimal values.
    //
    save.EXPVAL = (save.EXPVAL + save.ECOUNT);
    //
    // Now take care of the exponent contribution to the answer.
    //
    // If the exponent is negative ...
    //
    if (save.EXPVAL < 0 as f64) {
        while (save.EXPVAL < -BASE) {
            save.VALUE = (save.VALUE / save.LOOKUP[IBASE]);
            save.EXPVAL = (save.EXPVAL + BASE);
        }

        save.VALUE = (save.VALUE / save.LOOKUP[-(save.EXPVAL as i32)]);
    //
    // If the exponent is positive ...
    //
    } else if (save.EXPVAL > 0 as f64) {
        while (save.EXPVAL > BASE) {
            //
            // Make sure that a multiply isn't going to create
            // a number that overflows.
            //
            if (save.VALUE >= save.SMLBND) {
                fstr::assign(ERROR, &save.TOOBIG);
                *PTR = (intrinsics::LEN(STRING) + 1);
                return;
            } else {
                save.VALUE = (save.VALUE * save.LOOKUP[IBASE]);
                save.EXPVAL = (save.EXPVAL - BASE);
            }
        }

        save.EXP = intrinsics::IDNINT(save.EXPVAL);
        //
        // Again, make sure that a floating point overflow isn't
        // going to happen.
        //
        if (save.VALUE < (DPMAX() / save.LOOKUP[save.EXP])) {
            save.VALUE = (save.VALUE * save.LOOKUP[save.EXP]);
        } else {
            fstr::assign(ERROR, &save.TOOBIG);
            *PTR = (intrinsics::LEN(STRING) + 1);
            return;
        }
    }

    *X = (save.DPSIGN[1] * save.VALUE);
}
