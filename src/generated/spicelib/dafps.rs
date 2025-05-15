//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// DAF, pack summary
///
/// Pack (assemble) an array summary from its double precision and
/// integer components.
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
///  ND         I   Number of double precision components.
///  NI         I   Number of integer components.
///  DC         I   Double precision components.
///  IC         I   Integer components.
///  SUM        O   Array summary.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ND       is the number of double precision components in
///           the summary to be packed.
///
///  NI       is the number of integer components in the summary.
///
///  DC       are the double precision components of the summary.
///
///  IC       are the integer components of the summary.
/// ```
///
/// # Detailed Output
///
/// ```text
///  SUM      is an array summary containing the components in DC
///           and IC. This identifies the contents and location of
///           a single array within a DAF.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If ND is zero or negative, no DP components are stored.
///
///  2)  If NI is zero or negative, no integer components are stored.
///
///  3)  If the total size of the summary is greater than 125 double
///      precision words, some components may not be stored. See
///      $Particulars for details.
/// ```
///
/// # Particulars
///
/// ```text
///  The components of array summaries are packed into double
///  precision arrays for reasons outlined in [1]. Two routines,
///  DAFPS (pack summary) and DAFUS (unpack summary) are provided
///  for packing and unpacking summaries.
///
///  The total size of the summary is
///
///          (NI - 1)
///     ND + -------- + 1
///              2
///
///  double precision words (where ND, NI are nonnegative).
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
///  1) Replace the body ID code 301 (Moon) with a test body ID,
///     e.g. -999, in every descriptor of an SPK file.
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFPS_EX1
///           IMPLICIT NONE
///
///     C
///     C     SPICELIB functions.
///     C
///           INTEGER               CARDI
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               LBCELL
///           PARAMETER           ( LBCELL = -5   )
///
///           INTEGER               DSCSIZ
///           PARAMETER           ( DSCSIZ = 5    )
///
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 256  )
///
///           INTEGER               MAXOBJ
///           PARAMETER           ( MAXOBJ = 1000 )
///
///           INTEGER               ND
///           PARAMETER           ( ND     = 2    )
///
///           INTEGER               NI
///           PARAMETER           ( NI     = 6    )
///
///           INTEGER               NEWCOD
///           PARAMETER           ( NEWCOD = -999 )
///
///           INTEGER               OLDCOD
///           PARAMETER           ( OLDCOD =  301 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FILSIZ)    FNAME
///
///           DOUBLE PRECISION      DC     ( ND )
///           DOUBLE PRECISION      SUM    ( DSCSIZ )
///
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               IC     ( NI )
///           INTEGER               IDS    ( LBCELL : MAXOBJ )
///
///           LOGICAL               FOUND
///
///     C
///     C     Get the SPK file name.
///     C
///           CALL PROMPT ( 'Enter name of the SPK file > ', FNAME )
///
///     C
///     C     Initialize the set IDS.
///     C
///           CALL SSIZEI ( MAXOBJ, IDS )
///
///     C
///     C     Open for writing the SPK file.
///     C
///           CALL DAFOPW ( FNAME, HANDLE )
///
///     C
///     C     Search the file in forward order.
///     C
///           CALL DAFBFS ( HANDLE )
///           CALL DAFFNA ( FOUND )
///
///           DO WHILE ( FOUND )
///
///     C
///     C        Fetch and unpack the descriptor (aka summary)
///     C        of the current segment.
///     C
///              CALL DAFGS ( SUM )
///              CALL DAFUS ( SUM, ND, NI, DC, IC )
///
///     C
///     C        Replace ID codes if necessary.
///     C
///              IF ( IC(1) .EQ. OLDCOD ) THEN
///
///                 IC(1) = NEWCOD
///
///              END IF
///
///              IF ( IC(2) .EQ. OLDCOD ) THEN
///
///                 IC(2) = NEWCOD
///
///              END IF
///
///     C
///     C        Re-pack the descriptor; replace the descriptor
///     C        in the file.
///     C
///              CALL DAFPS ( ND, NI, DC, IC, SUM )
///              CALL DAFRS ( SUM )
///
///     C
///     C        Find the next segment.
///     C
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///     C
///     C     Close the file.
///     C
///           CALL DAFCLS ( HANDLE )
///
///     C
///     C     Find the set of objects in the SPK file.
///     C
///           CALL SPKOBJ ( FNAME, IDS )
///
///           WRITE(*,'(A)') 'Objects in the DAF file:'
///           WRITE(*,*) ' '
///           WRITE(*,'(20I4)') ( IDS(I), I= 1, CARDI ( IDS ) )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, using the SPK file named de430.bsp, the output was:
///
///
///     Enter name of the SPK file > de430.bsp
///     Objects in the DAF file:
///
///     -999   1   2   3   4   5   6   7   8   9  10 199 299 399
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 06-JUL-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
/// -    SPICELIB Version 1.0.3, 10-OCT-2012 (EDW)
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
///         Corrected ordering of header section.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafps(nd: i32, ni: i32, dc: &[f64], ic: &[i32], sum: &mut [f64]) {
    DAFPS(nd, ni, dc, ic, sum);
}

