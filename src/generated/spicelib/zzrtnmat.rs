//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    Z: StackArray<f64, 3>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut Z = StackArray::<f64, 3>::new(1..=3);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(0.0), Val::D(0.0), Val::D(1.0)].into_iter();
            Z.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { Z }
    }
}

//$Procedure ZZRTNMAT ( RTN transformation matrix )
pub fn ZZRTNMAT(V: &[f64], M: &mut [f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let V = DummyArray::new(V, 1..=3);
    let mut M = DummyArrayMut2D::new(M, 1..=3, 1..=3);
    let mut EAST = StackArray::<f64, 3>::new(1..=3);
    let mut LON: f64 = 0.0;
    let mut NORTH = StackArray::<f64, 3>::new(1..=3);
    let mut RAD = StackArray::<f64, 3>::new(1..=3);
    let mut VLON = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //
    // Internally, we're going to use the more
    // descriptive names EAST for the "tangential"
    // direction and NORTH for the "normal" direction.
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Use discovery check-in. Just test the RETURN status.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if ((V[1] == 0.0) && (V[2] == 0.0)) {
        CLEARD(9, M.as_slice_mut());

        CHKIN(b"ZZRTNMAT", ctx)?;
        SETMSG(
            b"Input vector (# # #) lies on Z-axis; tangential and normal directions are undefined.",
            ctx,
        );
        ERRDP(b"#", V[1], ctx);
        ERRDP(b"#", V[2], ctx);
        ERRDP(b"#", V[3], ctx);
        SIGERR(b"SPICE(DEGENERATECASE)", ctx)?;
        CHKOUT(b"ZZRTNMAT", ctx)?;
        return Ok(());
    } else {
        //
        // The two-argument arctangent function gives us a
        // robust way of determining the longitude of V, even
        // when the magnitude of V is very small.
        //
        LON = f64::atan2(V[2], V[1]);

        //
        // Let VLON be a unit vector in the x-y plane whose
        // longitude is LON.
        //
        VLON[1] = f64::cos(LON);
        VLON[2] = f64::sin(LON);
        VLON[3] = 0.0;

        //
        // We can compute the East and North vectors
        // without much loss of precision, since VLON is
        // orthogonal to Z and EAST is orthogonal to V.
        //
        UCRSS(save.Z.as_slice(), VLON.as_slice(), EAST.as_slice_mut());
        UCRSS(V.as_slice(), EAST.as_slice(), NORTH.as_slice_mut());

        VHAT(V.as_slice(), RAD.as_slice_mut());

        //
        // The rows of M are the basis vectors of
        // the radial/East/North frame:
        //
        for I in 1..=3 {
            M[[1, I]] = RAD[I];
            M[[2, I]] = EAST[I];
            M[[3, I]] = NORTH[I];
        }
    }

    Ok(())
}
