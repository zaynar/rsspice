//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const INCLEN: i32 = 2;

//$Procedure ZZGFWSTS ( Private --- GF, sift first window thru second )
pub fn ZZGFWSTS(
    WNDW1: &[f64],
    WNDW2: &[f64],
    INCLSN: &[u8],
    WNDW3: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let WNDW1 = DummyArray::new(WNDW1, LBCELL..);
    let WNDW2 = DummyArray::new(WNDW2, LBCELL..);
    let mut WNDW3 = DummyArrayMut::new(WNDW3, LBCELL..);
    let mut LOCINC = [b' '; INCLEN as usize];
    let mut BEGP1: i32 = 0;
    let mut BEGP2: i32 = 0;
    let mut BEGP3: i32 = 0;
    let mut ENDP1: i32 = 0;
    let mut ENDP2: i32 = 0;
    let mut ENDP3: i32 = 0;
    let mut MAXPTS: i32 = 0;
    let mut OVFLOW: i32 = 0;
    let mut SIZE1: i32 = 0;
    let mut SIZE2: i32 = 0;
    let mut CLOSED: bool = false;
    let mut KEEP: bool = false;
    let mut LEFT: bool = false;
    let mut OPEN: bool = false;
    let mut RIGHT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZGFWSTS", ctx)?;

    //
    // Store the maximum number of endpoints that can be loaded into
    // WNDW3
    //
    MAXPTS = SIZED(WNDW3.as_slice(), ctx)?;
    SSIZED(MAXPTS, WNDW3.as_slice_mut(), ctx)?;

    //
    // Find the number of endpoints in each of the input windows.
    //
    SIZE1 = CARDD(WNDW1.as_slice(), ctx)?;
    SIZE2 = CARDD(WNDW2.as_slice(), ctx)?;

    //
    // Initialize the place holders for each of the input windows.
    //
    BEGP1 = 1;
    BEGP2 = 1;

    ENDP1 = 2;
    ENDP2 = 2;

    BEGP3 = -1;
    ENDP3 = 0;

    CMPRSS(b" ", 0, INCLSN, &mut LOCINC);

    OPEN = fstr::eq(&LOCINC, b"()");
    LEFT = fstr::eq(&LOCINC, b"[)");
    RIGHT = fstr::eq(&LOCINC, b"(]");
    CLOSED = fstr::eq(&LOCINC, b"[]");

    if !(((OPEN || LEFT) || RIGHT) || CLOSED) {
        SETMSG(b"The value of the inclusion flag must be one of the following: \'[]\', \'[)\', \'(]\', or \'()\'.  However the value supplied was \'#\'. ", ctx);
        ERRCH(b"#", INCLSN, ctx);
        SIGERR(b"SPICE(UNKNOWNINCLUSION)", ctx)?;
        CHKOUT(b"ZZGFWSTS", ctx)?;
        return Ok(());
    }

    //
    // We haven't had a chance to overflow yet.
    //
    OVFLOW = 0;

    while ((BEGP1 < SIZE1) && (BEGP2 < SIZE2)) {
        //
        // Using the current interval endpoints determine the overlap of
        // the two intervals.
        //
        if (WNDW1[ENDP1] < WNDW2[BEGP2]) {
            //
            // the end of the first interval precedes the beginning of the
            // second
            //
            BEGP1 = (BEGP1 + 2);
            ENDP1 = (ENDP1 + 2);
        } else if (WNDW2[ENDP2] < WNDW1[BEGP1]) {
            //
            // the end of the second interval precedes the beginning of the
            // first
            //
            BEGP2 = (BEGP2 + 2);
            ENDP2 = (ENDP2 + 2);
        } else {
            //
            // the intervals intersect.  Is the first contained in the
            // second?
            //
            if CLOSED {
                KEEP = ((WNDW1[BEGP1] >= WNDW2[BEGP2]) && (WNDW1[ENDP1] <= WNDW2[ENDP2]));
            } else if OPEN {
                KEEP = ((WNDW1[BEGP1] > WNDW2[BEGP2]) && (WNDW1[ENDP1] < WNDW2[ENDP2]));
            } else if LEFT {
                KEEP = ((WNDW1[BEGP1] >= WNDW2[BEGP2]) && (WNDW1[ENDP1] < WNDW2[ENDP2]));
            } else if RIGHT {
                KEEP = ((WNDW1[BEGP1] > WNDW2[BEGP2]) && (WNDW1[ENDP1] <= WNDW2[ENDP2]));
            }

            if KEEP {
                BEGP3 = (BEGP3 + 2);
                ENDP3 = (ENDP3 + 2);

                if (BEGP3 < MAXPTS) {
                    //
                    // Adequate room is left in WNDW3 to include this
                    // interval
                    //
                    WNDW3[BEGP3] = WNDW1[BEGP1];
                    WNDW3[ENDP3] = WNDW1[ENDP1];
                } else {
                    OVFLOW = (OVFLOW + 2);
                }
            }

            //
            // Determine which window pointers to increment
            //
            if (WNDW1[ENDP1] < WNDW2[ENDP2]) {
                //
                // The first interval lies before the end of the second
                //
                BEGP1 = (BEGP1 + 2);
                ENDP1 = (ENDP1 + 2);
            } else if (WNDW2[ENDP2] < WNDW1[ENDP1]) {
                //
                // The second interval lies before the end of the first
                //
                BEGP2 = (BEGP2 + 2);
                ENDP2 = (ENDP2 + 2);
            } else {
                //
                // The first and second intervals end at the same place
                //
                BEGP1 = (BEGP1 + 2);
                ENDP1 = (ENDP1 + 2);

                BEGP2 = (BEGP2 + 2);
                ENDP2 = (ENDP2 + 2);
            }
        }
    }

    if (OVFLOW > 0) {
        SETMSG(b"The output window does not have sufficient memory to contain the result of sifting the two given windows. The output window requires space for # more values than what has been provided. ", ctx);
        ERRINT(b"#", OVFLOW, ctx);
        SIGERR(b"SPICE(OUTOFROOM)", ctx)?;
    } else {
        SCARDD(ENDP3, WNDW3.as_slice_mut(), ctx)?;
    }

    CHKOUT(b"ZZGFWSTS", ctx)?;
    Ok(())
}
