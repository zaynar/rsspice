//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;

struct SaveVars {
    STRBFF: ActualCharArray,
    NATBFF: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut STRBFF = ActualCharArray::new(STRSIZ, 1..=NUMBFF);
        let mut NATBFF: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;
        NATBFF = 0;

        Self {
            STRBFF,
            NATBFF,
            FIRST,
        }
    }
}

//$Procedure T_XLTFWD ( Translate For Write, Double Precision Values )
pub fn T_XLTFWD(
    INPUT: &[f64],
    NUMDP: i32,
    OUTBFF: i32,
    OUTPUT: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let INPUT = DummyArray::new(INPUT, 1..);
    let mut TMPSTR = [b' '; STRSIZ as usize];
    let mut DEQUIV: f64 = 0.0;
    let mut J: i32 = 0;
    let mut LENOPT: i32 = 0;
    let mut SPACE: i32 = 0;
    let mut VALUE: i32 = 0;
    let IEQUIV = StackArray::<i32, 2>::new(1..=2);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Equivalence DEQUIV to IEQUIV
    //

    //
    // Saved Variables
    //

    //
    // Data Statements
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_XLTFWD", ctx)?;
    }

    //
    // Perform some initialization tasks.
    //
    if save.FIRST {
        //
        // Populate STRBFF with the appropriate binary file
        // format labels.
        //
        for I in 1..=NUMBFF {
            spicelib::ZZDDHGSD(b"BFF", I, &mut save.STRBFF[I], ctx);
        }

        //
        // Fetch the native binary file format.
        //
        spicelib::ZZPLATFM(b"FILE_FORMAT", &mut TMPSTR, ctx);
        spicelib::UCASE(&TMPSTR.clone(), &mut TMPSTR, ctx);

        save.NATBFF = spicelib::ISRCHC(&TMPSTR, NUMBFF, save.STRBFF.as_arg());

        if (save.NATBFF == 0) {
            spicelib::SETMSG(b"The binary file format, \'#\', is not supported by this version of the toolkit. This is a serious problem, contact NAIF.", ctx);
            spicelib::ERRCH(b"#", &TMPSTR, ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
            return Ok(());
        }

        //
        // Do not perform initialization tasks again.
        //
        save.FIRST = false;
    }

    //
    // Check to see if OUTBFF is valid.  This should never occur if this
    // routine is called properly.
    //
    if ((OUTBFF < 1) || (OUTBFF > NUMBFF)) {
        spicelib::SETMSG(b"The integer code used to indicate the binary file format of the input integers, #, is out of range.  This error should never occur.", ctx);
        spicelib::ERRINT(b"#", OUTBFF, ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
        return Ok(());
    }

    //
    // Get the length of the input string.
    //
    LENOPT = intrinsics::LEN(OUTPUT);

    //
    // Now branch depending on the value of NATBFF.
    //
    if (save.NATBFF == BIGI3E) {
        if (OUTBFF == LTLI3E) {
            //
            // Before we go any further check to see that the length
            // of the output string is appropriate and we have enough
            // room to store the results.  Since this string is to contain
            // LTL-IEEE double precision numbers and this is a BIG-IEEE
            // machine, characters are 1-byte and double precision numbers
            // are 8-bytes.  So the length of OUTPUT must be a multiple
            // of 8.
            //
            SPACE = (LENOPT / 8);

            if ((LENOPT - (SPACE * 8)) != 0) {
                spicelib::SETMSG(b"The output string that is to be translated from the binary format # to format # has a length that is not a multiple of 8 bytes.  This error should never occur.", ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[OUTBFF], ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
                return Ok(());
            }

            //
            // Now check to see that there is enough room to store
            // the number of integers we are to convert.
            //
            if (NUMDP > SPACE) {
                spicelib::SETMSG(b"The caller specified that # double precision numbers are to be translated from binary format # to #.  However there is only room to hold # numbers in the output array.  This error should never occur.", ctx);
                spicelib::ERRINT(b"#", NUMDP, ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[OUTBFF], ctx);
                spicelib::ERRINT(b"#", SPACE, ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
                return Ok(());
            }

            //
            // Start looping over each double precision number and
            // converting them to the 8-byte character packages to
            // be stored in OUTPUT.
            //
            for I in 1..=NUMDP {
                //
                // Compute the substring index of the first character
                // in OUTPUT for this double precision number.
                //
                J = ((8 * (I - 1)) + 1);

                //
                // Now extract and arrange the bytes properly.
                // Since these characters are to be stored in a file
                // utilizing LTL-IEEE, we know that J is the
                // least significant byte and that (J+7) is the
                // most significant.
                //
                // INPUT:
                //
                //         -------------------------------------
                //    . . .|     |  J  | J+1 | J+2 | J+3 |     |. . .
                //         -------------------------------------
                //
                //      OUTPUT(J:J) = CHAR( INPUT(I)'s LSB )
                //         .
                //         .
                //      OUTPUT(J+7:J+7) = CHAR( INPUT(I)'s MSB )
                //
                // Perform the necessary computations.
                //
                spicelib::MOVED(INPUT.subarray(I), 1, std::slice::from_mut(&mut DEQUIV));

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], 24);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(fstr::substr_mut(OUTPUT, J..=J), &intrinsics::CHAR(VALUE));

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], 16);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 1)..=(J + 1)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], 8);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 2)..=(J + 2)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 3)..=(J + 3)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], 24);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 4)..=(J + 4)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], 16);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 5)..=(J + 5)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], 8);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 6)..=(J + 6)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 7)..=(J + 7)),
                    &intrinsics::CHAR(VALUE),
                );
            }
        } else {
            spicelib::SETMSG(b"Unable to translate double precision numbers from binary file format # to #. This error should never occur and is indicative of a bug.  Contact NAIF.", ctx);
            spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
            spicelib::ERRCH(b"#", &save.STRBFF[OUTBFF], ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
            return Ok(());
        }
    } else if (save.NATBFF == LTLI3E) {
        if (OUTBFF == BIGI3E) {
            //
            // Before we go any further check to see that the length
            // of the output string is appropriate and we have enough
            // room to store the results.  Since this string is to
            // contain BIG-IEEE double precision values and this is a
            // LTL-IEEE machine, characters are 1-byte and double
            // precision values are 8-bytes.  So the length of OUTPUT
            // must be a multiple of 8.
            //
            SPACE = (LENOPT / 8);

            if ((LENOPT - (SPACE * 8)) != 0) {
                spicelib::SETMSG(b"The output string that is to be translated from the binary format # to format # has a length that is not a multiple of 8 bytes.  This error should never occur.", ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[OUTBFF], ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
                return Ok(());
            }

            //
            // Now check to see that there is enough room to store
            // the number of integers we are to convert.
            //
            if (NUMDP > SPACE) {
                spicelib::SETMSG(b"The caller specified that # double precision numbers are to be translated from binary format # to #. However there is only room to hold # numbers in the output array.  This error should never occur.", ctx);
                spicelib::ERRINT(b"#", NUMDP, ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
                spicelib::ERRCH(b"#", &save.STRBFF[OUTBFF], ctx);
                spicelib::ERRINT(b"#", SPACE, ctx);
                spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
                spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
                return Ok(());
            }

            //
            // Start looping over each double precision number and
            // converting them to the 8-byte character packages to
            // be stored in OUTPUT.
            //
            for I in 1..=NUMDP {
                //
                // Compute the substring index of the first character
                // in OUTPUT for this double precision number.
                //
                J = ((8 * (I - 1)) + 1);

                //
                // Now extract and arrange the bytes properly.
                // Since these characters are to be stored in a file
                // utilizing BIG-IEEE, we know that J is the most
                // significant byte and that (J+7) is the least
                // significant.
                //
                // INPUT:
                //
                //         -------------------------------------
                //    . . .|     |  J  | J+1 | J+2 | J+3 |     |. . .
                //         -------------------------------------
                //
                //      OUTPUT(J:J) = CHAR( INPUT(I)'s MSB )
                //         .
                //         .
                //      OUTPUT(J+7:J+7) = CHAR( INPUT(I)'s LSB )
                //
                // Perform the necessary computations.
                //
                spicelib::MOVED(INPUT.subarray(I), 1, std::slice::from_mut(&mut DEQUIV));

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], -24);
                fstr::assign(fstr::substr_mut(OUTPUT, J..=J), &intrinsics::CHAR(VALUE));

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], 8);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 1)..=(J + 1)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], 16);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 2)..=(J + 2)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[2], 24);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 3)..=(J + 3)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 4)..=(J + 4)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], 8);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 5)..=(J + 5)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], 16);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 6)..=(J + 6)),
                    &intrinsics::CHAR(VALUE),
                );

                VALUE = intrinsics::ISHFT(DummyArray::<i32>::from_equiv(&[DEQUIV], 1..=2)[1], 24);
                VALUE = intrinsics::ISHFT(VALUE, -24);
                fstr::assign(
                    fstr::substr_mut(OUTPUT, (J + 7)..=(J + 7)),
                    &intrinsics::CHAR(VALUE),
                );
            }
        } else {
            spicelib::SETMSG(b"Unable to translate double precision numbers from binary file format # to #. This error should never occur and is indicative of a bug.  Contact NAIF.", ctx);
            spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
            spicelib::ERRCH(b"#", &save.STRBFF[OUTBFF], ctx);
            spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
            spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
            return Ok(());
        }

    //
    // The native binary file format on this platform is not supported
    // for the conversion of integers.  This is a bug, as this branch
    // of code should never be reached in normal operation.
    //
    } else {
        spicelib::SETMSG(b"The native binary file format of this toolkit build, #, is not currently supported for translation of double precision numbers from non-native formats.", ctx);
        spicelib::ERRCH(b"#", &save.STRBFF[save.NATBFF], ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"T_XLTFWD", ctx)?;
    Ok(())
}
