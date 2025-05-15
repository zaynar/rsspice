//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;

fn ZZICHR(CARG: &[u8]) -> i32 {
    let CARG = &CARG[..1 as usize];
    (intrinsics::ICHAR(CARG)
        - (intrinsics::MAX0(&[-1, intrinsics::MIN0(&[0, intrinsics::ICHAR(CARG)])]) * 256))
}

//$Procedure ZZDDHIVF ( Private --- DDH Identify VAX DAF File Format )
pub fn ZZDDHIVF(NSUM: &[u8], BFF: &mut i32, FOUND: &mut bool) {
    let mut LEADER: i32 = 0;
    let mut TRAILR: i32 = 0;

    //
    // Local Variables
    //

    //
    // Statement Functions
    //

    //
    // Statement Function Definitions
    //
    // This function controls the conversion of characters to integers.
    // Some versions of the g77 implement ICHAR with a signed integer.
    // This function computes the value of ICHAR that this code requires
    // on any version of g77 for x86 Linux.
    //

    //
    // Before diving right into the code that examines the bit patterns
    // stored in NSUM, review exactly what checks require completion and
    // why they function.
    //
    // When this module is invoked, we already know that the DAF from
    // which NSUM was extracted is little endian, and that it is not
    // a LTL-IEEE file.  This leaves us with one of 3 sources for
    // NSUM:
    //
    //   (a) A VAX D-Floating file
    //   (b) A VAX G-Floating file
    //   (c) A damaged file
    //
    // In the case of (c) the algorithm outlined below is not guaranteed
    // to produce correct results.  If the case is either (a) or (b),
    // then the routine will correctly determine the source binary file
    // format.  Here's why:
    //
    //    NSUM is the third double precision number from the first
    //    descriptor record of a non-empty DAF file.  This number is
    //    an integral valued DP bounded between 1 and 125 inclusive.
    //
    //    An examination of a binary file created with the following
    //    code fragment:
    //
    //       INCLUDE              'zzddhman.inc'
    //
    //       DOUBLE PRECISION      DPDATA ( 125 )
    //       INTEGER               I
    //       INTEGER               LUN
    //         .
    //         .
    //         .
    //       CALL GETLUN( LUN )
    //
    //       DO I = 1, 125
    //          DPDATA (I) = DBLE (I)
    //       END DO
    //
    //       OPEN ( UNIT   = LUN,
    //      .       FILE   = FNAME,
    //      .       STATUS = 'NEW',
    //      .       ACCESS = 'DIRECT',
    //      .       RECL   = RECL      )
    //
    //       WRITE ( UNIT = LUN, REC = 1 ) ( DPDATA(I), I = 1, 125 )
    //
    //       END
    //
    //    This source file was compiled on a VMS VAX system both with
    //    G-Floating and D-Floating options, and executed to produce
    //    the binary file of interest.  The bit patterns for each of
    //    the 125 entries were compared using the UNIX command 'od'.
    //
    //    This comparison yielded the fact that these two sets of 125
    //    bit patterns did not intersect, and all that remained was to
    //    uncover an efficient means of identifying which set a
    //    particular member belonged to.
    //
    //    The following was observed:
    //
    //       With the exception of the first entry representing the
    //       number 1.0D0 in the D-Floating case, all entries
    //       appeared as: (hexadecimal byte dump from 'od' output)
    //
    //          0041 0000 0000 0000
    //          4041 0000 0000 0000
    //          8041 0000 0000 0000
    //                   .
    //                   .
    //                   .
    //          f643 0000 0000 0000
    //          f843 0000 0000 0000
    //          fa43 0000 0000 0000
    //
    //       While the G-Floating case:
    //
    //          1040 0000 0000 0000
    //          2040 0000 0000 0000
    //          2840 0000 0000 0000
    //                   .
    //                   .
    //                   .
    //          7e40 00c0 0000 0000
    //          7f40 0000 0000 0000
    //          7f40 0040 0000 0000
    //
    //       The important thing to note is that the fourth entry in
    //       G-Floating bit patterns is always '0', and in the
    //       D-Floating case (with the exception of the first entry)
    //       is always non-zero.  The first entry in the D-Floating
    //       table is:
    //
    //          8040 0000 0000 0000
    //
    //       It also happens to be the case that the leading value
    //       of all G-Floating cases are numbers less than 8.
    //       Constructing a series of tests around these observations
    //       will produce correct results.  When the input file meets
    //       the restrictions non-empty and correct.
    //
    // So now all that remains is to lay out the specifics of the test.
    // First extract the leading 4 bits from NSUM(1:1) and the trailing
    // four bits from NSUM(2:2).  Then enter this IF/ELSE IF block:
    //
    //    If the value of the leading 4 bits from NSUM(1:1) is 8 and
    //    the trailing 4 bits from NSUM(2:2) are 0, then the file is
    //    of the D-Floating binary format.
    //
    //    Else if the value of the trailing 4 bits of NSUM(2:2) is
    //    non-zero, then the file is also of the D-Floating binary
    //    format.
    //
    //    Else if the value of the leading 4 bits of NSUM(1:1) is
    //    strictly less than 8 and the trailing bits of NSUM(2:2)
    //    are 0, then the file is of the G-Floating binary format.
    //
    //    Else the file is not of VAX type.
    //
    // This routine could be reimplemented to examine all 8 bytes of
    // each double precision number and compare it to two tables of
    // values.  In the interest of simplicity the preceding option
    // was selected.
    //
    //
    //
    // Convert the first and second characters in NSUM to integers.
    //
    LEADER = ZZICHR(fstr::substr(NSUM, 1..=1));
    TRAILR = ZZICHR(fstr::substr(NSUM, 2..=2));

    //
    // Shift the trailing 4 bits off LEADER.
    //
    LEADER = (LEADER / 16);

    //
    // Subtract the leading bits off TRAILR.
    //
    TRAILR = (TRAILR - (16 * (TRAILR / 16)));

    //
    // Now determine what file we are looking at.
    //
    if ((LEADER == 8) && (TRAILR == 0)) {
        *FOUND = true;
        *BFF = VAXDFL;
    } else if (TRAILR != 0) {
        *FOUND = true;
        *BFF = VAXDFL;
    } else if ((LEADER < 8) && (TRAILR == 0)) {
        *FOUND = true;
        *BFF = VAXGFL;
    } else {
        *FOUND = false;
    }
}
