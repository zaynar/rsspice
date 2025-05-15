//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CHR: i32 = 1;
const DP: i32 = 2;
const INT: i32 = 3;
const TIME: i32 = 4;
const FPARSZ: i32 = 1;
const SGTIDX: i32 = 1;
const NWC: i32 = 1024;

/// EK, open new file
///
/// Open a new E-kernel file and prepare the file for writing.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [TIME](crate::required_reading::time)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of EK file.
///  IFNAME     I   Internal file name.
///  NCOMCH     I   The number of characters to reserve for comments.
///  HANDLE     O   Handle attached to new EK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new E-kernel file to be created.
///
///  IFNAME   is the internal file name of a new E-kernel. The
///           internal file name may be up to 60 characters in
///           length.
///
///  NCOMCH   is the amount of space, measured in characters, to
///           be allocated in the comment area when the new EK
///           file is created. It is not necessary to allocate
///           space in advance in order to add comments, but
///           doing so may greatly increase the efficiency with
///           which comments may be added. Making room for
///           comments after data has already been added to the
///           file involves moving the data, and thus is slower.
///
///           NCOMCH must be greater than or equal to zero.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the EK handle of the file designated by FNAME.
///           This handle is used to identify the file to other
///           EK routines.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of DAS files that a user can
///           have open simultaneously. This includes any files used
///           by the DAS system.
///
///           See the include file das.inc for the actual value of
///           this parameter.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If NCOMCH is less than zero, the error SPICE(INVALIDCOUNT)
///      is signaled. No file will be created.
///
///  2)  If IFNAME is invalid, an error is signaled by a routine in the
///      call tree of this routine.
///
///  3)  If the indicated file cannot be opened, an error is signaled
///      by a routine in the call tree of this routine. The new file
///      will be deleted.
///
///  4)  If an I/O error occurs while reading or writing the indicated
///      file, the error is signaled by a routine in the call tree of
///      this routine.
/// ```
///
/// # Files
///
/// ```text
///  See the EK Required Reading ek.req for a discussion of the EK file
///  format.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine operates by side effects: it opens and prepares
///  an EK for addition of data.
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
///  1) This example demonstrates how to open a new EK file, creating
///     the two segments described below, without any records.
///
///     The EK file will contain two segments, the first containing
///     the DATAORDERS table and the second containing the DATAITEMS
///     table.
///
///     The E-kernel DATAORDERS table called consists of the set of
///     columns listed below:
///
///        DATAORDERS
///
///           Column Name     Data Type
///           -----------     ---------
///           ORDER_ID        INTEGER
///           CUSTOMER_ID     INTEGER
///           LAST_NAME       CHARACTER*(*)
///           FIRST_NAME      CHARACTER*(*)
///           ORDER_DATE      TIME
///           COST            DOUBLE PRECISION
///
///     The columns of the DATAITEMS table are shown below:
///
///        DATAITEMS
///
///           Column Name     Data Type
///           -----------     ---------
///           ITEM_ID         INTEGER
///           ORDER_ID        INTEGER
///           ITEM_NAME       CHARACTER*(*)
///           DESCRIPTION     CHARACTER*(*)
///           PRICE           DOUBLE PRECISION
///
///     Note that it is not necessary to populate the first segment
///     with data before starting the second segment, or before
///     closing the EK.
///
///
///     Example code begins here.
///
///
///           PROGRAM EKOPN_EX1
///           IMPLICIT NONE
///
///     C
///     C     Include the EK Column Name Size (CNAMSZ)
///     C
///           INCLUDE 'ekcnamsz.inc'
///
///     C
///     C     Local parameters
///     C
///           CHARACTER*(*)         EKNAME
///           PARAMETER           ( EKNAME  = 'ekopn_ex1.bdb' )
///
///           INTEGER               DECLEN
///           PARAMETER           ( DECLEN = 200 )
///
///           INTEGER               NAMLEN
///           PARAMETER           ( NAMLEN = 40  )
///
///           INTEGER               NCOLS
///           PARAMETER           ( NCOLS  = 6   )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(DECLEN)    CDECLS ( NCOLS )
///           CHARACTER*(CNAMSZ)    CNAMES ( NCOLS )
///           CHARACTER*(NAMLEN)    IFNAME
///
///           INTEGER               HANDLE
///           INTEGER               NRESVC
///           INTEGER               SEGNO
///
///     C
///     C     Open a new EK file.  For simplicity, we will not
///     C     reserve any space for the comment area, so the
///     C     number of reserved comment characters is zero.
///     C     The variable IFNAME is the internal file name.
///     C
///           NRESVC  =  0
///           IFNAME  =  'Test EK/Created 01-JUN-2019'
///
///           CALL EKOPN ( EKNAME, IFNAME, NRESVC, HANDLE )
///
///     C
///     C     Set up the table and column names and declarations
///     C     for the DATAORDERS segment.  We'll index all of
///     C     the columns.  All columns are scalar, so we omit
///     C     the size declaration.  Only the COST column may take
///     C     null values.
///     C
///           CNAMES(1) =  'ORDER_ID'
///           CDECLS(1) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(2) =  'CUSTOMER_ID'
///           CDECLS(2) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(3) =  'LAST_NAME'
///           CDECLS(3) =  'DATATYPE = CHARACTER*(*), ' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(4) =  'FIRST_NAME'
///           CDECLS(4) =  'DATATYPE = CHARACTER*(*), ' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(5) =  'ORDER_DATE'
///           CDECLS(5) =  'DATATYPE = TIME, INDEXED  = TRUE'
///
///           CNAMES(6) =  'COST'
///           CDECLS(6) =  'DATATYPE = DOUBLE PRECISION,' //
///          .             'INDEXED  = TRUE,'             //
///          .             'NULLS_OK = TRUE'
///
///
///     C
///     C     Start the first segment. Since we have no data for this
///     C     segment, start the segment by just defining the new
///     C     segment's schema.
///     C
///           CALL EKBSEG ( HANDLE, 'DATAORDERS', 6,
///          .              CNAMES, CDECLS,       SEGNO )
///
///     C
///     C     At this point, the second segment could be
///     C     created by an analogous process.  In fact, the
///     C     second segment could be created at any time; it is
///     C     not necessary to populate the first segment with
///     C     data before starting the second segment.
///     C
///     C     Set up the table and column names and declarations
///     C     for the DATAITEMS segment.  We'll index all of
///     C     the columns.  All columns are scalar, so we omit
///     C     the size declaration.
///     C
///           CNAMES(1) =  'ITEM_ID'
///           CDECLS(1) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(2) =  'ORDER_ID'
///           CDECLS(2) =  'DATATYPE = INTEGER, INDEXED = TRUE'
///
///           CNAMES(3) =  'ITEM_NAME'
///           CDECLS(3) =  'DATATYPE = CHARACTER*(*),' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(4) =  'DESCRIPTION'
///           CDECLS(4) =  'DATATYPE = CHARACTER*(*),' //
///          .             'INDEXED  = TRUE'
///
///           CNAMES(5) =  'PRICE'
///           CDECLS(5) =  'DATATYPE = DOUBLE PRECISION,' //
///          .             'INDEXED  = TRUE'
///
///
///     C
///     C     Start the new segment. Since we have no data for this
///     C     segment, start the segment by just defining the new
///     C     segment's schema.
///     C
///           CALL EKBSEG ( HANDLE, 'DATAITEMS', 5,
///          .              CNAMES, CDECLS,      SEGNO )
///
///     C
///     C     Close the file by a call to EKCLS.
///     C
///           CALL EKCLS ( HANDLE )
///
///           END
///
///
///     When this program is executed, no output is presented on
///     screen. After run completion, a new EK file exists in the
///     output directory.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  No more than FTSIZE DAS files may be opened simultaneously.
///      See the include file das.inc for the value of FTSIZE.
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
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example, and updated $Restrictions and
///         $Parameters sections.
///
/// -    SPICELIB Version 1.0.0, 26-SEP-1995 (NJB)
/// ```
pub fn ekopn(
    ctx: &mut SpiceContext,
    fname: &str,
    ifname: &str,
    ncomch: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    EKOPN(
        fname.as_bytes(),
        ifname.as_bytes(),
        ncomch,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure EKOPN ( EK, open new file )
pub fn EKOPN(
    FNAME: &[u8],
    IFNAME: &[u8],
    NCOMCH: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut BASE: i32 = 0;
    let mut NCR: i32 = 0;
    let mut P: i32 = 0;

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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"EKOPN", ctx)?;
    }

    //
    // Check the comment character count.
    //
    if (NCOMCH < 0) {
        SETMSG(
            b"The number of reserved comment characters must be non-negative but was #.",
            ctx,
        );
        ERRINT(b"#", NCOMCH, ctx);
        SIGERR(b"SPICE(INVALIDCOUNT)", ctx)?;
        CHKOUT(b"EKOPN", ctx)?;
        return Ok(());
    }

    //
    // A new DAS file is a must.  The file type is EK.
    // Reserve enough comment records to accommodate the requested
    // number of comment characters.
    //
    NCR = (((NWC + NCOMCH) - 1) / NWC);

    DASONW(FNAME, b"EK", IFNAME, NCR, HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKOPN", ctx)?;
        return Ok(());
    }

    //
    // Initialize the file for paged access.  The EK architecture
    // code is automatically set by the paging initialization routine.
    //
    ZZEKPGIN(*HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"EKOPN", ctx)?;
        return Ok(());
    }

    //
    // Allocate the first integer page for the file's metadata.  We
    // don't need to examine the page number; it's 1.
    //
    ZZEKPGAN(*HANDLE, INT, &mut P, &mut BASE, ctx)?;

    //
    // Initialize a new tree.  This tree will point to the file's
    // segments.
    //
    ZZEKTRIT(*HANDLE, &mut P, ctx)?;

    //
    // Save the segment pointer's root page number.
    //
    DASUDI(*HANDLE, (BASE + SGTIDX), (BASE + SGTIDX), &[P], ctx)?;

    //
    // That's it.  We're ready to add data to the file.
    //
    CHKOUT(b"EKOPN", ctx)?;
    Ok(())
}
