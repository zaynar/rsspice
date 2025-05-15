//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    SEQNCE: StackArray<i32, 5>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut SEQNCE = StackArray::<i32, 5>::new(1..=5);

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(1), Val::I(2), Val::I(3), Val::I(1), Val::I(2)].into_iter();
            SEQNCE
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { SEQNCE }
    }
}

//$Procedure ZZTWOVXF ( Two states defining a frame transformation )
pub fn ZZTWOVXF(
    AXDEF: &[f64],
    INDEXA: i32,
    PLNDEF: &[f64],
    INDEXP: i32,
    XFORM: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let AXDEF = DummyArray::new(AXDEF, 1..=6);
    let PLNDEF = DummyArray::new(PLNDEF, 1..=6);
    let mut XFORM = DummyArrayMut2D::new(XFORM, 1..=6, 1..=6);
    let mut TMPSTA = StackArray::<f64, 6>::new(1..=6);
    let mut I1: i32 = 0;
    let mut I2: i32 = 0;
    let mut I3: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local Variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZTWOVXF", ctx)?;

    //
    // Check for obvious bad inputs.
    //
    if ((intrinsics::MAX0(&[INDEXP, INDEXA]) > 3) || (intrinsics::MIN0(&[INDEXP, INDEXA]) < 1)) {
        SETMSG(b"The definition indices must lie in the range from 1 to 3.  The value of INDEXA was #. The value of INDEXP was #. ", ctx);
        ERRINT(b"#", INDEXA, ctx);
        ERRINT(b"#", INDEXP, ctx);
        SIGERR(b"SPICE(BADINDEX)", ctx)?;
        CHKOUT(b"ZZTWOVXF", ctx)?;
        return Ok(());
    } else if (INDEXA == INDEXP) {
        SETMSG(b"The values of INDEXA and INDEXP were the same, namely #.  They are required to be different.", ctx);
        ERRINT(b"#", INDEXA, ctx);
        SIGERR(b"SPICE(UNDEFINEDFRAME)", ctx)?;
        CHKOUT(b"ZZTWOVXF", ctx)?;
        return Ok(());
    }

    //
    // Get indices for right-handed axes:
    //
    // First AXDEF ...
    //
    I1 = INDEXA;
    //
    // ... then the other two.
    //
    I2 = save.SEQNCE[(INDEXA + 1)];
    I3 = save.SEQNCE[(INDEXA + 2)];

    //
    // Column I1 of XFORM contains a unit vector parallel to AXDEF and
    // the derivative of the unit vector.
    //
    DVHAT(AXDEF.as_slice(), XFORM.subarray_mut([1, I1]));

    //
    // Obtain columns I2 and I3 of XFORM using cross products.
    // Which order to use depends on whether INDEXP = I2 (next axis in
    // right-handed order) or INDEXP = I3 (previous axis in right-handed
    // order).
    //
    // Select column indices...
    //
    if (INDEXP == I2) {
        //
        // We compute the third axis in the sequence, then the second.
        //
        DUCRSS(
            AXDEF.as_slice(),
            PLNDEF.as_slice(),
            XFORM.subarray_mut([1, I3]),
        );
        DUCRSS(
            XFORM.subarray([1, I3]),
            AXDEF.as_slice(),
            TMPSTA.as_slice_mut(),
        );
        MOVED(TMPSTA.as_slice(), 6, XFORM.subarray_mut([1, I2]));
    } else {
        DUCRSS(
            PLNDEF.as_slice(),
            AXDEF.as_slice(),
            XFORM.subarray_mut([1, I2]),
        );
        DUCRSS(
            AXDEF.as_slice(),
            XFORM.subarray([1, I2]),
            TMPSTA.as_slice_mut(),
        );
        MOVED(TMPSTA.as_slice(), 6, XFORM.subarray_mut([1, I3]));
    }

    //
    // ...and compute the output frame's non-principal unit basis
    // vectors and the derivatives of these vectors.
    //

    //
    // At this point, we've filled in the left half of XFORM.
    //
    // The upper right block is the 3x3 zero matrix.
    // The lower right block matches the upper left block.
    //
    CLEARD(3, XFORM.subarray_mut([1, 4]));
    CLEARD(3, XFORM.subarray_mut([1, 5]));
    CLEARD(3, XFORM.subarray_mut([1, 6]));

    for J in 1..=3 {
        for I in 1..=3 {
            XFORM[[(3 + I), (3 + J)]] = XFORM[[I, J]];
        }
    }

    //
    // Finally, check to see that we actually got something non-zero in
    // the first three components of at least one of the columns
    // XFORM(1,I2) and XFORM(1,I3) (we need only check one of them since
    // they are related by a cross product).
    //
    if VZERO(XFORM.subarray([1, I2])) {
        SETMSG(b"The direction vectors associated with states AXDEF and PLNDEF are linearly dependent.", ctx);
        SIGERR(b"SPICE(DEPENDENTVECTORS)", ctx)?;
        CHKOUT(b"ZZTWOVXF", ctx)?;
        return Ok(());
    }

    CHKOUT(b"ZZTWOVXF", ctx)?;
    Ok(())
}
