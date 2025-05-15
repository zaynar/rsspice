//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const VERIDX: i32 = 1;
const LLBIDX: i32 = (VERIDX + 1);
const LLEIDX: i32 = (LLBIDX + 1);
const NULPTR: i32 = -1;
const BWDIDX: i32 = 1;
const FWDIDX: i32 = (BWDIDX + 1);
const IBSIDX: i32 = (FWDIDX + 1);
const ISZIDX: i32 = (IBSIDX + 1);
const DBSIDX: i32 = (ISZIDX + 1);
const DSZIDX: i32 = (DBSIDX + 1);
const CBSIDX: i32 = (DSZIDX + 1);
const CSZIDX: i32 = (CBSIDX + 1);
const DLADSZ: i32 = CSZIDX;
const FMTVER: i32 = 1000000;
const NCHREC: i32 = 1024;
const SRFIDX: i32 = 1;
const CTRIDX: i32 = (SRFIDX + 1);
const CLSIDX: i32 = (CTRIDX + 1);
const TYPIDX: i32 = (CLSIDX + 1);
const FRMIDX: i32 = (TYPIDX + 1);
const SYSIDX: i32 = (FRMIDX + 1);
const PARIDX: i32 = (SYSIDX + 1);
const NSYPAR: i32 = 10;
const MN1IDX: i32 = (PARIDX + NSYPAR);
const MX1IDX: i32 = (MN1IDX + 1);
const MN2IDX: i32 = (MX1IDX + 1);
const MX2IDX: i32 = (MN2IDX + 1);
const MN3IDX: i32 = (MX2IDX + 1);
const MX3IDX: i32 = (MN3IDX + 1);
const BTMIDX: i32 = (MX3IDX + 1);
const ETMIDX: i32 = (BTMIDX + 1);
const DSKDSZ: i32 = ETMIDX;
const SVFCLS: i32 = 1;
const GENCLS: i32 = 2;
const LATSYS: i32 = 1;
const CYLSYS: i32 = 2;
const RECSYS: i32 = 3;
const PDTSYS: i32 = 4;
pub const LBCELL: i32 = -5;
const TOKLEN: i32 = 4;

