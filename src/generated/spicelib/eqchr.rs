//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const LSTCHR: i32 = 255;

struct SaveVars {
    EQCHR: bool,
    NECHR: bool,
    UVALUE: StackArray<i32, 256>,
    I: i32,
    J: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut EQCHR: bool = false;
        let mut NECHR: bool = false;
        let mut UVALUE = StackArray::<i32, 256>::new(0..=LSTCHR);
        let mut I: i32 = 0;
        let mut J: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            EQCHR,
            NECHR,
            UVALUE,
            I,
            J,
            FIRST,
        }
    }
}

/// Equivalent characters
///
/// Return .TRUE. if two given characters are equivalent when the
/// case of the characters is ignored.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   one of the characters to check
///  B          I   the other character to check
///
///  The function returns .TRUE. if the characters are equivalent
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are two characters that are to be compared to see
///           if they are the same letter (although possibly
///           having different case such as 'a' and 'A')
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .TRUE. if the two input characters
///  are the same or can be made the same by converting both to
///  upper or lower case.
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
///  This is a utility routine for comparing two characters to
///  see if they are the same when converted to upper case. It
///  is particularly useful when writing string analysis routines
///  that should be case insensitive. Instead of writing the
///  expression
///
///     A .EQ. B
///
///  use the expression
///
///     EQCHR ( A, B )
///
///  in all tests of equivalence for characters.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you want to determine whether or not two strings
///  are the same if differences in the case of letters are ignored.
///  The following code fragment shows how you can use this routine
///  to check for the equivalence of character strings.
///
///     MORE  = .TRUE.
///     SAME  = .TRUE.
///     L1    =  LEN(STR1)
///     L2    =  LEN(STR2)
///     CHECK = MIN ( L1, L2 )
///
///     DO WHILE ( SAME .AND. MORE )
///
///        SAME = EQCHR( STR1(I:I), STR2(I:I) )
///        I    = I + 1
///        MORE = I .LT. CHECK
///
///     END DO
///
///     IF ( .NOT. SAME ) THEN
///
///        There's nothing to do, we already know the strings
///        are not the same.
///
///     ELSE IF ( L1 .LT. L2 ) THEN
///
///        The only way the strings can be regarded as being equal
///        is if the extra unchecked characters in STR2 are all blank.
///
///        SAME = STR2(I:) .EQ. ' '
///
///     ELSE
///
///        Same test as previous one but with STR1 this time.
///
///        SAME = STR1(I:) .EQ. ' '
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 17-SEP-1998 (EDW)
///
///         Replace the UVALUE data statement with a loop to fill
///         UVALUE. The Absoft Mac compiler failed to compile the
///         data statement correctly, and so this function failed
///         to work properly in all situations on the Mac. The
///         corrects the problem and functions on all platforms.
///
/// -    SPICELIB Version 1.0.0, 16-MAY-1995 (WLT)
/// ```
pub fn eqchr(ctx: &mut SpiceContext, a: char, b: char) -> bool {
    let ret = EQCHR(
        &[u8::try_from(a).unwrap()],
        &[u8::try_from(b).unwrap()],
        ctx.raw_context(),
    );
    ret
}

