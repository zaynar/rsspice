//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const FTSIZE: i32 = 5000;
pub const ITSIZE: i32 = 5000;
pub const LBPOOL: i32 = -5;
pub const STSIZE: i32 = 100000;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = (ND + ((NI + 1) / 2));
const SIDLEN: i32 = 40;
const SLEN: i32 = 40;
const FORWRD: i32 = 1;
const BCKWRD: i32 = 2;
const FREE: i32 = 0;

struct SaveVars {
    NFT: i32,
    FTHAN: ActualArray<i32>,
    FTNUM: ActualArray<i32>,
    NEXT: i32,
    FINDEX: i32,
    ITPRVI: ActualCharArray,
    ITPRVD: ActualArray2D<f64>,
    ITLB: ActualArray<f64>,
    ITUB: ActualArray<f64>,
    IINDEX: i32,
    ITBEG: ActualArray<i32>,
    ITEXP: ActualArray<i32>,
    ITHFS: ActualArray<i32>,
    ITINS: ActualArray<i32>,
    ITLFS: ActualArray<i32>,
    ITPRVF: ActualArray<i32>,
    ITPRVH: ActualArray<i32>,
    ITRUEX: ActualArray<i32>,
    NIT: i32,
    ITCHKP: ActualArray<bool>,
    STIDNT: ActualCharArray,
    STDCD: ActualArray2D<f64>,
    STHAN: ActualArray<i32>,
    STICD: ActualArray2D<i32>,
    STPOOL: ActualArray2D<i32>,
    STATUS: Vec<u8>,
    ALPHA: f64,
    OMEGA: f64,
    REQT: f64,
    SAVTOL: f64,
    SAVEP: i32,
    SCINST: i32,
    SLBEG: i32,
    TOP: i32,
    AVNEED: bool,
    FND: bool,
    FRESUB: bool,
    NEWSCH: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut NFT: i32 = 0;
        let mut FTHAN = ActualArray::<i32>::new(1..=FTSIZE);
        let mut FTNUM = ActualArray::<i32>::new(1..=FTSIZE);
        let mut NEXT: i32 = 0;
        let mut FINDEX: i32 = 0;
        let mut ITPRVI = ActualCharArray::new(SIDLEN, 1..=ITSIZE);
        let mut ITPRVD = ActualArray2D::<f64>::new(1..=DSCSIZ, 1..=ITSIZE);
        let mut ITLB = ActualArray::<f64>::new(1..=ITSIZE);
        let mut ITUB = ActualArray::<f64>::new(1..=ITSIZE);
        let mut IINDEX: i32 = 0;
        let mut ITBEG = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITEXP = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITHFS = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITINS = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITLFS = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITPRVF = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITPRVH = ActualArray::<i32>::new(1..=ITSIZE);
        let mut ITRUEX = ActualArray::<i32>::new(1..=ITSIZE);
        let mut NIT: i32 = 0;
        let mut ITCHKP = ActualArray::<bool>::new(1..=ITSIZE);
        let mut STIDNT = ActualCharArray::new(SIDLEN, 1..=STSIZE);
        let mut STDCD = ActualArray2D::<f64>::new(1..=ND, 1..=STSIZE);
        let mut STHAN = ActualArray::<i32>::new(1..=STSIZE);
        let mut STICD = ActualArray2D::<i32>::new(1..=NI, 1..=STSIZE);
        let mut STPOOL = ActualArray2D::<i32>::new(1..=2, LBPOOL..=STSIZE);
        let mut STATUS = vec![b' '; SLEN as usize];
        let mut ALPHA: f64 = 0.0;
        let mut OMEGA: f64 = 0.0;
        let mut REQT: f64 = 0.0;
        let mut SAVTOL: f64 = 0.0;
        let mut SAVEP: i32 = 0;
        let mut SCINST: i32 = 0;
        let mut SLBEG: i32 = 0;
        let mut TOP: i32 = 0;
        let mut AVNEED: bool = false;
        let mut FND: bool = false;
        let mut FRESUB: bool = false;
        let mut NEWSCH: bool = false;

        FRESUB = false;
        NFT = 0;
        NIT = 0;
        NEXT = 0;
        SAVEP = 0;
        SAVTOL = 0.0;
        fstr::assign(&mut STATUS, b"BOGUS ENTRY");

        Self {
            NFT,
            FTHAN,
            FTNUM,
            NEXT,
            FINDEX,
            ITPRVI,
            ITPRVD,
            ITLB,
            ITUB,
            IINDEX,
            ITBEG,
            ITEXP,
            ITHFS,
            ITINS,
            ITLFS,
            ITPRVF,
            ITPRVH,
            ITRUEX,
            NIT,
            ITCHKP,
            STIDNT,
            STDCD,
            STHAN,
            STICD,
            STPOOL,
            STATUS,
            ALPHA,
            OMEGA,
            REQT,
            SAVTOL,
            SAVEP,
            SCINST,
            SLBEG,
            TOP,
            AVNEED,
            FND,
            FRESUB,
            NEWSCH,
        }
    }
}

