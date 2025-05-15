//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const CKFRAM: &[u8] = b"IAU_EARTH";
const PCK0: &[u8] = b"test.tpc";

struct SaveVars {
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PASS1: bool = false;

        PASS1 = true;

        Self { PASS1 }
    }
}

//*****************************************************************
//
//     T_GENCSM ( Generate C-matrices from smooth data )
//
//*****************************************************************

//
// Generate C-matrices for CK type 06 software testing.
// Use smooth data: earth rotation from a test utility PCK.
//
//    Version 1.0.0 03-FEB-2014 (NJB)
//
pub fn T_GENCSM(
    SEGNO: i32,
    MNSGNO: i32,
    BEGREC: i32,
    N: i32,
    FRAME: &[u8],
    RATE: f64,
    SUBTYP: i32,
    EPOCHS: &mut [f64],
    QUATS: &mut [f64],
    AVVS: &mut [f64],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut EPOCHS = DummyArrayMut::new(EPOCHS, 1..);
    let mut QUATS = DummyArrayMut2D::new(QUATS, 0..=3, 1..);
    let mut AVVS = DummyArrayMut2D::new(AVVS, 1..=3, 1..);
    let mut CMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut ET: f64 = 0.0;
    let mut QN = StackArray::<f64, 4>::new(0..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut I: i32 = 0;
    let mut J: i32 = 0;

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
    // Saved variables
    //

    //
    // Initial values
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    // CALL CHKIN ( 'T_GENCSM' )

    I = spicelib::TOUCHI(MNSGNO);
    I = spicelib::TOUCHI(SEGNO);

    //
    // Load a test PCK on the first pass.
    //
    if save.PASS1 {
        testutil::TSTPCK(PCK0, true, false, ctx)?;
        testutil::CHCKXC(false, b" ", OK, ctx)?;

        save.PASS1 = false;
    }

    //
    // We'll construct a sequence of C-matrices and angular velocities
    // for the orientation of the earth relative to the IREF frame.
    // Those rotations and corresponding angular velocities will be
    // transformed to the requested output frame.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = N;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            J = ((BEGREC - 1) + I);

            EPOCHS[I] = (J as f64);

            //
            // Convert SCLK to ET for SXFORM lookup. This is necessary
            // in order to derived angular velocity consistent with
            // orientation. RATE has units of seconds/tick.
            //
            ET = (EPOCHS[I] * RATE);

            //
            // Initial orientation is that of the earth relative
            // to J2000.
            //
            spicelib::SXFORM(FRAME, CKFRAM, ET, XFORM.as_slice_mut(), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::XF2RAV(
                XFORM.as_slice(),
                CMAT.as_slice_mut(),
                AVVS.subarray_mut([1, I]),
            );
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            spicelib::M2Q(CMAT.as_slice(), QUATS.subarray_mut([0, I]), ctx)?;
            testutil::CHCKXC(false, b" ", OK, ctx)?;

            //
            // Adjust data to achieve better consistency, if necessary.
            //
            if (I > 1) {
                if ((SUBTYP == C06TP0) || (SUBTYP == C06TP2)) {
                    spicelib::VMINUG(QUATS.subarray([0, I]), 4, QN.as_slice_mut());

                    if (spicelib::VDISTG(QUATS.subarray([0, I]), QUATS.subarray([0, (I - 1)]), 4)
                        > spicelib::VDISTG(QN.as_slice(), QUATS.subarray([0, (I - 1)]), 4))
                    {
                        //
                        // Replace the original quaternion with its negative.
                        //
                        spicelib::MOVED(QN.as_slice(), 4, QUATS.subarray_mut([0, I]));
                    }
                }
            }

            I += m3__;
        }
    }

    // CALL CHKOUT ( 'T_GENCSM' )
    Ok(())
}
