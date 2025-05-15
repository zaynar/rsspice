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

//*****************************************************************
//
//     T_XSUBTP
//
//*****************************************************************

//
// Transform record subtype. Input is always a quaternion
// and avv; output is any other type.
//
// For subtype 2, the caller must supply the derivative
// with respect to ticks of the angular velocity.
//
pub fn T_XSUBTP(
    QUAT: &[f64],
    AVV: &[f64],
    DAVV: &[f64],
    SUBTYP: i32,
    SIZE: i32,
    RECORD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let QUAT = DummyArray::new(QUAT, 0..=3);
    let AVV = DummyArray::new(AVV, 1..=3);
    let DAVV = DummyArray::new(DAVV, 1..=3);
    let mut RECORD = DummyArrayMut::new(RECORD, 1..);
    let mut AVQ = StackArray::<f64, 4>::new(0..=3);
    let mut DQ = StackArray::<f64, 4>::new(0..=3);

    //
    // SPICELIB functions
    //
    //  DOUBLE PRECISION      VDOTG

    //
    // Local variables
    //

    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    // CALL CHKIN ( 'T_XSUBTP' )

    spicelib::CLEARD(SIZE, RECORD.as_slice_mut());

    if (SUBTYP == C06TP0) {
        //
        // Create a quaternion derivative from a quaternion and avv. Pack
        // the quaternion and its derivative into the RECORD array.
        //
        spicelib::MOVED(QUAT.as_slice(), 4, RECORD.as_slice_mut());
        //
        // From the header of QDQ2AV we have the equation:
        //
        //                      *
        //    AV  =  Im ( -2 * Q  * DQ )                        (1)
        //
        // Here AV can be viewed as a quaternion with real part
        // equal to 0. Then
        //
        //    DQ = -0.5 * Q * AV
        //

        AVQ[0] = 0.0;

        spicelib::VSCL(-0.5, AVV.as_slice(), AVQ.subarray_mut(1));
        spicelib::QXQ(QUAT.as_slice(), AVQ.as_slice(), DQ.as_slice_mut());
        spicelib::MOVED(DQ.as_slice(), 4, RECORD.subarray_mut(5));
    } else if (SUBTYP == C06TP1) {
        //
        // No transformation required.
        //
        spicelib::MOVED(QUAT.as_slice(), 4, RECORD.as_slice_mut());
    } else if (SUBTYP == C06TP2) {
        //
        // Create a quaternion derivative from a quaternion and avv. Pack
        // the quaternion and its derivative into the RECORD array.
        //
        spicelib::MOVED(QUAT.as_slice(), 4, RECORD.as_slice_mut());
        //
        // From the header of QDQ2AV we have the equation:
        //
        //                      *
        //    AV  =  Im ( -2 * Q  * DQ )                        (1)
        //
        // Here AV can be viewed as a quaternion with real part
        // equal to 0. Then
        //
        //    DQ = -0.5 * Q * AV
        //
        //
        AVQ[0] = 0.0;

        spicelib::VSCL(-0.5, AVV.as_slice(), AVQ.subarray_mut(1));
        spicelib::QXQ(QUAT.as_slice(), AVQ.as_slice(), DQ.as_slice_mut());
        spicelib::MOVED(DQ.as_slice(), 4, RECORD.subarray_mut(5));
        //
        // Transfer the avv and its derivative.
        //
        spicelib::VEQU(AVV.as_slice(), RECORD.subarray_mut(9));
        spicelib::VEQU(DAVV.as_slice(), RECORD.subarray_mut(12));
    } else if (SUBTYP == C06TP3) {
        //
        // No transformation required.
        //
        spicelib::MOVED(QUAT.as_slice(), 4, RECORD.as_slice_mut());
        spicelib::VEQU(AVV.as_slice(), RECORD.subarray_mut(5));
    } else {
        spicelib::SETMSG(b"SUBTYP was #.", ctx);
        spicelib::ERRINT(b"#", SUBTYP, ctx);
        spicelib::SIGERR(b"SPICE(BUG)", ctx)?;
        // CALL CHKOUT ( 'T_XSUBTP' )
        return Ok(());
    }

    // CALL CHKOUT ( 'T_XSUBTP' )
    Ok(())
}