//$Procedure EQCHR (Equivalent characters)
pub fn EQCHR(A: &[u8], B: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let A = &A[..1 as usize];
    let B = &B[..1 as usize];

    //
    // Entry points.
    //
    //
    // Range of characters
    //
    //
    // Local Variables
    //
    // The array UVALUE contains the ICHAR values for the upper case
    // version of each character.
    //

    //
    // The first time through the loop we set the upper case values
    // for each of the lower case letters.
    //
    if save.FIRST {
        {
            let m1__: i32 = 0;
            let m2__: i32 = LSTCHR;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.UVALUE[save.I] = save.I;
                save.I += m3__;
            }
        }

        save.FIRST = false;

        save.UVALUE[intrinsics::ICHAR(b"a")] = intrinsics::ICHAR(b"A");
        save.UVALUE[intrinsics::ICHAR(b"b")] = intrinsics::ICHAR(b"B");
        save.UVALUE[intrinsics::ICHAR(b"c")] = intrinsics::ICHAR(b"C");
        save.UVALUE[intrinsics::ICHAR(b"d")] = intrinsics::ICHAR(b"D");
        save.UVALUE[intrinsics::ICHAR(b"e")] = intrinsics::ICHAR(b"E");
        save.UVALUE[intrinsics::ICHAR(b"f")] = intrinsics::ICHAR(b"F");
        save.UVALUE[intrinsics::ICHAR(b"g")] = intrinsics::ICHAR(b"G");
        save.UVALUE[intrinsics::ICHAR(b"h")] = intrinsics::ICHAR(b"H");
        save.UVALUE[intrinsics::ICHAR(b"i")] = intrinsics::ICHAR(b"I");
        save.UVALUE[intrinsics::ICHAR(b"j")] = intrinsics::ICHAR(b"J");
        save.UVALUE[intrinsics::ICHAR(b"k")] = intrinsics::ICHAR(b"K");
        save.UVALUE[intrinsics::ICHAR(b"l")] = intrinsics::ICHAR(b"L");
        save.UVALUE[intrinsics::ICHAR(b"m")] = intrinsics::ICHAR(b"M");
        save.UVALUE[intrinsics::ICHAR(b"n")] = intrinsics::ICHAR(b"N");
        save.UVALUE[intrinsics::ICHAR(b"o")] = intrinsics::ICHAR(b"O");
        save.UVALUE[intrinsics::ICHAR(b"p")] = intrinsics::ICHAR(b"P");
        save.UVALUE[intrinsics::ICHAR(b"q")] = intrinsics::ICHAR(b"Q");
        save.UVALUE[intrinsics::ICHAR(b"r")] = intrinsics::ICHAR(b"R");
        save.UVALUE[intrinsics::ICHAR(b"s")] = intrinsics::ICHAR(b"S");
        save.UVALUE[intrinsics::ICHAR(b"t")] = intrinsics::ICHAR(b"T");
        save.UVALUE[intrinsics::ICHAR(b"u")] = intrinsics::ICHAR(b"U");
        save.UVALUE[intrinsics::ICHAR(b"v")] = intrinsics::ICHAR(b"V");
        save.UVALUE[intrinsics::ICHAR(b"w")] = intrinsics::ICHAR(b"W");
        save.UVALUE[intrinsics::ICHAR(b"x")] = intrinsics::ICHAR(b"X");
        save.UVALUE[intrinsics::ICHAR(b"y")] = intrinsics::ICHAR(b"Y");
        save.UVALUE[intrinsics::ICHAR(b"z")] = intrinsics::ICHAR(b"Z");
    }

    save.I = intrinsics::ICHAR(A);
    save.J = intrinsics::ICHAR(B);

    if ((save.I > LSTCHR) || (save.J > LSTCHR)) {
        save.EQCHR = (save.I == save.J);
    } else {
        save.EQCHR = (save.UVALUE[save.I] == save.UVALUE[save.J]);
    }

    save.EQCHR
}

/// Not Equivalent characters
///
/// Return .TRUE. if two given characters are not equivalent if the
/// case of the characters is ignored.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  A          I   one of the characters to check
///  B          I   the other character to check
///
///  The function returns .TRUE. if the characters are not equivalent
/// ```
///
/// # Detailed Input
///
/// ```text
///  A,
///  B        are two characters that are to be compared to see
///           if they are different letters. Letters that have
///           the same value when converted to uppercase are
///           considered to be equivalent.
/// ```
///
/// # Detailed Output
///
/// ```text
///  The function returns the value .FALSE. if the two input characters
///  are the same or can be made the same by converting both to
///  upper or lower case. Otherwise it returns .TRUE.
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
///  This routine simply determines the truth value of .NOT. EQCHR.
///  See the entry point EQCHR for a discussion of that function.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you want to determine whether or not two strings
///  are the same up to differences in case. The following
///  code fragment shows how you can use this routine to check
///  for the equivalence of character strings.
///
///     MORE  = .TRUE.
///     SAME  = .TRUE.
///     L1    =  LEN(STR1)
///     L2    =  LEN(STR2)
///     CHECK = MIN ( L1, L2 )
///
///     DO WHILE ( SAME .AND. MORE )
///
///        IF ( NECHR(STR1(I:I),STR2(I:I) ) THEN
///           SAME = .FALSE.
///        END IF
///
///        I    = I + 1
///        MORE = I .LT. CHECK
///
///     END DO
///
///     IF ( .NOT. SAME ) THEN
///
///        There's nothing to do, we already know the strings
///        are not the same.
///
///     ELSE IF ( L1 .LT. L2 ) THEN
///
///        The only way the strings can be regarded as being equal
///        is if the extra unchecked characters in STR2 are all blank.
///
///        SAME = STR2(I:) .EQ. ' '
///
///     ELSE
///
///        Same test as previous one but with STR1 this time.
///
///        SAME = STR1(I:) .EQ. ' '
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 17-SEP-1998 (EDW)
///
///         Replace the UVALUE data statement with a loop to fill
///         UVALUE. The Absoft Mac compiler failed to compile the
///         data statement correctly, and so this function failed
///         to work properly in all situations on the Mac. The
///         corrects the problem and functions on all platforms.
///
/// -    SPICELIB Version 1.0.0, 16-MAY-1995 (WLT)
/// ```
pub fn nechr(ctx: &mut SpiceContext, a: char, b: char) -> bool {
    let ret = NECHR(
        &[u8::try_from(a).unwrap()],
        &[u8::try_from(b).unwrap()],
        ctx.raw_context(),
    );
    ret
}

