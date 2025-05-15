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

struct SaveVars {
    IOFFST: i32,
    LSTBEG: i32,
    LSTHAN: i32,
    META: StackArray<i32, 17>,
    METASZ: i32,
    NIEVEN: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut IOFFST: i32 = 0;
        let mut LSTBEG: i32 = 0;
        let mut LSTHAN: i32 = 0;
        let mut META = StackArray::<i32, 17>::new(1..=MXMETA);
        let mut METASZ: i32 = 0;
        let mut NIEVEN: bool = false;

        LSTBEG = -1;
        LSTHAN = 0;

        Self {
            IOFFST,
            LSTBEG,
            LSTHAN,
            META,
            METASZ,
            NIEVEN,
        }
    }
}

/// Generic segments: Fetch meta data value
///
/// Obtain the value of a specified generic segment meta data item.
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
///  HANDLE     I   Handle of a DAF open for reading.
///  DESCR      I   Descriptor for a generic segment in the DAF.
///  MNEMON     I   An integer mnemonic for the desired meta data.
///  VALUE      O   The value of the meta data item requested.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF opened for reading that
///           contains the generic segment described by DESCR.
///
///  DESCR    is the descriptor of a generic segment. This must
///           be the descriptor for a generic segment in the DAF
///           associated with HANDLE.
///
///  MNEMON   is the mnemonic used to represent the desired piece of
///           meta data. See the file 'sgparam.inc' for details, the
///           mnemonics, and their values.
/// ```
///
/// # Detailed Output
///
/// ```text
///  VALUE    is the value of the meta data item associated with
///           the mnemonic MNEMON that is in the generic segment
///           specified by HANDLE and DESCR.
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
///  1)  If the mnemonic for the meta data item is not valid, the error
///      SPICE(UNKNOWNMETAITEM) is signaled.
///
///  2)  If the last address in the DAF segment that reports the number
///      of meta data items that exist in the segment is less than
///      MNMETA, the error SPICE(INVALIDMETADATA) is signaled.
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
///  This routine is a utility for fetching the meta data associated
///  with a DAF generic segment.
///
///  A DAF generic segment contains several logical data partitions:
///
///     1) A partition for constant values to be associated with each
///        data packet in the segment.
///
///     2) A partition for the data packets.
///
///     3) A partition for reference values.
///
///     4) A partition for a packet directory, if the segment contains
///        variable sized packets.
///
///     5) A partition for a reference value directory.
///
///     6) A reserved partition that is not currently used. This
///        partition is only for the use of the NAIF group at the Jet
///        Propulsion Laboratory (JPL).
///
///     7) A partition for the meta data which describes the locations
///        and sizes of other partitions as well as providing some
///        additional descriptive information about the generic
///        segment.
///
///              +============================+
///              |         Constants          |
///              +============================+
///              |          Packet 1          |
///              |----------------------------|
///              |          Packet 2          |
///              |----------------------------|
///              |              .             |
///              |              .             |
///              |              .             |
///              |----------------------------|
///              |          Packet N          |
///              +============================+
///              |      Reference Values      |
///              +============================+
///              |      Packet Directory      |
///              +============================+
///              |    Reference  Directory    |
///              +============================+
///              |       Reserved  Area       |
///              +============================+
///              |     Segment Meta Data      |
///              +----------------------------+
///
///  Only the placement of the meta data at the end of a segment is
///  required. The other data partitions may occur in any order in the
///  segment because the meta data will contain pointers to the
///  appropriate locations of the other data partitions within the
///  segment.
///
///  The meta data for the segment should be obtained only through
///  use of this routine, SGMETA.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that we would like to know how many constants, data
///  packets, and reference values are in the generic segment that we
///  have located in the DAF file associated with HANDLE.
///
///  C
///  C     Get the number of constants.
///  C
///        CALL SGMETA ( HANDLE, DESCR, NCON, NCONST )
///  C
///  C     Get the number of data packets.
///  C
///        CALL SGMETA ( HANDLE, DESCR, NPKT, NPKTS )
///  C
///  C     Get the number of constants.
///  C
///        CALL SGMETA ( HANDLE, DESCR, NREF, NREFS )
///
///  C
///  C     Print the values.
///  C
///        WRITE (*, *) 'Number of Constants       : ', NCONST
///        WRITE (*, *) 'Number of Data Packets    : ', NPKTS
///        WRITE (*, *) 'Number of Reference Values: ', NREFS
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The segment described by DESCR MUST be a generic segment,
///      otherwise the results of this routine are not predictable.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.4.1, 20-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.4.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA call with DAFGDA.
///
/// -    SPICELIB Version 1.3.0, 14-JUN-1999 (FST)
///
///         Altered the check in/out structure to be more reasonable.
///         This introduced redundant code, but only to increase the
///         efficiency of the normal mode of operation.
///
/// -    SPICELIB Version 1.2.0, 24-SEP-1998 (FST)
///
///         Modified the code that handles reading the meta data from the
///         DAF to handle the case when the number of meta data items in
///         the file exceeds the current maximum defined in sgparam.inc.
///         In the event that this situation occurs, the routine loads
///         what meta data it can interpret and ignores the rest. In
///         this event if NMETA is requested, it is returned as MXMETA in
///         sgparam.inc.
///
///         An additional exception is now trapped by the routine. If
///         a generic segment in a DAF reports less than the known minimum
///         number of meta data items, then the routine signals the
///         error SPICE(INVALIDMETADATA).
///
///         The conditions that cause the SPICE(UNKNOWNMETAITEM) to be
///         signaled have been altered. Now if the integer mnemonic
///         is not between 1 and METASZ inclusive, or NMETA the error
///         is signaled. In the versions preceding this change, for
///         segments that reported less than NMETA items of meta data
///         could not use this routine to request the number of meta
///         data items without signaling SPICE(UNKNOWNMETAITEM).
///
/// -    SPICELIB Version 1.1.0, 11-APR-1995 (KRG)
///
///         Modified the code that deals with the EQUIVALENCEd part
///         descriptor. We now call MOVED rather than using a direct
///         assignment.
///
/// -    SPICELIB Version 1.0.0, 11-APR-1995 (KRG) (WLT)
/// ```
pub fn sgmeta(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    mnemon: i32,
    value: &mut i32,
) -> crate::Result<()> {
    SGMETA(handle, descr, mnemon, value, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGMETA ( Generic segments: Fetch meta data value )
pub fn SGMETA(
    HANDLE: i32,
    DESCR: &[f64],
    MNEMON: i32,
    VALUE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..);
    let mut AMETAS: i32 = 0;
    let mut BEGIN: i32 = 0;
    let mut BEGMTA: i32 = 0;
    let mut BEGM1: i32 = 0;
    let mut END: i32 = 0;
    let mut ENDMTA: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NIOVR2: i32 = 0;
    let mut DMTASZ: f64 = 0.0;
    let mut XMETA = StackArray::<f64, 17>::new(1..=MXMETA);
    let mut DTEMP = StackArray::<f64, 2>::new(1..=2);
    let ITEMP = StackArray::<i32, 4>::new(1..=4);

    //
    // Spicelib Functions
    //

    //
    // Local Parameters
    //
    // Include the mnemonic values for the generic segment declarations.
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

    //
    // Handle the case when we are looking at the same file and segment
    // descriptor first.  This will result in duplicated code, but will
    // increase efficiency for the usual execution case. We need not
    // worry about the first time through, since LSTHAN and LSTBEG are
    // set to values that are bogus for actual DAF files.
    //
    if (HANDLE == save.LSTHAN) {
        //
        // Get the begin and end values from the descriptor. They are
        // located in the last two "integer" positions of the descriptor.
        //
        if save.NIEVEN {
            MOVED(DESCR.subarray(save.IOFFST), 1, DTEMP.as_slice_mut());
            BEGIN = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[1];
            END = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[2];
        } else {
            MOVED(DESCR.subarray(save.IOFFST), 2, DTEMP.as_slice_mut());
            BEGIN = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[2];
            END = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[3];
        }

        //
        // Check the segment start address. This will tell us whether we
        // are looking at the same segment.
        //
        if (save.LSTBEG == BEGIN) {
            //
            // The only acceptable integer mnemonics at this point are 1
            // through METASZ inclusive, and NMETA.  All other requests
            // should signal the SPICE(UNKNOWNMETAITEM) error, since the
            // current segment has no knowledge of these values.
            //
            if ((MNEMON <= 0) || ((MNEMON > save.METASZ) && (MNEMON != NMETA))) {
                CHKIN(b"SGMETA", ctx)?;
                *VALUE = -1;
                SETMSG(b"The item requested, #, is not one of the recognized meta data items associated with this generic segment.", ctx);
                ERRINT(b"#", MNEMON, ctx);
                SIGERR(b"SPICE(UNKNOWNMETAITEM)", ctx)?;
                CHKOUT(b"SGMETA", ctx)?;
                return Ok(());
            }

            //
            // Set the value for the desired meta data item and return.
            //
            *VALUE = save.META[MNEMON];

            return Ok(());
        }
    }

    //
    // At this point we are going to have to load the meta data.  If
    // the new handle and the old handle are the same, then the above
    // code has already retrieved the relevant segment addresses. If not
    // we need to fetch them.  First check in.
    //

    CHKIN(b"SGMETA", ctx)?;

    if (HANDLE != save.LSTHAN) {
        DAFHSF(HANDLE, &mut ND, &mut NI, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGMETA", ctx)?;
            return Ok(());
        }

        NIOVR2 = (NI / 2);

        save.NIEVEN = ((2 * NIOVR2) == NI);

        save.IOFFST = (ND + NIOVR2);
        save.LSTHAN = HANDLE;

        //
        // Get the begin and end values from the descriptor. They are
        // located in the last two "integer" positions of the descriptor.
        //
        if save.NIEVEN {
            MOVED(DESCR.subarray(save.IOFFST), 1, DTEMP.as_slice_mut());
            BEGIN = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[1];
            END = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[2];
        } else {
            MOVED(DESCR.subarray(save.IOFFST), 2, DTEMP.as_slice_mut());
            BEGIN = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[2];
            END = DummyArray::<i32>::from_equiv(DTEMP.as_slice(), 1..=4)[3];
        }
    }

    //
    // Save the new begin address. Remember we have either just computed
    // this from the IF block above, or we computed it in the very
    // first IF block.
    //
    save.LSTBEG = BEGIN;

    //
    // Compute the begin address of the meta data and compute the
    // end address of the number we will be collecting.
    //
    DAFGDA(HANDLE, END, END, std::slice::from_mut(&mut DMTASZ), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGMETA", ctx)?;
        return Ok(());
    }

    save.METASZ = intrinsics::IDNINT(DMTASZ);
    //
    // Store the actual meta size in AMETAS, in case METASZ ends up
    // being modified to conform to our current understanding of
    // meta data items.
    //
    AMETAS = save.METASZ;

    //
    // Check to see if METASZ is an unacceptable value.
    //
    if (save.METASZ < MNMETA) {
        *VALUE = -1;
        SETMSG(b"This segment reports that it has # meta data items. Every generic segment must have at least #.", ctx);
        ERRINT(b"#", save.METASZ, ctx);
        ERRINT(b"#", MNMETA, ctx);
        SIGERR(b"SPICE(INVALIDMETADATA)", ctx)?;
        CHKOUT(b"SGMETA", ctx)?;
        return Ok(());

    //
    // If it is not, we may need to fix a few things to work around some
    // older files that have been delivered. We perform these kludges
    // here. Originally, the number of meta data items was not
    // considered to be part of the meta data. It now is, so if we
    // encounter an older version of the file, we need to increment the
    // meta data size by 1. The number of meta data items is always
    // after all of the meta data items, so we can do this.
    //
    } else if (save.METASZ == 15) {
        save.METASZ = (save.METASZ + 1);
        AMETAS = save.METASZ;

    //
    // If not check to see if METASZ is greater than the known MXMETA.
    // If it is then this segment most likely was constructed from
    // some newer version of the toolkit.  Load what meta data we
    // currently know about as laid out in sgparam.inc.
    //
    } else if (save.METASZ > MXMETA) {
        //
        // Leave AMETAS alone, since we need to know how far back
        // into the DAF file to begin reading.
        //
        save.METASZ = MXMETA;
    }

    //
    // The address computations that follow are precisely the same
    // as the previous version of the file, except when AMETAS is not
    // METASZ.  This only happens when METASZ is greater than MXMETA.
    //
    BEGMTA = ((END - AMETAS) + 1);
    ENDMTA = ((BEGMTA + save.METASZ) - 1);

    DAFGDA(HANDLE, BEGMTA, ENDMTA, XMETA.as_slice_mut(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGMETA", ctx)?;
        return Ok(());
    }

    //
    // Convert all of the meta data values into integers.
    //
    for I in 1..=save.METASZ {
        save.META[I] = intrinsics::IDNINT(XMETA[I]);
    }

    //
    // The kludge continues... NMETA and MXMETA are ALWAYS the same
    // value, and any missing values must appear between the last known
    // value, META(METASZ-1), and the end value, META(NMETA), so we zero
    // them out.
    //
    save.META[NMETA] = save.METASZ;

    for I in save.METASZ..=(MXMETA - 1) {
        save.META[I] = 0;
    }

    //
    // Adjust the bases so that the N'th item of a partition is at
    // address META(PARTITION_BASE) + N
    //
    BEGM1 = (BEGIN - 1);
    save.META[CONBAS] = (save.META[CONBAS] + BEGM1);
    save.META[REFBAS] = (save.META[REFBAS] + BEGM1);
    save.META[RDRBAS] = (save.META[RDRBAS] + BEGM1);
    save.META[PDRBAS] = (save.META[PDRBAS] + BEGM1);
    save.META[PKTBAS] = (save.META[PKTBAS] + BEGM1);
    save.META[RSVBAS] = (save.META[RSVBAS] + BEGM1);

    //
    // The only acceptable integer mnemonics at this point are 1 through
    // METASZ inclusive, and NMETA.  All other requests should signal
    // the SPICE(UNKNOWNMETAITEM) error, since the current segment has
    // no knowledge of these values.
    //
    if ((MNEMON <= 0) || ((MNEMON > save.METASZ) && (MNEMON != NMETA))) {
        *VALUE = -1;
        SETMSG(b"The item requested, #, is not one of the recognized meta data items associated with this generic segment.", ctx);
        ERRINT(b"#", MNEMON, ctx);
        SIGERR(b"SPICE(UNKNOWNMETAITEM)", ctx)?;
        CHKOUT(b"SGMETA", ctx)?;
        return Ok(());
    }

    //
    // Set the value for the desired meta data item, check out if we
    // need to, and return.
    //
    *VALUE = save.META[MNEMON];

    CHKOUT(b"SGMETA", ctx)?;
    Ok(())
}
