//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const MAXREC: i32 = 128;
const FRMSIZ: i32 = 16;
const ND: i32 = 2;
const NI: i32 = 6;
const NSTATE: i32 = 6;

/// S/P Kernel, subset, type 14
///
/// Extract a subset of the data in a type 14 SPK segment into a new
/// type 14 segment.
///
/// # Required Reading
///
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SRCHAN     I   Handle of the SPK file with the source segment.
///  SRCDSC     I   Descriptor for the source segment.
///  DSTHAN     I   Handle of the SPK file for the destination segment.
///  DSTDSC     I   Descriptor for the destination segment.
///  DSTSID     I   Segment identifier for the new segment.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SRCHAN   is the handle of the SPK file containing the source
///           segment.
///
///  SRCDSC   is the SPK descriptor for the source segment.
///
///  DSTHAN   is the handle of the SPK file containing the new segment.
///
///  DSTDSC   is the SPK descriptor for the destination segment. It
///           contains the desired start and stop times for the
///           requested subset.
///
///  DSTSID   is the segment identifier for the destination segment.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the length of the SPK record that is to be moved is larger
///      than MAXREC, the error SPICE(SPKRECTOOLARGE) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See arguments SRCHAN, DSTHAN.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine copies a subset of the data form one SPK segment
///  to another.
///
///  The exact structure of a segment of SPK type 14 is detailed in
///  the SPK Required Reading. Please see this document for details.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  We assume that the source descriptor actually describes a
///      segment in the source SPK file containing the time coverage
///      that is desired for the subsetting operation.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 08-MAR-1995 (KRG)
/// ```
pub fn spks14(
    ctx: &mut SpiceContext,
    srchan: i32,
    srcdsc: &[f64],
    dsthan: i32,
    dstdsc: &[f64],
    dstsid: &str,
) -> crate::Result<()> {
    SPKS14(
        srchan,
        srcdsc,
        dsthan,
        dstdsc,
        dstsid.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKS14 ( S/P Kernel, subset, type 14 )
pub fn SPKS14(
    SRCHAN: i32,
    SRCDSC: &[f64],
    DSTHAN: i32,
    DSTDSC: &[f64],
    DSTSID: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRCDSC = DummyArray::new(SRCDSC, 1..);
    let DSTDSC = DummyArray::new(DSTDSC, 1..);
    let mut MYFRAM = [b' '; FRMSIZ as usize];
    let mut BEGTIM: f64 = 0.0;
    let mut DTEMP = StackArray::<f64, 2>::new(1..=ND);
    let mut ENDTIM: f64 = 0.0;
    let mut MYREF: f64 = 0.0;
    let mut RECORD = StackArray::<f64, 128>::new(1..=MAXREC);
    let mut BEGIDX: i32 = 0;
    let mut BODY: i32 = 0;
    let mut CENTER: i32 = 0;
    let mut CHBDEG: i32 = 0;
    let mut DUMMY: i32 = 0;
    let mut ENDIDX: i32 = 0;
    let mut IFRAME: i32 = 0;
    let mut ITEMP = StackArray::<i32, 6>::new(1..=NI);
    let mut RECSIZ: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    // This is the maximum size type 14 record that we can move. This
    // allows a 20th degree Chebyshev Polynomial, which should be more
    // than sufficient. This should be the same as the value in SPKPV.
    //
    //
    // Reference frame name size. See CHGIRF.
    //
    //
    // DAF ND and NI values for SPK files.
    //

    //
    // Length of a state.
    //
    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"SPKS14", ctx)?;
    }
    //
    // First, unpack the destination segment descriptor and set some
    // local variables.
    //
    DAFUS(
        DSTDSC.as_slice(),
        ND,
        NI,
        DTEMP.as_slice_mut(),
        ITEMP.as_slice_mut(),
    );

    BEGTIM = DTEMP[1];
    ENDTIM = DTEMP[2];
    BODY = ITEMP[1];
    CENTER = ITEMP[2];
    IFRAME = ITEMP[3];

    IRFNAM(IFRAME, &mut MYFRAM, ctx)?;
    //
    // If we can't find the code, it can't be an SPK file.
    //
    if FAILED(ctx) {
        CHKOUT(b"SPKS14", ctx)?;
        return Ok(());
    }
    //
    // Get the constants for this segment. There is only one.
    //
    SGFCON(SRCHAN, SRCDSC.as_slice(), 1, 1, DTEMP.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS14", ctx)?;
        return Ok(());
    }
    //
    // The first element of DTEMP now contains the number of coefficients
    // used for the Chebyshev polynomials. We need the degree of the
    // polynomial which is one less than the number of coefficients.
    //
    CHBDEG = ((DTEMP[1] as i32) - 1);
    //
    // Compute the size of the SPK record and signal an error if there is
    // not enough room in the variable RECORD to hold it.
    //
    RECSIZ = (((CHBDEG + 1) * NSTATE) + 2);

    if (RECSIZ > MAXREC) {
        SETMSG(b"Storage for # double precision numbers is needed for an SPK data record and only # locations were available. Update the parameter MAXREC in the subroutine SPKS14 and notify the NAIF group of this problem.", ctx);
        ERRINT(b"#", RECSIZ, ctx);
        ERRINT(b"#", MAXREC, ctx);
        SIGERR(b"SPICE(SPKRECTOOLARGE)", ctx)?;
        CHKOUT(b"SPKS14", ctx)?;
        return Ok(());
    }

    //
    // Get the beginning and ending indices for the packets we need for
    // the destination segment.
    //
    SGFRVI(
        SRCHAN,
        SRCDSC.as_slice(),
        BEGTIM,
        &mut MYREF,
        &mut BEGIDX,
        &mut FOUND,
        ctx,
    )?;
    SGFRVI(
        SRCHAN,
        SRCDSC.as_slice(),
        ENDTIM,
        &mut MYREF,
        &mut ENDIDX,
        &mut FOUND,
        ctx,
    )?;
    //
    // Begin the destination segment.
    //
    SPK14B(
        DSTHAN, DSTSID, BODY, CENTER, &MYFRAM, BEGTIM, ENDTIM, CHBDEG, ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS14", ctx)?;
        return Ok(());
    }
    //
    // Now we get the data one record at a time from the source segment
    // and write it out to the destination segment.
    //
    for I in BEGIDX..=ENDIDX {
        SGFPKT(
            SRCHAN,
            SRCDSC.as_slice(),
            I,
            I,
            RECORD.as_slice_mut(),
            std::slice::from_mut(&mut DUMMY),
            ctx,
        )?;
        SGFREF(
            SRCHAN,
            SRCDSC.as_slice(),
            I,
            I,
            std::slice::from_mut(&mut MYREF),
            ctx,
        )?;
        SPK14A(DSTHAN, 1, RECORD.as_slice(), &[MYREF], ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SPKS14", ctx)?;
            return Ok(());
        }
    }
    //
    // Now all we need to do is end the segment.
    //
    SPK14E(DSTHAN, ctx)?;

    CHKOUT(b"SPKS14", ctx)?;
    Ok(())
}