/// C-kernel, buffer segments for readers
///
/// Load and unload files for use by the readers. Buffer segments
/// for readers.
///
/// # Required Reading
///
/// * [CK](crate::required_reading::ck)
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  FNAME      I   CKLPF
///  HANDLE    I-O  CKLPF, CKUPF, CKSNS
///  INST       I   CKBSS
///  SCLKDP     I   CKBSS
///  TOL        I   CKBSS
///  NEEDAV     I   CKBSS
///  DESCR      O   CKSNS
///  SEGID      O   CKSNS
///  FOUND      O   CKSNS, CKHAVE
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a binary C-kernel file to be loaded.
///
///  HANDLE   on input, is the handle of a binary C-kernel file to be
///           unloaded.
///
///  INST     is the NAIF ID of an instrument.
///
///  SCLKDP   is an encoded spacecraft clock time.
///
///  TOL      is a time tolerance, measured in the same units as
///           encoded spacecraft clock.
///
///  NEEDAV   indicates whether or not angular velocity data are
///           required.
///
///           If .TRUE., only segments containing pointing and angular
///           velocity data will be checked. If .FALSE., segments
///           containing just pointing data will also be considered.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   on output, is the handle of the C-kernel file
///           containing a located segment.
///
///  DESCR    is the packed descriptor of a located segment.
///
///  SEGID    is the identifier of a located segment.
///
///  FOUND    indicates whether a requested segment was found or not.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of pointing files that can
///           be loaded by CKLPF at any given time for use by the
///           readers.
///
///  ITSIZE   is the maximum number of instruments whose segments
///           are buffered by CKSNS.
///
///  STSIZE   is the maximum number of segments that can be buffered
///           at any given time by CKSNS.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If CKBSR is called directly, the error SPICE(CKBOGUSENTRY)
///      is signaled.
///
///  2)  See entry points CKLPF, CKUPF, CKBSS, and CKSNS for
///      exceptions specific to them.
/// ```
///
/// # Files
///
/// ```text
///  C-kernel pointing files are indicated by filename before loading
///  (see CKLPF) and handle after loading (all other places).
/// ```
///
/// # Particulars
///
/// ```text
///  CKBSR serves as an umbrella, allowing data to be shared by its
///  entry points:
///
///     CKLPF       Load pointing file.
///     CKUPF       Unload pointing file.
///     CKBSS       Begin search for segment.
///     CKSNS       Select next segment.
///     CKHAVE      Determine whether or not any CKs are loaded.
///
///  Before a file can be read by the C-kernel readers, it must be
///  loaded by CKLPF, which among other things load the file into
///  the DAF subsystem.
///
///  Up to FTSIZE files may be loaded for use simultaneously, and a
///  file only has to be loaded once to become a potential search
///  target for any number of subsequent reads.
///
///  Once a C-kernel has been loaded, it is assigned a file
///  handle, which is used to keep track of the file internally, and
///  which is used by the calling program to refer to the file in all
///  subsequent calls to CK routines.
///
///  A file may be removed from the list of files for potential
///  searching by unloading it via a call to CKUPF.
///
///  The purpose of entry points CKBSS and CKSNS is to search for
///  segments in CK files matching certain criteria, which is
///  established based on CKBSS input arguments INST, SCLKDP, TOL and
///  NEEDAV. These two routines are used together to search through
///  all loaded CK files for segments.
///
///  CKBSS sets up a search for segments by CKSNS. It records the
///  instrument and time to be searched for, and whether to require
///  segments containing angular velocity data. If angular velocity
///  data are required, only segments containing angular velocity
///  data will be returned by CKSNS. If angular velocity data are
///  not required, segments returned by CKSNS may or may not contain
///  angular velocity data.
///
///  CKBSS determines the first task that CKSNS will have to perform
///  if it is called to get an applicable segment.
///
///  CKSNS finds segments matching the search criteria set up by
///  CKBSS. Last-loaded files get searched first, and individual files
///  are searched backwards.
///
///  A segment matches the CKBSS/CKSNS search criteria when the
///  following statements are true.
///
///     1) INST matches the instrument number for the segment.
///
///     2) The time interval [SCLKDP - TOL, SCLKDP + TOL] intersects
///        the time interval of the segment.
///
///     3) If angular velocity data are required, as indicated by
///        NEEDAV, the segment contains angular velocity data.
///
///  When an applicable segment is found, CKSNS returns that segment's
///  descriptor and identifier, along with the handle of the file
///  containing the segment.
///
///  Subsequent calls to CKSNS continue the search, picking up where
///  the previous call to this routine left off.
///
///  CKSNS uses information on loaded files to manage a buffer
///  of saved segment descriptors and identifiers. The buffer is used
///  to speed up access time by minimizing file reads.
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
///  1) Suppose that pointing data for the Viking Orbiter 2 scan
///     platform orientation for a certain interval of time are
///     contained in three separate files, one containing data for the
///     original SEDR (Supplemental Experiment Data Record) files,
///     contains the complete set of pointing data and another two
///     which contain two separate pointing updates based on
///     reconstruction, one of them containing discrete data, and the
///     other continuous pointing data.
///
///     In the following example, pointing from the C-kernel is
///     extracted in two different ways for the purpose of comparing
///     the two updates:
///
///     First, the original pointing file and one of the update files
///     are both loaded and pointing is retrieved for all of the
///     pictures. The update file is searched through first, and if
///     no data for the desired picture is located, then the original
///     file provides the requested pointing.
///
///     Then, the first update file is unloaded, the second update
///     file is loaded, and the same search is performed, as above.
///
///
///     Use the CK kernel below to load the Viking Orbiter 2 scan
///     platform orientation containing a combination of data from
///     SEDR files.
///
///        vo2_sedr_ck2.bc
///
///
///     Use the CK kernel below to load the Viking Orbiter 2 scan
///     platform orientation discrete data reconstructed during
///     cartographic image registration by S. Wu, USGS.
///
///        vo2_swu.bc
///
///
///     Use the CK kernel below to load the Viking Orbiter 2 scan
///     platform orientation discrete data reconstructed during
///     cartographic image registration by S. Wu, USGS, where each
///     discrete pointing instance was "expanded" into a 2 second
///     window.
///
///        vo2_swu_ck2.bc
///
///
///     Use the meta-kernel shown below to load the required SPICE
///     kernels.
///
///
///        KPL/MK
///
///        File name: ckbsr_ex1.tm
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
///           File name                     Contents
///           ---------                     --------
///           vo2_fict.tsc                  Viking 2 SCLK
///           naif0012.tls                  Leapseconds
///
///
///        \begindata
///
///           KERNELS_TO_LOAD = ( 'vo2_fict.tsc',
///                               'naif0012.tls'  )
///
///        \begintext
///
///        End of meta-kernel
///
///
///     Example code begins here.
///
///
///           PROGRAM CKBSR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NPICS
///           PARAMETER           ( NPICS  = 5  )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN = 24 )
///
///
///     C
///     C     Local variables.
///     C
///           INTEGER               HANDLE
///           INTEGER               HNORIG
///           INTEGER               HUPDT
///           INTEGER               UPDATE
///           INTEGER               INST
///           INTEGER               SC
///           INTEGER               I
///
///           DOUBLE PRECISION      ET
///           DOUBLE PRECISION      DESCR    (    5 )
///           DOUBLE PRECISION      SCLKDP
///           DOUBLE PRECISION      TOL
///           DOUBLE PRECISION      CLKOUT
///           DOUBLE PRECISION      CMAT     ( 3, 3 )
///           DOUBLE PRECISION      AV       (    3 )
///
///           CHARACTER*(TIMLEN)    FDS      ( NPICS )
///           CHARACTER*(25)        FNAME
///           CHARACTER*(40)        SEGID
///           CHARACTER*(16)        OUTFDS
///           CHARACTER*(14)        TOLSTR
///           CHARACTER*(25)        UDFILE   (    2 )
///
///           LOGICAL               PFOUND
///           LOGICAL               SFOUND
///           LOGICAL               NEEDAV
///
///     C
///     C     Set the times for the pictures.
///     C
///           DATA                  FDS /
///          .                 '1976 OCT 31 22:07:21.000',
///          .                 '1977-JAN-09 18:33:13.707',
///          .                 '1977 APR 24 11:48:05.000',
///          .                 '1977 JUN 07 00:13:15.000',
///          .                 '1977-AUG-07 14:55:12.019' /
///
///
///           UDFILE ( 1 ) = 'vo2_swu.bc'
///           UDFILE ( 2 ) = 'vo2_swu_ck2.bc'
///
///     C
///     C     The NAIF integer ID codes for the Viking Orbiter 2
///     C     spacecraft and scan platform on Viking Orbiter 2 are
///     C     -30 and -30000, respectively.
///     C
///           SC           = -30
///           INST         = -30000
///
///     C
///     C     Load the LSK and Viking 2 SCLK files.
///     C
///           CALL FURNSH ( 'ckbsr_ex1.tm' )
///
///     C
///     C     Allow a time tolerance of 500 milliseconds.  Convert
///     C     the tolerance to 'ticks', the units of encoded
///     C     spacecraft clock time.
///     C
///           TOLSTR  = '0.500'
///           CALL SCTIKS ( SC, TOLSTR, TOL )
///
///     C
///     C     Don't care about angular velocity data.
///     C
///           NEEDAV = .FALSE.
///
///     C
///     C     Load the original CK file first.
///     C
///           CALL CKLPF ( 'vo2_sedr_ck2.bc', HNORIG )
///
///     C
///     C     Write banner.
///     C
///           WRITE(*,'(A)') '     Input UTC time       '
///          .            // 'Pointing found in      SCLK time'
///           WRITE(*,'(A)') '------------------------  '
///          .            // '-----------------  ----------------'
///
///           DO UPDATE = 1, 2
///     C
///     C        Load the update file.  Last-loaded files get searched
///     C        first, so the update file will be searched before
///     C        the original file.
///     C
///              CALL CKLPF ( UDFILE ( UPDATE ), HUPDT )
///
///              DO I = 1, NPICS
///
///     C
///     C           Encode the character string representation of
///     C           spacecraft clock time in FDS.
///     C
///                 CALL STR2ET ( FDS( I ) , ET  )
///                 CALL SCE2C  ( SC, ET, SCLKDP )
///
///     C
///     C           Begin a search for this instrument and time, and
///     C           get the first applicable segment.
///     C
///                 CALL CKBSS ( INST,   SCLKDP, TOL,   NEEDAV  )
///                 CALL CKSNS ( HANDLE, DESCR,  SEGID, SFOUND  )
///
///     C
///     C           Keep trying candidate segments until a segment can
///     C           produce a pointing instance within the specified
///     C           time tolerance of SCLKDP, the encoded spacecraft
///     C           clock time.
///     C
///                 PFOUND = .FALSE.
///                 DO WHILE (  SFOUND .AND. ( .NOT. PFOUND )  )
///
///                    CALL CKPFS ( HANDLE, DESCR,  SCLKDP,
///          .                      TOL,    NEEDAV, CMAT,
///          .                      AV,     CLKOUT, PFOUND )
///
///                    IF ( PFOUND ) THEN
///
///     C
///     C                 Get the name of the file from whence the
///     C                 pointing instance came, decode the
///     C                 spacecraft clock time associated with the
///     C                 instance, and write the results to the
///     C                 table.
///     C
///                       CALL DAFHFN ( HANDLE, FNAME          )
///                       CALL SCDECD ( SC,     CLKOUT, OUTFDS )
///
///                       WRITE(*,'(A,2X,A17,2X,A)') FDS( I ), FNAME,
///          .                                       OUTFDS
///
///                    ELSE
///
///     C
///     C                 Look for another candidate segment.
///     C
///                       CALL CKSNS ( HANDLE, DESCR, SEGID, SFOUND )
///
///                    END IF
///
///                 END DO
///
///                 IF ( .NOT. PFOUND ) THEN
///
///                    WRITE(*,'(A)') FDS( I ) // '  pointing not '
///          .                     // 'found in any file.'
///
///                 END IF
///
///              END DO
///
///              WRITE(*,*) ' '
///
///     C
///     C        Unload the update file. The original file stays
///     C        loaded.
///     C
///              CALL CKUPF  ( HUPDT )
///
///           END DO
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///          Input UTC time       Pointing found in      SCLK time
///     ------------------------  -----------------  ----------------
///     1976 OCT 31 22:07:21.000  vo2_sedr_ck2.bc    1/0026345241.000
///     1977-JAN-09 18:33:13.707  vo2_swu.bc         1/0032380394.707
///     1977 APR 24 11:48:05.000  vo2_sedr_ck2.bc    1/0041428086.000
///     1977 JUN 07 00:13:15.000  pointing not found in any file.
///     1977-AUG-07 14:55:12.019  vo2_sedr_ck2.bc    1/0050511313.019
///
///     1976 OCT 31 22:07:21.000  vo2_sedr_ck2.bc    1/0026345241.000
///     1977-JAN-09 18:33:13.707  vo2_swu_ck2.bc     1/0032380394.707
///     1977 APR 24 11:48:05.000  vo2_sedr_ck2.bc    1/0041428086.000
///     1977 JUN 07 00:13:15.000  pointing not found in any file.
///     1977-AUG-07 14:55:12.019  vo2_swu_ck2.bc     1/0050511313.019
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  If Fortran I/O errors occur while searching a loaded CK
///      file, the internal state of this suite of routines may
///      be corrupted. It may be possible to correct the state
///      by unloading the pertinent CK files and then re-loading
///      them.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.1.0, 25-OCT-2021 (JDR) (BVS) (NJB)
///
///         Increased ITSIZE (from 100 to 5000).
///
///         Updated entry point CKSNS to always initialize FOUND.
///
///         Edited the header of umbrella routine CKBSR, and all entry
///         points to comply with NAIF standard. Created complete code
///         example from existing fragments in CKBSR $Examples section.
///
///         Added references to CKHAVE entry point in CKBSR header.
///
///         Moved details related to search criteria and conditions to meet
///         it from the $Detailed_Input to $Particulars.
///
/// -    SPICELIB Version 5.0.1, 30-JAN-2017 (NJB)
///
///         Corrected various spelling errors within comments.
///
/// -    SPICELIB Version 5.0.0, 17-MAR-2014 (NJB)
///
///         Updated segment pool initialization condition in entry
///         point CKLPF so that the pool is initialized only if the file
///         table is empty.
///
/// -    SPICELIB Version 4.6.0, 13-JUN-2013 (BVS)
///
///         Increased FTSIZE (from 1000 to 5000).
///
///         Increased STSIZE (from 50000 to 100000).
///
/// -    SPICELIB Version 4.5.0, 24-FEB-2011 (NJB)
///
///         Bug fixes:
///
///           1) In the CKSNS 'MAKE ROOM' state, when the
///              suspended activity is 'ADD TO FRONT' and no segment table
///              room is available, the instrument table's pointer to the
///              current segment list is now set to null. Previously the
///              pointer was allowed to go stale.
///
///           2) In CKUPF, the null pointer test used to determine
///              eligibility for segment list deletion now uses the .LE.
///              operator instead of the .EQ. operator.
///
///
/// -    SPICELIB Version 4.4.0, 07-APR-2010 (NJB)
///
///         Increased STSIZE to 50000.
///
/// -    SPICELIB Version 4.3.1, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section
///         of the CKHAVE entry point header.
///
/// -    SPICELIB Version 4.3.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments in
///         MOVED calls in entry points CKUPF and CKSNS. Replaced header
///         reference to LDPOOL with reference to FURNSH.
///
/// -    SPICELIB Version 4.2.0, 30-DEC-2004 (NJB)
///
///         Increased STSIZE to 20000.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///            1) When a segment list is freed because the entire list
///               is contributed by a single CK file, and the list is
///               too large to be buffered, the corresponding instrument
///               table pointer is now set to null.
///
///            2) An algorithm change has eliminated a bug caused by not
///               updating the current instrument index when instrument
///               table entries having empty segment lists were compressed
///               out of the instrument table. Previously the instrument
///               table pointer IINDEX could go stale after the
///               compression.
///
///            3) When a already loaded kernel is re-opened with DAFOPR,
///               it now has its link count reset to 1 via a call to
///               DAFCLS.
///
///            4) The load routine CKLPF now resets all file numbers when
///               the next file number reaches INTMAX()-1, thereby
///               avoiding arithmetic overflow.
///
///            5) The unload routine CKUPF now calls RETURN() on entry and
///               returns if so directed.
///
///            6) In CKSNS, DAF calls are followed by tests of FAILED()
///               in order to ensure that the main state loop terminates.
///
///         The "re-use interval" feature was introduced to improve speed
///         in the case where repeated, consecutive requests are satisfied
///         by the same segment.
///
///         The segment list cost algorithm was modified slightly:
///         the contribution of a file search to the cost of a list
///         is included only when the file search is completed. The
///         cost of finding the re-use interval is accounted for when
///         unbuffered searches are required.
///
///         The file table size has been increased to 1000, in order
///         to take advantage of the DAF system's new ability to load
///         1000 files.
///
///         The instrument table size has been increased to 100 in order
///         to decrease the chance of thrashing due to swapping segment
///         lists for different bodies.
///
///         Various small updates and corrections were made to the
///         comments throughout the file.
///
/// -    SPICELIB Version 4.0.0, 17-FEB-2000 (WLT)
///
///         Added the Entry point CKHAVE
///
/// -    SPICELIB Version 3.0.0, 03-MAR-1999 (WLT)
///
///         The parameter STSIZE was increased from 1000 to 4000 to
///         avoid the buffering error that exists in the CKBSR.
///
/// -    SPICELIB Version 2.0.0, 25-NOV-1992 (JML)
///
///      1) When loading a file, CKLPF now checks if the file table is
///         full only after determining that the file is not currently
///         loaded. Previously, if the file table was full and an attempt
///         was made to reload a file, an error was signaled. A new
///         exception was added as a result of this change.
///
///      2) A bug in the way that CKLPF and CKUPF clean up the instrument
///         tables after a file is unloaded was fixed.
///
///      3) Variable declarations were added to the example program
///         so that it can now be compiled.
///
///      4) The length of the elements in the array of segment
///         identifiers ( STIDNT ) was changed from 56 to 40.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 01-NOV-1990 (JML)
///
///         An initial value was assigned to the variable STATUS so
///         that an error will be signaled if CKSNS is called
///         without CKBSS ever having been called to initiate the
///         search.
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.5.0, 24-FEB-2011 (NJB)
///
///         Bug fixes:
///
///           1) In the CKSNS 'MAKE ROOM' state, when the
///              suspended activity is 'ADD TO FRONT' and no segment table
///              room is available, the instrument table's pointer to the
///              current segment list is now set to null. Previously the
///              pointer was allowed to go stale.
///
///           2) In CKUPF, the null pointer test used to determine
///              eligibility for segment list deletion now uses the .LE.
///              operator instead of the .EQ. operator.
///
///
/// -    SPICELIB Version 4.3.0, 23-OCT-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments in
///         MOVED calls in entry points CKUPF and CKSNS. Replaced header
///         reference to LDPOOL with reference to FURNSH.
///
/// -    SPICELIB Version 4.2.0, 30-DEC-2004 (NJB)
///
///         Increased STSIZE to 20000.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///            1) When a segment list is freed because the entire list
///               is contributed by a single CK file, and the list is
///               too large to be buffered, the corresponding instrument
///               table pointer is now set to null.
///
///            2) An algorithm change has eliminated a bug caused by not
///               updating the current instrument index when instrument
///               table entries having empty segment lists were compressed
///               out of the instrument. Previously the instrument table
///               pointer IINDEX could go stale after the compression.
///
///            3) When a already loaded kernel is re-opened with DAFOPR,
///               it now has its link count reset to 1 via a call to
///               DAFCLS.
///
///            4) The load routine CKLPF now resets all file numbers when
///               the next file number reaches INTMAX()-1, thereby
///               avoiding arithmetic overflow.
///
///            5) The unload routine CKUPF now calls RETURN() on entry and
///               returns if so directed.
///
///            6) In CKSNS, DAF calls are followed by tests of FAILED()
///               in order to ensure that the main state loop terminates.
///
///         The "re-use interval" feature was introduced to improve speed
///         in the case where repeated, consecutive requests are satisfied
///         by the same segment. For each instrument, the associated
///         re-use interval marks the time interval containing the previous
///         request time for which the previously returned segment provides
///         the  highest-priority data available.
///
///         The segment list cost algorithm was modified slightly:
///         the contribution of a file search to the cost of a list
///         is included only when the file search is completed. The
///         cost of finding the re-use interval is accounted for when
///         unbuffered searches are required.
///
///         The file table size has been increased to 1000, in order
///         to take advantage of the DAF system's new ability to load
///         1000 files.
///
///         The instrument table size has been increased to 100 in order
///         to decrease the chance of thrashing due to swapping segment
///         lists for different instruments.
///
///         Various small updates and corrections were made to the
///         comments throughout the file.
///
///         In order to simplify the source code, the in-line singly
///         linked list implementation of the segment table has been
///         replaced by an implementation relying on the SPICELIB
///         doubly linked list routines.
///
///
/// -    SPICELIB Version 2.0.0, 25-NOV-1992 (JML)
///
///         1) When loading a file, CKLPF now checks if the file table is
///            full only after determining that the file is not currently
///            loaded. Previously, if the file table was full and an
///            attempt was made to reload a file, an error was signaled. A
///            new exception was added as a result of this change.
///
///         2) A bug in the way that CKLPF and CKUPF clean up the
///            instrument tables after a file is unloaded was fixed.
///
///         3) Variable declarations were added to the example program
///            so that it can now be compiled.
///
///         4) The length of the elements in the array of segment
///            identifiers ( STIDNT ) was changed from 56 to 40.
///
/// -    SPICELIB Version 1.1.0, 01-NOV-1990 (JML)
///
///         An initial value was assigned to the variable STATUS so
///         that an error will be signaled if CKSNS is called
///         without CKBSS ever having been called to initiate the
///         search.
///
///
/// -    Beta Version 1.1.0, 28-AUG-1990 (MJS) (JEM)
///
///         The following changes were made as a result of the
///         NAIF CK Code and Documentation Review:
///
///            1) The variable SCLK  was changed to SCLKDP.
///            2) The variable IDENT was changed to SEGID.
///            3) The parameterized values for FTSIZE and ITSIZE were
///                increased from 5 to 20.
///            4) The parameterized value for STSIZE was increased from 100
///               to 1000.
///            5) The local variables INTDES and DPDES were changed to
///               ICD and DCD.
///            6) The extended SAVE statement was broken in to single
///               SAVE statements.
///            7) Header and internal documentation was corrected and
///               updated.
///
/// -    Beta Version 1.0.0, 14-MAR-1990 (RET) (IMU)
/// ```
pub fn ckbsr(
    ctx: &mut SpiceContext,
    fname: &str,
    handle: i32,
    inst: i32,
    sclkdp: f64,
    tol: f64,
    needav: bool,
    descr: &[f64],
    segid: &str,
    found: bool,
) -> crate::Result<()> {
    CKBSR(
        fname.as_bytes(),
        handle,
        inst,
        sclkdp,
        tol,
        needav,
        descr,
        segid.as_bytes(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKBSR ( C-kernel, buffer segments for readers )
pub fn CKBSR(
    FNAME: &[u8],
    HANDLE: i32,
    INST: i32,
    SCLKDP: f64,
    TOL: f64,
    NEEDAV: bool,
    DESCR: &[f64],
    SEGID: &[u8],
    FOUND: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //    ND         is the number of double precision components in an
    //               unpacked C-kernel descriptor.
    //
    //    NI         is the number of integer components in an unpacked
    //               C-kernel descriptor.
    //
    //    DSCSIZ     is the number of components in a packed C-kernel
    //               descriptor.  All DAF summaries have this formulaic
    //               relationship between the number of its integer and
    //               double precision components and the number of packed
    //               components.
    //

    //
    // Constants used in the doubly linked list structure:
    //

    //
    // Local variables
    //

    //
    // The file table contains the handle and file number of each file
    // that has been loaded for use with the CK readers. File
    // numbers begin at one, and are incremented until they reach a
    // value of INTMAX() - 1, at which point they are mapped to the
    // range 1:NFT, where NFT is the number of loaded CK files.
    //
    // A file number is similar to a file handle, but it is assigned
    // and used exclusively by this module. The purpose of file numbers
    // is to keep track of the order in which files are loaded and the
    // order in which they are searched.
    //
    // All names begin with FT.
    //
    //    HAN      Handle
    //    NUM      File number
    //
    // NFT is the number of currently loaded CK files. NEXT is
    // incremented whenever a new file is loaded to give the file
    // number for that file. FINDEX is the index of whatever file is
    // of current interest.
    //
    // New files are added at the end of the table. As files are
    // removed, succeeding files are moved forward to take up the
    // slack. This keeps the table ordered by file number.
    //

    //
    // The instrument table contains the beginning of the list of the
    // stored segments for each spacecraft/instrument pair, and the
    // expense at which that list was constructed. (The expense of an
    // instrument list is the number of segment descriptors examined
    // during the construction of the list.) It also contains the
    // highest and lowest file numbers searched during the construction
    // of the list.
    //
    // For each instrument, the time bounds of the "re-use interval"
    // of the last segment found are stored.  This interval is the
    // maximal interval containing the epoch of the last request for
    // data for this instrument, such that the interval is not masked
    // by higher-priority segments.  The handle, segment descriptor,
    // and segment identifier returned on the last request are also
    // stored.
    //
    // The reuse-interval is computed without regard to presence of
    // angular velocity:  all segments seen while searching for
    // a segment satisfying a request are used to define the bounds
    // of the re-use interval.
    //
    // Re-use intervals are defined on the *first* search following
    // a setup call to CKBSS.  If a search is resumed (multiple calls
    // to CKSNS are made consecutively), the re-use interval becomes
    // invalid after the first CKSNS call.
    //
    // All names begin with IT.
    //
    //    INS      Spacecraft/instrument number
    //    EXP      Expense
    //    HFS      Highest file (number) searched
    //    LFS      Lowest  file (number) searched
    //    BEG      Beginning of segment list
    //    LB       Lower bound of effective coverage interval of
    //             previous segment returned.
    //    UB       Upper bound of effective coverage interval of
    //             previous segment returned.
    //    PRVD     Previous descriptor.
    //    PRVF     Previous descriptor angular velocity flag.  Angular
    //             velocity is present when ITPRVF is non-zero.
    //    PRVI     Previous segment identifier returned.
    //    PRVH     Previous handle returned.
    //    CHKP     Logical indicating that previous segment should
    //             be checked to see whether it satisfies a request.
    //    RUEX     Expense of the re-use interval.
    //
    // NIT is the number of instruments for which segments are currently
    // being stored in the table. IINDEX is the index of whatever
    // instrument is of current interest at any given time.
    //
    // New instruments are added at the end of the table. As instruments
    // are removed, the last instrument is moved forward to take up the
    // slack. This keeps the entries in the table contiguous.
    //

    //
    // The segment table contains the handle, descriptor, and identifier
    // for each segment that has been found so far.
    //
    // The segment table is implemented as a set of arrays indexed by
    // a SPICE doubly linked list structure.  For each instrument
    // in the instrument table, there is a segment table list; each
    // node of a list points to data associated with a segment.  In
    // each list, the head node corresponds to the highest-priority
    // segment in that list, and segment priority decreases in the
    // forward direction.
    //
    // All names begin with ST.
    //
    //    IDNT     Identifier
    //    DCD      Double Precision component of descriptor
    //    HAN      Handle
    //    ICD      Integer component of descriptor
    //    POOL     Doubly linked list pool.
    //
    // New segments are added to the front or end of an instrument list
    // as appropriate, according to the rules spelled out under
    // entry point CKSNS.
    //

    //
    // Other local variables
    //

    //
    // Saved variables
    //

    //
    // Initial values
    //

    //
    // Nobody has any business calling CKBSR directly.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKBSR", ctx)?;
    SIGERR(b"SPICE(CKBOGUSENTRY)", ctx)?;
    CHKOUT(b"CKBSR", ctx)?;

    Ok(())
}

/// CK, load pointing file
///
/// Load a CK pointing file for use by the CK readers. Return that
/// file's handle, to be used by other CK routines to refer to the
/// file.
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
///  FNAME      I   Name of the CK file to be loaded.
///  HANDLE     O   Loaded file's handle.
///  FTSIZE     P   Maximum number of loaded CK files.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FNAME    is the name of a C-kernel file to be loaded.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is an integer handle assigned to the file upon loading.
///           Almost every other CK routine will subsequently use
///           this number to refer to the file.
/// ```
///
/// # Parameters
///
/// ```text
///  FTSIZE   is the maximum number of CK files that may
///           be loaded simultaneously under any circumstances.
///           FTSIZE is currently set to match the maximum number
///           of DAF files that may be loaded simultaneously.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an attempt is made to load more DAF files than is
///      specified by the parameter FTSIZE in DAF system, an error
///      is signaled by a routine in the call tree of this routine.
///
///  2)  If an attempt is made to load more files than is specified
///      by the parameter FTSIZE in the CK subsystem, and if the DAF
///      system has room to load another file, the error
///      SPICE(CKTOOMANYFILES) is signaled. The current setting of
///      FTSIZE does not allow this situation to arise: the DAF
///      system will trap the error before this routine has the
///      chance.
///
///  3)  If the file specified by FNAME can not be opened, an error
///      is signaled by a routine in the call tree of this routine.
///
///  4)  If the file specified by FNAME has already been loaded,
///      it will become the "last-loaded" file. The readers
///      search the last-loaded file first.
/// ```
///
/// # Files
///
/// ```text
///  The C-kernel file specified by FNAME is loaded. The file is
///  assigned an integer handle by CKLPF. Other CK routines will refer
///  to this file by its handle.
/// ```
///
/// # Particulars
///
/// ```text
///  Before a file can be read by the C-kernel readers, it must be
///  loaded by CKLPF, which among other things load the file into
///  the DAF subsystem.
///
///  Up to FTSIZE files may be loaded for use simultaneously, and a
///  file only has to be loaded once to become a potential search
///  target for any number of subsequent reads.
///
///  Once a C-kernel has been loaded, it is assigned a file
///  handle, which is used to keep track of the file internally, and
///  which is used by the calling program to refer to the file in all
///  subsequent calls to CK routines.
///
///  If there is room for a new file, CKLPF opens the file for
///  reading. This routine must be called prior to a call to CKGP or
///  CKGPAV.
///
///  CK readers search files loaded with CKLPF in the reverse order
///  in which they were loaded. That is, last-loaded files are
///  searched first.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in CKBSR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.0.2, 25-OCT-2021 (JDR)
///
///         Updated the header to comply with NAIF standard. Extended
///         $Particulars section.
///
/// -    SPICELIB Version 5.0.1, 30-JAN-2017 (NJB)
///
///         Corrected various spelling errors within comments.
///
/// -    SPICELIB Version 5.0.0, 17-MAR-2014 (NJB)
///
///         Updated segment pool initialization condition in entry
///         point CKLPF so that the pool is initialized only if the file
///         table is empty.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///         1) When an already loaded kernel is opened with DAFOPR,
///            it now has its link count reset to 1 via a call to
///            DAFCLS.
///
///         2) This routine now resets all file numbers when
///            the next file number reaches INTMAX()-1, thereby avoiding
///            arithmetic overflow. The numbers in the file table
///            are replaced with consecutive integers in the range
///            1 : NFT, such that the ordering of the numbers is not
///            changed. The HFS and LFS arrays are updated accordingly.
///
///         Also, the flags indicating validity of the re-use intervals
///         are set to .FALSE. here.
///
/// -    SPICELIB Version 4.0.0, 17-FEB-2000 (WLT)
///
///         Added the Entry point CKHAVE
///
/// -    SPICELIB Version 3.0.0, 03-MAR-1999 (WLT)
///
///         The parameter STSIZE was increased from 1000 to 4000 to
///         avoid the buffering error that exists in the CKBSR.
///
/// -    SPICELIB Version 2.0.0, 25-NOV-1992 (JML)
///
///         When loading a file, CKLPF now checks if the file table is
///         full only after determining that the file is not currently
///         loaded. Previously, if the file table was full and an attempt
///         was made to reload a file, an error was signaled. A new
///         exception was added as a result of this change.
///
///         A bug in the way that CKLPF and CKUPF clean up the instrument
///         tables after a file is unloaded was fixed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///         1) When a loaded kernel is opened with DAFOPR,
///            it now has its link count reset to 1 via a call to
///            DAFCLS.
///
///         2) This routine now resets all file numbers when
///            the next file number reaches INTMAX()-1, thereby avoiding
///            arithmetic overflow. The numbers in the file table
///            are replaced with consecutive integers in the range
///            1 : NFT, such that the ordering of the numbers is not
///            changed. The HFS and LFS arrays are updated accordingly.
///            HFS and LFS entries that have gone stale are set to zero.
///
///         Also, the flags indicating validity of the re-use intervals
///         are set to .FALSE. here.
///
/// -    SPICELIB Version 2.0.0, 25-NOV-1992 (JML)
///
///         Temp version for testing purposes.
///
///         When loading a file, CKLPF now checks if the file table is
///         full only after determining that the file is not currently
///         loaded. Previously, if the file table was full and an attempt
///         was made to reload a file, an error was signaled. A new
///         exception was added as a result of this change.
///
///         A bug in the way that CKLPF and CKUPF clean up the instrument
///         tables after a file is unloaded was fixed.
///
///         If as the result of loading a file that was previously loaded,
///         there are no more segments buffered for a particular
///         instrument, the counter variable for the instruments is no
///         longer incremented.
///
///         The following code fragment changed:
///
///            IF ( ITBEG( I ) .EQ. 0 ) THEN
///
///               .
///               .
///               .
///               NIT = NIT - 1
///
///            END IF
///
///            I = I + 1
///
///         This is the fix:
///
///            IF ( ITBEG( I ) .EQ. 0 ) THEN
///
///               .
///               .
///               .
///               NIT = NIT - 1
///
///            ELSE
///
///               I = I + 1
///
///            END IF
///
/// -    Beta Version 1.1.0, 28-AUG-1990 (MJS) (JEM)
///
///         Header documentation was updated, and error handling was
///         modified.
///
/// -    Beta Version 1.0.0, 14-MAR-1990 (RET) (IMU)
/// ```
pub fn cklpf(ctx: &mut SpiceContext, fname: &str, handle: &mut i32) -> crate::Result<()> {
    CKLPF(fname.as_bytes(), handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKLPF ( CK, load pointing file )
pub fn CKLPF(FNAME: &[u8], HANDLE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;
    let mut J: i32 = 0;
    let mut NXTSEG: i32 = 0;
    let mut P: i32 = 0;
    let mut TAIL: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKLPF", ctx)?;
    }

    //
    // Don't allow a search to continue after loading a file; a new
    // search should be re-started.
    //
    fstr::assign(&mut save.STATUS, b"BOGUS ENTRY");

    //
    // Since a current search cannot be continued at this point,
    // free the left-over partial list searched in the
    // 'CHECK PARTIAL LIST' state, if the list is present.
    //
    if save.FRESUB {
        //
        // Return the partial list to the free list.
        //
        TAIL = LNKTL(save.SLBEG, save.STPOOL.as_slice(), ctx)?;
        LNKFSL(save.SLBEG, TAIL, save.STPOOL.as_slice_mut(), ctx)?;

        save.FRESUB = false;
    }

    //
    // Any time we load a file, there is a possibility that the
    // re-use intervals are invalid because they're been superseded
    // by higher-priority data.  Since we're not going to examine
    // the loaded file, simply indicate that all of the re-use
    // intervals are invalid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NIT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.ITCHKP[I] = false;
            I += m3__;
        }
    }

    //
    // Nothing works unless at least one file has been loaded, so this
    // is as good a place as any to initialize the segment table pool.
    // We want to avoid unnecessary initializations, so we only
    // initialize the list when no files are loaded. It's quite possible
    // to have files loaded and an empty instrument table, so we don't
    // want to re-initialize just because there are no instrument table
    // entries.
    //
    if (save.NFT == 0) {
        LNKINI(STSIZE, save.STPOOL.as_slice_mut(), ctx)?;
    }

    //
    // To load a new file, first try to open it for reading.
    //
    DAFOPR(FNAME, HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"CKLPF", ctx)?;
        return Ok(());
    }

    //
    // Determine if the file is already in the table.
    //
    save.FINDEX = ISRCHI(*HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX > 0) {
        //
        // The last call we made to DAFOPR added another DAF link to
        // the CK file.  Remove this link.
        //
        DAFCLS(*HANDLE, ctx)?;

        //
        // Handle is already in the table.  Remove it.
        //
        save.NFT = (save.NFT - 1);

        {
            let m1__: i32 = save.FINDEX;
            let m2__: i32 = save.NFT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.FTHAN[I] = save.FTHAN[(I + 1)];
                save.FTNUM[I] = save.FTNUM[(I + 1)];
                I += m3__;
            }
        }

        //
        // Unlink any segments that came from this file.
        //
        I = 1;

        while (I <= save.NIT) {
            P = save.ITBEG[I];

            while (P > 0) {
                //
                // Find the successor of P, if any.
                //
                NXTSEG = LNKNXT(P, save.STPOOL.as_slice(), ctx)?;

                if (save.STHAN[P] == *HANDLE) {
                    //
                    // The segment corresponding to node P came from
                    // the file we're unloading.  Delete the node for
                    // P from the segment list for instrument I; if P happens
                    // to be the head node for instrument I's segment list,
                    // make the successor of P the head of the list.
                    //
                    LNKFSL(P, P, save.STPOOL.as_slice_mut(), ctx)?;

                    if (P == save.ITBEG[I]) {
                        save.ITBEG[I] = NXTSEG;
                    }
                }
                //
                // Update P.
                //
                P = NXTSEG;
            }

            //
            // If the list for this instrument is now empty, shorten the
            // current table by one: put all the entries for the last
            // instrument in the table into the space occupied by the
            // one we've deleted.
            //
            if (save.ITBEG[I] <= 0) {
                //
                // Because all of the re-use intervals are invalid, we need
                // not copy the saved items associated with them.  The
                // items not copied are
                //
                //    ITCHKP
                //    ITLB
                //    ITPRVD
                //    ITPRVF
                //    ITPRVH
                //    ITPRVI
                //    ITRUEX
                //    ITUB
                //
                save.ITINS[I] = save.ITINS[save.NIT];
                save.ITEXP[I] = save.ITEXP[save.NIT];
                save.ITHFS[I] = save.ITHFS[save.NIT];
                save.ITLFS[I] = save.ITLFS[save.NIT];
                save.ITBEG[I] = save.ITBEG[save.NIT];

                save.NIT = (save.NIT - 1);
            } else {
                I = (I + 1);
            }
        }
    } else {
        //
        // This is a new file.  Make sure that there are unused slots
        // in the file table.
        //
        if (save.NFT == FTSIZE) {
            DAFCLS(*HANDLE, ctx)?;

            SETMSG(b"Number of files loaded is at a maximum, as specified by the parameter FTSIZE, the value of which is #. You will need to either load fewer files, or change the parameter FTSIZE.", ctx);
            ERRINT(b"#", FTSIZE, ctx);
            SIGERR(b"SPICE(CKTOOMANYFILES)", ctx)?;
            CHKOUT(b"CKLPF", ctx)?;
            return Ok(());
        }
    }

    //
    // Determine the next file number.
    //
    if (save.NEXT < (INTMAX() - 1)) {
        save.NEXT = (save.NEXT + 1);
    } else {
        //
        // The user is to be congratulated:  we've run out of file
        // numbers.
        //
        // Re-set the valid file numbers so they lie in the range 1:NFT,
        // with the Ith file in the file table having file number I.
        // First update the LFS and HFS components of the instrument table
        // according to this mapping.
        //
        // Set any instrument table entries that are lower than FTNUM(1)
        // to zero.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NIT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                //
                // Re-map the HFS table for the Ith instrument.
                //
                J = ISRCHI(save.ITHFS[I], save.NFT, save.FTNUM.as_slice());

                if (J > 0) {
                    //
                    // The highest file searched for instrument I is the Jth
                    // file in the file table.
                    //
                    save.ITHFS[I] = J;
                } else {
                    //
                    // The highest file searched for instrument I is not in the
                    // file table.  This occurs when the highest file searched
                    // has been unloaded.  Note that this assignment makes all
                    // files appear to be "new" when a lookup for instrument
                    // I is performed.
                    //
                    save.ITHFS[I] = 0;
                }

                //
                // Re-map the LFS table for the Ith instrument.
                //
                J = ISRCHI(save.ITLFS[I], save.NFT, save.FTNUM.as_slice());

                if (J > 0) {
                    //
                    // The lowest file searched for instrument I is the Jth file
                    // in the file table.
                    //
                    save.ITLFS[I] = J;
                } else {
                    //
                    // The lowest file searched for instrument I is not in the
                    // file table.  This occurs when the lowest file searched
                    // has been unloaded.  Zero out both the lowest and
                    // highest file searched to force reconstruction of the
                    // list.
                    //
                    save.ITLFS[I] = 0;
                    save.ITHFS[I] = 0;
                }

                I += m3__;
            }
        }

        //
        // Re-map the file number table itself.
        //
        {
            let m1__: i32 = 1;
            let m2__: i32 = save.NFT;
            let m3__: i32 = 1;
            I = m1__;
            for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                save.FTNUM[I] = I;

                I += m3__;
            }
        }

        //
        // Assign a new file number.
        //
        save.NEXT = (save.NFT + 1);
    }

    //
    // Now add this file to file table.
    //
    save.NFT = (save.NFT + 1);
    save.FTHAN[save.NFT] = *HANDLE;
    save.FTNUM[save.NFT] = save.NEXT;

    CHKOUT(b"CKLPF", ctx)?;
    Ok(())
}

