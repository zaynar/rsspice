//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LT: i32 = -1;
pub const EQ: i32 = 0;
pub const GT: i32 = 1;

struct SaveVars {
    APEX: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut APEX = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            APEX.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { APEX }
    }
}

//$Procedure ZZPDCMPL (Planetodetic coordinates, compare latitudes )
pub fn ZZPDCMPL(
    RE: f64,
    F: f64,
    P: &[f64],
    LAT: f64,
    REL: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let P = DummyArray::new(P, 1..=3);
    let mut LON: f64 = 0.0;
    let mut OFFPCL: f64 = 0.0;
    let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
    let mut R: f64 = 0.0;
    let mut RP: f64 = 0.0;
    let mut XINCPT: f64 = 0.0;
    let mut YINCPT: f64 = 0.0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZPDCMPL", ctx)?;

    //
    // Treat points on the Z axis as a special case. The
    // computations performed in the general case may introduce
    // round-off errors that will lead to false results for
    // this case.
    //
    if ((P[1] == 0.0) && (P[2] == 0.0)) {
        if (P[3] > 0.0) {
            if (LAT == HALFPI(ctx)) {
                *REL = EQ;
            } else {
                *REL = GT;
            }
        } else if (P[3] == 0.0) {
            //
            // We consider the latitude of P to be zero.
            //
            if (LAT > 0.0) {
                *REL = LT;
            } else if (LAT == 0.0) {
                *REL = EQ;
            } else {
                *REL = GT;
            }
        } else {
            //
            // P(3) < 0.
            //
            if (LAT == -HALFPI(ctx)) {
                *REL = EQ;
            } else {
                *REL = LT;
            }
        }

        CHKOUT(b"ZZPDCMPL", ctx)?;
        return Ok(());
    }

    //
    // Latitude zero is a special case. The planetodetic latitude of the
    // input point has the same sign as the Z component of the point.
    //
    RP = (RE * (1.0 - F));
    //
    // Get the y-intercept of the latitude cone for LAT. Note that a
    // result is defined for LAT = +/- pi/2.
    //
    ZZELNAXX(RE, RP, LAT, &mut XINCPT, &mut YINCPT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"ZZPDCMPL", ctx)?;
        return Ok(());
    }

    //
    // Ideally YINCPT is zero if and only if LAT is zero.
    // We'll group these conditions together.
    //
    if ((LAT == 0.0) || (YINCPT == 0.0)) {
        if (P[3] > 0.0) {
            *REL = GT;
        } else if (P[3] == 0.0) {
            *REL = EQ;
        } else {
            *REL = LT;
        }

        CHKOUT(b"ZZPDCMPL", ctx)?;
        return Ok(());
    }

    //
    // This is the normal case.
    //
    // Find the offset of the point from the latitude cone's apex.
    // Create a unit-length copy of the offset vector.
    //
    save.APEX[3] = YINCPT;

    VSUB(P.as_slice(), save.APEX.as_slice(), OFFSET.as_slice_mut());

    // We'll use the planetocentric [sic] latitude of the offset
    // vector for comparison.
    //
    RECLAT(OFFSET.as_slice(), &mut R, &mut LON, &mut OFFPCL);

    if (LAT > 0.0) {
        if (YINCPT > 0 as f64) {
            //
            // This is the prolate case.
            //
            if (OFFPCL > LAT) {
                *REL = GT;
            } else if (OFFPCL == LAT) {
                *REL = EQ;
            } else {
                *REL = LT;
            }
        } else {
            //
            // YINCPT = 0 was handled previously, so YINCPT < 0.
            //
            // This is the oblate case.
            //
            // In addition to the comparison of angles, we need to know
            // the input point is above the X-Y plane in order for the
            // GT or EQ relations to hold.
            //
            if (P[3] > 0.0) {
                if (OFFPCL > LAT) {
                    *REL = GT;
                } else if (OFFPCL == LAT) {
                    *REL = EQ;
                } else {
                    *REL = LT;
                }
            } else {
                //
                // The input latitude is positive, while the point
                // is on or below the X-Y plane.
                //
                *REL = LT;
            }
        }
    } else {
        //
        // LAT < 0, since the case LAT = 0 has already been handled.
        //
        if (YINCPT < 0.0) {
            //
            // This is the prolate case.
            //
            if (OFFPCL > LAT) {
                *REL = GT;
            } else if (OFFPCL == LAT) {
                *REL = EQ;
            } else {
                *REL = LT;
            }
        } else {
            //
            // YINCPT > 0, since the case YINCPT = 0 was handled
            // previously.
            //
            // This is the oblate case.
            //
            if (P[3] < 0.0) {
                if (OFFPCL > LAT) {
                    *REL = GT;
                } else if (OFFPCL == LAT) {
                    *REL = EQ;
                } else {
                    *REL = LT;
                }
            } else {
                //
                // The input latitude is negative, while the point
                // is on or above the X-Y plane.
                //
                *REL = GT;
            }
        }
    }

    CHKOUT(b"ZZPDCMPL", ctx)?;
    Ok(())
}