//$Procedure DAFPS ( DAF, pack summary )
pub fn DAFPS(ND: i32, NI: i32, DC: &[f64], IC: &[i32], SUM: &mut [f64]) {
    let DC = DummyArray::new(DC, 1..);
    let IC = DummyArray::new(IC, 1..);
    let mut SUM = DummyArrayMut::new(SUM, 1..);
    let mut DEQUIV = StackArray::<f64, 125>::new(1..=125);
    let mut IEQUIV = StackArray::<i32, 250>::new(1..=250);
    let mut N: i32 = 0;
    let mut M: i32 = 0;

    //
    // Local variables
    //

    //
    // Equivalences
    //

    //
    // Here's the deal: the DP components always precede the integer
    // components, avoiding alignment problems. The DP components can
    // be stored directly.
    //
    N = intrinsics::MIN0(&[125, intrinsics::MAX0(&[0, ND])]);

    MOVED(DC.as_slice(), N, SUM.as_slice_mut());

    //
    // The integer components must detour through an equivalence.
    //
    M = intrinsics::MIN0(&[(250 - (2 * N)), intrinsics::MAX0(&[0, NI])]);

    MOVEI(
        IC.as_slice(),
        M,
        DummyArrayMut::<i32>::from_equiv(DEQUIV.as_slice_mut(), 1..=250).as_slice_mut(),
    );
    MOVED(
        DEQUIV.as_slice(),
        (((M - 1) / 2) + 1),
        SUM.subarray_mut((N + 1)),
    );
}