/// CK, Unload pointing file
///
/// Unload a CK pointing file so that it will no longer be searched
/// by the readers.
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
///  HANDLE     I   Handle of CK file to be unloaded
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the integer handle assigned to the CK file upon
///           loading.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  Unloading a file that has not been loaded is a no-op.
///      No error is signaled.
/// ```
///
/// # Files
///
/// ```text
///  The file referred to by HANDLE is unloaded.
/// ```
///
/// # Particulars
///
/// ```text
///  Unloading a file with CKUPF removes that file from consideration
///  by the CK readers. In doing so, it frees up space for another
///  file to be loaded.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in CKBSR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.3.2, 16-JUL-2020 (JDR)
///
///         Updated the header to comply with NAIF standard.
///
/// -    SPICELIB Version 4.3.1, 30-JAN-2017 (NJB)
///
///         Corrected various spelling errors within comments.
///
/// -    SPICELIB Version 4.3.0, 24-FEB-2011 (NJB)
///
///         Bug fix: the null pointer test used to determine eligibility
///         for segment list deletion now uses the .LE. operator instead
///         of the .EQ. operator.
///
/// -    SPICELIB Version 4.2.0, 08-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MOVED call.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///            1) This routine now calls RETURN() on entry and
///               returns if so directed.
///
///         Also, the flags indicating validity of those re-use intervals
///         whose data comes from the unloaded file are set to .FALSE.
///
/// -    SPICELIB Version 4.0.0, 17-FEB-2000 (WLT)
///
///         Added the Entry point CKHAVE.
///
/// -    SPICELIB Version 3.0.0, 03-MAR-1999 (WLT)
///
///         The parameter STSIZE was increased from 1000 to 4000 to
///         avoid the buffering error that exists in the CKBSR.
///
/// -    SPICELIB Version 2.0.0, 25-NOV-1992 (JML)
///
///         A bug in the way that CKLPF and CKUPF clean up the instrument
///         tables after a file is unloaded was fixed.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.3.0, 24-FEB-2011 (NJB)
///
///         Bug fix: the null pointer test used to determine eligibility
///         for segment list deletion now uses the .LE. operator instead
///         of the .EQ. operator.
///
/// -    SPICELIB Version 4.2.0, 08-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MOVED call.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///            1) This routine now calls RETURN() on entry and
///               returns if so directed.
///
///         Also, the flags indicating validity of those re-use intervals
///         whose data comes from the unloaded file are set to .FALSE.
///
/// -    SPICELIB Version 2.0.0, 25-NOV-1992 (JML)
///
///         1) A bug in the way that CKLPF and CKUPF clean up the
///            instrument tables after a file is unloaded was fixed.
///
///            If as the result of unloading a file there are no more
///            segments buffered for a particular instrument, the counter
///            variable for the instruments in the instrument table is no
///            longer incremented.
///
///            The following code fragment changed:
///
///               IF ( ITBEG( I ) .EQ. 0 ) THEN
///
///                  .
///                  .
///                  .
///                  NIT = NIT - 1
///
///               END IF
///
///               I = I + 1
///
///            This is the fix:
///
///               IF ( ITBEG( I ) .EQ. 0 ) THEN
///
///                  .
///                  .
///                  .
///                  NIT = NIT - 1
///
///               ELSE
///
///                  I = I + 1
///
///               END IF
///
/// -    Beta Version 1.0.1, 29-AUG-1990 (MJS) (JEM)
///
///          Comments were updated.
///
/// -    Beta Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
pub fn ckupf(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    CKUPF(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKUPF ( CK, Unload pointing file )
pub fn CKUPF(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut I: i32 = 0;
    let mut NXTSEG: i32 = 0;
    let mut P: i32 = 0;
    let mut TAIL: i32 = 0;

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKUPF", ctx)?;

    //
    // Don't allow a search to continue after unloading a file; a new
    // search should be re-started.
    //
    fstr::assign(&mut save.STATUS, b"BOGUS ENTRY");

    //
    // Since a current search cannot be continued at this point,
    // free the left-over partial list searched in the
    // 'CHECK PARTIAL LIST' state, if the list is present.
    //
    if save.FRESUB {
        //
        // Return the partial list to the free list.
        //
        TAIL = LNKTL(save.SLBEG, save.STPOOL.as_slice(), ctx)?;
        LNKFSL(save.SLBEG, TAIL, save.STPOOL.as_slice_mut(), ctx)?;

        save.FRESUB = false;
    }

    //
    // All of the stored segments from the file must be removed
    // from the segment table (by returning the corresponding nodes
    // to the segment table pool.)
    //
    // Don't do anything if the given handle is not in the file table.
    //
    save.FINDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

    if (save.FINDEX == 0) {
        CHKOUT(b"CKUPF", ctx)?;
        return Ok(());
    }
    //
    //
    // First get rid of the entry in the file table. Close the file
    // before wiping out the handle.
    //
    DAFCLS(save.FTHAN[save.FINDEX], ctx)?;

    save.NFT = (save.NFT - 1);

    {
        let m1__: i32 = save.FINDEX;
        let m2__: i32 = save.NFT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            save.FTHAN[I] = save.FTHAN[(I + 1)];
            save.FTNUM[I] = save.FTNUM[(I + 1)];
            I += m3__;
        }
    }

    //
    // Check each instrument list individually. Note that the first
    // node on each list, having no predecessor, must be handled
    // specially.
    //
    I = 1;

    while (I <= save.NIT) {
        P = save.ITBEG[I];

        while (P > 0) {
            NXTSEG = LNKNXT(P, save.STPOOL.as_slice(), ctx)?;

            if (save.STHAN[P] == HANDLE) {
                if (P == save.ITBEG[I]) {
                    save.ITBEG[I] = NXTSEG;
                }
                //
                // Free this segment table entry.
                //
                LNKFSL(P, P, save.STPOOL.as_slice_mut(), ctx)?;
            }

            P = NXTSEG;
        }

        //
        // If the list for this instrument is now empty, shorten the
        // current table by one: put all the entries for the last
        // instrument in the table into the space occupied by the
        // one we've deleted.
        //
        if (save.ITBEG[I] <= 0) {
            if (I != save.NIT) {
                save.ITINS[I] = save.ITINS[save.NIT];
                save.ITEXP[I] = save.ITEXP[save.NIT];
                save.ITHFS[I] = save.ITHFS[save.NIT];
                save.ITLFS[I] = save.ITLFS[save.NIT];
                save.ITBEG[I] = save.ITBEG[save.NIT];
                save.ITLB[I] = save.ITLB[save.NIT];
                save.ITUB[I] = save.ITUB[save.NIT];
                save.ITPRVF[I] = save.ITPRVF[save.NIT];
                save.ITPRVH[I] = save.ITPRVH[save.NIT];
                let val = save.ITPRVI.get(save.NIT).to_vec();
                fstr::assign(save.ITPRVI.get_mut(I), &val);
                save.ITCHKP[I] = save.ITCHKP[save.NIT];
                save.ITRUEX[I] = save.ITRUEX[save.NIT];

                MOVED(
                    &save.ITPRVD.subarray([1, save.NIT]).to_vec(),
                    DSCSIZ,
                    save.ITPRVD.subarray_mut([1, I]),
                );
            }

            save.NIT = (save.NIT - 1);
        } else {
            I = (I + 1);
        }
    }

    //
    // Any time we unload a file, we may be removing the file
    // providing data for the re-use interval for one or more
    // instruments.  For each instrument, if the handle associated
    // with the re-use interval happens to be that of the file
    // we're unloading, indicate that the re-use interval is invalid.
    //
    {
        let m1__: i32 = 1;
        let m2__: i32 = save.NIT;
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            if save.ITCHKP[I] {
                if (save.ITPRVH[I] == HANDLE) {
                    save.ITCHKP[I] = false;
                }
            }

            I += m3__;
        }
    }

    CHKOUT(b"CKUPF", ctx)?;
    Ok(())
}

