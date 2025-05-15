//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

struct SaveVars {
    I: i32,
    AXIS: StackArray2D<f64, 60>,
    DTHETA: f64,
    REF: StackArray3D<f64, 180>,
    REFTIM: StackArray<f64, 20>,
    ROT: StackArray2D<f64, 9>,
    TEMP: StackArray2D<f64, 9>,
    TEMPAX: StackArray<f64, 3>,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut I: i32 = 0;
        let mut AXIS = StackArray2D::<f64, 60>::new(1..=3, 1..=20);
        let mut DTHETA: f64 = 0.0;
        let mut REF = StackArray3D::<f64, 180>::new(1..=3, 1..=3, 1..=20);
        let mut REFTIM = StackArray::<f64, 20>::new(1..=20);
        let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TEMP = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
        let mut TEMPAX = StackArray::<f64, 3>::new(1..=3);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            I,
            AXIS,
            DTHETA,
            REF,
            REFTIM,
            ROT,
            TEMP,
            TEMPAX,
            FIRST,
        }
    }
}

//$Procedure      TSTATD ( Test Attitude )
pub fn TSTATD(ET: f64, MATRIX: &mut [f64], ANGVEL: &mut [f64], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut MATRIX = DummyArrayMut2D::new(MATRIX, 1..=3, 1..=3);
    let mut ANGVEL = DummyArrayMut::new(ANGVEL, 1..=3);

    //
    // Spicelib Functions
    //

    //
    // Local Variables
    //

    if save.FIRST {
        save.FIRST = false;
        //
        // Setup.  Here we create the rotation matrices at the
        // boundaries of the intervals described in the header.
        //

        save.AXIS[[1, 1]] = 1.0;
        save.AXIS[[2, 1]] = 2.0;
        save.AXIS[[3, 1]] = 4.0;

        save.AXIS[[1, 2]] = 2.0;
        save.AXIS[[2, 2]] = 4.0;
        save.AXIS[[3, 2]] = 1.0;

        save.AXIS[[1, 3]] = 4.0;
        save.AXIS[[2, 3]] = 1.0;
        save.AXIS[[3, 3]] = 2.0;

        save.AXIS[[1, 4]] = 4.0;
        save.AXIS[[2, 4]] = 2.0;
        save.AXIS[[3, 4]] = 1.0;

        save.AXIS[[1, 5]] = 2.0;
        save.AXIS[[2, 5]] = 1.0;
        save.AXIS[[3, 5]] = 4.0;

        save.AXIS[[1, 6]] = 1.0;
        save.AXIS[[2, 6]] = 4.0;
        save.AXIS[[3, 6]] = 2.0;

        save.AXIS[[1, 7]] = 1.0;
        save.AXIS[[2, 7]] = 2.0;
        save.AXIS[[3, 7]] = 3.0;

        save.AXIS[[1, 8]] = 2.0;
        save.AXIS[[2, 8]] = 3.0;
        save.AXIS[[3, 8]] = 1.0;

        save.AXIS[[1, 9]] = 3.0;
        save.AXIS[[2, 9]] = 1.0;
        save.AXIS[[3, 9]] = 2.0;

        save.AXIS[[1, 10]] = 3.0;
        save.AXIS[[2, 10]] = 2.0;
        save.AXIS[[3, 10]] = 1.0;

        save.AXIS[[1, 11]] = 2.0;
        save.AXIS[[2, 11]] = 1.0;
        save.AXIS[[3, 11]] = 3.0;

        save.AXIS[[1, 12]] = 1.0;
        save.AXIS[[2, 12]] = 3.0;
        save.AXIS[[3, 12]] = 2.0;

        save.AXIS[[1, 13]] = 2.0;
        save.AXIS[[2, 13]] = 3.0;
        save.AXIS[[3, 13]] = 6.0;

        save.AXIS[[1, 14]] = 3.0;
        save.AXIS[[2, 14]] = 6.0;
        save.AXIS[[3, 14]] = 2.0;

        save.AXIS[[1, 15]] = 6.0;
        save.AXIS[[2, 15]] = 2.0;
        save.AXIS[[3, 15]] = 3.0;

        save.AXIS[[1, 16]] = 6.0;
        save.AXIS[[2, 16]] = 3.0;
        save.AXIS[[3, 16]] = 2.0;

        save.AXIS[[1, 17]] = 3.0;
        save.AXIS[[2, 17]] = 2.0;
        save.AXIS[[3, 17]] = 6.0;

        save.AXIS[[1, 18]] = 2.0;
        save.AXIS[[2, 18]] = 6.0;
        save.AXIS[[3, 18]] = 3.0;

        save.AXIS[[1, 19]] = 1.0;
        save.AXIS[[2, 19]] = 1.0;
        save.AXIS[[3, 19]] = 1.0;

        save.AXIS[[1, 20]] = 0.0;
        save.AXIS[[2, 20]] = 0.0;
        save.AXIS[[3, 20]] = 1.0;

        //
        // The 11'th reference matrix should be the identity matrix.
        //

        save.REF[[1, 1, 11]] = 1.0;
        save.REF[[2, 1, 11]] = 0.0;
        save.REF[[3, 1, 11]] = 0.0;

        save.REF[[1, 2, 11]] = 0.0;
        save.REF[[2, 2, 11]] = 1.0;
        save.REF[[3, 2, 11]] = 0.0;

        save.REF[[1, 3, 11]] = 0.0;
        save.REF[[2, 3, 11]] = 0.0;
        save.REF[[3, 3, 11]] = 1.0;

        save.REFTIM[11] = 0.0;

        {
            let m1__: i32 = 12;
            let m2__: i32 = 20;
            let m3__: i32 = 1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Recall that the rate of rotation is 1 radian every
                // 10 million seconds.  The axis of rotation changes
                // every 100 million seconds.  Hence a DTHETA of 10 radians.
                //
                save.DTHETA = 10.0;
                save.REFTIM[save.I] = (save.REFTIM[(save.I - 1)] + 100000000.0);

                spicelib::AXISAR(
                    save.AXIS.subarray([1, (save.I - 1)]),
                    save.DTHETA,
                    save.TEMP.as_slice_mut(),
                );
                spicelib::MXM(
                    save.TEMP.as_slice(),
                    &save.REF.subarray([1, 1, (save.I - 1)]).to_vec(),
                    save.REF.subarray_mut([1, 1, save.I]),
                );

                save.I += m3__;
            }
        }

        {
            let m1__: i32 = 10;
            let m2__: i32 = 1;
            let m3__: i32 = -1;
            save.I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.REFTIM[save.I] = (save.REFTIM[(save.I + 1)] - 100000000.0);
                save.DTHETA = -10.0;

                spicelib::AXISAR(
                    save.AXIS.subarray([1, save.I]),
                    save.DTHETA,
                    save.TEMP.as_slice_mut(),
                );
                spicelib::MXM(
                    save.TEMP.as_slice(),
                    &save.REF.subarray([1, 1, (save.I + 1)]).to_vec(),
                    save.REF.subarray_mut([1, 1, save.I]),
                );

                save.I += m3__;
            }
        }
    }

    //
    // Compute the offset from the appropriate reference time and
    // simply rotate about the appropriate axis.
    //
    save.I = intrinsics::MAX0(&[1, spicelib::LSTLED(ET, 20, save.REFTIM.as_slice())]);
    save.DTHETA = ((ET - save.REFTIM[save.I]) / 10000000.0);

    spicelib::AXISAR(
        save.AXIS.subarray([1, save.I]),
        save.DTHETA,
        save.TEMP.as_slice_mut(),
    );
    spicelib::MXM(
        save.TEMP.as_slice(),
        save.REF.subarray([1, 1, save.I]),
        save.ROT.as_slice_mut(),
    );
    spicelib::XPOSE(save.ROT.as_slice(), MATRIX.as_slice_mut());

    spicelib::VHAT(save.AXIS.subarray([1, save.I]), save.TEMPAX.as_slice_mut());
    spicelib::VSCL(0.0000001, save.TEMPAX.as_slice(), ANGVEL.as_slice_mut());
}