//$Procedure NECHR (Not Equivalent characters)
pub fn NECHR(A: &[u8], B: &[u8], ctx: &mut Context) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let A = &A[..1 as usize];
    let B = &B[..1 as usize];

    if save.FIRST {
        save.FIRST = false;

        {
            let m1__: i32 = 0;
            let m2__: i32 = LSTCHR;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.UVALUE[save.I] = save.I;
                save.I += m3__;
            }
        }

        save.UVALUE[intrinsics::ICHAR(b"a")] = intrinsics::ICHAR(b"A");
        save.UVALUE[intrinsics::ICHAR(b"b")] = intrinsics::ICHAR(b"B");
        save.UVALUE[intrinsics::ICHAR(b"c")] = intrinsics::ICHAR(b"C");
        save.UVALUE[intrinsics::ICHAR(b"d")] = intrinsics::ICHAR(b"D");
        save.UVALUE[intrinsics::ICHAR(b"e")] = intrinsics::ICHAR(b"E");
        save.UVALUE[intrinsics::ICHAR(b"f")] = intrinsics::ICHAR(b"F");
        save.UVALUE[intrinsics::ICHAR(b"g")] = intrinsics::ICHAR(b"G");
        save.UVALUE[intrinsics::ICHAR(b"h")] = intrinsics::ICHAR(b"H");
        save.UVALUE[intrinsics::ICHAR(b"i")] = intrinsics::ICHAR(b"I");
        save.UVALUE[intrinsics::ICHAR(b"j")] = intrinsics::ICHAR(b"J");
        save.UVALUE[intrinsics::ICHAR(b"k")] = intrinsics::ICHAR(b"K");
        save.UVALUE[intrinsics::ICHAR(b"l")] = intrinsics::ICHAR(b"L");
        save.UVALUE[intrinsics::ICHAR(b"m")] = intrinsics::ICHAR(b"M");
        save.UVALUE[intrinsics::ICHAR(b"n")] = intrinsics::ICHAR(b"N");
        save.UVALUE[intrinsics::ICHAR(b"o")] = intrinsics::ICHAR(b"O");
        save.UVALUE[intrinsics::ICHAR(b"p")] = intrinsics::ICHAR(b"P");
        save.UVALUE[intrinsics::ICHAR(b"q")] = intrinsics::ICHAR(b"Q");
        save.UVALUE[intrinsics::ICHAR(b"r")] = intrinsics::ICHAR(b"R");
        save.UVALUE[intrinsics::ICHAR(b"s")] = intrinsics::ICHAR(b"S");
        save.UVALUE[intrinsics::ICHAR(b"t")] = intrinsics::ICHAR(b"T");
        save.UVALUE[intrinsics::ICHAR(b"u")] = intrinsics::ICHAR(b"U");
        save.UVALUE[intrinsics::ICHAR(b"v")] = intrinsics::ICHAR(b"V");
        save.UVALUE[intrinsics::ICHAR(b"w")] = intrinsics::ICHAR(b"W");
        save.UVALUE[intrinsics::ICHAR(b"x")] = intrinsics::ICHAR(b"X");
        save.UVALUE[intrinsics::ICHAR(b"y")] = intrinsics::ICHAR(b"Y");
        save.UVALUE[intrinsics::ICHAR(b"z")] = intrinsics::ICHAR(b"Z");
    }

    save.I = intrinsics::ICHAR(A);
    save.J = intrinsics::ICHAR(B);

    if ((save.I > LSTCHR) || (save.J > LSTCHR)) {
        save.NECHR = (save.I != save.J);
    } else {
        save.NECHR = (save.UVALUE[save.I] != save.UVALUE[save.J]);
    }

    save.NECHR
}
