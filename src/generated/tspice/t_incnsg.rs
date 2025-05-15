//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const UBPL: i32 = 4;

struct SaveVars {
    ORIGIN: StackArray<f64, 3>,
    Y: StackArray<f64, 3>,
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ORIGIN = StackArray::<f64, 3>::new(1..=3);
        let mut Y = StackArray::<f64, 3>::new(1..=3);
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), 3 as usize))
                .chain([]);

            ORIGIN
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(1.0), Val::D(0.0)].into_iter();
            Y.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { ORIGIN, Y, Z }
    }
}

//$Procedure   T_INCNSG ( Test intersection of cone and line segment )
pub fn T_INCNSG(
    APEX: &[f64],
    AXIS: &[f64],
    ANGLE: f64,
    ENDPT1: &[f64],
    ENDPT2: &[f64],
    NXPTS: &mut i32,
    XPT1: &mut [f64],
    XPT2: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let APEX = DummyArray::new(APEX, 1..=3);
    let AXIS = DummyArray::new(AXIS, 1..=3);
    let ENDPT1 = DummyArray::new(ENDPT1, 1..=3);
    let ENDPT2 = DummyArray::new(ENDPT2, 1..=3);
    let mut XPT1 = DummyArrayMut::new(XPT1, 1..=3);
    let mut XPT2 = DummyArrayMut::new(XPT2, 1..=3);
    let mut AXMAG: f64 = 0.0;
    let mut COLAT: f64 = 0.0;
    let mut COSANG: f64 = 0.0;
    let mut DIR = StackArray::<f64, 3>::new(1..=3);
    let mut DMAG: f64 = 0.0;
    let mut DP1: f64 = 0.0;
    let mut DP2: f64 = 0.0;
    let mut LOCANG: f64 = 0.0;
    let mut MAXLAT: f64 = 0.0;
    let mut MAXP = StackArray::<f64, 3>::new(1..=3);
    let mut MINLAT: f64 = 0.0;
    let mut MINP = StackArray::<f64, 3>::new(1..=3);
    let mut NRMPLN = StackArray::<f64, 4>::new(1..=UBPL);
    let mut OFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut OFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut PLNX = StackArray::<f64, 3>::new(1..=3);
    let mut UAXIS = StackArray::<f64, 3>::new(1..=3);
    let mut UDIR = StackArray::<f64, 3>::new(1..=3);
    let mut UOFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut UOFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);
    let mut VTEMP2 = StackArray::<f64, 3>::new(1..=3);
    let mut X = StackArray::<f64, 3>::new(1..=3);
    let mut XOFF1 = StackArray::<f64, 3>::new(1..=3);
    let mut XOFF2 = StackArray::<f64, 3>::new(1..=3);
    let mut XFORM = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut NPLNX: i32 = 0;
    let mut IN1: bool = false;
    let mut IN2: bool = false;
    let mut ISBRCK: bool = false;
    let mut NEG1: bool = false;
    let mut NEG2: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // Local variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Use quasi-discovery check-in. We'll check in before
    // code sections that can generate SPICE errors, and check
    // out afterwards. When those code sections are skipped,
    // we avoid traceback participation.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    //
    // No intersection was found so far.
    //
    *NXPTS = 0;

    //
    // The cone's axis vector must be non-zero.
    //
    spicelib::UNORM(AXIS.as_slice(), UAXIS.as_slice_mut(), &mut AXMAG);

    if (AXMAG == 0.0) {
        spicelib::CHKIN(b"T_INCNSG", ctx)?;
        spicelib::SETMSG(b"The cone\'s axis vector must be non-zero but sadly, it failed to meet this criterion.", ctx);
        spicelib::SIGERR(b"SPICE(ZEROVECTOR)", ctx)?;
        spicelib::CHKOUT(b"T_INCNSG", ctx)?;
        return Ok(());
    }

    //
    // The cone's angular radius must be non-negative.
    //
    if (ANGLE < 0.0) {
        spicelib::CHKIN(b"T_INCNSG", ctx)?;
        spicelib::SETMSG(
            b"The cone\'s angular radius must be  non-negative but was # (radians).",
            ctx,
        );
        spicelib::ERRDP(b"#", ANGLE, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDANGLE)", ctx)?;
        spicelib::CHKOUT(b"T_INCNSG", ctx)?;
        return Ok(());
    }

    //
    // The endpoints of the segment must be distinct. Check this after
    // computing a unit direction vector for the line segment.
    //
    spicelib::VSUB(ENDPT2.as_slice(), ENDPT1.as_slice(), DIR.as_slice_mut());

    spicelib::UNORM(DIR.as_slice(), UDIR.as_slice_mut(), &mut DMAG);

    if (DMAG == 0.0) {
        spicelib::CHKIN(b"T_INCNSG", ctx)?;
        spicelib::SETMSG(
            b"The distance between the segment\'s endpoints was zero. First endpoint: (# # #).",
            ctx,
        );
        spicelib::ERRDP(b"#", ENDPT1[1], ctx);
        spicelib::ERRDP(b"#", ENDPT1[2], ctx);
        spicelib::ERRDP(b"#", ENDPT1[3], ctx);
        spicelib::SIGERR(b"SPICE(ENDPOINTSMATCH)", ctx)?;
        spicelib::CHKOUT(b"T_INCNSG", ctx)?;
        return Ok(());
    }

    //
    // Store the cosine of the cone's angular radius. We'll treat all
    // cases with COSANG equal to 0 as though the cone is actually a
    // plane normal to the axis and containing the apex.
    //
    COSANG = f64::cos(ANGLE);
    LOCANG = ANGLE;

    //
    // We'll work with a local axis that has angular separation of
    // no more than pi/2 from the nappe.
    //
    if (COSANG < 0.0) {
        LOCANG = (spicelib::PI(ctx) - ANGLE);
        COSANG = -COSANG;

        UAXIS[1] = -UAXIS[1];
        UAXIS[2] = -UAXIS[2];
        UAXIS[3] = -UAXIS[3];
    }

    //
    // Compute the offsets of the endpoints of the segment from
    // the cone's apex.
    //
    spicelib::VSUB(ENDPT1.as_slice(), APEX.as_slice(), OFF1.as_slice_mut());
    spicelib::VSUB(ENDPT2.as_slice(), APEX.as_slice(), OFF2.as_slice_mut());

    //
    // Deal with some of the simple cases first.
    //
    spicelib::VHAT(OFF1.as_slice(), UOFF1.as_slice_mut());
    spicelib::VHAT(OFF2.as_slice(), UOFF2.as_slice_mut());

    DP1 = spicelib::VDOT(UOFF1.as_slice(), UAXIS.as_slice());
    DP2 = spicelib::VDOT(UOFF2.as_slice(), UAXIS.as_slice());

    //
    // The given axis is inside the nappe defined by the angular radius.
    //
    // There's no intersection if both endpoints are in the interior of
    // the nappe of the cone (since the nappe is convex).
    //
    IN1 = (DP1 >= COSANG);
    IN2 = (DP2 >= COSANG);
    //
    // If the line segment lies on the far side of the plane that
    // contains the apex and is orthogonal to the axis, there's no
    // intersection.
    //
    NEG1 = (DP1 < 0.0);
    NEG2 = (DP2 < 0.0);

    if ((IN1 && IN2) || (NEG1 && NEG2)) {
        //
        // The segment is in the interior of the cone or
        // on the far side of the plane.
        //
        *NXPTS = 0;

        return Ok(());
    }

    //
    // Here's where we handle the half-space case.
    //
    if (COSANG == 0.0) {
        //
        // See whether the ray emanating from the first endpoint and
        // having direction UDIR hits the plane normal to the axis and
        // containing the apex. We'll call this plane NRMPLN.
        //
        // NVP2PL can signal an error only if the input axis is the
        // zero vector. We've ensured that it isn't.
        //
        spicelib::NVP2PL(
            UAXIS.as_slice(),
            APEX.as_slice(),
            NRMPLN.as_slice_mut(),
            ctx,
        )?;
        spicelib::INRYPL(
            ENDPT1.as_slice(),
            UDIR.as_slice(),
            NRMPLN.as_slice(),
            &mut NPLNX,
            PLNX.as_slice_mut(),
            ctx,
        )?;

        //
        // If the ray doesn't hit the plane, we're done. Otherwise,
        // check the intercept.
        //
        if (NPLNX == 1) {
            //
            // The ray does hit the plane. If the intersection is on the
            // line segment, we have a solution.
            //
            if (spicelib::VDIST(PLNX.as_slice(), ENDPT1.as_slice()) <= DMAG) {
                //
                // The intercept is not further along the ray than the
                // second endpoint. It's a valid solution.
                //
                *NXPTS = 1;
                spicelib::VEQU(PLNX.as_slice(), XPT1.as_slice_mut());
            }
        }
        //
        // This is the end of the half-space case.
        //
        return Ok(());
    }

    if (*NXPTS < 2) {
        //
        // We must determine the expected number of roots, and if
        // we didn't come up with them, we must find the roots
        // by an alternate method.
        //
        // We'll examine the containment of the endpoints within the
        // cone.
        //
        // The case where both endpoints are inside the cone was handled
        // earlier.
        //
        // If one endpoint is inside the cone and one is outside,
        // we expect to have one root.
        //
        if ((IN1 && !IN2) || (IN2 && !IN1)) {
            //
            // There's supposed to be one root. If we found none, find one
            // now.
            //
            if (*NXPTS == 0) {
                //
                // ZZCXBRUT signals an error if the axis is the zero
                // vector, but not otherwise. We've already ruled out this
                // situation. Therefore, we don't check in before the
                // following call.
                //
                spicelib::ZZCXBRUT(
                    APEX.as_slice(),
                    UAXIS.as_slice(),
                    LOCANG,
                    ENDPT1.as_slice(),
                    ENDPT2.as_slice(),
                    XPT1.as_slice_mut(),
                    &mut ISBRCK,
                    ctx,
                )?;

                if ISBRCK {
                    //
                    // As long as the root was bracketed, XPT1 is a
                    // solution.
                    //
                    *NXPTS = 1;
                }
            }
        } else {
            spicelib::CHKIN(b"T_INCNSG", ctx)?;
            //
            // Both endpoints are outside the cone. We could have zero to
            // two roots. If the minimum angular separation of the segment
            // from the axis is less than ANGLE, we expect to find two
            // roots; if it's equal to ANGLE, we expect to find one, and
            // if it's greater than ANGLE, none.
            //
            // We'll transform OFF1 and OFF2 into a reference frame in
            // which angular separation from the axis is equivalent to
            // colatitude. Then we'll find the maximum latitude attained
            // on the segment.
            //
            // We'll count the roots we find, so we'll start at zero.
            //
            *NXPTS = 0;

            spicelib::FRAME(
                UAXIS.as_slice_mut(),
                X.as_slice_mut(),
                save.Y.as_slice_mut(),
            );

            for I in 1..=3 {
                XFORM[[1, I]] = X[I];
                XFORM[[2, I]] = save.Y[I];
                XFORM[[3, I]] = UAXIS[I];
            }

            spicelib::MXV(XFORM.as_slice(), OFF1.as_slice(), XOFF1.as_slice_mut());
            spicelib::MXV(XFORM.as_slice(), OFF2.as_slice(), XOFF2.as_slice_mut());

            spicelib::ZZSGLATX(
                XOFF1.as_slice(),
                XOFF2.as_slice(),
                &mut MINLAT,
                MINP.as_slice_mut(),
                &mut MAXLAT,
                MAXP.as_slice_mut(),
                ctx,
            )?;

            if spicelib::FAILED(ctx) {
                spicelib::CHKOUT(b"T_INCNSG", ctx)?;
                return Ok(());
            }

            //
            // COLAT is the colatitude of the point of maximum latitude.
            //
            COLAT = (spicelib::HALFPI(ctx) - MAXLAT);

            if (COLAT < LOCANG) {
                //
                // MAXP is inside the cone. There should be an intersection
                // on the segment between XOFF1 and MAXP and another
                // between MAXP and XOFF2.
                //
                spicelib::ZZCXBRUT(
                    save.ORIGIN.as_slice(),
                    save.Z.as_slice(),
                    LOCANG,
                    XOFF1.as_slice(),
                    MAXP.as_slice(),
                    VTEMP.as_slice_mut(),
                    &mut ISBRCK,
                    ctx,
                )?;

                if ISBRCK {
                    //
                    // Convert VTEMP to the original frame, then translate
                    // it so that it's represented as an offset from the
                    // origin.
                    //
                    spicelib::MTXV(XFORM.as_slice(), VTEMP.as_slice(), VTEMP2.as_slice_mut());
                    spicelib::VADD(VTEMP2.as_slice(), APEX.as_slice(), XPT1.as_slice_mut());

                    *NXPTS = 1;
                }

                spicelib::ZZCXBRUT(
                    save.ORIGIN.as_slice(),
                    save.Z.as_slice(),
                    LOCANG,
                    MAXP.as_slice(),
                    XOFF2.as_slice(),
                    VTEMP.as_slice_mut(),
                    &mut ISBRCK,
                    ctx,
                )?;

                if ISBRCK {
                    //
                    // Convert VTEMP to the original frame, then translate
                    // it so that it's represented as an offset from the
                    // origin.
                    //
                    spicelib::MTXV(XFORM.as_slice(), VTEMP.as_slice(), VTEMP2.as_slice_mut());
                    spicelib::VADD(VTEMP2.as_slice(), APEX.as_slice(), XPT2.as_slice_mut());

                    if (*NXPTS == 1) {
                        //
                        // Both roots are valid.
                        //
                        *NXPTS = 2;
                    } else {
                        //
                        // The second root is the only valid root. Move it
                        // into XPT1.
                        //
                        spicelib::VEQU(XPT2.as_slice(), XPT1.as_slice_mut());

                        *NXPTS = 1;
                    }
                }
            } else if (COLAT == LOCANG) {
                //
                // The root corresponds to a point of tangency of
                // the segment and cone. This occurs at the point
                // having maximum latitude: MAXP.
                //
                spicelib::VEQU(MAXP.as_slice(), XPT1.as_slice_mut());

                *NXPTS = 1;

                //
                // Note that if COLAT > LOCANG, there are no roots.
                //
            }

            spicelib::CHKOUT(b"T_INCNSG", ctx)?;
        }
        //
        // This is the end of the "brute force" case with both endpoints
        // outside the cone.
        //
    }

    //
    // NXPTS  has been set.
    //
    // If NXPTS is 1, then XPT1 is set.
    //
    // If NXPTS is 2, then both XPT1 and XPT2 are set.
    //

    Ok(())
}
