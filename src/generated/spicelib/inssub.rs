//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Insert a substring
///
/// Insert a substring into a character string at a specified
/// location.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IN         I   Input string.
///  SUB        I   Substring to be inserted.
///  LOC        I   Position at which substring is to be inserted.
///  OUT        O   Output string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IN       is an input character string, into which a substring
///           is to be inserted.
///
///  SUB      is the substring to be inserted. Leading and trailing
///           blanks are significant.
///
///  LOC      is the index in the input string at which the substring
///           is to be inserted. The range of LOC is 1:LEN(IN). To
///           append to the string, set LOC equal to LEN(IN)+1.
/// ```
///
/// # Detailed Output
///
/// ```text
///  OUT      is the output string. This is equivalent to the
///           string that would be created by the concatenation
///
///              OUT = IN(1:LOC-1) // SUB // IN(LOC: )
///
///           If the output string is too long, it is truncated
///           on the right.
///
///           OUT may overwrite IN. OUT may NOT overwrite SUB.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If LOC is not in the interval [1, LEN(IN)+1], the error
///      SPICE(INVALIDINDEX) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  Shift the end of the input string, beginning with LOC, to the
///  right, leaving space for the substring. Then insert the substring
///  into the vacated space in the middle of the string. This has
///  the same effect as the concatenation
///
///     OUT = IN(1:LOC-1) // SUB // IN(LOC: )
///
///  Because this operation is not standard for strings of length (*),
///  this routine does not use concatenation.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) The following table illustrates the use of INSSUB.
///
///        IN                SUB      LOC    OUT
///        ----------------- -------  ---    ------------------------
///        'ABCDEFGHIJ'      ' YXZ '    3    'AB XYZ CDEFGHIJ'
///        'The rabbit'      'best '    5    'The best rabbit'
///        ' other woman'    'The'      1    'The other woman'
///        'An Apple a day'  ' keeps'  15    'An Apple a day keeps'
///        'Apple a day'     'An '      0     An error is signaled.
///        'Apple a day'     'An '     -3     An error is signaled.
///        'An Apple a day'  ' keeps'  16     An error is signaled.
///
///
///  2) INSSUB could be used to insert substrings at any position
///     within the declared length of the input string. Use it to
///     add an ascii arrow (--->) at the end of the string.
///
///
///     Example code begins here.
///
///
///           PROGRAM INSSUB_EX2
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               STRSZ
///           PARAMETER           ( STRSZ = 20 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(STRSZ)     STRIN
///           CHARACTER*(STRSZ)     STROUT
///
///     C
///     C     Assign the input string.
///     C
///           DATA                  STRIN / '0123456789         .' /
///
///
///     C
///     C     Insert a substring at index 17. This should leave 5
///     C     blanks after the sequence of numbers. Note that the
///     C     resulting string is truncated.
///     C
///           CALL INSSUB ( STRIN, '--->', 17, STROUT )
///
///           WRITE(*,*) 'Input string:  ', STRIN
///           WRITE(*,*) 'Output string: ', STROUT
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Input string:  0123456789         .
///      Output string: 0123456789      --->
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
/// -    SPICELIB Version 1.2.0, 01-OCT-2021 (JDR) (NJB)
///
///         Added IMPLICIT NONE statement.
///
///         Updated long error message for LOC out of range check,
///         providing valid range of values.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 1.1.0, 24-OCT-1994 (NJB)
///
///         Bug fixes made. Now does discovery check-in. Header sections
///         re-arranged. Some clean-up of header format done.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (HAN)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 24-OCT-1994 (NJB)
///
///         Bug fix: case where insertion location follows end of
///         input string is now handled correctly. Formerly, an
///         out-of-range substring bound violation was incurred in this
///         case.
///
///         Bug fix: use of SHIFTC routine in old implementation
///         resulted in output string being truncated at length
///         LEN(IN), which is not consistent with the routine's
///         specification.
///
///         Now does discovery check-in. Header sections re-arranged.
///         Some clean-up of header format done.
///
/// -    Beta Version 2.0.0, 04-JAN-1989 (HAN)
///
///         If the location at which the substring is to be inserted is
///         not in the interval [1, LEN(IN)+1], an error is signaled.
///         Locations not within that interval refer to non-existent
///         characters positions. (To append to the string, set the
///         location equal to LEN(IN)+1.)
/// ```
pub fn inssub(
    ctx: &mut SpiceContext,
    in_: &str,
    sub: &str,
    loc: i32,
    out: &mut str,
) -> crate::Result<()> {
    INSSUB(
        in_.as_bytes(),
        sub.as_bytes(),
        loc,
        fstr::StrBytes::new(out).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure INSSUB ( Insert a substring )
pub fn INSSUB(
    IN: &[u8],
    SUB: &[u8],
    LOC: i32,
    OUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CHR = [b' '; 1 as usize];
    let mut FROM: i32 = 0;
    let mut INLEN: i32 = 0;
    let mut NMOVE: i32 = 0;
    let mut OUTLEN: i32 = 0;
    let mut SUBEND: i32 = 0;
    let mut SUBLEN: i32 = 0;
    let mut TO: i32 = 0;
    let mut SAME: bool = false;

    //
    // Local Variables
    //

    //
    // Discovery check-in is used in this routine.
    //
    // Note to the careful reader:  in order to scrupulously avoid
    // non-standard assignments of characters from a substring of IN to
    // an overlapping substring of OUT, in the case where IN and OUT
    // refer to the same memory, we'll test whether the output and
    // input strings are the same.  If they're the same, we can avoid
    // various assignments that could cause trouble if IN and OUT
    // actually refer to the same memory.  This test has little effect on
    // performance, and allows the author to sleep more soundly at night.
    //
    // Capture the lengths of the input, output, and substitution
    // strings.
    //
    INLEN = intrinsics::LEN(IN);
    OUTLEN = intrinsics::LEN(OUT);
    SUBLEN = intrinsics::LEN(SUB);

    //
    // If insertion occurs before the beginning of the string
    // or after INLEN + 1, signal an error.
    //
    if ((LOC < 1) || (LOC > (INLEN + 1))) {
        CHKIN(b"INSSUB", ctx)?;
        SETMSG(b"Index at which the substring is to be inserted is out of the valid range [1,#]. Requested index was *.", ctx);
        ERRINT(b"#", (INLEN + 1), ctx);
        ERRINT(b"*", LOC, ctx);
        SIGERR(b"SPICE(INVALIDINDEX)", ctx)?;
        CHKOUT(b"INSSUB", ctx)?;
        return Ok(());
    }

    //
    // If the insertion occurs after the end of the output string,
    // just return the original string.  Don't do the assignment if
    // the output and input strings have equal values; the assignment
    // is not needed in this cause and could cause a run-time error if
    // OUT and IN refer to the same memory.
    //
    SAME = fstr::eq(OUT, IN);

    if (LOC > OUTLEN) {
        if !SAME {
            fstr::assign(OUT, IN);
        }

        return Ok(());
    }

    //
    // At this point, we're guaranteed that
    //
    //    LOC  <  OUTLEN
    //         -
    //
    //    LOC  <  INLEN + 1
    //         -
    //
    //    LOC  >  0
    //
    //
    // The first part of the input string is copied without change
    // to the output string, if this first part is non-empty.
    //
    if (LOC > 1) {
        //
        // Again, do the assignment only if it's required.
        //
        if !SAME {
            fstr::assign(fstr::substr_mut(OUT, 1..=(LOC - 1)), IN);
        }
    }

    //
    // The part following the new substring is shifted into place, if
    // there's both something to move and a place to put it.  Move the
    // rightmost characters first.
    //
    SUBEND = ((LOC - 1) + SUBLEN);

    if ((LOC <= INLEN) && (SUBEND < OUTLEN)) {
        NMOVE = intrinsics::MIN0(&[(OUTLEN - SUBEND), ((INLEN - LOC) + 1)]);

        for I in intrinsics::range(NMOVE, 1, -1) {
            FROM = ((LOC + I) - 1);
            TO = (SUBEND + I);
            fstr::assign(&mut CHR, fstr::substr(IN, FROM..=FROM));
            fstr::assign(fstr::substr_mut(OUT, TO..=TO), &CHR);
        }
    }

    //
    // And the new word is dropped into the middle.
    //
    fstr::assign(
        fstr::substr_mut(OUT, LOC..=intrinsics::MIN0(&[SUBEND, OUTLEN])),
        SUB,
    );

    //
    // Blank-pad the output string if necessary.
    //
    if (OUTLEN > (INLEN + SUBLEN)) {
        fstr::assign(fstr::substr_mut(OUT, ((INLEN + SUBLEN) + 1)..), b" ");
    }

    Ok(())
}
