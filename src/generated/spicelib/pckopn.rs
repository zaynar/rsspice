//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const TYPE: &[u8] = b"PCK";
const ND: i32 = 2;
const NI: i32 = 5;
const MXCREC: i32 = 1000;

/// PCK, open new file
///
/// Create a new PCK file, returning the handle of the opened file.
///
/// # Required Reading
///
/// * [PCK](crate::required_reading::pck)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  NAME       I   The name of the PCK file to be opened.
///  IFNAME     I   The internal filename for the PCK.
///  NCOMCH     I   The number of characters to reserve for comments.
///  HANDLE     O   The handle of the opened PCK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  NAME     is the name of the PCK file to be created.
///
///  IFNAME   is the internal filename for the PCK file that is being
///           created. The internal filename may be up to 60 characters
///           long. If you do not have any conventions for tagging your
///           files, an internal filename of 'PCK_file' is perfectly
///           acceptable. You may also leave it blank if you like.
///
///  NCOMCH   is the space, measured in characters, to be
///           initially set aside for the comment area when a new PCK
///           file is opened. The amount of space actually set aside
///           may be greater than the amount requested, due to the
///           manner in which comment records are allocated in an PCK
///           file. However, the amount of space set aside for comments
///           will always be at least the amount that was requested.
///
///           The value of NCOMCH should be greater than or equal to
///           zero, i.e., 0 <= NCOMCH. A negative value, should one
///           occur, will be assumed to be zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the handle of the opened PCK file. If an error occurs
///           when opening the file, the value of this variable should
///           not be used, as it will not represent a valid handle.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of NCOMCH is negative, a value of zero (0) will
///      be used for the number of comment characters to be set aside
///      for comments.
///
///  2)  If an error occurs while attempting to open a CK file the
///      value of HANDLE will not represent a valid file handle.
/// ```
///
/// # Files
///
/// ```text
///  See NAME and HANDLE.
/// ```
///
/// # Particulars
///
/// ```text
///  Open a new PCK file, reserving room for comments if requested.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Suppose that you have sets of Chebyshev polynomial
///     coefficients in an array pertaining to the orientation of
///     the Moon body-fixed frame with the frame class ID 301
///     relative to the J2000 reference frame, and want
///     to put these into a type 2 segment PCK file. The following
///     example could be used to add one new type 2 segment. To add
///     multiple segments, put the call to PCKW02 in a loop.
///
///
///     Example code begins here.
///
///
///           PROGRAM PCKOPN_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         FNAME
///           PARAMETER           ( FNAME = 'pckopn_ex1.bpc' )
///
///           INTEGER               BODY
///           PARAMETER           ( BODY   = 301  )
///
///           INTEGER               IIDLEN
///           PARAMETER           ( IIDLEN = 40   )
///
///           INTEGER               POLYDG
///           PARAMETER           ( POLYDG = 9    )
///
///           INTEGER               SZCDAT
///           PARAMETER           ( SZCDAT = 60   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(IIDLEN)    IFNAME
///           CHARACTER*(IIDLEN)    SEGID
///
///           DOUBLE PRECISION      BTIME
///           DOUBLE PRECISION      CDATA  ( SZCDAT )
///           DOUBLE PRECISION      FIRST
///           DOUBLE PRECISION      INTLEN
///           DOUBLE PRECISION      LAST
///
///           INTEGER               HANDLE
///           INTEGER               N
///           INTEGER               NRESVC
///
///     C
///     C     Set the input data: RA/DEC/W coefficients,
///     C     begin time for the first record, start/end times
///     C     for the segment, length of the time covered by
///     C     each record, and number of logical records.
///     C
///     C     CDATA contains the RA/DEC/W coefficients: first the
///     C     the POLYDEG + 1 for the RA first record, then the
///     C     POLYDEG + 1 for the DEC first record, then the
///     C     POLYDEG +1 for W first record, then the POLYDEG + 1
///     C     for the RA second record, and so on.
///     C
///           DATA                  CDATA /
///          .   -5.4242086033301107D-002, -5.2241405162792561D-005,
///          .    8.9751456289930307D-005, -1.5288696963234620D-005,
///          .    1.3218870864581395D-006,  5.9822156790328180D-007,
///          .   -6.5967702052551211D-008, -9.9084309118396298D-009,
///          .    4.9276055963541578D-010,  1.1612267413829385D-010,
///          .    0.42498898565916610D0,    1.3999219324235620D-004,
///          .   -1.8855140511098865D-005, -2.1964684808526649D-006,
///          .    1.4229817868138752D-006, -1.6991716166847001D-007,
///          .   -3.4824688140649506D-008,  2.9208428745895990D-009,
///          .    4.4217757657060300D-010, -3.9211207055305402D-012,
///          .    2565.0633504619473D0,     0.92003769451305328D0,
///          .   -8.0503797901914501D-005,  1.1960860244433900D-005,
///          .   -1.2237900518372542D-006, -5.3651349407824562D-007,
///          .    6.0843372260403005D-008,  9.0211287487688797D-009,
///          .   -4.6460429330339309D-010, -1.0446918704281774D-010,
///          .   -5.3839796353225056D-002,  4.3378021974424991D-004,
///          .    4.8130091384819459D-005, -1.2283066272873327D-005,
///          .   -5.4099296265403208D-006, -4.4237368347319652D-007,
///          .    1.3004982445546169D-007,  1.9017128275284284D-008,
///          .   -7.0368223839477803D-011, -1.7119414526133175D-010,
///          .    0.42507987850614548D0,   -7.1844899448557937D-005,
///          .   -5.1052122872412865D-005, -8.9810401387721321D-006,
///          .   -1.4611718567948972D-007,  4.0883847771062547D-007,
///          .    4.6812854485029333D-008, -4.5698075960784951D-009,
///          .   -9.8679875320349531D-010, -7.9392503778178240D-011,
///          .    2566.9029069934054D0,     0.91952244801740568D0,
///          .   -6.0426151041179828D-005,  1.0850559330577959D-005,
///          .    5.1756033678137497D-006,  4.2127585555214782D-007,
///          .   -1.1774737441872970D-007, -1.7397191490163833D-008,
///          .    5.8908810244396165D-012,  1.4594279337955166D-010 /
///
///           FIRST  =   -43200.D0
///           LAST   =  1339200.D0
///           BTIME  =  FIRST
///           INTLEN =   691200.D0
///           N      =  2
///
///     C
///     C     Open a new PCK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test PCK/Created 04-SEP-2019'
///
///           CALL PCKOPN ( FNAME, IFNAME, NRESVC, HANDLE    )
///
///     C
///     C     Create a segment identifier.
///     C
///           SEGID = 'MY_SAMPLE_PCK_TYPE_2_SEGMENT'
///
///     C
///     C     Write the segment.
///     C
///           CALL PCKW02 (  HANDLE, BODY,  'J2000',
///          .               FIRST,  LAST,   SEGID,   INTLEN,
///          .               N,      POLYDG, CDATA,   BTIME)
///
///     C
///     C     Close the file.
///     C
///           CALL PCKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new PCK type 2 exists in
///     the output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.2, 04-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section. Added complete code example
///         based on existing fragment.
///
/// -    SPICELIB Version 2.0.1, 03-JAN-2014 (EDW)
///
///         Minor edits to $Procedure; clean trailing whitespace.
///         Corrected order of header sections to conform to NAIF
///         standard.
///
/// -    SPICELIB Version 2.0.0, 09-NOV-2006 (NJB)
///
///         Routine has been upgraded to support comment
///         area allocation using NCOMCH.
///
/// -    SPICELIB Version 1.0.0, 26-JAN-1995 (KRG)
/// ```
pub fn pckopn(
    ctx: &mut SpiceContext,
    name: &str,
    ifname: &str,
    ncomch: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    PCKOPN(
        name.as_bytes(),
        ifname.as_bytes(),
        ncomch,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKOPN ( PCK, open new file )
pub fn PCKOPN(
    NAME: &[u8],
    IFNAME: &[u8],
    NCOMCH: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut NCOMR: i32 = 0;

    //
    // SPICELIB functions
    //
    //
    // Local parameters
    //

    //
    // DAF ND and NI values for PCK files.
    //

    //
    // Length of a DAF comment record, in characters.
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PCKOPN", ctx)?;
    //
    // Compute the number of comment records that we want to allocate, if
    // the number of comment characters requested is greater than zero,
    // we always allocate an extra record to account for the end of line
    // marks in the comment area.
    //
    if (NCOMCH > 0) {
        NCOMR = (((NCOMCH - 1) / MXCREC) + 1);
    } else {
        NCOMR = 0;
    }

    //
    // Just do it. All of the error handling is taken care of for us.
    //
    DAFONW(NAME, TYPE, ND, NI, IFNAME, NCOMR, HANDLE, ctx)?;

    if FAILED(ctx) {
        //
        // If we failed, make sure that HANDLE does not contain a value
        // that represents a valid DAF file handle.
        //
        *HANDLE = 0;
    }

    CHKOUT(b"PCKOPN", ctx)?;
    Ok(())
}
