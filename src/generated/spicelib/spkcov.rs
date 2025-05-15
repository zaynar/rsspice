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

/// SPK coverage
///
/// Find the coverage window for a specified ephemeris object in a
/// specified SPK file.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [DAF](crate::required_reading::daf)
/// * [SPK](crate::required_reading::spk)
/// * [TIME](crate::required_reading::time)
/// * [WINDOWS](crate::required_reading::windows)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SPKFNM     I   Name of SPK file.
///  IDCODE     I   ID code of ephemeris object.
///  COVER     I-O  Window giving coverage in SPKFNM for IDCODE.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SPKFNM   is the name of an SPK file.
///
///  IDCODE   is the integer ID code of an object for which ephemeris
///           data are expected to exist in the specified SPK file.
///
///  COVER    is an initialized SPICE window data structure. COVER
///           optionally may contain coverage data on input; on output,
///           the data already present in COVER will be combined with
///           coverage found for the object designated by IDCODE in the
///           file SPKFNM.
///
///           If COVER contains no data on input, its size and
///           cardinality still must be initialized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  COVER    is a SPICE window data structure which represents the
///           merged coverage for IDCODE. This is the set of time
///           intervals for which data for IDCODE are present in the
///           file SPKFNM, merged with the set of time intervals
///           present in COVER on input. The merged coverage is
///           represented as the union of one or more disjoint time
///           intervals. The window COVER contains the pairs of
///           endpoints of these intervals.
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
///  3)  If the input file is a binary DAF file of type other than SPK,
///      the error SPICE(INVALIDFILETYPE) is signaled.
///
///  4)  If the SPK file cannot be opened or read, an error is signaled
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
///  This routine reads an SPK file.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides an API via which applications can determine
///  the coverage a specified SPK file provides for a specified
///  ephemeris object.
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
///  1) This example demonstrates combined usage of SPKCOV and the
///     related SPK utility SPKOBJ.
///
///     Display the coverage for each object in a specified SPK file.
///     Find the set of objects in the file; for each object, find
///     and display the coverage.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKCOV_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions
///     C
///           INTEGER               CARDI
///           INTEGER               WNCARD
///     C
///     C     Local parameters
///     C
///     C
///     C     Declare the coverage window. Make enough room
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
///           INTEGER               MAXOBJ
///           PARAMETER           ( MAXOBJ = 1000 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(FILSIZ)    LSK
///           CHARACTER*(FILSIZ)    SPKFNM
///           CHARACTER*(TIMLEN)    TIMSTR
///
///           DOUBLE PRECISION      B
///           DOUBLE PRECISION      COVER ( LBCELL : WINSIZ )
///           DOUBLE PRECISION      E
///
///           INTEGER               I
///           INTEGER               IDS   ( LBCELL : MAXOBJ )
///           INTEGER               J
///           INTEGER               NIV
///
///
///     C
///     C     Load a leapseconds kernel for output time conversion.
///     C     SPKCOV itself does not require a leapseconds kernel.
///     C
///           CALL PROMPT ( 'Name of leapseconds kernel > ', LSK )
///           CALL FURNSH ( LSK )
///
///     C
///     C     Get name of SPK file.
///     C
///           CALL PROMPT ( 'Name of SPK file           > ', SPKFNM )
///
///     C
///     C     Initialize the set IDS.
///     C
///           CALL SSIZEI ( MAXOBJ, IDS )
///
///     C
///     C     Initialize the window COVER.
///     C
///           CALL SSIZED ( WINSIZ, COVER )
///
///     C
///     C     Find the set of objects in the SPK file.
///     C
///           CALL SPKOBJ ( SPKFNM, IDS )
///
///     C
///     C     We want to display the coverage for each object.  Loop
///     C     over the contents of the ID code set, find the coverage
///     C     for each item in the set, and display the coverage.
///     C
///           DO I = 1, CARDI( IDS )
///     C
///     C        Find the coverage window for the current
///     C        object. Empty the coverage window each time
///     C        so we don't include data for the previous object.
///     C
///              CALL SCARDD ( 0,      COVER )
///              CALL SPKCOV ( SPKFNM, IDS(I), COVER )
///
///     C
///     C        Get the number of intervals in the coverage
///     C        window.
///     C
///              NIV = WNCARD ( COVER )
///
///     C
///     C        Display a simple banner.
///     C
///              WRITE (*,*) '========================================'
///              WRITE (*,*) 'Coverage for object ', IDS(I)
///
///     C
///     C        Convert the coverage interval start and stop
///     C        times to TDB calendar strings.
///     C
///              DO J = 1, NIV
///     C
///     C           Get the endpoints of the Jth interval.
///     C
///                 CALL WNFETD ( COVER, J, B, E )
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
///     platform, using the LSK file named naif0012.tls and the SPK
///     file named mar097.bsp, the output was:
///
///
///     Name of leapseconds kernel > naif0012.tls
///     Name of SPK file           > mar097.bsp
///      ========================================
///      Coverage for object            3
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///      ========================================
///      Coverage for object            4
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///      ========================================
///      Coverage for object           10
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///      ========================================
///      Coverage for object          399
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///      ========================================
///      Coverage for object          401
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///      ========================================
///      Coverage for object          402
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///      ========================================
///      Coverage for object          499
///
///      Interval:            1
///      Start:    1900 JAN 04 00:00:41.184 (TDB)
///      Stop:     2100 JAN 01 00:01:07.183 (TDB)
///
///      ========================================
///
///
///  2) Find the coverage for the object designated by IDCODE
///     provided by the set of SPK files loaded via a metakernel.
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File: spkcov_ex2.tm
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
///           de430.bsp                        Planetary ephemeris
///           mar097.bsp                       Mars satellite ephemeris
///           naif0011.tls                     Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'de430.bsp',
///                               'mar097.bsp',
///                               'naif0011.tls' )
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKCOV_EX2
///           IMPLICIT NONE
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
///     C     The metakernel lists the SPK files whose coverage
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
///     C     kernels:  for each loaded SPK file, add its coverage
///     C     for IDCODE, if any, to the coverage window.
///     C
///           CALL KTOTAL ( 'SPK', COUNT )
///
///           DO I = 1, COUNT
///
///              CALL KDATA  ( I,       'SPK',   FILE,  TYPE,
///          .                 SOURCE,  HANDLE,  FOUND       )
///
///              CALL SPKCOV ( FILE,    IDCODE,  COVER )
///
///           END DO
///
///     C
///     C     Display results.
///     C
///     C     Get the number of intervals in the coverage
///     C     window.
///     C
///           NIV = WNCARD ( COVER )
///
///     C
///     C     Display a simple banner.
///     C
///           WRITE (*,*) ' '
///           WRITE (*,*) 'Coverage for object ', IDCODE
///
///     C
///     C     Convert the coverage interval start and stop
///     C     times to TDB calendar strings.
///     C
///           DO I = 1, NIV
///     C
///     C        Get the endpoints of the Ith interval.
///     C
///              CALL WNFETD ( COVER, I, B, E )
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
///     platform, using the meta-kernel named spkcov_ex2.tm provided
///     above to find the coverage window for the Earth using its ID
///     code, '399', the output was:
///
///
///     Enter name of metakernel > spkcov_ex2.tm
///     Enter ID code            > 399
///
///      Coverage for object          399
///
///      Interval:            1
///      Start:    1549 DEC 31 00:00:00.000 (TDB)
///      Stop:     2650 JAN 25 00:00:00.000 (TDB)
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
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 08-OCT-2021 (JDR) (NJB)
///
///         Changed input argument name "SPK" to "SPKFNM" for consistency
///         with other routines.
///
///         Bug fix: added call to FAILED after call to GETFAT.
///
///         Edited the header to comply with NAIF standard. Added
///         examples' solution and meta-kernel for example #2.
///
///         Corrected short error message in entries #2 and #3 in
///         $Exceptions section.
///
/// -    SPICELIB Version 1.0.2, 01-JUL-2014 (NJB)
///
///         Added new index entries.
///
/// -    SPICELIB Version 1.0.1, 30-NOV-2007 (NJB)
///
///         Corrected bug in first program in header $Examples section:
///         program now empties the coverage window prior to collecting
///         data for the current object. Updated examples to use WNCARD
///         rather than CARDD.
///
/// -    SPICELIB Version 1.0.0, 30-DEC-2004 (NJB)
/// ```
pub fn spkcov(
    ctx: &mut SpiceContext,
    spkfnm: &str,
    idcode: i32,
    cover: &mut [f64],
) -> crate::Result<()> {
    SPKCOV(spkfnm.as_bytes(), idcode, cover, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKCOV ( SPK coverage )
pub fn SPKCOV(
    SPKFNM: &[u8],
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

    CHKIN(b"SPKCOV", ctx)?;

    //
    // See whether GETFAT thinks we've got a binary SPK file.
    // If not, indicate the specific problem.
    //
    GETFAT(SPKFNM, &mut ARCH, &mut KERTYP, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKCOV", ctx)?;
        return Ok(());
    }

    if fstr::eq(&ARCH, b"XFR") {
        SETMSG(b"Input file # has architecture #. The file must be a binary SPK file to be readable by this routine.  If the input file is an SPK file in transfer format, run TOBIN on the file to convert it to binary format.", ctx);
        ERRCH(b"#", SPKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"SPKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&ARCH, b"DAF") {
        SETMSG(b"Input file # has architecture #. The file must be a binary SPK file to be readable by this routine.  Binary SPK files have DAF architecture.  If you expected the file to be a binary SPK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", SPKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDARCHTYPE)", ctx)?;
        CHKOUT(b"SPKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&KERTYP, b"SPK") {
        SETMSG(b"Input file # has file type #. The file must be a binary SPK file to be readable by this routine. If you expected the file to be a binary SPK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", SPKFNM, ctx);
        ERRCH(b"#", &KERTYP, ctx);
        SIGERR(b"SPICE(INVALIDFILETYPE)", ctx)?;
        CHKOUT(b"SPKCOV", ctx)?;
        return Ok(());
    }

    //
    // Open the file for reading.
    //
    DAFOPR(SPKFNM, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKCOV", ctx)?;
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

    CHKOUT(b"SPKCOV", ctx)?;
    Ok(())
}
