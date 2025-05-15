//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const NDC: i32 = 2;
const NIC: i32 = 6;
const NC: i32 = (NDC + ((NIC + 1) / 2));
const IDLEN: i32 = (NC * 8);

//$Procedure      CKGP_O ( C-kernel, get pointing )
pub fn CKGP_O(
    INST: i32,
    SCLKDP: f64,
    TOL: f64,
    REF: &[u8],
    CMAT: &mut [f64],
    CLKOUT: &mut f64,
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CMAT = DummyArrayMut2D::new(CMAT, 1..=3, 1..=3);
    let mut SEGID = [b' '; IDLEN as usize];
    let mut AV = StackArray::<f64, 3>::new(1..=3);
    let mut DCD = StackArray::<f64, 2>::new(1..=NDC);
    let mut DESCR = StackArray::<f64, 5>::new(1..=NC);
    let mut ET: f64 = 0.0;
    let mut OMEGA = StackArray::<f64, 3>::new(1..=3);
    let mut ROT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut TMPMAT = StackArray2D::<f64, 9>::new(1..=3, 1..=3);
    let mut XFORM = StackArray2D::<f64, 36>::new(1..=6, 1..=6);
    let mut CENTER: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NIC);
    let mut REFREQ: i32 = 0;
    let mut REFSEG: i32 = 0;
    let mut SCLK: i32 = 0;
    let mut TYPE1: i32 = 0;
    let mut TYPE2: i32 = 0;
    let mut TYPEID: i32 = 0;
    let mut NEEDAV: bool = false;
    let mut PFND: bool = false;
    let mut SFND: bool = false;
    let mut GOTIT: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    NDC        is the number of double precision components in an
    //               unpacked C-kernel segment descriptor.
    //
    //    NIC        is the number of integer components in an unpacked
    //               C-kernel segment descriptor.
    //
    //    NC         is the number of components in a packed C-kernel
    //               descriptor.  All DAF summaries have this formulaic
    //               relationship between the number of its integer and
    //               double precision components and the number of packed
    //               components.
    //
    //    IDLEN      is the length of the C-kernel segment identifier.
    //               All DAF names have this formulaic relationship
    //               between the number of summary components and
    //               the length of the name (You will notice that
    //               a name and a summary have the same length in bytes.)
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"CKGP_O", ctx)?;
    }

    //
    // Don't need angular velocity data.
    // Assume the segment won't be found until it really is.
    //
    NEEDAV = false;
    *FOUND = false;

    //
    // If the tolerance is less than zero, we go no further.
    //
    if (TOL < 0.0) {
        spicelib::CHKOUT(b"CKGP_O", ctx)?;
        return Ok(());
    }

    //
    // Begin a search for this instrument and time, and get the first
    // applicable segment.
    //
    spicelib::CKBSS(INST, SCLKDP, TOL, NEEDAV, ctx)?;
    spicelib::CKSNS(
        &mut HANDLE,
        DESCR.as_slice_mut(),
        &mut SEGID,
        &mut SFND,
        ctx,
    )?;

    //
    // Keep trying candidate segments until a segment can produce a
    // pointing instance within the specified time tolerance of the
    // input time.
    //
    // Check FAILED to prevent an infinite loop if an error is detected
    // by a SPICELIB routine and the error handling is not set to abort.
    //

    while (SFND && !spicelib::FAILED(ctx)) {
        spicelib::CKPFS(
            HANDLE,
            DESCR.as_slice(),
            SCLKDP,
            TOL,
            NEEDAV,
            CMAT.as_slice_mut(),
            AV.as_slice_mut(),
            CLKOUT,
            &mut PFND,
            ctx,
        )?;

        if PFND {
            //
            // Found one. If the C-matrix doesn't already rotate from the
            // requested frame, convert it to one that does.
            //
            spicelib::DAFUS(
                DESCR.as_slice(),
                NDC,
                NIC,
                DCD.as_slice_mut(),
                ICD.as_slice_mut(),
            );
            REFSEG = ICD[2];
            //
            // Look up the id code for the requested reference frame.
            //
            spicelib::NAMFRM(REF, &mut REFREQ, ctx)?;

            if (REFREQ != REFSEG) {
                //
                // We may need to convert the output ticks CLKOUT to ET
                // so that we can get the needed state transformation
                // matrix.  This is the case if either of the frames
                // is non-inertial.
                //
                spicelib::FRINFO(
                    REFREQ,
                    &mut CENTER,
                    &mut TYPE1,
                    &mut TYPEID,
                    &mut GOTIT,
                    ctx,
                )?;
                spicelib::FRINFO(
                    REFSEG,
                    &mut CENTER,
                    &mut TYPE2,
                    &mut TYPEID,
                    &mut GOTIT,
                    ctx,
                )?;

                if ((TYPE1 == INERTL) && (TYPE2 == INERTL)) {
                    //
                    // Any old value of ET will do in this case.  We'll
                    // use zero.
                    //
                    ET = 0.0;
                } else {
                    //
                    // Look up the spacecraft clock id to use to conver
                    // the output CLKOUT to ET.
                    //
                    spicelib::CKMETA(INST, b"SCLK", &mut SCLK, ctx)?;
                    spicelib::SCT2E(SCLK, *CLKOUT, &mut ET, ctx)?;
                }
                //
                // Get the transformation from the requested frame to
                // the segment frame at ET.
                //
                spicelib::FRMCHG(REFREQ, REFSEG, ET, XFORM.as_slice_mut(), ctx)?;
                //
                // If FRMCHG detects that the reference frame is invalid
                // then return from this routine with FOUND equal to false.
                //
                if spicelib::FAILED(ctx) {
                    spicelib::CHKOUT(b"CKGP_O", ctx)?;
                    return Ok(());
                }
                //
                // Transform the attitude information.
                //
                // Get the rotation and angular velocity associated with the
                // transformation from request frame to segment frame.
                // Then convert CMAT so that it maps from request frame
                // to C-matrix frame.
                //
                spicelib::XF2RAV(XFORM.as_slice(), ROT.as_slice_mut(), OMEGA.as_slice_mut());
                spicelib::MXM(CMAT.as_slice(), ROT.as_slice(), TMPMAT.as_slice_mut());
                spicelib::MOVED(TMPMAT.as_slice(), 9, CMAT.as_slice_mut());
            }

            *FOUND = true;

            spicelib::CHKOUT(b"CKGP_O", ctx)?;
            return Ok(());
        }

        spicelib::CKSNS(
            &mut HANDLE,
            DESCR.as_slice_mut(),
            &mut SEGID,
            &mut SFND,
            ctx,
        )?;
    }

    spicelib::CHKOUT(b"CKGP_O", ctx)?;
    Ok(())
}
