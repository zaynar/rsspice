//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MOSTDG: i32 = 14;
const MAXPOW: i32 = 17;
const MAXEXP: i32 = 40;

struct SaveVars {
    POWER: StackArray<f64, 18>,
    IPOWER: StackArray<f64, 18>,
    DIGITS: ActualCharArray,
    VALUES: StackArray<f64, 10>,
    VAXEXP: ActualCharArray,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut POWER = StackArray::<f64, 18>::new(0..=MAXPOW);
        let mut IPOWER = StackArray::<f64, 18>::new(0..=MAXPOW);
        let mut DIGITS = ActualCharArray::new(1, 0..=9);
        let mut VALUES = StackArray::<f64, 10>::new(0..=9);
        let mut VAXEXP = ActualCharArray::new(2, 0..=40);

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
                Val::D(100000000000.0),
                Val::D(1000000000000.0),
                Val::D(10000000000000.0),
                Val::D(100000000000000.0),
                Val::D(1000000000000000.0),
                Val::D(10000000000000000.0),
                Val::D(100000000000000000.0),
            ]
            .into_iter();
            POWER
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(1.0),
                Val::D(0.1),
                Val::D(0.01),
                Val::D(0.001),
                Val::D(0.0001),
                Val::D(0.00001),
                Val::D(0.000001),
                Val::D(0.0000001),
                Val::D(0.00000001),
                Val::D(0.000000001),
                Val::D(0.0000000001),
                Val::D(0.00000000001),
                Val::D(0.000000000001),
                Val::D(0.0000000000001),
                Val::D(0.00000000000001),
                Val::D(0.000000000000001),
                Val::D(0.0000000000000001),
                Val::D(0.00000000000000001),
            ]
            .into_iter();
            IPOWER
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"0"),
                Val::C(b"1"),
                Val::C(b"2"),
                Val::C(b"3"),
                Val::C(b"4"),
                Val::C(b"5"),
                Val::C(b"6"),
                Val::C(b"7"),
                Val::C(b"8"),
                Val::C(b"9"),
            ]
            .into_iter();
            DIGITS
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(1.0),
                Val::D(2.0),
                Val::D(3.0),
                Val::D(4.0),
                Val::D(5.0),
                Val::D(6.0),
                Val::D(7.0),
                Val::D(8.0),
                Val::D(9.0),
            ]
            .into_iter();
            VALUES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"00"),
                Val::C(b"01"),
                Val::C(b"02"),
                Val::C(b"03"),
                Val::C(b"04"),
                Val::C(b"05"),
                Val::C(b"06"),
                Val::C(b"07"),
                Val::C(b"08"),
                Val::C(b"09"),
                Val::C(b"10"),
                Val::C(b"11"),
                Val::C(b"12"),
                Val::C(b"13"),
                Val::C(b"14"),
                Val::C(b"15"),
                Val::C(b"16"),
                Val::C(b"17"),
                Val::C(b"18"),
                Val::C(b"19"),
                Val::C(b"20"),
                Val::C(b"21"),
                Val::C(b"22"),
                Val::C(b"23"),
                Val::C(b"24"),
                Val::C(b"25"),
                Val::C(b"26"),
                Val::C(b"27"),
                Val::C(b"28"),
                Val::C(b"29"),
                Val::C(b"30"),
                Val::C(b"31"),
                Val::C(b"32"),
                Val::C(b"33"),
                Val::C(b"34"),
                Val::C(b"35"),
                Val::C(b"36"),
                Val::C(b"37"),
                Val::C(b"38"),
                Val::C(b"39"),
                Val::C(b"40"),
            ]
            .into_iter();
            VAXEXP
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            POWER,
            IPOWER,
            DIGITS,
            VALUES,
            VAXEXP,
        }
    }
}

