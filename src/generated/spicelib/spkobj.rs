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

/// SPK objects
///
/// Find the set of ID codes of all objects in a specified SPK file.
///
/// # Required Reading
///
/// * [CELLS](crate::required_reading::cells)
/// * [DAF](crate::required_reading::daf)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
/// * [SETS](crate::required_reading::sets)
/// * [SPK](crate::required_reading::spk)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SPKFNM     I   Name of SPK file.
///  IDS       I-O  Set of ID codes of objects in SPK file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SPKFNM   is the name of an SPK file.
///
///  IDS      is an initialized SPICE set data structure. IDS
///           optionally may contain a set of ID codes on input; on
///           output, the data already present in IDS will be combined
///           with ID code set found for the file SPKFNM.
///
///           If IDS contains no data on input, its size and
///           cardinality still must be initialized.
/// ```
///
/// # Detailed Output
///
/// ```text
///  IDS      is a SPICE set data structure which contains the union
///           of its contents upon input with the set of ID codes of
///           each object for which ephemeris data are present in the
///           indicated SPK file. The elements of SPICE sets are
///           unique; hence each ID code in IDS appears only once, even
///           if the SPK file contains multiple segments for that ID
///           code.
///
///           See the $Examples section below for a complete example
///           program showing how to retrieve the ID codes from IDS.
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
///      by a routine in the call tree of this routine.
///
///  5)  If the size of the output set argument IDS is insufficient to
///      contain the actual number of ID codes of objects covered by
///      the indicated SPK file, an error is signaled by a routine in
///      the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine provides an API via which applications can determine
///  the set of objects for which there are ephemeris data in a
///  specified SPK file.
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
///  1) This example demonstrates combined usage of SPKOBJ and the
///     related SPK utility SPKCOV.
///
///     Display the coverage for each object in a specified SPK file.
///     Find the set of objects in the file. Loop over the contents
///     of the ID code set: find the coverage for each item in the
///     set and display the coverage.
///
///
///     Example code begins here.
///
///
///           PROGRAM SPKOBJ_EX1
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
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If an error occurs while this routine is updating the set
///      IDS, the set may be corrupted.
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
///         solution to code example.
///
///         Corrected short error message in entries #2 and #3 in
///         $Exceptions section. Added NAIF_IDS to $Required_Reading
///         section.
///
/// -    SPICELIB Version 1.0.2, 01-JUL-2014 (NJB)
///
///         Updated index entries.
///
/// -    SPICELIB Version 1.0.1, 30-NOV-2007 (NJB)
///
///         Corrected bug in program in header $Examples section:
///         program now empties the coverage window prior to collecting
///         data for the current object. Deleted declaration of unused
///         parameter NAMLEN in example program. Updated example to
///         use WNCARD rather than CARDD.
///
/// -    SPICELIB Version 1.0.0, 30-DEC-2004 (NJB)
/// ```
pub fn spkobj(ctx: &mut SpiceContext, spkfnm: &str, ids: &mut [i32]) -> crate::Result<()> {
    SPKOBJ(spkfnm.as_bytes(), ids, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SPKOBJ ( SPK objects )
pub fn SPKOBJ(SPKFNM: &[u8], IDS: &mut [i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDS = DummyArrayMut::new(IDS, LBCELL..);
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

    CHKIN(b"SPKOBJ", ctx)?;

    //
    // See whether GETFAT thinks we've got an SPK file.
    //
    GETFAT(SPKFNM, &mut ARCH, &mut KERTYP, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKOBJ", ctx)?;
        return Ok(());
    }

    if fstr::eq(&ARCH, b"XFR") {
        SETMSG(b"Input file # has architecture #. The file must be a binary SPK file to be readable by this routine.  If the input file is an SPK file in transfer format, run TOBIN on the file to convert it to binary format.", ctx);
        ERRCH(b"#", SPKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        CHKOUT(b"SPKOBJ", ctx)?;
        return Ok(());
    } else if fstr::ne(&ARCH, b"DAF") {
        SETMSG(b"Input file # has architecture #. The file must be a binary SPK file to be readable by this routine.  Binary SPK files have DAF architecture.  If you expected the file to be a binary SPK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", SPKFNM, ctx);
        ERRCH(b"#", &ARCH, ctx);
        SIGERR(b"SPICE(INVALIDARCHTYPE)", ctx)?;
        CHKOUT(b"SPKOBJ", ctx)?;
        return Ok(());
    } else if fstr::ne(&KERTYP, b"SPK") {
        SETMSG(b"Input file # has file type #. The file must be a binary SPK file to be readable by this routine. If you expected the file to be a binary SPK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        ERRCH(b"#", SPKFNM, ctx);
        ERRCH(b"#", &KERTYP, ctx);
        SIGERR(b"SPICE(INVALIDFILETYPE)", ctx)?;
        CHKOUT(b"SPKOBJ", ctx)?;
        return Ok(());
    }

    //
    // Open the file for reading.
    //
    DAFOPR(SPKFNM, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SPKOBJ", ctx)?;
        return Ok(());
    }

    //
    // We will examine each segment descriptor in the file, and
    // we'll update our ID code set according to the data found
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

        //
        // Insert the current ID code into the output set.
        // The insertion algorithm will handle duplicates; no special
        // action is required here.
        //
        INSRTI(IC[1], IDS.as_slice_mut(), ctx)?;

        DAFFNA(&mut FOUND, ctx)?;
    }

    //
    // Release the file.
    //
    DAFCLS(HANDLE, ctx)?;

    CHKOUT(b"SPKOBJ", ctx)?;
    Ok(())
}
