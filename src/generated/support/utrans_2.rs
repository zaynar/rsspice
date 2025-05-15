//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure      UTRANS_2 ( Translate Units To Default Units )
pub fn UTRANS_2(STRING: &mut [u8], PLACES: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut DPNUM = [b' '; 32 as usize];
    let mut MYERR = [b' '; 80 as usize];
    let mut BASICS = [b' '; 127 as usize];
    let mut BEG: i32 = 0;
    let mut BU: i32 = 0;
    let mut END: i32 = 0;
    let mut EU: i32 = 0;
    let mut POINTR: i32 = 0;
    let mut START: i32 = 0;
    let mut F: i32 = 0;
    let mut L: i32 = 0;
    let mut ERASED: bool = false;
    let mut MEASEQ: bool = false;
    let mut X: f64 = 0.0;
    let mut CONVERT: f64 = 0.0;

    //
    // NAIFLIB functions
    //

    //
    // Local variables
    //

    //
    // First thing, we left justify the command.
    //
    spicelib::LJUST(&STRING.to_vec(), STRING);

    MEASEQ = false;
    ERASED = false;

    //
    // Find the last word of the string.
    //
    START = (intrinsics::LEN(STRING) + 1);
    FNDPTK(STRING, b" ", START, &mut BEG, &mut END, ctx)?;

    while (BEG > 0) {
        //
        // If we are in a measurement sequence, then we need to see if
        // the current word is a number.
        //
        if MEASEQ {
            fstr::assign(&mut MYERR, b" ");
            spicelib::NPARSD(
                fstr::substr(STRING, BEG..=END),
                &mut X,
                &mut MYERR,
                &mut POINTR,
                ctx,
            );

            //
            // If no error occurred in the attempt to parse this number
            // the measurement sequence continues.
            //
            if fstr::eq(&MYERR, b" ") {
                //
                // If we haven't already erased the current unit, do so
                // now and record our action.
                //
                if !ERASED {
                    fstr::assign(fstr::substr_mut(STRING, BU..=EU), b" ");
                    ERASED = true;
                }

                fstr::assign(fstr::substr_mut(STRING, BEG..=END), b" ");
                X = (X * CONVERT);

                spicelib::DPSTRF(X, PLACES, b"E", &mut DPNUM, ctx);
                spicelib::SIGDGT(&DPNUM.clone(), &mut DPNUM);
                spicelib::PREFIX(&DPNUM, 1, fstr::substr_mut(STRING, BEG..));

            //
            // If an error DID occur while attempting to parse the
            // current word, we are ending the current measurment
            // sequence.  However, we might be beginning another ...
            //
            } else {
                //
                // ... search the list of recognized units for this word
                //
                //
                if UNITP(fstr::substr(STRING, BEG..=END), ctx)? {
                    // WRITE (*,*) STRING(BEG:END)
                    fstr::assign(&mut BASICS, b" ");
                    TRANSU(fstr::substr(STRING, BEG..=END), &mut BASICS, ctx)?;
                    F = intrinsics::MAX0(&[1, spicelib::FRSTNB(&BASICS)]);
                    L = intrinsics::MAX0(&[1, spicelib::LASTNB(&BASICS)]);
                    // WRITE (*,*) BASICS(F:L)
                    CONVRT_2(
                        1.0,
                        fstr::substr(STRING, BEG..=END),
                        fstr::substr(&BASICS, F..=L),
                        &mut CONVERT,
                        ctx,
                    )?;
                    MEASEQ = true;
                } else {
                    MEASEQ = false;
                }

                //
                // ... if this word is on the list, record its place in the
                // string.
                //

                if MEASEQ {
                    BU = BEG;
                    EU = END;

                    //
                    // We haven't erased this unit from the string yet.
                    // Record this observation.
                    //
                    ERASED = false;
                }
            }
        } else {
            //
            // We were not in a measurment sequence, but we might be
            // starting one.  Search the list of known units for the
            // current word.
            //
            if UNITP(fstr::substr(STRING, BEG..=END), ctx)? {
                // WRITE (*,*) STRING(BEG:END)
                fstr::assign(&mut BASICS, b" ");
                TRANSU(fstr::substr(STRING, BEG..=END), &mut BASICS, ctx)?;
                F = intrinsics::MAX0(&[1, spicelib::FRSTNB(&BASICS)]);
                L = intrinsics::MAX0(&[1, spicelib::LASTNB(&BASICS)]);
                // WRITE (*,*) BASICS(F:L)
                CONVRT_2(
                    1.0,
                    fstr::substr(STRING, BEG..=END),
                    fstr::substr(&BASICS, F..=L),
                    &mut CONVERT,
                    ctx,
                )?;
                MEASEQ = true;
            } else {
                MEASEQ = false;
            }

            if MEASEQ {
                BU = BEG;
                EU = END;

                //
                // We certainly haven't erased this unit yet.
                //
                ERASED = false;
            }
        }

        //
        // Find the word previous to the current one.
        //
        START = BEG;
        FNDPTK(STRING, b" ", START, &mut BEG, &mut END, ctx)?;
    }

    Ok(())
}
