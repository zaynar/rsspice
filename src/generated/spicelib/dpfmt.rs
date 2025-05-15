//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const WDSIZE: i32 = 32;

/// Format a double precision number
///
/// Create a formatted string that represents a double precision
/// number, using a format picture.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  X          I   a double precision number.
///  PICTUR     I   a string describing the appearance of the output
///  STR        O   a string representing X as prescribed by PICTUR
/// ```
///
/// # Detailed Input
///
/// ```text
///  X        is any double precision number.
///
///  PICTUR   is a string used to describe the format of the
///           output string. There are four special characters
///           recognized by DPFMT --- a leading + or -, a leading
///           zero ( '0' ) or a zero that follows a leading + or -,
///           and the first decimal point of the string.
///
///           All other non-blank characters are regarded as
///           equivalent. The picture ends at the first blank
///           character. The effects associated with the various
///           characters in a picture are spelled out in the
///           description of the output STRING.
///
///           The following pictures are treated as errors.
///
///           ' ', '+', '-', '.', '+.', '-.'
/// ```
///
/// # Detailed Output
///
/// ```text
///  STRING   is a string representing X that matches the input
///           picture. The format of STRING is governed by PICTUR.
///           It will represent X rounded to the level of precision
///           specified by PICTUR.
///
///           If the first character of the picture is a minus sign,
///           the first character in the output string will be
///           a blank if the number is non-negative, a minus sign
///           if the number is negative.
///
///           If the first character of the picture is a plus sign,
///           the first character of the output string will be a
///           plus if the number is positive, a blank if the number
///           is zero, and a minus sign if the number is negative.
///
///           If the first character of the string is NOT a sign
///           (plus or minus) the first character of the output
///           string will be a minus sign if the number is negative
///           and will be the first character of the integer part
///           of the number otherwise.
///
///           The integer portion of STRING will contain the same
///           number of characters as appear before the decimal
///           point (or last character if there is no decimal
///           point) but after a leading + or -.
///
///           If the picture begins with any of the following
///
///              '+0', '-0', or '0'
///
///           it is said to have a leading zero. If a picture has
///           a leading zero and the integer portion is not large
///           enough to fill up the integer space specified by
///           PICTUR, STRING will be zero padded from the sign (if
///           one is required) up to the first character of the
///           integer part of the number.
///
///           If picture does NOT have a leading zero and the
///           integer portion is not large enough to fill up the
///           space specified by PICTUR, STRING will be blank
///           padded on the left between the sign (if one is
///           required) and the first character of the integer part
///           of the number.
///
///           If a decimal point ( '.' ) is present in PICTUR it
///           will be present following the integer portion of
///           STRING. Moreover, the decimal portion of STRING will
///           contain the same number of digits as there are
///           non-blank characters following the decimal point in
///           PICTUR. However, only the first 14 digits starting
///           with the first non-zero digit are meaningful.
///
///           If the format specified by PICTUR does not provide
///           enough room for the integer portion of X, the routine
///           determines whether or not the number of characters
///           present in the picture is sufficient to create a
///           representation for X using scientific notation. If
///           so, the output is displayed using scientific notation
///           (leading signs, if they are present in PICTUR, will
///           also appear in STRING).   If the format specified by
///           PICTUR is too short to accommodate scientific
///           notation, the output string is filled with '*' to the
///           same length as the length of PICTUR. Leading signs
///           are not preserved in this overflow case.
///
///           STRING may overwrite PICTUR.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If PICTUR begins with a blank, the error SPICE(NOPICTURE) is
///      signaled.
///
///  2)  If PICTUR consists only of '+', '-', '.', '+.' or '-.' are
///      regarded as invalid (there's no significant component to the
///      picture.) therefore, the error SPICE(BADPICTURE) is signaled.
///
///  3)  If the length of STR is less than the length of the first
///      non-blank portion of PICTUR, the error SPICE(OUTPUTTOOSHORT)
///      is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides a mechanism for producing numeric strings
///  formatted according to a user supplied picture. We expect that
///  the string produced by this routine will be used to assist in
///  the construction of a string that can be read by people.
///
///  Note that the process of converting a double precision number
///  to a string, in not precisely invertible even if the string
///  contains all of the significant figures allowed by this
///  routine. You should not anticipate that the string produced
///  by this routine can be "read" into a double precision number
///  to reproduce the double precision number X. To the level of
///  accuracy implied by the string representation, they will be
///  the same. But, they are unlikely to have the same internal
///  binary representation.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that X has the binary representation of PI. Then the
///  table below illustrates the strings that would be produced
///  by a variety of different pictures.
///
///  PICTUR         |    STRING
///  -------------------------------
///  '0x.xxx'       |  '03.142'
///  'xx.xxx'       |  ' 3.142'
///  '+xxx.yyyy'    |  '+  3.1416'
///  '-.yyyy'       |  '******'
///  'xxxxxxxx'     |  '       3'
///  '00xx'         |  '0003'
///  '-00.0000000'  |  ' 03.1415927'
///  '00'           |  '03'
///  'x.'           |  '3.'
///  '.mynumber'    |  '3.142E+00'
///  'my dog spot'  |  ' 3'
///  'my.dog spot'  |  ' 3.142'
///  '+my.dog,spot' |  '+ 3.14159265'
///
///
///
///  Suppose that X has the binary representation of 2/3. Then the
///  table below illustrates the strings that would be produced
///  by a variety of different pictures.
///
///  PICTUR         |    STRING
///  -------------------------------
///  '+x.xxx'       |  '+0.667'
///  '+xx.xxx'      |  '+ 0.667'
///  'xxx.yyyy'     |  '  0.6667'
///  '.yyyy'        |  '.6667'
///  'xxxxxxxx'     |  '       1'
///  '00xx'         |  '0001'
///  '-0.0000000'   |  ' 0.6666667'
///  '00'           |  '01'
///  'x.'           |  '1.'
///  'mynumber'     |  '       1'
///  'my dog spot'  |  ' 1'
///  'my.dog spot'  |  ' 0.667'
///  'my.dog,spot'  |  ' 0.66666667'
///
///  Suppose that X has the binary representation of -8/9. Then the
///  table below illustrates the strings that would be produced
///  by a variety of different pictures.
///
///
///  PICTUR         |    STRING
///  -------------------------------
///  '+x.xxx'       |  '-0.889'
///  '-00.xxxx'     |  '-00.8889'
///  'xxx.xxx'      |  ' -0.889'
///  '000.000'      |  '-00.889'
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.3, 27-OCT-2021 (JDR) (EDW) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.2, 31-JAN-2008 (BVS)
///
///         Removed non-standard end-of-declarations marker
///         'C%&END_DECLARATIONS' from comments.
///
/// -    SPICELIB Version 1.0.1, 22-JUN-1998 (WLT)
///
///         A number of typographical and grammatical errors
///         were corrected in the header.
///
/// -    SPICELIB Version 1.0.0, 17-SEP-1996 (WLT)
/// ```
pub fn dpfmt(ctx: &mut SpiceContext, x: f64, pictur: &str, str: &mut str) -> crate::Result<()> {
    DPFMT(
        x,
        pictur.as_bytes(),
        fstr::StrBytes::new(str).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DPFMT ( Format a double precision number )
pub fn DPFMT(X: f64, PICTUR: &[u8], STR: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut FILL = [b' '; 1 as usize];
    let mut SIGN = [b' '; 1 as usize];
    let mut MYSTR = [b' '; WDSIZE as usize];
    let mut Y: f64 = 0.0;
    let mut DECLEN: i32 = 0;
    let mut DPAT: i32 = 0;
    let mut EXP: i32 = 0;
    let mut EXPSIZ: i32 = 0;
    let mut FRSTCH: i32 = 0;
    let mut INTLEN: i32 = 0;
    let mut LASTCH: i32 = 0;
    let mut FIRSTB: i32 = 0;
    let mut SGNLEN: i32 = 0;
    let mut SIGDIG: i32 = 0;
    let mut SPRSIZ: i32 = 0;
    let mut START: i32 = 0;
    let mut NEEDSN: bool = false;
    let mut OVFLOW: bool = false;
    let mut SHIFT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Initial values
    //

    //
    // Determine where the picture ends.
    //
    FIRSTB = POS(PICTUR, b" ", 1);

    if (FIRSTB == 0) {
        LASTCH = intrinsics::LEN(PICTUR);
    } else {
        LASTCH = (FIRSTB - 1);
    }

    //
    // Make sure there is a picture to worry about.
    //
    if (LASTCH == 0) {
        CHKIN(b"DPFMT", ctx)?;
        SETMSG(b"The format picture must begin with a non-blank character. The picture supplied was begun with a blank.", ctx);
        SIGERR(b"SPICE(NOPICTURE)", ctx)?;
        CHKOUT(b"DPFMT", ctx)?;
        return Ok(());
    } else if (LASTCH == 1) {
        if ((fstr::eq(PICTUR, b"+") || fstr::eq(PICTUR, b"-")) || fstr::eq(PICTUR, b".")) {
            CHKIN(b"DPFMT", ctx)?;
            SETMSG(b"Format pictures must have at least one significant character. The picture provided \'#\' does not. ", ctx);
            ERRCH(b"#", fstr::substr(PICTUR, 1..=1), ctx);
            SIGERR(b"SPICE(BADPICTURE)", ctx)?;
            CHKOUT(b"DPFMT", ctx)?;
            return Ok(());
        }
    } else if (LASTCH == 2) {
        if (fstr::eq(PICTUR, b"+.") || fstr::eq(PICTUR, b"-.")) {
            CHKIN(b"DPFMT", ctx)?;
            SETMSG(b"Format pictures must have at least one significant character. The picture provided \'#\' does not. ", ctx);
            ERRCH(b"#", fstr::substr(PICTUR, 1..=2), ctx);
            SIGERR(b"SPICE(BADPICTURE)", ctx)?;
            CHKOUT(b"DPFMT", ctx)?;
            return Ok(());
        }
    } else if (LASTCH > intrinsics::LEN(STR)) {
        CHKIN(b"DPFMT", ctx)?;
        SETMSG(b"The output string is not long enough to accommodate a number formatted according to the supplied format picture. The output string has length #. The output picture \'#\' requires # characters. ", ctx);

        ERRINT(b"#", intrinsics::LEN(STR), ctx);
        ERRCH(b"#", fstr::substr(PICTUR, 1..=LASTCH), ctx);
        ERRINT(b"#", LASTCH, ctx);
        SIGERR(b"SPICE(OUTPUTTOOSHORT)", ctx)?;
        CHKOUT(b"DPFMT", ctx)?;
        return Ok(());
    }

    //
    // If we get this far, the routine can go ahead and do its business.
    // Determine the sign of X.  Also, determine how many characters
    // are needed to represent the sign if leading sign is suppressed for
    // positive numbers.
    //
    if (X > 0 as f64) {
        fstr::assign(&mut SIGN, b"+");
        SPRSIZ = 0;
    } else if (X < 0 as f64) {
        fstr::assign(&mut SIGN, b"-");
        SPRSIZ = 1;
    } else {
        fstr::assign(&mut SIGN, b" ");
        SPRSIZ = 0;
    }
    //
    // Look at the picture and see if a leading sign is required and
    // if so whether the sign just determined should use a different
    // character and how many characters are needed for the sign.
    //
    if fstr::eq(fstr::substr(PICTUR, 1..=1), b"+") {
        NEEDSN = true;
        SGNLEN = 1;
    } else if fstr::eq(fstr::substr(PICTUR, 1..=1), b"-") {
        NEEDSN = true;
        SGNLEN = 1;

        if (X > 0 as f64) {
            fstr::assign(&mut SIGN, b" ");
        }
    } else {
        if (X > 0 as f64) {
            fstr::assign(&mut SIGN, b" ");
        }

        NEEDSN = false;
        SGNLEN = SPRSIZ;
    }

    //
    // If we need a leading sign. The numeric part of the string
    // will start at character 2.  Otherwise it starts at character 1.
    //
    if NEEDSN {
        START = 2;
    } else {
        START = 1;
    }

    //
    // We can set the sign portion of the string now.
    //
    fstr::assign(STR, &SIGN);

    //
    // Determine what character should be use for leading characters
    // before the first significant character of the output string.
    //
    if fstr::eq(fstr::substr(PICTUR, START..=START), b"0") {
        fstr::assign(&mut FILL, b"0");
    } else {
        fstr::assign(&mut FILL, b" ");
    }

    //
    // See if there is a decimal point.
    //
    DPAT = POS(PICTUR, b".", 1);
    //
    // The integer part is the stuff to the left of the first
    // decimal point and that follows the sign (if there is one
    // that is explicitly required.  The length of the decimal
    // portion is the stuff to the right of the decimal point.
    //
    if (DPAT > 0) {
        INTLEN = (DPAT - START);
        DECLEN = (LASTCH - DPAT);
    } else {
        INTLEN = ((LASTCH - START) + 1);
        DECLEN = -1;
    }
    //
    // If a sign was not explicitly requested by placing it in
    // the first digit of the picture START will be 1.  If in
    // addition X is less than zero ( SGNLEN will be 1 in this
    // case) we have one fewer digits available for the integer
    // portion of the string than is currently set in INTLEN.
    // Adjust INTLEN to reflect the actual number of digits
    // available.
    //
    // Also set the SHIFT flag to .TRUE. so that we know to swap
    // the sign and any blanks that might lie between the sign
    // and the first significant character of the output string.
    //
    if ((START == 1) && (SGNLEN == 1)) {
        INTLEN = (INTLEN - 1);
        SHIFT = true;
        //
        // If INTLEN has become negative (i.e. -1) the picture
        // must be of the form .xxxxx and the input number must
        // be negative. Add 1 back onto the INTLEN but take one
        // away from the decimal length DECLEN.
        //
        if (INTLEN == -1) {
            INTLEN = 0;
            DECLEN = (DECLEN - 1);

            if ((DECLEN == 0) && (INTLEN == 0)) {
                //
                // There is no room for anything other than a
                // decimal point.  We simply fill the output
                // string with the '*' character.
                //
                for I in 1..=LASTCH {
                    fstr::assign(fstr::substr_mut(STR, I..=I), b"*");
                }
                return Ok(());
            }
        }
    } else {
        SHIFT = false;
    }

    //
    // Create the "virtual decimal string" associated with the
    // unsigned part of X.
    //
    Y = f64::abs(X);

    ZZVSTSTR(Y, &FILL, &mut EXP, ctx);

    //
    // The actual number of digits required to print the unsigned integer
    // portion X is EXP + 1 (provided EXP is at least 0.) We have
    // INTLEN slots available.  So if EXP + 1 is more than INTLEN
    // ( which is equivalent to EXP being at least INTLEN) we don't
    // have enough room to print the unsigned integer portion of the
    // number.
    //

    if ((EXP >= INTLEN) && (Y != 0.0)) {
        //
        // See if we have room to print an exponential form.
        // First we need the number of characters for the
        // exponent which is always of the form 'E+dd...'
        //
        EXPSIZ = ((4 + intrinsics::MIN0(&[1, (EXP / 1000)])) + intrinsics::MIN0(&[1, (EXP / 100)]));
        //
        // The number of significant digits that can be printed is the
        // size of the picture minus:   the size of the sign
        //                              the size of the exponent
        //                              the size of the decimal point.
        //
        SIGDIG = (((LASTCH - SGNLEN) - EXPSIZ) - 1);
        //
        // If we don't have room for at least one significant digit,
        // there's not much we can do.  Fill the string with '*'.
        //

        if (SIGDIG < 1) {
            for I in 1..=LASTCH {
                fstr::assign(fstr::substr_mut(STR, I..=I), b"*");
            }
        } else {
            DPSTR(X, SIGDIG, &mut MYSTR, ctx);
            fstr::assign(fstr::substr_mut(&mut MYSTR, 1..=1), &SIGN);
            LJUST(&MYSTR, STR);
            RJUST(
                &fstr::substr(STR, 1..=LASTCH).to_vec(),
                fstr::substr_mut(STR, 1..=LASTCH),
            );
        }

        return Ok(());
    }
    //
    // One more check.  If -INTLEN is greater than DECLEN, or if
    // both are zero, we don't have room to create an output string.
    //
    if (((INTLEN == 0) && (DECLEN == 0)) || (-INTLEN > DECLEN)) {
        for I in 1..=LASTCH {
            fstr::assign(fstr::substr_mut(STR, I..=I), b"*");
        }

        return Ok(());
    }
    //
    // We have a reasonable chance of successfully constructing
    // the string without overflow.
    //
    START = (SGNLEN + 1);
    ZZVSBSTR(
        -INTLEN,
        DECLEN,
        true,
        fstr::substr_mut(STR, START..),
        &mut OVFLOW,
        ctx,
    );

    //
    // We might be done at this point.  The IF-THEN block below
    // handles the one snag that could arise.
    //
    // If the first digit is a zero as a result of rounding it up
    // OVFLOW will be true.  This means we don't have enough room
    // in the picture for the integer portion of the string.  We try
    // to make an exponential picture.
    //
    if OVFLOW {
        //
        // See if we have room to print an exponential form.
        //
        EXPSIZ = ((4 + intrinsics::MIN0(&[1, (EXP / 1000)])) + intrinsics::MIN0(&[1, (EXP / 100)]));
        //
        // The number of significant digits that can be printed is the
        // size of the picture minus:   the size of the sign
        //                              the size of the exponent
        //                              the size of the decimal point.
        //
        SIGDIG = (((LASTCH - SGNLEN) - EXPSIZ) - 1);

        if (SIGDIG < 1) {
            for I in 1..=LASTCH {
                fstr::assign(fstr::substr_mut(STR, I..=I), b"*");
            }
        } else {
            DPSTR(X, SIGDIG, &mut MYSTR, ctx);
            fstr::assign(fstr::substr_mut(&mut MYSTR, 1..=1), &SIGN);
            LJUST(&MYSTR, STR);
            RJUST(
                &fstr::substr(STR, 1..=LASTCH).to_vec(),
                fstr::substr_mut(STR, 1..=LASTCH),
            );
            return Ok(());
        }
    } else if SHIFT {
        //
        // We need to move the sign right until, there are no
        // blanks between it and the next character.
        //
        FRSTCH = NCPOS(STR, b" -", 1);

        if (FRSTCH > 2) {
            let val = fstr::substr(STR, 1..=1).to_vec();
            fstr::assign(fstr::substr_mut(STR, (FRSTCH - 1)..=(FRSTCH - 1)), &val);
            fstr::assign(fstr::substr_mut(STR, 1..=1), b" ");
        }
    }

    Ok(())
}