/// DAF, unpack summary
///
/// Unpack an array summary into its double precision and integer
/// components.
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
///  SUM        I   Array summary.
///  ND         I   Number of double precision components.
///  NI         I   Number of integer components.
///  DC         O   Double precision components.
///  IC         O   Integer components.
/// ```
///
/// # Detailed Input
///
/// ```text
///  SUM      is an array summary. This identifies the contents and
///           location of a single array within a DAF.
///
///  ND       is the number of double precision components in
///           the summary.
///
///  NI       is the number of integer components in the summary.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DC       are the double precision components of the summary.
///
///  IC       are the integer components of the summary.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If ND is zero or negative, no double precision components
///      are returned.
///
///  2)  If NI is zero or negative, no integer components are returned.
///
///  3)  If the total size of the summary is greater than 125 double
///      precision words, some components may not be returned.
/// ```
///
/// # Particulars
///
/// ```text
///  The components of array summaries are packed into double
///  precision arrays for reasons outlined in [1]. Two routines,
///  DAFPS (pack summary) and DAFUS (unpack summary) are provided
///  for packing and unpacking summaries.
///
///  The total size of the summary is
///
///          (NI - 1)
///     ND + -------- + 1
///              2
///
///  double precision words (where ND, NI are nonnegative).
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
///  1) Use a simple routine to output the double precision and
///     integer values stored in an SPK's segments descriptors. This
///     function opens a DAF for read, performs a forwards search for
///     the DAF arrays, prints segments description for each array
///     found, then closes the DAF.
///
///     Use the SPK kernel below as input DAF file for the program.
///
///        de421.bsp
///
///
///     Example code begins here.
///
///
///           PROGRAM DAFUS_EX1
///           IMPLICIT NONE
///
///     C
///     C     Define the summary parameters appropriate
///     C     for an SPK file.
///     C
///           INTEGER               MAXSUM
///           PARAMETER           ( MAXSUM = 125 )
///
///           INTEGER               ND
///           PARAMETER           ( ND = 2       )
///
///           INTEGER               NI
///           PARAMETER           ( NI = 6       )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(32)        KERNEL
///
///           DOUBLE PRECISION      DC     ( ND     )
///           DOUBLE PRECISION      SUM    ( MAXSUM )
///
///           INTEGER               HANDLE
///           INTEGER               IC     ( NI )
///
///           LOGICAL               FOUND
///
///
///     C
///     C     Open a DAF for read. Return a HANDLE referring to the
///     C     file.
///     C
///           KERNEL = 'de421.bsp'
///           CALL DAFOPR ( KERNEL, HANDLE )
///
///     C
///     C     Begin a forward search on the file.
///     C
///           CALL DAFBFS ( HANDLE )
///
///     C
///     C     Search until a DAF array is found.
///     C
///           CALL DAFFNA ( FOUND )
///
///     C
///     C     Loop while the search finds subsequent DAF arrays.
///     C
///           DO WHILE ( FOUND )
///
///              CALL DAFGS ( SUM )
///              CALL DAFUS ( SUM, ND, NI, DC, IC )
///
///              WRITE(*,*)                'Doubles:', DC(1:ND)
///              WRITE(*, FMT='(A,6I9)' ) 'Integers:', IC(1:NI)
///
///     C
///     C        Check for another segment.
///     C
///              CALL DAFFNA ( FOUND )
///
///           END DO
///
///     C
///     C     Safely close the DAF.
///     C
///           CALL DAFCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        1        0        1        2      641   310404
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        2        0        1        2   310405   423048
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        3        0        1        2   423049   567372
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        4        0        1        2   567373   628976
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        5        0        1        2   628977   674740
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        6        0        1        2   674741   715224
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        7        0        1        2   715225   750428
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        8        0        1        2   750429   785632
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:        9        0        1        2   785633   820836
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:       10        0        1        2   820837   944040
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      301        3        1        2   944041  1521324
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      399        3        1        2  1521325  2098608
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      199        1        1        2  2098609  2098620
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      299        2        1        2  2098621  2098632
///      Doubles:  -3169195200.0000000        1696852800.0000000
///     Integers:      499        4        1        2  2098633  2098644
///
///
///     Note, the final entries in the integer array contains the
///     segment start/end indexes. The output indicates the search
///     proceeded from the start of the file (low value index) towards
///     the end (high value index).
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 04-AUG-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard..
///         Added undeclared variables to code example.
///
/// -    SPICELIB Version 1.0.3, 10-OCT-2012 (EDW)
///
///         Added a functional code example to the $Examples section.
///
///         Removed the obsolete Reference citation to "NAIF
///         Document 167.0."
///
///         Corrected ordering of header section.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 04-AUG-2020 (JDR)
///
///         Added IMPLICIT NONE statement.
/// ```
pub fn dafus(sum: &[f64], nd: i32, ni: i32, dc: &mut [f64], ic: &mut [i32]) {
    DAFUS(sum, nd, ni, dc, ic);
}

//$Procedure DAFUS ( DAF, unpack summary )
pub fn DAFUS(SUM: &[f64], ND: i32, NI: i32, DC: &mut [f64], IC: &mut [i32]) {
    let mut DC = DummyArrayMut::new(DC, 1..);
    let mut IC = DummyArrayMut::new(IC, 1..);
    let SUM = DummyArray::new(SUM, 1..);
    let mut DEQUIV = StackArray::<f64, 125>::new(1..=125);
    let IEQUIV = StackArray::<i32, 250>::new(1..=250);
    let mut N: i32 = 0;
    let mut M: i32 = 0;

    //
    // Just undo whatever DAFPS did.
    //
    N = intrinsics::MIN0(&[125, intrinsics::MAX0(&[0, ND])]);

    MOVED(SUM.as_slice(), N, DC.as_slice_mut());

    M = intrinsics::MIN0(&[(250 - (2 * N)), intrinsics::MAX0(&[0, NI])]);

    MOVED(
        SUM.subarray((N + 1)),
        (((M - 1) / 2) + 1),
        DEQUIV.as_slice_mut(),
    );
    MOVEI(
        DummyArray::<i32>::from_equiv(DEQUIV.as_slice(), 1..=250).as_slice(),
        M,
        IC.as_slice_mut(),
    );
}
