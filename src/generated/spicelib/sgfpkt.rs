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

/// Generic Segment: Fetch data packets
///
/// Fetch the data packets indexed from FIRST to LAST from the
/// packet partition of a generic segment. The segment is
/// identified by a DAF file handle and segment descriptor.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The file handle attached to an open DAF.
///  DESCR      I   The descriptor associated with a generic segment.
///  FIRST      I   The index of the first data packet to fetch.
///  LAST       I   The index of the last data packet to fetch.
///  VALUES     O   The data packets that have been fetched.
///  ENDS       O   An array of pointers to the ends of the packets.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF opened for reading that
///           contains the segment described by DESCR.
///
///  DESCR    is the descriptor of the segment with the desired
///           constant values. This must be the descriptor for a
///           generic segment in the DAF associated with HANDLE.
///
///  FIRST    is the index of the first value to fetch from the
///           constants section of the DAF segment described
///           by DESCR.
///
///  LAST     is the index of the last value to fetch from the
///           constants section of the DAF segment described
///           by DESCR
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUES   is the array of values constructed by concatenating
///           requested packets one after the other into
///           an array. Pictorially we can represent VALUES
///           as:
///
///              +--------------------------+
///              | first requested packet   |
///              +--------------------------+
///              | second requested packet  |
///              +--------------------------+
///                         .
///                         .
///                         .
///              +--------------------------+
///              | first requested packet   |
///              +--------------------------+
///
///  ENDS     is an array of pointers to the ends of the
///           fetched packets.  ENDS(1) gives the index
///           of the last item of the first packet fetched.
///           ENDS(2) gives the index of the last item of
///           the second packet fetched, etc.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If FIRST is less than 1 or LAST is greater than the number of
///      packets, the error SPICE(REQUESTOUTOFBOUNDS) is signaled.
///
///  2)  If LAST is less than FIRST, the error SPICE(REQUESTOUTOFORDER)
///      is signaled.
///
///  3)  If the packet directory structure is unrecognized, the error
///      SPICE(UNKNOWNPACKETDIR) is signaled. The most likely cause of
///      this error is that an upgrade to your version of the SPICE
///      toolkit is needed.
/// ```
///
/// # Files
///
/// ```text
///  See the description of HANDLE above.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine fetches requested packets from a generic
///  DAF segment. The two arrays returned have the following
///  relationship to one another. The first packet returned
///  resides in VALUES between indexes 1 and ENDS(1).  If a
///  second packet is returned it resides in VALUES between
///  indices ENDS(1)+1 and ENDS(2).  This relations ship is
///  repeated so that if I is greater than 1 and at least I
///  packets were returned then the I'th packet resides in
///  VALUES between index ENDS(I-1) + 1 and ENDS(I).
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you have located a generic DAF segment (as
///  identified by the contents of a segment descriptor).  The
///  fragment of code below shows how you could fetch packets
///  3 through 7 (assuming that many packets are present).
///  from the segment.
///
///     Declarations:
///
///     DOUBLE PRECISION   MYPKSZ (<enough room to hold all packets>)
///
///     INTEGER               ENDS  ( 5 )
///     INTEGER               MYNPKT
///
///     get the number of packets
///
///     CALL SGMETA ( HANDLE, DESCR, NPKT, MYNPKT )
///
///     finally, fetch the packets from the segment.
///
///     IF ( 7 .LE. MYNPKT ) THEN
///        CALL SGFPKT ( HANDLE, DESCR, 3, 7,  MYPKSZ, ENDS )
///     END IF
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The segment described by DESCR must be a generic segment,
///      otherwise the results of this routine are not predictable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.1, 26-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA calls with DAFGDA.
///
/// -    SPICELIB Version 1.1.0, 30-JUL-1996 (KRG) (NJB)
///
///         Found and fixed a bug in the calculation of the beginning
///         address for variable length packet fetching. The base address
///         for the packet directory was not added into the value. This
///         bug went unnoticed because of a bug in SGSEQW, entry SGWES,
///         that put absolute addresses into the packet directory rather
///         than addresses that were relative to the start of the DAF
///         array. The bug in SGSEQW has also been fixed.
///
/// -    SPICELIB Version 1.0.0, 06-JAN-1994 (KRG) (WLT)
/// ```
pub fn sgfpkt(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    first: i32,
    last: i32,
    values: &mut [f64],
    ends: &mut [i32],
) -> crate::Result<()> {
    SGFPKT(handle, descr, first, last, values, ends, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGFPKT ( Generic Segment: Fetch data packets )
pub fn SGFPKT(
    HANDLE: i32,
    DESCR: &[f64],
    FIRST: i32,
    LAST: i32,
    VALUES: &mut [f64],
    ENDS: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let DESCR = DummyArray::new(DESCR, 1..);
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
    let mut ENDS = DummyArrayMut::new(ENDS, 1..);
    let mut DTEMP = StackArray::<f64, 2>::new(1..=2);
    let mut B: i32 = 0;
    let mut BEGIN1: i32 = 0;
    let mut BEGIN2: i32 = 0;
    let mut E: i32 = 0;
    let mut MYNPDR: i32 = 0;
    let mut MYNPKT: i32 = 0;
    let mut MYPDRB: i32 = 0;
    let mut MYPDRT: i32 = 0;
    let mut MYPKSZ: i32 = 0;
    let mut MYPKTB: i32 = 0;
    let mut MYPKTO: i32 = 0;
    let mut SIZE: i32 = 0;
    let mut SOFFST: i32 = 0;
    let mut VOFFST: i32 = 0;

    //
    // Spicelib Functions
    //

    //
    // Local Parameters
    //
    // Include the mnemonic values.
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGFPKT", ctx)?;
    //
    // Perform the needed initialization
    //
    SGMETA(HANDLE, DESCR.as_slice(), NPKT, &mut MYNPKT, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), PDRTYP, &mut MYPDRT, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), PKTOFF, &mut MYPKTO, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), PKTSZ, &mut MYPKSZ, ctx)?;
    SGMETA(HANDLE, DESCR.as_slice(), PKTBAS, &mut MYPKTB, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGFPKT", ctx)?;
        return Ok(());
    }

    //
    // Perform checks on the inputs for reasonableness.
    //
    if ((FIRST < 1) || (LAST > MYNPKT)) {
        SETMSG(b"The range of packets requested extends beyond the available packet data.  The packet data is available for indexes 1 to #.  You\'ve requested data from # to #. ", ctx);
        ERRINT(b"#", MYNPKT, ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        SIGERR(b"SPICE(REQUESTOUTOFBOUNDS)", ctx)?;
        CHKOUT(b"SGFPKT", ctx)?;
        return Ok(());
    }

    if (LAST < FIRST) {
        SETMSG(
            b"The last packet requested, #, is before the first packet requested, #. ",
            ctx,
        );
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", FIRST, ctx);
        SIGERR(b"SPICE(REQUESTOUTOFORDER)", ctx)?;
        CHKOUT(b"SGFPKT", ctx)?;
        return Ok(());
    }
    //
    // We've passed the sanity tests, if the packet directory structure
    // is recognized fetch the values and ends. We assume that we are
    // reading data from a correctly constructed generic segment, so we
    // do not need to worry about the type of reference index, as this is
    // not needed to fetch a data packet.
    // Currently, only two packet directory types are supported, and this
    // subroutine is the only place that this is documented. The types
    // have values zero (0) and one (1) for, respectively, fixed size
    // packets and variable size packets.
    //
    if (MYPDRT == 0) {
        //
        // All packets have the same size MYPKSZ so the address of the
        // start of the first packet and end of the last packet are easily
        // computed.
        //
        if (MYPKTO == 0) {
            //
            // Compute tha addresses for the packet data in the generic
            // segment.
            //
            B = ((MYPKTB + ((FIRST - 1) * MYPKSZ)) + 1);
            E = (MYPKTB + (LAST * MYPKSZ));
            //
            // Get the packet data all in one shot since we know it's
            // contiguous.
            //
            DAFGDA(HANDLE, B, E, VALUES.as_slice_mut(), ctx)?;
        } else {
            //
            // Compute the addresses for the packet data in the generic
            // segment. Remember that we need to account for an offset
            // here to get to the start of the actual data packet.
            //
            SIZE = (MYPKSZ + MYPKTO);
            //
            // Get the packet data. Because there is an offset from the
            // address to the start of the packet data, we need to get
            // the data one packet at a time rather than all at once.
            //
            for I in FIRST..=LAST {
                SOFFST = (((I - 1) * SIZE) + 1);
                VOFFST = (((I - FIRST) * MYPKSZ) + 1);

                B = ((MYPKTB + SOFFST) + MYPKTO);
                E = ((MYPKTB + SOFFST) + MYPKSZ);

                DAFGDA(HANDLE, B, E, VALUES.subarray_mut(VOFFST), ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGFPKT", ctx)?;
                    return Ok(());
                }
            }
        }
        //
        // Compute the ends for each of the data packets. This is the
        // same for both of the cases above because we have fixed size
        // data packets.
        //
        for I in 1..=((LAST - FIRST) + 1) {
            ENDS[I] = (I * MYPKSZ);
        }
    } else {
        //
        // In addition to the other meta data items already retrieved, we
        // will also need a few others.
        //
        SGMETA(HANDLE, DESCR.as_slice(), PDRBAS, &mut MYPDRB, ctx)?;
        SGMETA(HANDLE, DESCR.as_slice(), NPDR, &mut MYNPDR, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGFPKT", ctx)?;
            return Ok(());
        }
        //
        // Each packet has a different size, so we need to fetch each one
        // individually, keeping track of the ends and things. We assume
        // that there is enough room in the array of values to hold all of
        // the packets. For the variable packet case, however, we do not
        // need to treat the implicit indexing and explicit indexing cases
        // separately.
        //
        VOFFST = 1;

        for I in 1..=((LAST - FIRST) + 1) {
            //
            // Compute the addresses in the generic segment for the
            // beginning of data packets I and I+1. We need these to
            // compute the size of the packet.
            //
            B = (((MYPDRB + FIRST) + I) - 1);
            E = (B + 1);
            //
            // Get the beginning addresses for the two data packets and
            // convert them into integers.
            //
            DAFGDA(HANDLE, B, E, DTEMP.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGFPKT", ctx)?;
                return Ok(());
            }

            BEGIN1 = (DTEMP[1] as i32);
            BEGIN2 = (DTEMP[2] as i32);
            //
            // Compute the size of data packet I, remembering to deal with
            // the packet offset that might be present, and the beginning
            // and ending addresses for the packet data.
            //
            SIZE = ((BEGIN2 - BEGIN1) - MYPKTO);
            B = (MYPKTB + BEGIN1);
            E = ((B + SIZE) - 1);
            //
            // Get the data for packet I.
            //
            DAFGDA(HANDLE, B, E, VALUES.subarray_mut(VOFFST), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGFPKT", ctx)?;
                return Ok(());
            }
            //
            // Compute the end for packet I and store it.
            //
            VOFFST = (VOFFST + SIZE);
            ENDS[I] = (VOFFST - 1);
        }
    }

    CHKOUT(b"SGFPKT", ctx)?;
    Ok(())
}
