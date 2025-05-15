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

/// DSK, open new file
///
/// Open a new DSK file for subsequent write operations.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
/// * [DSK](crate::required_reading::dsk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FNAME      I   Name of a DSK file to be opened.
///  IFNAME     I   Internal file name.
///  NCOMCH     I   Number of comment characters to allocate.
///  HANDLE     O   Handle assigned to the opened DSK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a new DSK file to be created. The
///           file will be left opened for write access.
///
///  IFNAME   is the internal file name for the new file. The name
///           may contain as many as 60 characters. All characters
///           of IFNAME should be printing characters (ASCII codes
///           32-126 decimal). This name should uniquely identify
///           the file.
///
///  NCOMCH   is the number of comment characters to allocate.
///           Allocating comment characters at file creation time
///           may reduce the likelihood of having to expand the
///           comment area later.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is the file handle associated with the file. This
///           handle is used to identify the file in subsequent
///           calls to other DSK routines.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input filename is blank, an error is signaled by a
///      routine in the call tree of this routine. No file will be
///      created.
///
///  2)  If the specified file cannot be opened without exceeding the
///      maximum allowed number of open DAS files, an error is signaled
///      by a routine in the call tree of this routine. No file will be
///      created.
///
///  3)  If the file cannot be opened properly, an error is signaled by
///      a routine in the call tree of this routine. No file will be
///      created.
///
///  4)  If the initial records in the file cannot be written, an error
///      is signaled by a routine in the call tree of this routine. No
///      file will be created.
///
///  5)  If no logical units are available, an error is signaled by a
///      routine in the call tree of this routine. No file will be
///      created.
///
///  6)  If the internal file name contains nonprinting characters
///      (ASCII codes decimal 0-31 and 127-255), an error is signaled
///      by a routine in the call tree of this routine. No file will be
///      created.
///
///  7)  If the number of comment characters allocated NCOMCH is
///      negative, an error is signaled by a routine in the call
///      tree of this routine. No file will be created.
/// ```
///
/// # Files
///
/// ```text
///  See argument FNAME.
/// ```
///
/// # Particulars
///
/// ```text
///  DSK files are built using the DLA low-level format and
///  the DAS architecture; DLA files are a specialized type of DAS
///  file in which data are organized as a doubly linked list of
///  segments. Each segment's data belong to contiguous components of
///  character, double precision, and integer type.
///
///  This routine creates a new DSK file and sets the type of the
///  file to the mnemonic code passed to it.
///
///  DSK files created by this routine have initialized file records.
///  The ID word in a DSK file record has the form
///
///     DAS/DSK
///
///  where the characters following the slash are supplied by the
///  caller of this routine.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Create a new DSK file, using an internal file name that
///      attempts to serve as an unique identifier. No room for
///      comments will be reserved.
///
///         FNAME  =  'TEST.DSK'
///         IFNAME =  'TEST.DSK/NAIF/NJB/20-OCT-2006/14:37:00'
///         NCOMCH =   0
///
///         CALL DSKOPN ( FNAME, IFNAME, NCOMCH, HANDLE )
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
/// -    SPICELIB Version 1.0.1, 02-JUL-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 08-FEB-2017 (NJB)
///
///         Corrected a few header typos.
///
///         29-APR-2010 (NJB)
///
///            Now passes NCOMCH to DLAOPN.
///
///         08-OCT-2009 (NJB)
///
///            Updated header.
///
///         20-OCT-2006 (NJB)
/// ```
pub fn dskopn(
    ctx: &mut SpiceContext,
    fname: &str,
    ifname: &str,
    ncomch: i32,
    handle: &mut i32,
) -> crate::Result<()> {
    DSKOPN(
        fname.as_bytes(),
        ifname.as_bytes(),
        ncomch,
        handle,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DSKOPN ( DSK, open new file )
pub fn DSKOPN(
    FNAME: &[u8],
    IFNAME: &[u8],
    NCOMCH: i32,
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DSKOPN", ctx)?;

    DLAOPN(FNAME, b"DSK", IFNAME, NCOMCH, HANDLE, ctx)?;

    CHKOUT(b"DSKOPN", ctx)?;
    Ok(())
}