/// CK, begin search for segment
///
/// Initiate search through loaded files to find segments applicable
/// to the spacecraft instrument and time specified.
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
///  INST       I   Spacecraft and instrument ID.
///  SCLKDP     I   Encoded spacecraft clock time.
///  TOL        I   Time tolerance.
///  NEEDAV     I   Is there a need for angular velocity?
/// ```
///
/// # Detailed Input
///
/// ```text
///  INST     is the NAIF ID of an instrument.
///
///  SCLKDP   is an encoded spacecraft clock time.
///
///  TOL      is a time tolerance, measured in the same units as
///           encoded spacecraft clock.
///
///  NEEDAV   indicates whether or not angular velocity data is
///           required.
///
///           If .TRUE., only segments containing pointing and angular
///           velocity data will be checked. If .FALSE., segments
///           containing just pointing data will also be considered.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If no files have been loaded, the error SPICE(NOLOADEDFILES)
///      is signaled.
/// ```
///
/// # Files
///
/// ```text
///  All files loaded by CKLPF are potential search targets for
///  CKSNS.
/// ```
///
/// # Particulars
///
/// ```text
///  CKBSS sets up a search for segments by CKSNS. It records the
///  instrument and time to be searched for, and whether to require
///  segments containing angular velocity data. If angular velocity
///  data are required, only segments containing angular velocity
///  data will be returned by CKSNS. If angular velocity data are
///  not required, segments returned by CKSNS may or may not contain
///  angular velocity data.
///
///  CKBSS determines the first task that CKSNS will have to perform
///  if it is called to get an applicable segment.
///
///  A segment matches the CKBSS/CKSNS search criteria when the
///  following statements are true.
///
///     1) INST matches the instrument number for the segment.
///
///     2) The time interval [SCLKDP - TOL, SCLKDP + TOL] intersects
///        the time interval of the segment.
///
///     3) If angular velocity data is required, as indicated by
///        NEEDAV, the segment contains angular velocity data.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in CKBSR.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.1.2, 13-JAN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Moved details related to search criteria and conditions to meet
///         it from the $Detailed_Input to $Particulars.
///
///         Fixed typo in the long error message ('needs must' -> 'must').
///
/// -    SPICELIB Version 4.1.1, 30-JAN-2017 (NJB)
///
///         Corrected various spelling errors within comments.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Updated to support new doubly-linked list implementation:
///         partial segment list that cannot be buffered is now
///         deallocated here rather than in CKSNS. Minor changes to
///         comments were made as well.
///
/// -    SPICELIB Version 4.0.0, 17-FEB-2000 (WLT)
///
///         Added the Entry point CKHAVE.
///
/// -    SPICELIB Version 3.0.0, 03-MAR-1999 (WLT)
///
///         The parameter STSIZE was increased from 1000 to 4000 to
///         avoid the buffering error that exists in the CKBSR.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Updated to support new doubly-linked list implementation:
///         partial segment list that cannot be buffered is now
///         deallocated here rather than in CKSNS. Minor changes to
///         comments were made as well.
///
/// -    Beta Version 1.1.0, 28-AUG-1990 (MJS) (JEM)
///
///         The following changes were made as a result of the
///         NAIF CK Code and Documentation Review:
///
///           1) The variable SCLK was changed to SCLKDP.
///           2) Header documentation was updated.
///
/// -    Beta Version 1.0.0, 20-APR-1990 (RET) (IMU)
/// ```
pub fn ckbss(
    ctx: &mut SpiceContext,
    inst: i32,
    sclkdp: f64,
    tol: f64,
    needav: bool,
) -> crate::Result<()> {
    CKBSS(inst, sclkdp, tol, needav, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKBSS ( CK, begin search for segment )
pub fn CKBSS(
    INST: i32,
    SCLKDP: f64,
    TOL: f64,
    NEEDAV: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut TAIL: i32 = 0;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"CKBSS", ctx)?;
    }

    //
    // If we're starting a new search after passing through the
    // 'CHECK PARTIAL LIST' state, free the left-over partial list
    // that was searched in that state, if necessary.
    //
    if save.FRESUB {
        //
        // Return the partial list to the free list.
        //
        TAIL = LNKTL(save.SLBEG, save.STPOOL.as_slice(), ctx)?;
        LNKFSL(save.SLBEG, TAIL, save.STPOOL.as_slice_mut(), ctx)?;

        save.FRESUB = false;
    }

    //
    // Make copies of the instrument ID code and angular velocity flag.
    // Save the request time itself.
    //
    // And form the endpoints of the acceptable time interval using the
    // input time and time tolerance.
    //
    save.SCINST = INST;
    save.ALPHA = (SCLKDP - TOL);
    save.OMEGA = (SCLKDP + TOL);
    save.AVNEED = NEEDAV;
    save.REQT = SCLKDP;
    save.SAVTOL = TOL;

    //
    // There must be at least one file loaded.
    //
    if (save.NFT == 0) {
        SETMSG(
            b"At least one CK file must be loaded by CKLPF before beginning a search.",
            ctx,
        );
        SIGERR(b"SPICE(NOLOADEDFILES)", ctx)?;
        CHKOUT(b"CKBSS", ctx)?;
        return Ok(());
    }

    //
    // The stack of suspended tasks is empty.
    //
    save.TOP = 0;

    //
    // Is the instrument already in the instrument table?  The answer
    // determines what the first task for CKSNS will be.
    //
    save.IINDEX = ISRCHI(save.SCINST, save.NIT, save.ITINS.as_slice());

    if (save.IINDEX == 0) {
        fstr::assign(&mut save.STATUS, b"NEW INSTRUMENT");
    } else {
        //
        // Set the status so that CKSNS will determine whether to check
        // the segment list, search new files, or return data from the
        // re-use interval.
        //
        fstr::assign(&mut save.STATUS, b"?");
    }

    //
    // Indicate a new search has started.
    //
    save.NEWSCH = true;

    CHKOUT(b"CKBSS", ctx)?;
    Ok(())
}

