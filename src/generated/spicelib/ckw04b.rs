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
const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const ND: i32 = 2;
const NI: i32 = 6;
const NDESCR: i32 = (ND + ((NI + 1) / 2));
const NCONST: i32 = 0;
const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const SIDLEN: i32 = 40;

/// CK type 04: Begin a segment
///
/// Begin a type CK04 segment in the DAF file associated with
/// HANDLE. See also CKW04A and CKW04E.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   The handle of an DAF file open for writing.
///  SEGID      I   The string to use for segment identifier.
///  INST       I   The NAIF ID code for the SC or instrument.
///  AVFLAG     I   The angular rates flag.
///  REF        I   The reference frame for this segment.
///  BEGTIM     I   The segment coverage start encoded SCLK time
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the file handle of a CK file that has been
///           opened for writing.
///
///  SEGID    is the segment identifier. CK segment identifier
///           may contain up to 40 printing ASCII characters.
///
///  INST     is the SPICE ID for the SC structure or instrument
///           whose orientation are to be recorded in a CK file.
///
///  AVFLAG   is a logical flag that indicates whether segment will
///           contain angular rate information.
///
///  REF      is the name of a reference frame that pointing is
///           given with respect to, for example 'J2000'.
///
///  BEGTIM   is the encoded SCLK time for the start of the segment
///           coverage.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  The input data is used to create the segment summary for the
///  segment being started in the DAF file associated with HANDLE.
///
///  See the $Particulars section for details about the structure of a
///  type 4 CK segment.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the files
///  'sgparam.inc' and 'ckparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a file access error occurs while this routine begins a new
///      type 04 segment, the error is signaled by a routine in the
///      call tree of this routine.
///
///  2)  If numeric ID for given reference frame cannot be resolved
///      from it's name, the error SPICE(INVALIDREFFRAME) is signaled.
///
///  3)  If SEGID is more than 40 characters long, the error
///      SPICE(SEGIDTOOLONG) is signaled.
///
///  4)  If SEGID contains any nonprintable characters, the error
///      SPICE(NONPRINTABLECHARS) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine begins writing a type 4 CK segment to the open DAF
///  file that is associated with HANDLE. The file must have been
///  opened with write access.
///
///  This routine is one of a set of three routines for creating and
///  adding data to type 4 CK segments. These routines are:
///
///     CKW04B: Begin a type 4 CK segment. This routine must be
///             called before any data may be added to a type 4
///             segment.
///
///     CKW04A: Add data to a type 4 CK segment. This routine may be
///             called any number of times after a call to CKW04B to
///             add type 4 records to the CK segment that was
///             started.
///
///     CKW04E: End a type 4 CK segment. This routine is called to
///             make the type 4 segment a permanent addition to the
///             DAF file. Once this routine is called, no further type
///             4 records may be added to the segment. A new segment
///             must be started.
///
///  A type 4 CK segment consists of coefficient sets for variable
///  order Chebyshev polynomials over consecutive time intervals of
///  a variable length. The gaps between intervals are allowed.
///  The Chebyshev polynomials represent individual quaternion
///  components q0, q1, q2 and q3 and individual angular velocities
///  AV1, AV2 and AV3 if they are included with the data.
///
///  The pointing data supplied to the type 4 CK writer (CKW04A)
///  is packed into an array as a sequence of records,
///
///     ----------------------------------------------------
///     | Record 1 | Record 2 | .. | Record N-1 | Record N |
///     ----------------------------------------------------
///
///  with each record in data packets has the following format.
///
///     ----------------------------------------------------
///     | The midpoint of the approximation interval       |
///     ----------------------------------------------------
///     | The radius of the approximation interval         |
///     ----------------------------------------------------
///     | Number of coefficients for q0                    |
///     ----------------------------------------------------
///     | Number of coefficients for q1                    |
///     ----------------------------------------------------
///     | Number of coefficients for q2                    |
///     ----------------------------------------------------
///     | Number of coefficients for q3                    |
///     ----------------------------------------------------
///     | Number of coefficients for AV1                   |
///     ----------------------------------------------------
///     | Number of coefficients for AV2                   |
///     ----------------------------------------------------
///     | Number of coefficients for AV3                   |
///     ----------------------------------------------------
///     | q0 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q1 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q2 Cheby coefficients                            |
///     ----------------------------------------------------
///     | q3 Cheby coefficients                            |
///     ----------------------------------------------------
///     | AV1 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///     | AV2 Cheby coefficients (optional)                |
///     ----------------------------------------------------
///     | AV3 Cheby coefficients (optional)                |
///     ----------------------------------------------------
/// ```
///
/// # Examples
///
/// ```text
///  Assume that we have:
///
///     HANDLE   is the handle of an CK file opened with write
///              access.
///
///     SEGID    is a character string of no more than 40 characters
///              which provides a pedigree for the data in the CK
///              segment we will create.
///
///     INST     is the SPICE ID code for the instrument whose
///              pointing data is to be placed into the file.
///
///     AVFLAG   angular rates flag.
///
///     REFFRM   is the name of the SPICE reference frame for the
///              pointing data.
///
///     BEGTIM   is the starting encoded SCLK time for which the
///              segment is valid.
///
///     ENDTIM   is the ending encoded SCLK time for which the segment
///              is valid.
///
///     N        is the number of type 4 records that we want to
///              put into a segment in an CK file.
///
///     NPKTS    is integer array which contains the lengths of
///              variable size data packets
///
///     RECRDS   contains N type 4 records packaged for the CK
///              file.
///
///     SCSTRT   contains the initial encoded SC time for each of
///              the records contained in RECRDS, where
///
///                 SCSTRT(I) < SCSTRT(I+1), I = 1, N-1
///
///                 SCSTRT(1) <= FIRST, SCSTRT(N) < LAST
///
///  Then the following code fragment demonstrates how to create
///  a type 4 CK segment if all of the data for the segment is
///  available at one time.
///
///  C
///  C     Begin the segment.
///  C
///        CALL CKW04B ( HANDLE, BEGTIM, INST, REF, AVFLAG, SEGID )
///  C
///  C     Add the data to the segment all at once.
///  C
///        CALL CKW04A ( HANDLE, N, NPKTS, RECRDS, SCSTRT )
///  C
///  C     End the segment, making the segment a permanent
///  C     addition to the CK file.
///  C
///        CALL CKW04E ( HANDLE, ENDTIM )
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  The file containing the segment should be opened for read
///      or write access either by CKOPN or DAFOPW.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  Y.K. Zaiko         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.2, 17-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.0, 05-MAY-1999 (YKZ) (BVS)
/// ```
pub fn ckw04b(
    ctx: &mut SpiceContext,
    handle: i32,
    begtim: f64,
    inst: i32,
    ref_: &str,
    avflag: bool,
    segid: &str,
) -> crate::Result<()> {
    CKW04B(
        handle,
        begtim,
        inst,
        ref_.as_bytes(),
        avflag,
        segid.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKW04B ( CK type 04: Begin a segment )
pub fn CKW04B(
    HANDLE: i32,
    BEGTIM: f64,
    INST: i32,
    REF: &[u8],
    AVFLAG: bool,
    SEGID: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let DCOEFF: f64 = 0.0;
    let mut DESCR = StackArray::<f64, 5>::new(1..=NDESCR);
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut REFCOD: i32 = 0;
    let mut VALUE: i32 = 0;

    //
    // Spicelib functions
    //

    //
    // Local Parameters
    //

    //
    // DAF ND and NI values for CK files and length of a DAF descriptor.
    //

    //
    // The number of generic segment constants in a type 4 CK segment.
    //

    //
    // The integer codes of the first and last printable ASCII
    // characters.
    //

    //
    // The maximum number of characters allowed in a CK segment
    // identifier.
    //

    //
    // Local variables
    //

    //
    // Standard SPICELIB error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKW04B", ctx)?;
    }

    //
    // Create a descriptor for the segment we are about to write. First
    // assign start and stop times.
    //
    DCD[1] = BEGTIM;
    DCD[2] = 0.0;

    //
    // Second, resolve reference frame ID code from its name and
    // assign it to the corresponding descriptor component. Signal
    // an error if frame is not recognized.
    //
    NAMFRM(REF, &mut REFCOD, ctx)?;

    if (REFCOD == 0) {
        SETMSG(b"The reference frame # is not supported.", ctx);
        ERRCH(b"#", REF, ctx);
        SIGERR(b"SPICE(INVALIDREFFRAME)", ctx)?;
        CHKOUT(b"CKW04B", ctx)?;
        return Ok(());
    }

    ICD[2] = REFCOD;

    //
    // Third, assign values to the rest of the integer components of
    // the segment descriptor.
    //
    ICD[1] = INST;
    ICD[3] = CK4DTP;

    if AVFLAG {
        ICD[4] = 1;
    } else {
        ICD[4] = 0;
    }

    //
    // Now pack the segment descriptor.
    //
    DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

    //
    // Check that all characters in the SEGID are printable.
    //
    for I in 1..=LASTNB(SEGID) {
        VALUE = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((VALUE < FPRINT) || (VALUE > LPRINT)) {
            SETMSG(
                b"The segment identifier contains nonprintable characters",
                ctx,
            );
            SIGERR(b"SPICE(NONPRINTABLECHARS)", ctx)?;
            CHKOUT(b"CKW04B", ctx)?;
            return Ok(());
        }
    }

    //
    // Also check if the segment identifier is too long.
    //
    if (LASTNB(SEGID) > SIDLEN) {
        SETMSG(b"Segment identifier contains more than 40 characters.", ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"CKW04B", ctx)?;
        return Ok(());
    }

    //
    // We've got a valid descriptor and identifier and can begin
    // the segment. For this data type, we want to use an explicit
    // reference value index where the reference epochs are in
    // increasing order. We also want the index returned for a
    // particular request epoch to be the index of the greatest
    // reference epoch less than or equal to the request epoch. These
    // characteristics are prescribed by the mnemonic EXPLE. See the
    // include file 'sgparam.inc' for more details.
    //
    SGBWVS(
        HANDLE,
        DESCR.as_slice(),
        SEGID,
        NCONST,
        &[DCOEFF],
        EXPLE,
        ctx,
    )?;

    //
    // No need to check FAILED() here, since all we do after this
    // point is checking out.
    //
    CHKOUT(b"CKW04B", ctx)?;

    Ok(())
}
