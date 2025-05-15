//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;
const WDSIZE: i32 = 40;
const ND: i32 = 2;
const NI: i32 = 6;
const NCONST: i32 = 8;
const NELEMS: i32 = 10;
const NANGS: i32 = 4;
const PKTSIZ: i32 = (NELEMS + NANGS);

/// S/P Kernel, subset, type 10
///
/// Extract a subset of the data in a type 10 SPK segment into a new
/// type 10 segment.
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
///  The exact structure of a segment of SPK type 10 is detailed in
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
/// -    SPICELIB Version 1.0.0, 30-JUN-1997 (KRG)
/// ```
pub fn spks10(
    ctx: &mut SpiceContext,
    srchan: i32,
    srcdsc: &[f64],
    dsthan: i32,
    dstdsc: &[f64],
    dstsid: &str,
) -> crate::Result<()> {
    SPKS10(
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

//$Procedure SPKS10 ( S/P Kernel, subset, type 10 )
pub fn SPKS10(
    SRCHAN: i32,
    SRCDSC: &[f64],
    DSTHAN: i32,
    DSTDSC: &[f64],
    DSTSID: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let SRCDSC = DummyArray::new(SRCDSC, 1..);
    let DSTDSC = DummyArray::new(DSTDSC, 1..);
    let mut TIME = [b' '; WDSIZE as usize];
    let mut BEGTIM: f64 = 0.0;
    let mut CONSTS = StackArray::<f64, 8>::new(1..=NCONST);
    let mut DTEMP = StackArray::<f64, 2>::new(1..=ND);
    let mut ENDTIM: f64 = 0.0;
    let mut MYREF: f64 = 0.0;
    let mut PACKET = StackArray::<f64, 14>::new(1..=PKTSIZ);
    let mut BEGIDX: i32 = 0;
    let mut DUMMY: i32 = 0;
    let mut ENDIDX: i32 = 0;
    let mut ITEMP = StackArray::<i32, 6>::new(1..=NI);
    let mut NEPOCH: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local Parameters
    //
    //
    // DAF ND and NI values for SPK files.
    //

    //
    // The number of geophysical constants:
    //
    //
    // The number of elements per two-line set:
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
        CHKIN(b"SPKS10", ctx)?;
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
    //
    // Get the constants for the input segment and send them to the
    // output segment by beginning a fixed packet size segment.
    //
    SGFCON(
        SRCHAN,
        SRCDSC.as_slice(),
        1,
        NCONST,
        CONSTS.as_slice_mut(),
        ctx,
    )?;
    SGBWFS(
        DSTHAN,
        DSTDSC.as_slice(),
        DSTSID,
        NCONST,
        CONSTS.as_slice(),
        &[PKTSIZ],
        EXPCLS,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"SPKS10", ctx)?;
        return Ok(());
    }

    //
    // Get the beginning and ending indices for the packets we need for
    // the destination segment.  Note we need to get the preceding
    // and succeeding packets (if there are any) corresponding to the
    // start and end times of the output segments
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

    if !FOUND {
        ETCAL(BEGTIM, &mut TIME, ctx);
        SETMSG(b"An error has occurred while attempting to subset the a type 10 SPK segment. The error occurred while attempting to locate a packet for the epoch #.  There does not appear to be such a packet. ", ctx);
        ERRCH(b"#", &TIME, ctx);
        SIGERR(b"SPICE(CANNOTGETPACKET)", ctx)?;
        CHKOUT(b"SPKS10", ctx)?;
        return Ok(());
    }

    if (MYREF > BEGTIM) {
        BEGIDX = intrinsics::MAX0(&[1, (BEGIDX - 1)]);
    }

    SGFRVI(
        SRCHAN,
        SRCDSC.as_slice(),
        ENDTIM,
        &mut MYREF,
        &mut ENDIDX,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        ETCAL(ENDTIM, &mut TIME, ctx);
        SETMSG(b"An error has occurred while attempting to subset the a type 10 SPK segment. The error occurred while attempting to locate a packet for the epoch #.  There does not appear to be such a packet. ", ctx);
        ERRCH(b"#", &TIME, ctx);
        SIGERR(b"SPICE(CANNOTGETPACKET)", ctx)?;
        CHKOUT(b"SPKS10", ctx)?;
        return Ok(());
    }

    //
    // Get the total number of epochs.
    //
    SGMETA(SRCHAN, SRCDSC.as_slice(), NREF, &mut NEPOCH, ctx)?;

    if (MYREF < ENDTIM) {
        ENDIDX = intrinsics::MIN0(&[NEPOCH, (ENDIDX + 1)]);
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
            PACKET.as_slice_mut(),
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
        SGWFPK(DSTHAN, 1, PACKET.as_slice(), 1, &[MYREF], ctx)?;
    }

    //
    // Now all we need to do is end the segment.
    //
    SGWES(DSTHAN, ctx)?;

    CHKOUT(b"SPKS10", ctx)?;
    Ok(())
}