/// C-kernel, Select next segment
///
/// Search through loaded files to find a segment matching the
/// requested instrument, time, and need for angular velocity,
/// buffering segment descriptors, identifiers, and handles in the
/// process to minimize file reads.
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
///  HANDLE     O   Handle of file containing the applicable segment.
///  DESCR      O   Descriptor of the applicable segment.
///  SEGID      O   Identifier of the applicable segment.
///  FOUND      O   .TRUE. if a segment was found.
/// ```
///
/// # Detailed Output
///
/// ```text
///  HANDLE   is an integer handle of the file containing the
///           segment matching the instrument and time
///           specifications made in the last call to CKBSS.
///
///  DESCR,
///  SEGID    are the descriptor and identifier of the segment found
///           which matches the instrument and time specifications
///           made in the last call to CKBSS.
///
///  FOUND    is .TRUE. if an applicable segment was found, .FALSE.
///           otherwise. If FOUND is .FALSE., the values of the
///           other arguments are meaningless.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If CKSNS is called without CKBSS ever having been called,
///      the error SPICE(CALLCKBSSFIRST) is signaled.
///
///  2)  If no segment is found that matches the search criteria,
///      FOUND is set to .FALSE., but the values of HANDLE, DESCR,
///      and SEGID will be meaningless.
/// ```
///
/// # Files
///
/// ```text
///  All files loaded by CKLPF are potential search targets for
///  CKSNS. The files are all referred to by their integer handles.
/// ```
///
/// # Particulars
///
/// ```text
///  CKSNS is used to locate segments based on the search criteria
///  established by the most recent call to CKBSS. When a segment
///  is found it will have the following characteristics:
///
///     1) Its instrument will match the instrument specified in the
///        call to CKBSS.
///
///     2) Its time interval will intersect the time interval
///
///           [SCLKDP - TOL, SCLKDP + TOL],
///
///        where SCLKDP and TOL were specified in the call to CKBSS.
///
///     3) If there is a need for angular velocity data, as specified
///        by NEEDAV in the call to CKBSS, a returned segment
///        will contain angular velocity data. If there is no need
///        for such data, the returned segment may or may not contain
///        angular velocity data.
///
///  The first call to CKSNS following a call to CKBSS starts a search
///  through loaded files and either returns the first applicable
///  segment, or indicates that no segment was found.
///
///  CKSNS searches through last-loaded files first. Individual
///  files are searched backwards, so that segments that were inserted
///  last into the file get checked first.
///
///  Subsequent calls to CKSNS pick up the search exactly where the
///  previous calls left off. If a segment is not found, future calls
///  will also indicate that no segment could be found, until a new
///  search is begun.
///
///  CKSNS also buffers segment descriptors and identifiers, to
///  attempt to minimize file reads.
/// ```
///
/// # Examples
///
/// ```text
///  See $Examples in CKBSR.
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  This subroutine assumes that a search has been initiated by
///      a call to CKBSS.
///
///  2)  When a CK file is loaded or unloaded, a new search must
///      be started via a call to CKBSS before this routine may
///      be called.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  J.M. Lynch         (JPL)
///  W.L. Taber         (JPL)
///  R.E. Thurman       (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.6.0, 13-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard.
///
///         Relocated initialization of FOUND so it is always
///         executed, even if an error state is indicated by RETURN().
///
/// -    SPICELIB Version 4.5.1, 30-JAN-2017 (NJB)
///
///         Corrected various spelling errors within comments.
///
/// -    SPICELIB Version 4.5.0, 24-FEB-2011 (NJB)
///
///         Bug fix: in the 'MAKE ROOM' state, when the suspended activity
///         is 'ADD TO FRONT' and no segment table room is available, the
///         instrument table's pointer to the current segment list is now
///         set to null. Previously the pointer was allowed to go stale.
///
/// -    SPICELIB Version 4.2.0, 08-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MOVED call.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///            1) When a segment list is freed because the entire list
///               is contributed by a single CK file, and the list is
///               too large to be buffered, the corresponding instrument
///               table pointer is now set to null.
///
///            2) An algorithm change has eliminated a bug caused by not
///               updating the current instrument index when instrument
///               table entries  having empty segment lists were compressed
///               out of the instrument table. Previously the instrument
///               table pointer IINDEX could go stale after the
///               compression.
///
///            3) DAF calls are now followed by tests of FAILED()
///               in order to ensure that the main state loop terminates.
///
///         The "re-use interval" feature was introduced to improve speed
///         in the case where repeated, consecutive requests are satisfied
///         by the same segment.
///
///         The segment list cost algorithm was modified slightly:
///         the contribution of a file search to the cost of a list
///         is included only when the file search is completed. The
///         cost of finding the re-use interval is accounted for when
///         unbuffered searches are required.
///
///         The file table size has been increased to 1000, in order
///         to take advantage of the DAF system's new ability to load
///         1000 files.
///
///         The instrument table size has been increased to 100 in order to
///         decrease the chance of thrashing due to swapping segment
///         lists for different bodies.
///
///         Various small updates and corrections were made to the
///         comments throughout the file.
///
/// -    SPICELIB Version 4.0.0, 17-FEB-2000 (WLT)
///
///         Added the Entry point CKHAVE.
///
/// -    SPICELIB Version 3.0.0, 03-MAR-1999 (WLT)
///
///         The parameter STSIZE was increased from 1000 to 4000 to
///         avoid the buffering error that exists in the CKBSR.
///
/// -    SPICELIB Version 1.1.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.1.0, 01-NOV-1990 (JML)
///
///         A check on the initial value of the variable STATUS
///         was added in order to detect the situation in which
///         CKBSS was never called to initiate a search.
///
///
/// -    SPICELIB Version 1.0.0, 07-SEP-1990 (RET) (IMU)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.5.0, 24-FEB-2011 (NJB)
///
///         Bug fix: in the 'MAKE ROOM' state, when the suspended activity
///         is 'ADD TO FRONT' and no segment table room is available, the
///         instrument table's pointer to the current segment list is now
///         set to null. Previously the pointer was allowed to go stale.
///
/// -    SPICELIB Version 4.2.0, 08-SEP-2005 (NJB)
///
///         Updated to remove non-standard use of duplicate arguments
///         in MOVED call.
///
/// -    SPICELIB Version 4.1.0, 20-NOV-2001 (NJB)
///
///         Bug fixes:
///
///            1) When a segment list is freed because the entire list
///               is contributed by a single CK file, and the list is
///               too large to be buffered, the corresponding instrument
///               table pointer is now set to null.
///
///            2) An algorithm change has eliminated a bug caused by not
///               updating the current instrument index when instrument
///               table entries  having empty segment lists were compressed
///               out of the instrument table. Previously the instrument
///               table pointer IINDEX could go stale after the
///               compression.
///
///            3) DAF calls are now followed by tests of FAILED()
///               in order to ensure that the main state loop terminates.
///
///         The "re-use interval" feature was introduced to improve speed
///         in the case where repeated, consecutive requests are satisfied
///         by the same segment.
///
///         The segment list cost algorithm was modified slightly:
///         the contribution of a file search to the cost of a list
///         is included only when the file search is completed. The
///         cost of finding the re-use interval is accounted for when
///         unbuffered searches are required.
///
///         The file table size has been increased to 1000, in order
///         to take advantage of the DAF system's new ability to load
///         1000 files.
///
///         The instrument table size has been increased to 100 in order to
///         decrease the chance of thrashing due to swapping segment
///         lists for different instruments.
///
///         Various small updates and corrections were made to the
///         comments throughout the file.
///
///
/// -    SPICELIB Version 1.1.0, 01-NOV-1990 (JML)
///
///         A check on the initial value of the variable STATUS
///         was added in order to detect the situation in which
///         CKBSS was never called to initiate a search.
///
///
/// -    Beta Version 1.1.0, 28-AUG-1990 (MJS) (JEM)
///
///         The following changes were made as a result of the
///         NAIF CK Code and Documentation Review:
///
///            1) The variable IDENT was changed to SEGID.
///            2) The local variables INTDES and DPDES were changed to
///               ICD and DCD.
///            3) Header and internal documentation was corrected and
///               updated.
///
/// -    Beta Version 1.0.0, 20-APR-1990 (RET) (IMU)
/// ```
pub fn cksns(
    ctx: &mut SpiceContext,
    handle: &mut i32,
    descr: &mut [f64],
    segid: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    CKSNS(
        handle,
        descr,
        fstr::StrBytes::new(segid).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CKSNS ( C-kernel, Select next segment )
pub fn CKSNS(
    HANDLE: &mut i32,
    DESCR: &mut [f64],
    SEGID: &mut [u8],
    FOUND: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DESCR = DummyArrayMut::new(DESCR, 1..);
    let mut DOING = [b' '; SLEN as usize];
    let mut STACK = ActualCharArray::new(SLEN, 1..=2);
    let mut URGENT = [b' '; SLEN as usize];
    let mut DCD = StackArray::<f64, 2>::new(1..=ND);
    let mut CHEAP: i32 = 0;
    let mut COST: i32 = 0;
    let mut HEAD: i32 = 0;
    let mut I: i32 = 0;
    let mut ICD = StackArray::<i32, 6>::new(1..=NI);
    let mut MINEXP: i32 = 0;
    let mut NEW: i32 = 0;
    let mut P: i32 = 0;
    let mut TAIL: i32 = 0;
    let mut FNDHAN: bool = false;

    //
    // Nothing's been found yet.
    //
    *FOUND = false;

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"CKSNS", ctx)?;

    //
    // Initialize the segment list pointer to the saved value from
    // the previous pass through this routine, if any.
    //
    P = save.SAVEP;

    //
    // CKSNS buffers segment descriptors and identifiers, to
    // attempt to minimize file reads. Buffering segments involves
    // maintaining three tables:  the file table, the instrument table,
    // and the segment table. CKSNS is broken down into various tasks,
    // described in the code below, which perform these manipulations.
    //
    // A description of the components of each table is provided in
    // the declarations section of CKBSR.
    //
    // Basically, the buffering is performed as follows: once a request
    // for a segment for a particular instrument is made, if there are
    // no adequate entries in the buffer already, a search is made
    // through loaded files for applicable segments.  Every segment
    // pertaining to that instrument in a searched file is buffered,
    // before a check of the current buffer is made.  If the search
    // doesn't turn up a segment matching the specified search criteria
    // the next file is searched and new segments are added to the list,
    // and so on.
    //
    // The information in the segment table (ST) is stored in a
    // doubly-linked list. Each node in the list contains several
    // individual pieces of data, which are stored in parallel
    // arrays.
    //
    // In the following loop, we will try to simplify things by
    // doing exactly one thing on each pass through the loop.
    // After each pass, the status of the loop (STATUS) will be
    // adjusted to reflect the next thing that needs to be done.
    // The first task is set by CKBSS.
    //
    // Occasionally, the current task will have to be interrupted
    // until another task can be carried out. (For example, when
    // collecting new segments, an interrupt might place a segment
    // at the front or end of the current instrument list; when placing
    // the segment on the list, a second interrupt might free
    // room in the segment table in order to allow the addition
    // to proceed.) In this case, the current task will be saved and
    // restored after the more urgent task has been completed.
    //
    // The loop can terminate in only one of two ways (unless an error
    // occurs). First, if an applicable segment is found in the segment
    // table, the handle, descriptor, and identifier for the segment
    // are returned immediately.  Second, if the table does not contain
    // an applicable segment, and if no files remain to be searched,
    // the loop terminates normally, and no data are returned.
    //
    // The status is saved on exit, however, so that subsequent calls
    // will resume a search exactly where previous calls left off.
    //
    // Each status is described below.
    //
    // 'NEW INSTRUMENT'
    //
    //    This indicates that the specified spacecraft/instrument has
    //    no segments stored for it at all. It must be added to the
    //    instrument table.  (This is followed immediately by an
    //    OLD FILES search, in which every file loaded is considered an
    //    old file.)
    //
    // 'NEW FILES'
    //
    //    This indicates that at least one new file has been added
    //    since the last time the segment list for the specified
    //    instrument was searched. Find the oldest of these new files,
    //    and begin a NEW SEGMENTS search in forward order for
    //    segments to add to the front of the list.
    //
    // 'NEW SEGMENTS'
    //
    //    Continue a NEW FILES search, adding segments for the specified
    //    instrument to the front of the list.
    //
    // 'OLD FILES'
    //
    //    This indicates that although the list has been searched
    //    and found to contain no applicable segment, some of the
    //    older files remain to be searched. Find the newest of these
    //    old files, and begin an OLD SEGMENTS search in backward order.
    //
    // 'OLD SEGMENTS'
    //
    //    Continue an OLD FILES search, adding segments for the specified
    //    instrument to the end of the list.
    //
    // 'CHECK LIST'
    //
    //    This indicates that the list is ready to be searched,
    //    either because no new files have been added, or because
    //    segments from a new file or an old file have recently
    //    been added.
    //
    //    The list is never checked until all new files have been
    //    searched.
    //
    //    If an applicable segment is found, it is returned.
    //
    // 'MAKE ROOM' (Interrupt)
    //
    //    This indicates that one of the instruments must be removed,
    //    along with its stored segments, to make room for another
    //    instrument or segment.  The instrument (other than the
    //    specified instrument) with the smallest expense is selected
    //    for this honor.
    //
    // 'ADD TO FRONT' (Interrupt)
    //
    //    This indicates that a segment has been found (during the
    //    course of a NEW FILES search) and must be added to the front
    //    of the list.
    //
    // 'ADD TO END' (Interrupt)
    //
    //    This indicates that a segment has been found (during the
    //    course of an OLD FILES search) and must be added to the end
    //    of the list.
    //
    // 'PREPARE PARTIAL LIST'
    //
    //    This indicates that an attempt to 'MAKE ROOM' failed when
    //    trying to 'ADD TO END' because all of the segments in the
    //    table were for the instrument being searched on.  The partial
    //    list is found that contains all of the segments that were in
    //    the process of being added to the table for the current old
    //    file.  Next a 'CHECK PARTIAL LIST' is performed. Following
    //    that, a 'SEARCH W/O BUFF' is performed on all unsearched
    //    files.
    //
    // 'CHECK PARTIAL LIST'
    //
    //    This indicates that a portion of the list can't be buffered.
    //    Before this portion is freed, it is to be checked for
    //    applicable segments.
    //
    // 'SEARCH W/O BUFF'
    //
    //    This indicates that the segment table was too small to handle
    //    all of the segments for the current instrument, and that the
    //    remaining unchecked old files should be searched for applicable
    //    segments, without buffering the segments.
    //
    // 'SUSPEND'
    //
    //    This indicates that the current task (DOING) should be
    //    interrupted until a more urgent task (URGENT) can be
    //    carried out. The current task is placed on a stack for
    //    safekeeping.
    //
    // 'RESUME'
    //
    //    This indicates that the most recently interrupted task
    //    should be resumed immediately.
    //
    // '?'
    //
    //    This indicates that the next task is not immediately
    //    apparent: if new files exist, they should be searched;
    //    otherwise the list should be checked.
    //
    // 'HOPELESS'
    //
    //    This indicates that the table does not contain an applicable
    //    segment, and no files remain to be searched.
    //
    //  'BOGUS ENTRY'
    //
    //    This is the initial value of STATUS and indicates that no
    //    call to CKBSS was ever made. If this is the case then an
    //    error will be signaled.
    //

    if fstr::eq(&save.STATUS, b"BOGUS ENTRY") {
        SETMSG(b"Must begin a search by calling CKBSS first.", ctx);
        SIGERR(b"SPICE(CALLCKBSSFIRST)", ctx)?;
        CHKOUT(b"CKSNS", ctx)?;
        return Ok(());
    }

    while fstr::ne(&save.STATUS, b"HOPELESS") {
        //
        // If new files have been added, they have to be searched.
        // Otherwise, go right to the list of stored segments.
        //
        if fstr::eq(&save.STATUS, b"?") {
            //
            // There are two ways to get to this point.
            //
            // 1)  Status may have been set to '?' by CKBSS.
            //
            // 2)  Status was set to '?' by the NEW SEGMENTS block
            //     of code as the result of finishing the read of
            //     a new file.
            //

            if (save.ITHFS[save.IINDEX] < save.FTNUM[save.NFT]) {
                fstr::assign(&mut save.STATUS, b"NEW FILES");
            } else {
                //
                // Much of the time, the segment used to satisfy the
                // previous request will also satisfy the current
                // request.  Check whether this is the case.
                //
                if save.ITCHKP[save.IINDEX] {
                    //
                    // The previous segment found for the current instrument
                    // is a viable candidate for the current request.  See
                    // whether the request time REQT falls into the time
                    // interval for which this segment provides the
                    // highest-priority coverage.
                    //
                    // We treat the re-use interval as topologically open
                    // because one or both endpoints may belong to
                    // higher-priority segments.
                    //
                    if ((save.REQT > (save.ITLB[save.IINDEX] + save.SAVTOL))
                        && (save.REQT < (save.ITUB[save.IINDEX] - save.SAVTOL)))
                    {
                        //
                        // The request time falls into the portion of
                        // the re-use interval that isn't blocked by
                        // higher-priority segments, when the coverage of
                        // those segments is extended in either direction
                        // by TOL.
                        //
                        if (!save.AVNEED || (save.ITPRVF[save.IINDEX] != 0)) {
                            //
                            // This segment has angular velocity if we
                            // need it.  The segment satisfies the
                            // request.
                            //
                            *HANDLE = save.ITPRVH[save.IINDEX];
                            fstr::assign(SEGID, save.ITPRVI.get(save.IINDEX));

                            MOVED(
                                save.ITPRVD.subarray([1, save.IINDEX]),
                                DSCSIZ,
                                DESCR.as_slice_mut(),
                            );

                            *FOUND = true;

                            //
                            // We can only use the re-use interval once on
                            // a given search.  If this search is continued,
                            // we'll have to check the list.  Prepare now.
                            //
                            save.SAVEP = save.ITBEG[save.IINDEX];
                            fstr::assign(&mut save.STATUS, b"CHECK LIST");

                            CHKOUT(b"CKSNS", ctx)?;
                            return Ok(());
                        }
                        //
                        // We needed angular velocity data but didn't have
                        // it if we reached this point.
                        //
                    }

                    //
                    // Adjust the expense here. If the expense of the list
                    // contains a component due to the cost of finding the
                    // unbuffered segment providing data for re-use, subtract
                    // that component from the expense.
                    //
                    save.ITEXP[save.IINDEX] = (save.ITEXP[save.IINDEX] - save.ITRUEX[save.IINDEX]);
                    save.ITRUEX[save.IINDEX] = 0;

                    //
                    // The re-use interval becomes invalid if it didn't
                    // satisfy the request.  The validity flag gets
                    // re-set below.
                    //
                    // At this point, the previous segment is not a candidate
                    // to satisfy the request---at least not until we've done
                    // some file searches to verify that
                    //
                    //    - The previous segment is still available.
                    //
                    //    - The previous segment hasn't been superseded by a
                    //      more recently loaded segment.
                    //
                    // Carry on with the usual search algorithm.
                    //
                    save.ITCHKP[save.IINDEX] = false;
                }

                //
                // If the segment list for this instrument is empty, make
                // sure the expense is reset to 0.
                //
                if (save.ITBEG[save.IINDEX] == 0) {
                    save.ITEXP[save.IINDEX] = 0;
                }

                //
                // Prepare to look at the first segment in the list for
                // this instrument.
                //
                P = save.ITBEG[save.IINDEX];
                fstr::assign(&mut save.STATUS, b"CHECK LIST");
            }
        } else if fstr::eq(&save.STATUS, b"NEW INSTRUMENT") {
            //
            // New instruments are added to the end of the instrument
            // table. If the table is full, one of the current occupants
            // must be removed to make room for the new one.
            //
            // Setting LFS to one more than the highest current file
            // number means the 'OLD FILES' search that follows will
            // begin with the last-loaded file.
            //
            // There is one way to get here:
            //
            // 1)  The variable STATUS was set to NEW INSTRUMENT prior
            //     in CKBSS.
            //
            // Find the cheapest slot in the instrument table to store
            // the initial information about this instrument.
            //
            // NOTE:  This used to be handled by the MAKE ROOM section.
            // However, trying to handle this special case there was
            // just more trouble than it was worth.
            //
            if (save.NIT < ITSIZE) {
                //
                // If the instrument table isn't full, the cheapest place is
                // just the next unused row of the table.
                //
                save.NIT = (save.NIT + 1);
                CHEAP = save.NIT;
            } else {
                //
                // The instrument table is full.  Find the least
                // expensive instrument in the table and remove it.
                //
                CHEAP = 1;
                MINEXP = save.ITEXP[1];

                {
                    let m1__: i32 = 2;
                    let m2__: i32 = save.NIT;
                    let m3__: i32 = 1;
                    I = m1__;
                    for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                        if (save.ITEXP[I] < MINEXP) {
                            CHEAP = I;
                            MINEXP = save.ITEXP[I];
                        }

                        I += m3__;
                    }
                }

                //
                // If there are any segments associated with the
                // least expensive instrument, we put them back on the free
                // list.
                //
                HEAD = save.ITBEG[CHEAP];

                if (HEAD > 0) {
                    TAIL = -LNKPRV(HEAD, save.STPOOL.as_slice(), ctx)?;
                    LNKFSL(HEAD, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                }
            }

            //
            // Set up a table entry for the new instrument.
            //
            save.ITINS[CHEAP] = save.SCINST;
            save.ITEXP[CHEAP] = 0;
            save.ITHFS[CHEAP] = save.FTNUM[save.NFT];
            save.ITLFS[CHEAP] = (save.FTNUM[save.NFT] + 1);
            save.ITBEG[CHEAP] = 0;
            save.ITCHKP[CHEAP] = false;
            save.IINDEX = CHEAP;

            //
            // The following items associated with the re-use interval
            // need not be initialized at this point:
            //
            //    ITRUEX
            //    ITLB
            //    ITUB
            //    ITPRVF
            //    ITPRVH
            //    ITPRVI
            //    ITPRVD
            //
            // However, we'll give these items initial values to
            // help prevent compilation warnings from zealous
            // compilers.
            //
            save.ITRUEX[CHEAP] = 0;
            save.ITLB[CHEAP] = DPMIN();
            save.ITUB[CHEAP] = DPMAX();
            save.ITPRVF[CHEAP] = 0;
            save.ITPRVH[CHEAP] = 0;
            fstr::assign(save.ITPRVI.get_mut(CHEAP), b" ");
            CLEARD(DSCSIZ, save.ITPRVD.subarray_mut([1, CHEAP]));

            //
            // Now search all of the files for segments relating to
            // this instrument.
            //
            fstr::assign(&mut save.STATUS, b"OLD FILES");
        } else if fstr::eq(&save.STATUS, b"NEW FILES") {
            //
            // When new files exist, they should be searched in forward
            // order, beginning with the oldest new file not yet searched.
            // All new files must be searched before the list can be
            // checked, to ensure that the best (newest) segments are
            // being used.
            //
            // Begin a forward search, and prepare to look for individual
            // segments from the file.
            //
            // The only way to get here is to have STATUS set to
            // the value NEW FILES in the STATUS .EQ. '?' block
            // of the IF structure.
            //
            // Find the next file to search; set FINDEX to the
            // corresponding file table entry.

            save.FINDEX = 1;

            while (save.ITHFS[save.IINDEX] >= save.FTNUM[save.FINDEX]) {
                save.FINDEX = (save.FINDEX + 1);
            }

            save.ITHFS[save.IINDEX] = save.FTNUM[save.FINDEX];

            DAFBFS(save.FTHAN[save.FINDEX], ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"CKSNS", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.STATUS, b"NEW SEGMENTS");

            //
            // The cost of the list contributed by the new file is
            // zero so far.
            //
            COST = 0;
        } else if fstr::eq(&save.STATUS, b"NEW SEGMENTS") {
            //
            // New files are searched in forward order. Segments, when
            // found, are inserted at the front of the list. Invisible
            // segments (initial time > final time) are ignored.
            //
            // Each segment examined, whether applicable or not, adds to
            // the expense of the list.
            //
            // The only way to get here is from the NEW FILES block
            // of the IF structure.

            DAFFNA(&mut save.FND, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"CKSNS", ctx)?;
                return Ok(());
            }

            if !save.FND {
                //
                // We're out of segments in the current file.  Decide
                // whether we need to examine another new file, or
                // whether we're ready to check the list.
                //
                fstr::assign(&mut save.STATUS, b"?");
                save.ITEXP[save.IINDEX] = (save.ITEXP[save.IINDEX] + COST);
            } else {
                DAFGS(DESCR.as_slice_mut(), ctx)?;
                DAFUS(
                    DESCR.as_slice(),
                    ND,
                    NI,
                    DCD.as_slice_mut(),
                    ICD.as_slice_mut(),
                );

                if FAILED(ctx) {
                    CHKOUT(b"CKSNS", ctx)?;
                    return Ok(());
                }

                if ((ICD[1] == save.SCINST) && (DCD[1] <= DCD[2])) {
                    fstr::assign(&mut DOING, b"NEW SEGMENTS");
                    fstr::assign(&mut URGENT, b"ADD TO FRONT");
                    fstr::assign(&mut save.STATUS, b"SUSPEND");
                }

                COST = (COST + 1);
            }
        //
        // If we haven't reset the status, we'll return for another
        // 'NEW SEGMENTS' pass.
        //
        } else if fstr::eq(&save.STATUS, b"OLD FILES") {
            //
            // When old files must be searched (because the segments in
            // the list are inadequate), they should be searched in
            // backward order, beginning with the newest old file not
            // yet searched.  The segment list will be re-checked
            // after each file is searched.  If a match is found,
            // the search terminates, so some old files may not be
            // searched.
            //
            // Begin a backwards search, and prepare to look for
            // individual segments from the file.
            //
            // You can get to this block in two ways.
            //
            // 1) We can have a NEW INSTRUMENT.
            //
            // 2) We have checked the current list (CHECK LIST) for
            //    this instrument, didn't find an applicable segment and
            //    have some files left that have not been searched.

            save.FINDEX = save.NFT;

            while (save.ITLFS[save.IINDEX] <= save.FTNUM[save.FINDEX]) {
                save.FINDEX = (save.FINDEX - 1);
            }

            DAFBBS(save.FTHAN[save.FINDEX], ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"CKSNS", ctx)?;
                return Ok(());
            }

            fstr::assign(&mut save.STATUS, b"OLD SEGMENTS");

            //
            // The next thing we'll do is search through all the segments
            // of this file for those that applicable to this instrument.
            // The cost of the list contributed by the current file is
            // zero so far.
            //
            COST = 0;

        //
        // Old files are searched in backward order. Segments, when
        // found, are inserted at the end of the list. Invisible
        // segments (initial time > final time) are ignored.
        //
        // Each segment examined, whether applicable or not, adds to
        // the expense of the list.
        //
        } else if fstr::eq(&save.STATUS, b"OLD SEGMENTS") {
            //
            // There is only one way to get here---from the
            // block 'OLD FILES'.  Note we do not add to the
            // expense of the list for this instrument until we've
            // completely searched this file.
            //
            DAFFPA(&mut save.FND, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"CKSNS", ctx)?;
                return Ok(());
            }

            if !save.FND {
                //
                // All of the segments in this file have been exhausted.
                // Change the lowest file searched indicator for this
                // instrument to be the current file, and go check the
                // current list.
                //
                save.ITLFS[save.IINDEX] = save.FTNUM[save.FINDEX];
                save.ITEXP[save.IINDEX] = (save.ITEXP[save.IINDEX] + COST);
                P = save.ITBEG[save.IINDEX];
                fstr::assign(&mut save.STATUS, b"CHECK LIST");
            } else {
                DAFGS(DESCR.as_slice_mut(), ctx)?;
                DAFUS(
                    DESCR.as_slice(),
                    ND,
                    NI,
                    DCD.as_slice_mut(),
                    ICD.as_slice_mut(),
                );

                if FAILED(ctx) {
                    CHKOUT(b"CKSNS", ctx)?;
                    return Ok(());
                }

                if ((ICD[1] == save.SCINST) && (DCD[1] <= DCD[2])) {
                    fstr::assign(&mut DOING, b"OLD SEGMENTS");
                    fstr::assign(&mut URGENT, b"ADD TO END");
                    fstr::assign(&mut save.STATUS, b"SUSPEND");
                }

                COST = (COST + 1);
            }
        } else if fstr::eq(&save.STATUS, b"CHECK LIST") {
            //
            // Okay, all the new files (and maybe an old file or two)
            // have been searched. Time to look at the list of segments
            // stored for the instrument, to see if there is one applicable
            // to the specified epoch and need for angular velocity data.
            //
            // If so, return it.  If not, try another old file.  If there
            // are no more old files, give up the ghost.
            //
            // There are two ways to get to this point.
            //
            // 1) From the '?' block.
            // 2) From the 'OLD SEGMENTS' block.
            //
            // For every segment examined, adjust the re-use interval
            // associated with the current instrument.
            //
            // P always points to the current segment in the list. Reject
            // a segment if there is a need for angular velocity data and
            // the segment doesn't have it.
            //
            // If this is a new search, initialize the re-use interval.
            // If we're resuming a search, the re-use interval is invalid.
            //
            if save.NEWSCH {
                save.ITLB[save.IINDEX] = DPMIN();
                save.ITUB[save.IINDEX] = DPMAX();
            }

            while (P > 0) {
                if save.NEWSCH {
                    //
                    // Trim the re-use interval if the request time lies
                    // outside of the current segment.
                    //
                    if (save.REQT > save.STDCD[[2, P]]) {
                        //
                        // REQT is to the right of the coverage interval of
                        // this segment.  Trim the re-use interval on the
                        // left, if necessary.
                        //
                        save.ITLB[save.IINDEX] =
                            intrinsics::DMAX1(&[save.ITLB[save.IINDEX], save.STDCD[[2, P]]]);
                    } else if (save.REQT < save.STDCD[[1, P]]) {
                        //
                        // REQT is to the left of the coverage interval of
                        // this segment.  Trim the re-use interval on the
                        // right, if necessary.
                        //
                        save.ITUB[save.IINDEX] =
                            intrinsics::DMIN1(&[save.ITUB[save.IINDEX], save.STDCD[[1, P]]]);
                    }
                }

                if ((save.OMEGA >= save.STDCD[[1, P]]) && (save.ALPHA <= save.STDCD[[2, P]])) {
                    //
                    // The segment coverage interval intersects the request
                    // interval ALPHA:OMEGA.
                    //
                    if (!save.AVNEED || (save.STICD[[4, P]] != 0)) {
                        //
                        // This segment satisfies the request.
                        //
                        DAFPS(
                            ND,
                            NI,
                            save.STDCD.subarray([1, P]),
                            save.STICD.subarray([1, P]),
                            DESCR.as_slice_mut(),
                        );

                        fstr::assign(SEGID, save.STIDNT.get(P));
                        *HANDLE = save.STHAN[P];
                        *FOUND = true;

                        //
                        // If the segment actually contains the request
                        // time, and if this is a new search, set the
                        // re-use interval.  We require the request time
                        // to be in the interior of the interval:  it
                        // cannot be one of the endpoints.
                        //
                        if ((save.NEWSCH && (save.REQT > save.STDCD[[1, P]]))
                            && (save.REQT < save.STDCD[[2, P]]))
                        {
                            //
                            // Set the re-use interval for the current
                            // instrument.
                            //
                            save.ITLB[save.IINDEX] =
                                intrinsics::DMAX1(&[save.ITLB[save.IINDEX], save.STDCD[[1, P]]]);
                            save.ITUB[save.IINDEX] =
                                intrinsics::DMIN1(&[save.ITUB[save.IINDEX], save.STDCD[[2, P]]]);

                            //
                            // Save the returned output items, in case this
                            // segment may satisfy the next request.
                            //
                            save.ITPRVH[save.IINDEX] = *HANDLE;
                            fstr::assign(save.ITPRVI.get_mut(save.IINDEX), SEGID);
                            save.ITPRVF[save.IINDEX] = save.STICD[[4, P]];

                            MOVED(
                                DESCR.as_slice(),
                                DSCSIZ,
                                save.ITPRVD.subarray_mut([1, save.IINDEX]),
                            );

                            save.ITCHKP[save.IINDEX] = true;
                        }

                        //
                        // Go ahead and move the pointer up before returning
                        // so that the search for the next applicable segment
                        // will start at the right place.
                        //
                        save.SAVEP = save.STPOOL[[FORWRD, P]];

                        //
                        // Indicate the first pass of this search has been
                        // completed.
                        //
                        save.NEWSCH = false;

                        CHKOUT(b"CKSNS", ctx)?;
                        return Ok(());
                    }
                }
                //
                // Get the next node.  We avoid LNKNXT here in order
                // to speed up the operation.
                //
                P = save.STPOOL[[FORWRD, P]];
            }

            //
            // If we're still here we didn't have information for this
            // instrument in the segment list.
            //
            // If there are more files, search them.
            // Otherwise, things are hopeless, set the status that way.
            //
            if (save.ITLFS[save.IINDEX] > save.FTNUM[1]) {
                fstr::assign(&mut save.STATUS, b"OLD FILES");
            } else {
                fstr::assign(&mut save.STATUS, b"HOPELESS");
            }
        } else if fstr::eq(&save.STATUS, b"MAKE ROOM") {
            //
            // When adding a new segment to a full table, one of the
            // current instruments must be dropped.  The ideal
            // candidate is the one whose list was constructed at the
            // lowest expense.  The candidate should be removed from
            // the instrument table, and its list transferred to the
            // segment table pool.
            //
            // There is ``room'' if the segment table pool contains at
            // least one free node.
            //
            // It is possible that a single instrument requires more
            // than the entire segment table for its own segments.
            // Two things might happen in such a case:
            //
            //    1) If the list under consideration was being added to at
            //       the end, then a search is continued without buffering
            //       any segments.
            //
            //    2) If the list was being added to at the beginning, then
            //       that means there was a NEW FILES search going on, and
            //       so a brand new list is constructed for the instrument,
            //       much as in a 'NEW INSTRUMENT' task.
            //
            // There are two different ways to get to this point.
            //
            //    1) From 'ADD TO FRONT' if the segment table pool is full.
            //    2) From 'ADD TO END' if the segment table pool is full.
            //
            // Try to make room by deleting a segment list.  CHEAP will
            // be the index of the "cheapest" segment list in the
            // instrument table.
            //
            MINEXP = INTMAX();
            CHEAP = 0;

            {
                let m1__: i32 = 1;
                let m2__: i32 = save.NIT;
                let m3__: i32 = 1;
                I = m1__;
                for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
                    if (I != save.IINDEX) {
                        if ((save.ITEXP[I] < MINEXP) || (CHEAP == 0)) {
                            //
                            // This list is the cheapest seen so far,
                            // possibly because it's the first one
                            // considered.  At the moment, it's as good
                            // a candidate for removal as any.
                            //
                            CHEAP = I;
                            MINEXP = save.ITEXP[I];
                        }
                    }

                    I += m3__;
                }
            }

            if (CHEAP == 0) {
                //
                // If there are no deletable segments, the Thing To
                // Do depends on the task that was suspended before
                // entering MAKE ROOM.
                //
                if fstr::eq(STACK.get(save.TOP), b"ADD TO END") {
                    //
                    // The segment meta-data from the current file cannot
                    // be buffered.  We'll search the partial list of
                    // segments from this file, then proceed to search
                    // the rest of the file and any other old files, until
                    // we find an applicable segment or run out of segments.
                    //
                    fstr::assign(&mut save.STATUS, b"PREPARE PARTIAL LIST");
                } else {
                    //
                    // STACK(TOP) is set to 'ADD TO FRONT'.
                    //
                    // If there is no room left in the table in the middle
                    // of an attempt to add to the front of the list, just
                    // start from scratch by effectively initiating a 'NEW
                    // INSTRUMENT' task.
                    //
                    // Return the current list to the segment table pool.
                    // Note this list is non-empty.
                    //
                    P = save.ITBEG[save.IINDEX];
                    TAIL = -LNKPRV(P, save.STPOOL.as_slice(), ctx)?;

                    LNKFSL(P, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                    //
                    // Re-initialize the table for this instrument, and
                    // initiate an 'OLD FILES' search, just as in 'NEW
                    // INSTRUMENT'.
                    //
                    save.ITBEG[save.IINDEX] = 0;
                    save.ITEXP[save.IINDEX] = 0;
                    save.ITHFS[save.IINDEX] = save.FTNUM[save.NFT];
                    save.ITLFS[save.IINDEX] = (save.FTNUM[save.NFT] + 1);

                    fstr::assign(&mut save.STATUS, b"OLD FILES");
                }

                //
                // Unwind the stack; we've set the target states already.
                //
                save.TOP = 0;
            } else {
                //
                // Return this cheapest list to the segment pool.  This
                // list could be empty.
                //
                HEAD = save.ITBEG[CHEAP];

                if (HEAD > 0) {
                    TAIL = -LNKPRV(HEAD, save.STPOOL.as_slice(), ctx)?;

                    LNKFSL(HEAD, TAIL, save.STPOOL.as_slice_mut(), ctx)?;
                }

                //
                // Fill the deleted instrument's space in the table with
                // the final entry in the table.
                //
                if (CHEAP != save.NIT) {
                    save.ITINS[CHEAP] = save.ITINS[save.NIT];
                    save.ITEXP[CHEAP] = save.ITEXP[save.NIT];
                    save.ITHFS[CHEAP] = save.ITHFS[save.NIT];
                    save.ITLFS[CHEAP] = save.ITLFS[save.NIT];
                    save.ITBEG[CHEAP] = save.ITBEG[save.NIT];
                    save.ITLB[CHEAP] = save.ITLB[save.NIT];
                    save.ITUB[CHEAP] = save.ITUB[save.NIT];
                    save.ITPRVH[CHEAP] = save.ITPRVH[save.NIT];
                    let val = save.ITPRVI.get(save.NIT).to_vec();
                    fstr::assign(save.ITPRVI.get_mut(CHEAP), &val);
                    save.ITPRVF[CHEAP] = save.ITPRVF[save.NIT];
                    save.ITCHKP[CHEAP] = save.ITCHKP[save.NIT];
                    save.ITRUEX[CHEAP] = save.ITRUEX[save.NIT];

                    MOVED(
                        &save.ITPRVD.subarray([1, save.NIT]).to_vec(),
                        DSCSIZ,
                        save.ITPRVD.subarray_mut([1, CHEAP]),
                    );
                }

                if (save.IINDEX == save.NIT) {
                    save.IINDEX = CHEAP;
                }

                //
                // One less instrument now.
                //
                save.NIT = (save.NIT - 1);
                fstr::assign(&mut save.STATUS, b"RESUME");
            }
        //
        // Either we made room by freeing a non-empty segment list,
        // or we're going to work without additional space.  In the
        // latter case, the state is now 'OLD FILES' or
        // 'PREPARE PARTIAL LIST'.
        //
        } else if fstr::eq(&save.STATUS, b"ADD TO FRONT") {
            //
            // The current segment information should be linked in at
            // the head of the segment list for the current instrument,
            // and the pertinent instrument table entry should point
            // to the new head of the list.
            //
            // The only way to get here is from the block NEW SEGMENTS
            // after suspending that task.

            if (LNKNFN(save.STPOOL.as_slice()) == 0) {
                fstr::assign(&mut DOING, b"ADD TO FRONT");
                fstr::assign(&mut URGENT, b"MAKE ROOM");
                fstr::assign(&mut save.STATUS, b"SUSPEND");
            } else {
                //
                // Allocate a node and link it to the front of the list
                // for the current instrument.
                //
                LNKAN(save.STPOOL.as_slice_mut(), &mut NEW, ctx)?;

                save.STHAN[NEW] = save.FTHAN[save.FINDEX];

                DAFGN(&mut save.STIDNT[NEW], ctx)?;

                DAFUS(
                    DESCR.as_slice(),
                    ND,
                    NI,
                    save.STDCD.subarray_mut([1, NEW]),
                    save.STICD.subarray_mut([1, NEW]),
                );

                if FAILED(ctx) {
                    CHKOUT(b"CKSNS", ctx)?;
                    return Ok(());
                }

                //
                // If the current list is empty, this append operation
                // is a no-op.
                //
                LNKILB(
                    NEW,
                    save.ITBEG[save.IINDEX],
                    save.STPOOL.as_slice_mut(),
                    ctx,
                )?;
                save.ITBEG[save.IINDEX] = NEW;

                fstr::assign(&mut save.STATUS, b"RESUME");
            }
        } else if fstr::eq(&save.STATUS, b"ADD TO END") {
            //
            // The current segment information should be linked in at
            // the tail of the segment list for the current instrument.
            //
            // The only way to get to this task is from the OLD SEGMENTS
            // block after suspending that task.
            //
            if (LNKNFN(save.STPOOL.as_slice()) == 0) {
                fstr::assign(&mut DOING, b"ADD TO END");
                fstr::assign(&mut URGENT, b"MAKE ROOM");
                fstr::assign(&mut save.STATUS, b"SUSPEND");
            } else {
                //
                // Allocate a new node in the segment table pool.
                //
                LNKAN(save.STPOOL.as_slice_mut(), &mut NEW, ctx)?;

                save.STHAN[NEW] = save.FTHAN[save.FINDEX];

                DAFGN(&mut save.STIDNT[NEW], ctx)?;

                DAFUS(
                    DESCR.as_slice(),
                    ND,
                    NI,
                    save.STDCD.subarray_mut([1, NEW]),
                    save.STICD.subarray_mut([1, NEW]),
                );

                if FAILED(ctx) {
                    CHKOUT(b"CKSNS", ctx)?;
                    return Ok(());
                }

                if (save.ITBEG[save.IINDEX] <= 0) {
                    //
                    // This is the first node in the list for this
                    // instrument.
                    //
                    save.ITBEG[save.IINDEX] = NEW;
                } else {
                    //
                    // Link the new node to the tail of the list.
                    //
                    TAIL = -LNKPRV(save.ITBEG[save.IINDEX], save.STPOOL.as_slice(), ctx)?;
                    LNKILA(TAIL, NEW, save.STPOOL.as_slice_mut(), ctx)?;
                }

                fstr::assign(&mut save.STATUS, b"RESUME");
            }
        } else if fstr::eq(&save.STATUS, b"PREPARE PARTIAL LIST") {
            //
            // When the segment table is completely full, continue
            // the search by looking through the unchecked portion
            // of the segment list for the current instrument, and
            // then searching old, unchecked files without buffering
            // their segments.
            //
            // The only way to get here is from the MAKE ROOM state
            // via the block ADD TO END.  If you get here there is no
            // free space in the segment table pool.
            //
            // At this point, we need to initialize the cost of
            // the re-use interval.
            //
            save.ITRUEX[save.IINDEX] = 0;

            //
            // Find the portion of the current instrument's segment list
            // which comes from the current file of interest.  SLBEG
            // will point to the beginning of this sublist.
            //
            save.SLBEG = save.ITBEG[save.IINDEX];
            FNDHAN = false;

            while (!FNDHAN && (save.SLBEG > 0)) {
                FNDHAN = (save.STHAN[save.SLBEG] == save.FTHAN[save.FINDEX]);

                if !FNDHAN {
                    //
                    // Get the next node.  We avoid LNKNXT here in order
                    // to speed up the operation.
                    //
                    save.SLBEG = save.STPOOL[[FORWRD, save.SLBEG]];
                }
            }

            //
            // If the list contains segments from the current file,
            // check that portion of the list.
            //
            // Otherwise, finish searching old files without buffering
            // anything.
            //
            if (save.SLBEG > 0) {
                //
                // The partial list from the current node onwards is to be
                // returned to the free list.  Save this node, since
                // we'll finish searching the list before freeing the
                // partial list.
                //
                P = save.SLBEG;

                //
                // Record the fact that we'll need to free the partial list
                // later.
                //
                save.FRESUB = true;

                //
                // It may be that the partial list we're going to delete is
                // the entire segment list for this instrument.  If so, the
                // corresponding instrument table entry should be set to
                // a non-positive value to indicate an empty segment list.
                //
                if (P == save.ITBEG[save.IINDEX]) {
                    save.ITBEG[save.IINDEX] = 0;

                    //
                    // Also in this case, we must initialize the time
                    // bounds for this instrument.
                    //
                    save.ITLB[save.IINDEX] = DPMIN();
                    save.ITUB[save.IINDEX] = DPMAX();
                }

                fstr::assign(&mut save.STATUS, b"CHECK PARTIAL LIST");
            } else {
                fstr::assign(&mut save.STATUS, b"SEARCH W/O BUFF");
            }
        } else if fstr::eq(&save.STATUS, b"CHECK PARTIAL LIST") {
            //
            // The only ways to get here are from the
            // 'PREPARE PARTIAL LIST' state, or by resuming a search of
            // the partial list.
            //
            // The portion of the segment list from the current file
            // is to be checked.
            //
            // BEG points to the current segment in the temporary portion
            // of the list.
            //
            // Reject a segment if there is a need for angular velocity
            // data and the segment doesn't have it.
            //
            while (P > 0) {
                //
                // If this is a new search, update the re-use interval
                // and its expense.
                //
                if save.NEWSCH {
                    //
                    // Every segment seen from the current file contributes
                    // to the expense of the re-use interval.
                    //
                    save.ITRUEX[save.IINDEX] = (save.ITRUEX[save.IINDEX] + 1);

                    //
                    // Trim the re-use interval if the request time lies
                    // outside the coverage of the current segment.
                    //
                    if (save.REQT > save.STDCD[[2, P]]) {
                        //
                        // REQT is to the right of the coverage interval of
                        // this segment.  Trim the re-use interval on the
                        // left, if necessary.
                        //
                        save.ITLB[save.IINDEX] =
                            intrinsics::DMAX1(&[save.ITLB[save.IINDEX], save.STDCD[[2, P]]]);
                    } else if (save.REQT < save.STDCD[[1, P]]) {
                        //
                        // REQT is to the left of the coverage interval of
                        // this segment.  Trim the re-use interval on the
                        // right, if necessary.
                        //
                        save.ITUB[save.IINDEX] =
                            intrinsics::DMIN1(&[save.ITUB[save.IINDEX], save.STDCD[[1, P]]]);
                    }
                }
                //
                // We've updated the re-use interval if so required.
                //

                if ((save.OMEGA >= save.STDCD[[1, P]]) && (save.ALPHA <= save.STDCD[[2, P]])) {
                    //
                    // The segment coverage interval intersects the request
                    // interval ALPHA:OMEGA.
                    //
                    if (!save.AVNEED || (save.STICD[[4, P]] != 0)) {
                        //
                        // This segment satisfies the request.  Set the
                        // output arguments.
                        //
                        DAFPS(
                            ND,
                            NI,
                            save.STDCD.subarray([1, P]),
                            save.STICD.subarray([1, P]),
                            DESCR.as_slice_mut(),
                        );

                        fstr::assign(SEGID, save.STIDNT.get(P));
                        *HANDLE = save.STHAN[P];
                        *FOUND = true;

                        //
                        // If this is the first pass performed for the
                        // current search, then we can set the re-use
                        // interval.  The re-use interval becomes invalid
                        // after the first pass.
                        //
                        // If the segment actually contains the request
                        // time, set the re-use interval.  We require
                        // the request time to be in the interior of the
                        // interval:  it cannot be one of the endpoints.
                        //
                        if ((save.NEWSCH && (save.REQT > save.STDCD[[1, P]]))
                            && (save.REQT < save.STDCD[[2, P]]))
                        {
                            //
                            // Adjust the re-use interval for the current
                            // instrument.
                            //
                            save.ITLB[save.IINDEX] =
                                intrinsics::DMAX1(&[save.ITLB[save.IINDEX], save.STDCD[[1, P]]]);
                            save.ITUB[save.IINDEX] =
                                intrinsics::DMIN1(&[save.ITUB[save.IINDEX], save.STDCD[[2, P]]]);
                            //
                            // Save the returned output items, in case this
                            // segment may satisfy the next request.
                            //
                            save.ITPRVH[save.IINDEX] = *HANDLE;
                            fstr::assign(save.ITPRVI.get_mut(save.IINDEX), SEGID);
                            save.ITPRVF[save.IINDEX] = save.STICD[[4, P]];

                            MOVED(
                                DESCR.as_slice(),
                                DSCSIZ,
                                save.ITPRVD.subarray_mut([1, save.IINDEX]),
                            );

                            save.ITCHKP[save.IINDEX] = true;
                            //
                            // Update the expense of the list to reflect
                            // the cost of locating this segment.
                            //
                            save.ITEXP[save.IINDEX] =
                                (save.ITEXP[save.IINDEX] + save.ITRUEX[save.IINDEX]);
                        }
                        //
                        // We've set the re-use interval.
                        //
                        // Go ahead and move the pointer up before returning
                        // so that the search for the next applicable segment
                        // will start at the right place.
                        //
                        // We avoid LNKNXT here in order to speed up the
                        // operation.
                        //
                        save.SAVEP = save.STPOOL[[FORWRD, P]];

                        //
                        // We cannot free the partial list yet, because
                        // we may return to search it again if the current
                        // segment doesn't have pointing that satisfies
                        // the caller's request.  The list will be freed
                        // at the start of the next search if it's not
                        // freed at the end of this block or in the
                        // 'SEARCH W/O BUFFERING' block.
                        //
                        // Indicate the first pass of this search has been
                        // completed.
                        //
                        save.NEWSCH = false;

                        CHKOUT(b"CKSNS", ctx)?;
                        return Ok(());
                    }
                    //
                    // Getting here implies angular velocity was
                    // requested but was not present in the segment.
                    //
                }
                //
                // The current segment didn't match.  Look at the next
                // segment in the list.
                //
                P = save.STPOOL[[FORWRD, P]];
            }
            //
            // We're done looking at the partial list.
            //
            // Return the partial list to the segment table pool.
            // P at this point is the negative of the list head.
            // The list tail is (by the spec of the SPICELIB doubly
            // linked list routines) the negative of the predecessor
            // of the head.
            //
            // Note the list is always non-empty at this point.
            //
            TAIL = -LNKPRV(-P, save.STPOOL.as_slice(), ctx)?;

            LNKFSL(save.SLBEG, TAIL, save.STPOOL.as_slice_mut(), ctx)?;

            save.FRESUB = false;

            //
            // Search the remaining files.
            //
            fstr::assign(&mut save.STATUS, b"SEARCH W/O BUFF");
        } else if fstr::eq(&save.STATUS, b"SEARCH W/O BUFF") {
            //
            // The only ways to get here are from the
            // 'PREPARE PARTIAL LIST' and 'CHECK PARTIAL LIST' states.
            //
            // When the segment table is full with the current instrument's
            // segments and any freed up portions have been checked,
            // continue the search for applicable segments in old files,
            // without buffering any of the segments in the segment table.
            //
            // Recall that a search is already in progress and that a
            // segment is currently under consideration (FND = .TRUE.).
            //
            while (save.FINDEX > 0) {
                while save.FND {
                    if save.NEWSCH {
                        //
                        // Each segment found contributes to the expense of
                        // the re-use interval.
                        //
                        save.ITRUEX[save.IINDEX] = (save.ITRUEX[save.IINDEX] + 1);
                    }

                    DAFGS(DESCR.as_slice_mut(), ctx)?;
                    DAFUS(
                        DESCR.as_slice(),
                        ND,
                        NI,
                        DCD.as_slice_mut(),
                        ICD.as_slice_mut(),
                    );

                    if FAILED(ctx) {
                        CHKOUT(b"CKSNS", ctx)?;
                        return Ok(());
                    }

                    if (save.SCINST == ICD[1]) {
                        //
                        // This is a segment for the instrument of interest.

                        if save.NEWSCH {
                            //
                            // Update the re-use interval for this instrument.
                            //
                            if (save.REQT > DCD[2]) {
                                //
                                // REQT is to the right of the coverage interval
                                // of this segment.  Trim the re-use interval
                                // on the left, if necessary.
                                //
                                save.ITLB[save.IINDEX] =
                                    intrinsics::DMAX1(&[save.ITLB[save.IINDEX], DCD[2]]);
                            } else if (save.REQT < DCD[1]) {
                                //
                                // REQT is to the left of the coverage interval
                                // of this segment.  Trim the re-use interval
                                // on the right, if necessary.
                                //
                                save.ITUB[save.IINDEX] =
                                    intrinsics::DMIN1(&[save.ITUB[save.IINDEX], DCD[1]]);
                            }
                        }
                        //
                        // We've trimmed the re-use interval if necessary.
                        //
                        if ((save.OMEGA >= DCD[1]) && (save.ALPHA <= DCD[2])) {
                            //
                            // The segment coverage interval intersects the
                            // request interval ALPHA:OMEGA.
                            //
                            if (!save.AVNEED || (ICD[4] != 0)) {
                                //
                                // This segment satisfies the request.  Set
                                // the output arguments.
                                //
                                DAFPS(ND, NI, DCD.as_slice(), ICD.as_slice(), DESCR.as_slice_mut());

                                DAFGN(SEGID, ctx)?;

                                *HANDLE = save.FTHAN[save.FINDEX];
                                *FOUND = true;

                                if save.NEWSCH {
                                    //
                                    // Adjust the re-use interval for the current
                                    // instrument.
                                    //
                                    save.ITLB[save.IINDEX] =
                                        intrinsics::DMAX1(&[save.ITLB[save.IINDEX], DCD[1]]);
                                    save.ITUB[save.IINDEX] =
                                        intrinsics::DMIN1(&[save.ITUB[save.IINDEX], DCD[2]]);

                                    //
                                    // Save the returned output items, in case
                                    // this segment may satisfy the next request.
                                    //
                                    save.ITPRVH[save.IINDEX] = *HANDLE;
                                    fstr::assign(save.ITPRVI.get_mut(save.IINDEX), SEGID);
                                    save.ITPRVF[save.IINDEX] = ICD[4];

                                    MOVED(
                                        DESCR.as_slice(),
                                        DSCSIZ,
                                        save.ITPRVD.subarray_mut([1, save.IINDEX]),
                                    );

                                    save.ITCHKP[save.IINDEX] = true;

                                    //
                                    // Update the expense of the list to reflect
                                    // cost of locating this segment.
                                    //
                                    save.ITEXP[save.IINDEX] =
                                        (save.ITEXP[save.IINDEX] + save.ITRUEX[save.IINDEX]);
                                }
                                //
                                // The re-use interval is set.
                                //
                                // Go ahead and point to the next segment in the
                                // file in case an attempt is made to continue
                                // the search: you want to pick up exactly where
                                // you  left off.
                                //
                                DAFFPA(&mut save.FND, ctx)?;
                                //
                                // Indicate the first pass of this search has
                                // been completed.
                                //
                                save.NEWSCH = false;

                                CHKOUT(b"CKSNS", ctx)?;
                                return Ok(());
                            }
                            //
                            // Getting here implies angular velocity was
                            // requested but was not present in the segment.
                            //
                        }
                        //
                        // The current segment's coverage didn't intersect
                        // the request interval.
                        //
                    }
                    //
                    // The current segment didn't contain data for the
                    // specified instrument.
                    //
                    // Look at the next segment in the current file.
                    //
                    DAFFPA(&mut save.FND, ctx)?;
                }

                //
                // Try the next oldest file.
                //
                save.FINDEX = (save.FINDEX - 1);

                if (save.FINDEX > 0) {
                    DAFBBS(save.FTHAN[save.FINDEX], ctx)?;
                    DAFFPA(&mut save.FND, ctx)?;
                }
            }

            //
            // There's nothing nowhere if you get to here.
            //
            save.ITRUEX[save.IINDEX] = 0;
            fstr::assign(&mut save.STATUS, b"HOPELESS");
        } else if fstr::eq(&save.STATUS, b"SUSPEND") {
            //
            // When a task is suspended, the current activity is placed on
            // a stack, to be restored later. Two levels are provided,
            // since some interrupts can be interrupted by others.
            //
            save.TOP = (save.TOP + 1);
            fstr::assign(STACK.get_mut(save.TOP), &DOING);
            fstr::assign(&mut save.STATUS, &URGENT);
        } else if fstr::eq(&save.STATUS, b"RESUME") {
            fstr::assign(&mut save.STATUS, STACK.get(save.TOP));
            save.TOP = (save.TOP - 1);
        }
    }

    //
    // Can only get here if status is 'HOPELESS', in which case a
    // segment was not found.
    //
    *FOUND = false;

    //
    // If we didn't find a segment, don't attempt to use saved
    // outputs from a previous call.  IINDEX will always be set
    // at this point.  Also, make sure the expense of the re-use
    // interval is zeroed out.
    //
    if (save.IINDEX > 0) {
        save.ITCHKP[save.IINDEX] = false;
        save.ITRUEX[save.IINDEX] = 0;
    }

    //
    // For safety, indicate the first pass of this search has been
    // completed.  Normally, we won't return here before CKBSS is
    // called again, but it's possible.
    //
    save.NEWSCH = false;

    CHKOUT(b"CKSNS", ctx)?;
    Ok(())
}

