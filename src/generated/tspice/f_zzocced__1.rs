//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    ANGLES: StackArray<f64, 3>,
    OFFSET: StackArray<f64, 3>,
    P231: StackArray2D<f64, 9>,
    P312: StackArray2D<f64, 9>,
    R: StackArray2D<f64, 9>,
    AXES: StackArray<i32, 3>,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ANGLES = StackArray::<f64, 3>::new(1..=3);
        let mut OFFSET = StackArray::<f64, 3>::new(1..=3);
        let mut P231 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut P312 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut R = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut AXES = StackArray::<i32, 3>::new(1..=3);
        let mut PASS1: bool = false;

        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(30.0), Val::D(-50.0), Val::D(85.0)].into_iter();
            ANGLES
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::I(3), Val::I(1), Val::I(3)].into_iter();
            AXES.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
            ]
            .into_iter();
            P231.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(1.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(0.0),
                Val::D(1.0),
                Val::D(0.0),
            ]
            .into_iter();
            P312.iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        PASS1 = true;
        {
            use f2rust_std::data::Val;

            let mut clist = [Val::D(1000.0), Val::D(-70000.0), Val::D(30000000000.0)].into_iter();
            OFFSET
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            ANGLES,
            OFFSET,
            P231,
            P312,
            R,
            AXES,
            PASS1,
        }
    }
}

//
// Utility for "deranging" inputs to more fully exercise
// the frame transformation code in ZZOCCED.
//
pub fn XINPUT(
    VIEWPT: &[f64],
    CENTR1: &[f64],
    SEMAX1: &[f64],
    CENTR2: &[f64],
    SEMAX2: &[f64],
    OVIEW: &mut [f64],
    OCTR1: &mut [f64],
    OAX1: &mut [f64],
    OCTR2: &mut [f64],
    OAX2: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let VIEWPT = DummyArray::new(VIEWPT, 1..=3);
    let CENTR1 = DummyArray::new(CENTR1, 1..=3);
    let SEMAX1 = DummyArray2D::new(SEMAX1, 1..=3, 1..=3);
    let CENTR2 = DummyArray::new(CENTR2, 1..=3);
    let SEMAX2 = DummyArray2D::new(SEMAX2, 1..=3, 1..=3);
    let mut OVIEW = DummyArrayMut::new(OVIEW, 1..=3);
    let mut OCTR1 = DummyArrayMut::new(OCTR1, 1..=3);
    let mut OAX1 = DummyArrayMut2D::new(OAX1, 1..=3, 1..=3);
    let mut OCTR2 = DummyArrayMut::new(OCTR2, 1..=3);
    let mut OAX2 = DummyArrayMut2D::new(OAX2, 1..=3, 1..=3);
    let mut M1 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut M2 = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut VTEMP = StackArray::<f64, 3>::new(1..=3);

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Initial values
    //
    //
    // Saved variables
    //

    //
    // Initial values
    //

    if save.PASS1 {
        //
        // Create the rotation matrix that we'll use to transform
        // all input vectors.
        //
        spicelib::EUL2M(
            (save.ANGLES[3] * spicelib::RPD(ctx)),
            (save.ANGLES[2] * spicelib::RPD(ctx)),
            (save.ANGLES[1] * spicelib::RPD(ctx)),
            save.AXES[3],
            save.AXES[2],
            save.AXES[1],
            save.R.as_slice_mut(),
            ctx,
        )?;

        save.PASS1 = false;
    }

    //
    // Permute the columns of the first semi-axis matrix using
    // the matrix P312.  Apply P231 to the second matrix.
    //
    spicelib::MXM(SEMAX1.as_slice(), save.P312.as_slice(), M1.as_slice_mut());
    spicelib::MXM(SEMAX2.as_slice(), save.P231.as_slice(), M2.as_slice_mut());

    //
    // Rotate all input vectors using R.
    //
    spicelib::MXV(save.R.as_slice(), VIEWPT.as_slice(), OVIEW.as_slice_mut());
    spicelib::MXV(save.R.as_slice(), CENTR1.as_slice(), OCTR1.as_slice_mut());
    spicelib::MXV(save.R.as_slice(), CENTR2.as_slice(), OCTR2.as_slice_mut());

    for I in 1..=3 {
        spicelib::MXV(
            save.R.as_slice(),
            M1.subarray([1, I]),
            OAX1.subarray_mut([1, I]),
        );
        spicelib::MXV(
            save.R.as_slice(),
            M2.subarray([1, I]),
            OAX2.subarray_mut([1, I]),
        );
    }

    //
    // Translate all position vectors using OFFSET.
    //
    spicelib::VADD(
        save.OFFSET.as_slice(),
        OVIEW.as_slice(),
        VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(VTEMP.as_slice(), OVIEW.as_slice_mut());

    spicelib::VADD(
        save.OFFSET.as_slice(),
        OCTR1.as_slice(),
        VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(VTEMP.as_slice(), OCTR1.as_slice_mut());

    spicelib::VADD(
        save.OFFSET.as_slice(),
        OCTR2.as_slice(),
        VTEMP.as_slice_mut(),
    );
    spicelib::VEQU(VTEMP.as_slice(), OCTR2.as_slice_mut());

    Ok(())
}
