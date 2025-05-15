//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;

/// PCK, coverage
///
/// Find the coverage window for a specified reference frame in a
/// specified binary PCK file.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [DAF](crate::required_reading::daf)
/// * [PCK](crate::required_reading::pck)
/// * [TIME](crate::required_reading::time)
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  PCKFNM     I   Name of PCK file.
///  IDCODE     I   Class ID code of PCK reference frame.
///  COVER     I-O  Window giving coverage in PCKFNM for IDCODE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  PCKFNM   is the name of a binary PCK file.
///
///  IDCODE   is the integer frame class ID code of a PCK reference
///           frame for which data are expected to exist in the
///           specified PCK file.
///
///  COVER    is an initialized SPICE window data structure. COVER
///           optionally may contain coverage data on input; on output,
///           the data already present in COVER will be combined with
///           coverage found for the reference frame designated by
///           IDCODE in the file PCKFNM.
///
///           If COVER contains no data on input, its size and
///           cardinality still must be initialized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COVER    is a SPICE window data structure which represents the
///           merged coverage for the reference frame having frame
///           class ID IDCODE. This is the set of time intervals for
///           which data for IDCODE are present in the file PCKFNM,
///           merged with the set of time intervals present in COVER on
///           input. The merged coverage is represented as the union of
///           one or more disjoint time intervals. The window COVER
///           contains the pairs of endpoints of these intervals.
///
///           The interval endpoints contained in COVER are ephemeris
///           times, expressed as seconds past J2000 TDB.
///
///           See the $Examples section below for a complete example
///           program showing how to retrieve the endpoints from COVER.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file has transfer format, the error
///      SPICE(INVALIDFORMAT) is signaled.
///
///  2)  If the input file is not a transfer file but has architecture
///      other than DAF, the error SPICE(INVALIDARCHTYPE) is signaled.
///
///  3)  If the input file is a binary DAF file of type other than PCK,
///      the error SPICE(INVALIDFILETYPE) is signaled.
///
///  4)  If the PCK file cannot be opened or read, an error is signaled
///      by a routine in the call tree of this routine. The output
///      window will not be modified.
///
///  5)  If the size of the output window argument COVER is
///      insufficient to contain the actual number of intervals in the
///      coverage window for IDCODE, an error is signaled by a routine
///      in the call tree of this routine.
/// ```
///
/// # Files
///
/// ```text
///  This routine reads a PCK file.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides an API via which applications can determine
///  the coverage a specified PCK file provides for a specified PCK
///  class reference frame.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for these examples may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) This example demonstrates combined usage of PCKCOV and the
///     related PCK utility PCKFRM.
///
///     Display the coverage for each object in a specified PCK file.
///     Find the set of objects in the file; for each object, find
///     and display the coverage.
///
///
///     Example code begins here.
///
///
///           PROGRAM PCKCOV_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///           INTEGER               CARDI
///     C
///     C     Local parameters
///     C
///     C
///     C     Declare the coverage window.  Make enough room
///     C     for MAXIV intervals.
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               MAXIV
///           PARAMETER           ( MAXIV  = 1000 )
///
///           INTEGER               WINSIZ
///           PARAMETER           ( WINSIZ = 2 * MAXIV )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 50 )
///
///           INTEGER               MAXFRM
///           PARAMETER           ( MAXFRM = 1000 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    LSK
///           CHARACTER*(FILSIZ)    PCKFNM
///           CHARACTER*(TIMLEN)    TIMSTR
///
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      COVER ( LBCELL : WINSIZ )
///           DOUBLE PRECISION      E
///
///           INTEGER               I
///           INTEGER               IDS   ( LBCELL : MAXFRM )
///           INTEGER               J
///           INTEGER               NIV
///
///
///     C
///     C     Load a leapseconds kernel for output time conversion.
///     C     PCKCOV itself does not require a leapseconds kernel.
///     C
///           CALL PROMPT ( 'Name of leapseconds kernel > ', LSK )
///           CALL FURNSH ( LSK )
///
///     C
///     C     Get name of PCK file.
///     C
///           CALL PROMPT ( 'Name of PCK file           > ', PCKFNM )
///
///     C
///     C     Initialize the set IDS.
///     C
///           CALL SSIZEI ( MAXFRM, IDS )
///
///     C
///     C     Initialize the window COVER.
///     C
///           CALL SSIZED ( WINSIZ, COVER )
///
///     C
///     C     Find the set of frames in the PCK file.
///     C
///           CALL PCKFRM ( PCKFNM, IDS )
///
///     C
///     C     We want to display the coverage for each frame.  Loop
///     C     over the contents of the ID code set, find the coverage
///     C     for each item in the set, and display the coverage.
///     C
///           DO I = 1, CARDI( IDS )
///
///     C
///     C        Find the coverage window for the current frame.
///     C        Empty the coverage window each time so
///     C        we don't include data for the previous frame.
///     C
///              CALL SCARDD ( 0,      COVER )
///              CALL PCKCOV ( PCKFNM, IDS(I), COVER )
///
///     C
///     C        Get the number of intervals in the coverage
///     C        window.
///     C
///              NIV = WNCARD( COVER )
///
///     C
///     C        Display a simple banner.
///     C
///              WRITE (*,*) '========================================'
///              WRITE (*,*) 'Coverage for reference frame ', IDS(I)
///
///     C
///     C        Convert the coverage interval start and stop
///     C        times to TDB calendar strings.
///     C
///              DO J = 1, NIV
///
///     C
///     C           Get the endpoints of the Jth interval.
///     C
///                 CALL WNFETD ( COVER, J, B, E )
///
///     C
///     C           Convert the endpoints to TDB calendar
///     C           format time strings and display them.
///     C
///                 CALL TIMOUT ( B,
///          .                    'YYYY MON DD HR:MN:SC.### ' //
///          .                    '(TDB) ::TDB',
///          .                    TIMSTR                        )
///                 WRITE (*,*) ' '
///                 WRITE (*,*) 'Interval: ', J
///                 WRITE (*,*) 'Start:    ', TIMSTR
///
///                 CALL TIMOUT ( E,
///          .                    'YYYY MON DD HR:MN:SC.### ' //
///          .                    '(TDB) ::TDB',
///          .                    TIMSTR                        )
///                 WRITE (*,*) 'Stop:     ', TIMSTR
///                 WRITE (*,*) ' '
///
///              END DO
///
///              WRITE (*,*) '========================================'
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the LSK file named naif0012.tls, and the PCK
///     file named earth_720101_070426.bpc, the output was:
///
///
///     Name of leapseconds kernel > naif0012.tls
///     Name of PCK file           > earth_720101_070426.bpc
///      ========================================
///      Coverage for reference frame         3000
///
///      Interval:            1
///      Start:    1962 JAN 20 00:00:41.184 (TDB)
///      Stop:     2007 APR 26 00:01:05.185 (TDB)
///
///      ========================================
///
///
///  2) Find the coverage for the frame designated by IDCODE
///     provided by the set of PCK files loaded via a metakernel.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: pckcov_ex2.tm
///
///        This meta-kernel is intended to support operation of SPICE
///        example programs. The kernels shown here should not be
///        assumed to contain adequate or correct versions of data
///        required by SPICE-based user applications.
///
///        In order for an application to use this meta-kernel, the
///        kernels referenced here must be present in the user's
///        current working directory.
///
///        The names and contents of the kernels referenced
///        by this meta-kernel are as follows:
///
///           File name                        Contents
///           ---------                        --------
///           earth_720101_070426.bpc          Earth historical
///                                            binary PCK
///           earth_070425_370426_predict.bpc  Earth predicted
///                                            binary PCK
///           naif0012.tls                     Leapseconds
///
///
///        \begindata
///
///        KERNELS_TO_LOAD = ( 'earth_070425_370426_predict.bpc',
///                            'earth_720101_070426.bpc',
///                            'naif0012.tls'                    )
///
///
///        \begintext
///
///        End of meta-kernel.
///
///
///     Example code begins here.
///
///
///           PROGRAM PCKCOV_EX2
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               WNCARD
///
///     C
///     C     Local parameters
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5 )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               LNSIZE
///           PARAMETER           ( LNSIZE = 80 )
///
///           INTEGER               MAXCOV
///           PARAMETER           ( MAXCOV = 100000 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 50 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    FILE
///           CHARACTER*(LNSIZE)    IDCH
///           CHARACTER*(FILSIZ)    META
///           CHARACTER*(FILSIZ)    SOURCE
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(LNSIZE)    TYPE
///
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      COVER  ( LBCELL : 2*MAXCOV )
///           DOUBLE PRECISION      E
///
///           INTEGER               COUNT
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IDCODE
///           INTEGER               NIV
///
///           LOGICAL               FOUND
///
///     C
///     C     Prompt for the metakernel name; load the metakernel.
///     C     The metakernel lists the PCK files whose coverage
///     C     for IDCODE we'd like to determine.  The metakernel
///     C     must also specify a leapseconds kernel.
///     C
///           CALL PROMPT ( 'Enter name of metakernel > ', META )
///
///           CALL FURNSH ( META )
///
///     C
///     C     Get the ID code of interest.
///     C
///           CALL PROMPT ( 'Enter ID code            > ', IDCH )
///
///           CALL PRSINT ( IDCH,  IDCODE )
///
///     C
///     C     Initialize the coverage window.
///     C
///           CALL SSIZED ( MAXCOV, COVER )
///
///     C
///     C     Find out how many kernels are loaded.  Loop over the
///     C     kernels:  for each loaded PCK file, add its coverage
///     C     for IDCODE, if any, to the coverage window.
///     C
///           CALL KTOTAL ( 'PCK', COUNT )
///
///           DO I = 1, COUNT
///
///              CALL KDATA  ( I,       'PCK',   FILE,  TYPE,
///          .                 SOURCE,  HANDLE,  FOUND       )
///
///              CALL PCKCOV ( FILE,    IDCODE,  COVER )
///
///           END DO
///
///     C
///     C     Display results.
///     C
///     C     Get the number of intervals in the coverage
///     C     window.
///     C
///           NIV = WNCARD( COVER )
///
///     C
///     C     Display a simple banner.
///     C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Coverage for frame ', IDCODE
///
///     C
///     C     Convert the coverage interval start and stop
///     C     times to TDB calendar strings.
///     C
///           DO I = 1, NIV
///
///     C
///     C        Get the endpoints of the Ith interval.
///     C
///              CALL WNFETD ( COVER, I, B, E )
///
///     C
///     C        Convert the endpoints to TDB calendar
///     C        format time strings and display them.
///     C
///              CALL TIMOUT ( B,
///          .                 'YYYY MON DD HR:MN:SC.### ' //
///          .                 '(TDB) ::TDB',
///          .                 TIMSTR                        )
///              WRITE (*,*) ' '
///              WRITE (*,*) 'Interval: ', I
///              WRITE (*,*) 'Start:    ', TIMSTR
///
///              CALL TIMOUT ( E,
///          .                 'YYYY MON DD HR:MN:SC.### ' //
///          .                 '(TDB) ::TDB',
///          .                 TIMSTR                        )
///              WRITE (*,*) 'Stop:     ', TIMSTR
///              WRITE (*,*) ' '
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the meta-kernel named pckcov_ex2.tm provided
///     above to find the coverage window for the ITRF93 frame using
///     its ID code, '3000', the output was:
///
///
///     Enter name of metakernel > pckcov_ex2.tm
///     Enter ID code            > 3000
///
///      Coverage for frame         3000
///
///      Interval:            1
///      Start:    1962 JAN 20 00:00:41.184 (TDB)
///      Stop:     2037 JUL 17 00:01:05.183 (TDB)
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If an error occurs while this routine is updating the window
///      COVER, the window may be corrupted.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 08-OCT-2021 (JDR) (NJB)
///
///         Changed input argument name "PCK" to "PCKFNM" for consistency
///         with other routines.
///
///         Bug fix: added call to FAILED after call to GETFAT.
///
///         Edited the header to comply with NAIF standard.
///         Added examples' solution.
///
///         Corrected short error message in entries #2 and #3 in
///         $Exceptions section.
///
/// -    SPICELIB Version 1.1.1, 03-JAN-2014 (NJB) (EDW)
///
///         Updated index entries.
///
///         Minor edits to $Procedure; clean trailing whitespace.
///
/// -    SPICELIB Version 1.0.0, 30-NOV-2007 (NJB)
/// ```
pub fn pckcov(
    ctx: &mut SpiceContext,
    pckfnm: &str,
    idcode: i32,
    cover: &mut [f64],
) -> crate::Result<()> {
    PCKCOV(pckfnm.as_bytes(), idcode, cover, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure PCKCOV ( PCK, coverage )
pub fn PCKCOV(
    PCKFNM: &[u8],
    IDCODE: i32,
    COVER: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COVER = DummyArrayMut::new(COVER, LBCELL..);
    let mut ARCH = [b' '; LNSIZE as usize];
    let mut KERTYP = [b' '; LNSIZE as usize];
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=(ND + (NI / 2)));
    let mut HANDLE: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
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

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"PCKCOV", ctx)?;

    //
    // See whether GETFAT thinks we've got a binary PCK file.
    // If not, indicate the specific problem.
    //
    GETFAT(PCKFNM, &mut ARCH, &mut KERTYP, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"PCKCOV", ctx)?;
        return Ok(());
    }

    if fstr::eq(&ARCH, b"XFR") {
        SETMSG(b"Input file # has architecture #. The file must be a binary PCK file to be readable by this routine.  If the input file is an PCK file in transfer format, run TOBIN on the file to convert it to binary format.", ctx);
        ERRCH(b"#", PCKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"PCKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&ARCH, b"DAF") {
        SETMSG(b"Input file # has architecture #. The file must be a binary PCK file to be readable by this routine.  Binary PCK files have DAF architecture.  If you expected the file to be a binary PCK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", PCKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDARCHTYPE)", ctx)?;
        CHKOUT(b"PCKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&KERTYP, b"PCK") {
        SETMSG(b"Input file # has file type #. The file must be a binary PCK file to be readable by this routine. If you expected the file to be a binary PCK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", PCKFNM, ctx);
        ERRCH(b"#", &KERTYP, ctx);
        SIGERR(b"SPICE(INVALIDFILETYPE)", ctx)?;
        CHKOUT(b"PCKCOV", ctx)?;
        return Ok(());
    }

    //
    // Open the file for reading.
    //
    DAFOPR(PCKFNM, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"PCKCOV", ctx)?;
        return Ok(());
    }

    //
    // We will examine each segment descriptor in the file, and
    // we'll update our coverage bounds according to the data found
    // in these descriptors.
    //
    // Start a forward search.
    //
    DAFBFS(HANDLE, ctx)?;

    //
    // Find the next DAF array.
    //
    DAFFNA(&mut FOUND, ctx)?;

    while (FOUND && !FAILED(ctx)) {
        //
        // Fetch and unpack the segment descriptor.
        //
        DAFGS(DESCR.as_slice_mut(), ctx)?;
        DAFUS(
            DESCR.as_slice(),
            ND,
            NI,
            DC.as_slice_mut(),
            IC.as_slice_mut(),
        );

        if (IC[1] == IDCODE) {
            //
            // This segment is for the body of interest.  Insert the
            // coverage bounds into the coverage window.
            //
            WNINSD(DC[1], DC[2], COVER.as_slice_mut(), ctx)?;
        }

        DAFFNA(&mut FOUND, ctx)?;
    }

    //
    // Release the file.
    //
    DAFCLS(HANDLE, ctx)?;

    CHKOUT(b"PCKCOV", ctx)?;
    Ok(())
}