/// CK --- Are there any loaded?
///
/// Determine whether or not any C-kernels are currently loaded.
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
///  FOUND      O   .TRUE. if at least one C-kernel is loaded.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FOUND    is returned with the value .TRUE. if at least one
///           C-kernel is currently loaded. Otherwise it returns
///           the value .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
/// ```
///
/// # Particulars
///
/// ```text
///  This entry point allows the user to query the set of "loaded"
///  C-kernels to make sure that at least one C-kernel has been loaded.
///  This allows you to avoid making a search of an empty set of
///  loaded kernels which forces a SPICELIB error to be signaled.
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
///  1) Suppose you want to call one of the C-kernel readers, but wish
///     to handle the exceptional case of "no kernels loaded" so that
///     the SPICE exception handling mechanism is avoided in the case
///     of an empty set of loaded kernels. The code example below
///     shows how you might do this.
///
///
///     Example code begins here.
///
///
///           PROGRAM CKHAVE_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local variables.
///     C
///           LOGICAL               LOADED
///
///     C
///     C     We have not loaded any CK.
///     C
///           CALL CKHAVE ( LOADED )
///
///           IF ( LOADED ) THEN
///
///     C
///     C        Here we could be calling CK readers. For this
///     C        example, just display a message indicating that
///     C        at least one CK has been loaded.
///     C
///              WRITE(*,*) 'At least one CK has been loaded. '
///          .          //  'You may call CK readers.'
///
///           ELSE
///
///              WRITE(*,*) 'No CK has been loaded.'
///              WRITE(*,*) 'Calling CK readers will result in an '
///          .          //  'error.'
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      No CK has been loaded.
///      Calling CK readers will result in an error.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.0.4, 16-JUL-2020 (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example based on existing example code fragment.
///
/// -    SPICELIB Version 4.0.3, 30-JAN-2017 (NJB)
///
///         Corrected various spelling errors within comments.
///
/// -    SPICELIB Version 4.0.2, 28-FEB-2008 (BVS)
///
///         Corrected the contents of the $Required_Reading section.
///
/// -    SPICELIB Version 4.0.1, 31-OCT-2001 (NJB)
///
///         Typo corrected.
///
/// -    SPICELIB Version 4.0.0, 17-FEB-2000 (WLT)
///
///         Added the Entry point CKHAVE.
///
/// -    SPICELIB Version 3.0.0, 03-MAR-1999 (WLT)
/// ```
pub fn ckhave(ctx: &mut SpiceContext, found: &mut bool) {
    CKHAVE(found, ctx.raw_context());
}

//$Procedure CKHAVE ( CK --- Are there any loaded? )
pub fn CKHAVE(FOUND: &mut bool, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *FOUND = (save.NFT > 0);
}