/// DSK, get object IDs
///
/// Find the set of body ID codes of all objects for which
/// topographic data are provided in a specified DSK file.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [DAS](crate::required_reading::das)
/// * [DSK](crate::required_reading::dsk)
/// * [SETS](crate::required_reading::sets)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  DSKFNM     I   Name of DSK file.
///  BODIDS    I-O  Set of ID codes of objects in DSK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  DSKFNM   is the name of a DSK file. This file will be opened for
///           read access by this routine.
///
///  BODIDS   is an initialized SPICE set data structure.
///
///           BODIDS optionally may contain a set of body ID codes on
///           input; on output, the data already present in BODIDS will
///           be combined with ID code set found for the file DSKFNM.
///
///           If BODIDS contains no data on input, its size and
///           cardinality still must be initialized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  BODIDS   is a SPICE set data structure that contains the union
///           of its contents upon input with the set of body ID codes
///           of segments in the indicated DSK file.
///
///           The elements of SPICE sets are unique; each ID code in
///           BODIDS appears only once, even if the DSK file contains
///           multiple segments for that ID code.
///
///           See the $Examples section below for a complete example
///           program showing how to retrieve the body and surface ID
///           codes from a DSK file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file has transfer format, the error
///      SPICE(INVALIDFORMAT) is signaled.
///
///  2)  If the input file is not a transfer file but has architecture
///      other than DAS, the error SPICE(INVALIDARCHTYPE) is signaled.
///
///  3)  If the input file is a binary DAS file of type other than DSK,
///      the error SPICE(INVALIDFILETYPE) is signaled.
///
///  4)  If the DSK file cannot be opened or read, an error is signaled
///      by a routine in the call tree of this routine.
///
///  5)  If the size of the output set argument BODIDS is insufficient
///      to contain the actual number of ID codes of objects covered by
///      the indicated DSK file, the error SPICE(CELLTOOSMALL) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument DSKFNM above.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides an API via which applications can determine
///  the set of objects for which there are topographic data in a
///  specified DSK file.
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
///  1) Display the coverage for each object in a specified DSK file.
///     Find the set of objects in the file. Loop over the contents
///     of the ID code set: find the surface ID for each item in the
///     set and display the surface ID.
///
///
///     Example code begins here.
///
///
///     C
///     C     Examine a DSK file and identify the set of
///     C     central bodies associated with the segments
///     C     in the file. For each body, find the
///     C     set of surfaces associated with that body.
///     C
///           PROGRAM DSKOBJ_EX1
///           IMPLICIT NONE
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               CARDI
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               MAXID
///           PARAMETER           ( MAXID  = 10000 )
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    DSKFNM
///
///           INTEGER               BODIDS ( LBCELL : MAXID )
///           INTEGER               I
///           INTEGER               J
///           INTEGER               SRFIDS ( LBCELL : MAXID )
///
///     C
///     C     Initialize body ID and surface ID cells.
///     C
///           CALL SSIZEI ( MAXID, BODIDS )
///           CALL SSIZEI ( MAXID, SRFIDS )
///
///     C
///     C     Prompt for the name of a DSK file.
///     C
///           CALL PROMPT ( 'Enter name of DSK file > ', DSKFNM )
///
///     C
///     C     Obtain body ID set for the DSK.
///     C
///           CALL DSKOBJ ( DSKFNM, BODIDS )
///
///           DO I = 1, CARDI( BODIDS )
///
///              WRITE (*,*) ' '
///              WRITE (*,*) 'Body ID:     ', BODIDS(I)
///     C
///     C        Get the surface IDs for the Ith body.
///     C
///              CALL DSKSRF ( DSKFNM, BODIDS(I), SRFIDS )
///
///              DO J = 1, CARDI( SRFIDS )
///                 WRITE (*,*) '   Surface ID: ', SRFIDS(J)
///              END DO
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the DSK file named phobos512.bds, the output
///     was:
///
///
///     Enter name of DSK file > phobos512.bds
///
///      Body ID:              401
///         Surface ID:          401
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If an error occurs while this routine is updating the set
///      BODIDS, the set may be corrupted.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 08-OCT-2021 (JDR) (NJB)
///
///         Changed input argument name "DSK" to "DSKFNM" for consistency
///         with other routines.
///
///         Bug fix: added call to FAILED after call to GETFAT.
///
///         Edited the header comments to comply with NAIF standard.
///         Changed the DKS file used in the code example's solution.
///         Added NAIF_IDS to the $Required_Reading section.
///
///         Corrected short error message in entries #2, #3 and #5 in
///         $Exceptions section.
///
/// -    SPICELIB Version 1.0.0, 22-AUG-2016 (NJB)
/// ```
pub fn dskobj(ctx: &mut SpiceContext, dskfnm: &str, bodids: &mut [i32]) -> crate::Result<()> {
    DSKOBJ(dskfnm.as_bytes(), bodids, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKOBJ ( DSK, get object IDs )
pub fn DSKOBJ(DSKFNM: &[u8], BODIDS: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut BODIDS = DummyArrayMut::new(BODIDS, LBCELL..);
    let mut ARCH = [b' '; TOKLEN as usize];
    let mut KERTYP = [b' '; TOKLEN as usize];
    let mut DSKDSC = StackArray::<f64, 24>::new(1..=DSKDSZ);
    let mut BID: i32 = 0;
    let mut DLADSC = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut HANDLE: i32 = 0;
    let mut NXTDSC = StackArray::<i32, 8>::new(1..=DLADSZ);
    let mut FOUND: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKOBJ", ctx)?;

    //
    // See whether GETFAT thinks we've got a DSK file.
    //
    GETFAT(DSKFNM, &mut ARCH, &mut KERTYP, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DSKOBJ", ctx)?;
        return Ok(());
    }

    if fstr::eq(&ARCH, b"XFR") {
        SETMSG(b"Input file # has architecture #. The file must be a binary DSK file to be readable by this routine. If the input file is an DSK file in transfer format, run TOBIN on the file to convert it to binary format.", ctx);
        ERRCH(b"#", DSKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"DSKOBJ", ctx)?;
        return Ok(());
    } else if fstr::ne(&ARCH, b"DAS") {
        SETMSG(b"Input file # has architecture #. The file must be a binary DSK file to be readable by this routine. Binary DSK files have DAS architecture. If you expected the file to be a binary DSK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", DSKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDARCHTYPE)", ctx)?;
        CHKOUT(b"DSKOBJ", ctx)?;
        return Ok(());
    } else if fstr::ne(&KERTYP, b"DSK") {
        SETMSG(b"Input file # has file type #. The file must be a binary DSK file to be readable by this routine. If you expected the file to be a binary DSK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", DSKFNM, ctx);
        ERRCH(b"#", &KERTYP, ctx);
        SIGERR(b"SPICE(INVALIDFILETYPE)", ctx)?;
        CHKOUT(b"DSKOBJ", ctx)?;
        return Ok(());
    }

    //
    // Open the DSK for read access; start a forward search.
    //
    DASOPR(DSKFNM, &mut HANDLE, ctx)?;
    DLABFS(HANDLE, NXTDSC.as_slice_mut(), &mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DSKOBJ", ctx)?;
        return Ok(());
    }

    while (FOUND && !FAILED(ctx)) {
        //
        // Get the DSK descriptor of the current segment.
        // This is where we'll find the body ID code.
        //
        MOVEI(NXTDSC.as_slice(), DLADSZ, DLADSC.as_slice_mut());

        DSKGD(HANDLE, DLADSC.as_slice(), DSKDSC.as_slice_mut(), ctx)?;
        //
        // The body ID is at location CTRIDX ("center index")
        // of the DSK descriptor.
        //
        // Append, rather than insert, the new ID. We'll
        // turn the cell into a set at the end of the loop.
        //
        // Before appending, make sure there's room in the cell
        // for another entry. We can't afford to let APPNDI
        // catch an out-of-room error, because we would lose
        // the ability to close the file.
        //
        BID = intrinsics::IDNINT(DSKDSC[CTRIDX]);

        if (CARDI(BODIDS.as_slice(), ctx)? == SIZEI(BODIDS.as_slice(), ctx)?) {
            //
            // We're going to signal an error. Close the DSK
            // first.
            //
            DSKCLS(HANDLE, false, ctx)?;

            SETMSG(
                b"Cannot append body ID # to cell while reading DSK file #. Cell size is #.",
                ctx,
            );
            ERRINT(b"#", BID, ctx);
            ERRCH(b"#", DSKFNM, ctx);
            ERRINT(b"#", SIZEI(BODIDS.as_slice(), ctx)?, ctx);
            SIGERR(b"SPICE(CELLTOOSMALL)", ctx)?;
            CHKOUT(b"DSKOBJ", ctx)?;
            return Ok(());
        }

        APPNDI(BID, BODIDS.as_slice_mut(), ctx)?;
        //
        // Fetch the DLA descriptor of the next segment.
        //
        DLAFNS(
            HANDLE,
            DLADSC.as_slice(),
            NXTDSC.as_slice_mut(),
            &mut FOUND,
            ctx,
        )?;
    }

    VALIDI(
        SIZEI(BODIDS.as_slice(), ctx)?,
        CARDI(BODIDS.as_slice(), ctx)?,
        BODIDS.as_slice_mut(),
        ctx,
    )?;

    DASCLS(HANDLE, ctx)?;

    CHKOUT(b"DSKOBJ", ctx)?;
    Ok(())
}