/// Double Precision Number to Character
///
/// Take a double precision number and convert it to
/// an equivalent character string representation (base 10).
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   A double precision number
///  SIGDIG     I   The number of significant digits placed in output
///  STRING     O   A character string representation of X
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is a double precision number.
///
///  SIGDIG   is the number of significant digits that are desired
///           for the output string.
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is a character representation of X to the number of
///           significant digits specified by SIGDIG. The number of
///           spaces required to return the requested character
///           string is SIGDIG + 6. If STRING is not declared to
///           have adequate length, the number returned will be
///           truncated on the right.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If SIGDIG is less than one, this routine returns one
///      significant digit in the output string.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine computes an approximate character representation
///  of the input string X. The maximum number of significant
///  digits returned is 14. The representation returned will be
///  the same as that given by the FORTRAN write statement
///
///      WRITE ( STRING, FMT=(P1E23.xx)
///
///  where xx is a two digit number that represents MIN(14,SIGDIG).
///  The last decimal place is rounded. The output string is left
///  justified.
///
///  This routine has the advantage that it does not use an internal
///  file and is about 2.3 times as fast as an internal write. It can
///  be used as part of character function without fear of introducing
///  recursive I/O conflicts. It is intended to be an approximate
///  inverse to the subroutine NPARSD.
///
///  There is of course no formatting of the output string. All
///  outputs are written in scientific notation.
///
///  IF you want the character string representation of a double
///  precision number to be the same as that produced by a formatted
///  write statement use a FORTRAN write statement.
///
///  For example the number represented by the string
///
///        1.245454545454545454545E+01
///
///  when read (via a FORTRAN READ statement) into the DP variable X
///  and converted back to a character string having 14 significant
///  digits by this routine yields
///
///        1.2454545454545E+01
///
///  The FORTRAN write statement
///
///        WRITE ( 6, FMT='(P1E)' ) X
///
///  yields
///
///        1.2454545454545454E+01
///
///  If this is too much error for your application DO NOT use this
///  routine. You should be aware however, that a character string
///  read into a double precision number may not WRITE out with an
///  equivalent character representation as was input.
///
///  For example on a VAX 11/780 if you
///
///        READ  (5,*)         X
///        WRITE (6,FMT='(E)') X
///
///  and enter a value of 7.00000001 for the read statement
///  the output written will be 0.7000000010000001E+01
/// ```
///
/// # Examples
///
/// ```text
///  This routine is intended for use by routines that manipulate
///  character strings. For example, it may be desirable for a
///  routine to be able to take a character string input such as
///
///        12 miles
///
///  and convert it to the string
///
///        1.932E+02 km
///
///  or to simply
///
///        1.932E+02
///
///  The arithmetic is of course most easily handled using numeric
///  variables. However, it may be that a string is required for
///  subsequent processing of the input.  A SPICELIB routine NPARSD
///  exists that will take a character representation of a number
///  and convert it to a DOUBLE PRECISION number. The 12 above
///  can be converted to double precision using NPARSD,  the d.p.
///  number can then be multiplied by the 1.61... and the result
///  converted back to a string using this routine.
///
///  Suppose the following declarations are made
///
///        CHARACTER*(80)     TEXT
///        CHARACTER*(80)     NUMBER
///        CHARACTER*(80)     SCRATCH
///
///        DOUBLE PRECISION   X
///        INTEGER            I
///
///  and that TEXT contains the string '12 mi'.  Then the following
///  code would produce a character string  '1.932E+01 KM'
///
///        CALL NEXTWD (  TEXT,   NUMBER, SCRATCH   )
///        CALL NPARSD (  NUMBER, X,      ERROR,  I )
///
///        IF ( ERROR .EQ. ' ' ) THEN
///
///           X    = X * 1.61D0
///           CALL DPSTR ( X, 5, NUMBER )
///           TEXT = NUMBER(1:10) // 'KM'
///
///        ELSE
///           .
///           .
///           create an error message, try again, etc.
///           .
///           .
///        END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The format of the string returned by this routine is used in
///      DPSTRF which is in the call tree to DPFMT. Changes to the
///      format of the output string may have unexpected consequences
///      for these SPICE routines. Please check those routines before
///      modifying this routine.
///
///  2)  The maximum number of significant digits returned is 14.
///
///  3)  If the output string is not declared to be adequately large
///      (at least SIGDIG + 6), the numeric string will be truncated to
///      the side opposite its justification.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.1.1, 09-SEP-1996 (WLT)
///
///         Added a reference to the header concerning the dependency
///         of the SPICE routines DPSTRF and DPFMT on the format of
///         the string produced by this routine.
///
/// -    SPICELIB Version 1.1.0, 11-JUN-1992 (WLT)
///
///         A bug that caused this routine to have a floating point
///         overflow for values of X close to zero was corrected. In
///         addition the restriction on range of exponents supported
///         has been removed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (WLT) (HAN) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -     SPICELIB Version 1.1.0, 14-OCT-1992 (WLT)
///
///          A bug that caused this routine to have a floating point
///          overflow for values of X close to zero was corrected. In
///          addition the restriction on range of exponents supported
///          has been removed.
///
/// -     Beta Version 1.1.0, 16-FEB-1989 (HAN) (NJB)
///
///          Header was changed to reflect the "error free" status
///          of the module, and a comment was added stating what the
///          routine does if SIGIDG is less than one.
///
///          Declaration of the unused variable FRAC removed.
/// ```
pub fn dpstr(ctx: &mut SpiceContext, x: f64, sigdig: i32, string: &mut str) {
    DPSTR(
        x,
        sigdig,
        fstr::StrBytes::new(string).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure DPSTR ( Double Precision Number to Character )
pub fn DPSTR(X: f64, SIGDIG: i32, STRING: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut COPY: f64 = 0.0;
    let mut POSTN: i32 = 0;
    let mut EXPONT: i32 = 0;
    let mut MAXSIG: i32 = 0;
    let mut EXP10: f64 = 0.0;
    let mut I: i32 = 0;
    let mut K: i32 = 0;
    let mut LAST: i32 = 0;
    let mut EXPC = [b' '; 20 as usize];
    let mut ZERO = [b' '; 28 as usize];
    let mut NUMSTR = [b' '; 32 as usize];

    //
    // Maximum number of allowed significant digits.
    //

    //
    // Local variables
    //

    //
    // Transfer X to the local variable COPY and leave X alone for the
    // rest of the routine.
    //
    COPY = X;

    //
    // Wipe out anything sitting in NUMSTR
    //
    fstr::assign(&mut NUMSTR, b" ");

    //
    // At least 1 significant digit is required.  The most allowed is 14.
    // MAXSIG is the integer in this range that is closest to SIGDIG.
    //
    MAXSIG = intrinsics::MIN0(&[MOSTDG, intrinsics::MAX0(&[1, SIGDIG])]);

    //
    // Examine COPY to see if its positive, zero, or negative.
    // This determines whether we need a minus sign and where the
    // decimal point needs to go in the output string.
    //
    if (COPY < 0 as f64) {
        fstr::assign(fstr::substr_mut(&mut NUMSTR, 1..=1), b"-");
        COPY = -COPY;
        POSTN = 2;
        fstr::assign(fstr::substr_mut(&mut NUMSTR, 3..=3), b".");
    } else if (COPY > 0 as f64) {
        fstr::assign(fstr::substr_mut(&mut NUMSTR, 1..=1), b" ");
        POSTN = 2;
        fstr::assign(fstr::substr_mut(&mut NUMSTR, 3..=3), b".");
    } else {
        fstr::assign(&mut ZERO, b" 0.0000000000000000000000000");
        fstr::assign(
            &mut NUMSTR,
            &fstr::concat(fstr::substr(&ZERO, 1..=(MAXSIG + 2)), b"E+00"),
        );

        fstr::assign(STRING, &NUMSTR);
        return;
    }

    //
    // We need a first guess at the exponent string.  Compute the LOG
    // base 10 of COPY
    //
    EXP10 = f64::log10(COPY);

    //
    // Scale our copy of the input into the range 1 to 10.
    //
    if (EXP10 < 0 as f64) {
        //
        // In this case the exponent will be negative.  We want the
        // largest integer exponent less than EXP10,  but the FORTRAN
        // INT function gives the INTEGER closest to EXP10 between EXP10
        // and zero.  As a result we have to subtract 1 from INT(EXP10).
        //
        EXPONT = ((EXP10 as i32) - 1);
        K = -EXPONT;

        while (K > 16) {
            COPY = (COPY * 10000000000000000.0);
            K = (K - 16);
        }

        if (K != 0) {
            COPY = (COPY * save.POWER[K]);
        }
    } else {
        EXPONT = (EXP10 as i32);
        K = EXPONT;

        while (K > 16) {
            COPY = (COPY * 0.0000000000000001);
            K = (K - 16);
        }

        if (K != 0) {
            COPY = (COPY * save.IPOWER[K]);
        }
    }

    //
    // Round off the last significant digit.
    //
    COPY = ((f64::round((COPY * save.POWER[(MAXSIG - 1)])) + 0.125) * save.IPOWER[(MAXSIG - 1)]);

    //
    // We might have accidentally made copy as big as 10 by the
    // round off process.  If we did we need to divide by 10 and add 1
    // to the exponent value.  (COPY must always remain between 0 and 10)
    //
    if (COPY >= 10.0) {
        COPY = (COPY * 0.1);
        EXPONT = (EXPONT + 1);
    }
    //
    // Get the first digit of the decimal expansion of X.
    //
    I = (COPY as i32);
    fstr::assign(
        fstr::substr_mut(&mut NUMSTR, POSTN..=POSTN),
        save.DIGITS.get(I),
    );

    COPY = ((COPY - save.VALUES[I]) * 10.0);

    //
    // Set the string pointer to the next position and compute the
    // position of the last significant digit
    //
    POSTN = (POSTN + 2);
    LAST = ((POSTN + MAXSIG) - 1);

    //
    // Fetch digits until we fill in the last available slot for
    // significant digits.
    //
    while (POSTN < LAST) {
        I = (COPY as i32);
        fstr::assign(
            fstr::substr_mut(&mut NUMSTR, POSTN..=POSTN),
            save.DIGITS.get(I),
        );
        COPY = ((COPY - save.VALUES[I]) * 10.0);
        POSTN = (POSTN + 1);
    }

    //
    // Tack on the exponent to the output. Note that the rather odd
    // if, else if, else construction below is done to maintain backward
    // compatibility of the "look" of the output.
    //
    // First get the exponent symbol and sign of the exponent.
    //
    if (EXPONT >= 0) {
        fstr::assign(fstr::substr_mut(&mut NUMSTR, POSTN..), b"E+");
    } else {
        EXPONT = -EXPONT;
        fstr::assign(fstr::substr_mut(&mut NUMSTR, POSTN..), b"E-");
    }

    POSTN = (POSTN + 2);

    //
    // Now get the numeric representation.
    //
    if (EXPONT <= 40) {
        fstr::assign(&mut EXPC, save.VAXEXP.get(EXPONT));
    } else {
        INTSTR(EXPONT, &mut EXPC, ctx);
    }

    fstr::assign(fstr::substr_mut(&mut NUMSTR, POSTN..), &EXPC);
    fstr::assign(STRING, &NUMSTR);

    //
    // That's all folks.
    //
}
