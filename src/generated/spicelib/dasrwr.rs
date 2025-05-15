//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const NWD: i32 = 128;
pub const NWI: i32 = 256;
pub const NWC: i32 = 1024;
pub const BUFSZD: i32 = 10;
pub const BUFSZI: i32 = 10;
pub const BUFSZC: i32 = 10;
const LBPOOL: i32 = -5;
const FORWRD: i32 = 1;
const BUFNVD: i32 = (BUFSZD * NWD);
const BUFNVI: i32 = (BUFSZI * NWI);

struct SaveVars {
    RCBUFC: ActualCharArray,
    RCBUFD: ActualArray2D<f64>,
    RCBUFI: ActualArray2D<i32>,
    RNBUFC: StackArray<i32, 10>,
    RNBUFD: StackArray<i32, 10>,
    RNBUFI: StackArray<i32, 10>,
    HNBUFC: StackArray<i32, 10>,
    HNBUFD: StackArray<i32, 10>,
    HNBUFI: StackArray<i32, 10>,
    UPBUFC: StackArray<bool, 10>,
    UPBUFD: StackArray<bool, 10>,
    UPBUFI: StackArray<bool, 10>,
    POOLC: StackArray2D<i32, 32>,
    POOLD: StackArray2D<i32, 32>,
    POOLI: StackArray2D<i32, 32>,
    HEADC: i32,
    HEADD: i32,
    HEADI: i32,
    USEDC: i32,
    USEDD: i32,
    USEDI: i32,
    NEXT: i32,
    NODE: i32,
    UNIT: i32,
    WRUNIT: i32,
    PASS1: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut RCBUFC = ActualCharArray::new(NWC, 1..=BUFSZC);
        let mut RCBUFD = ActualArray2D::<f64>::new(1..=NWD, 1..=BUFSZD);
        let mut RCBUFI = ActualArray2D::<i32>::new(1..=NWI, 1..=BUFSZI);
        let mut RNBUFC = StackArray::<i32, 10>::new(1..=BUFSZC);
        let mut RNBUFD = StackArray::<i32, 10>::new(1..=BUFSZD);
        let mut RNBUFI = StackArray::<i32, 10>::new(1..=BUFSZI);
        let mut HNBUFC = StackArray::<i32, 10>::new(1..=BUFSZC);
        let mut HNBUFD = StackArray::<i32, 10>::new(1..=BUFSZD);
        let mut HNBUFI = StackArray::<i32, 10>::new(1..=BUFSZI);
        let mut UPBUFC = StackArray::<bool, 10>::new(1..=BUFSZC);
        let mut UPBUFD = StackArray::<bool, 10>::new(1..=BUFSZD);
        let mut UPBUFI = StackArray::<bool, 10>::new(1..=BUFSZI);
        let mut POOLC = StackArray2D::<i32, 32>::new(1..=2, LBPOOL..=BUFSZC);
        let mut POOLD = StackArray2D::<i32, 32>::new(1..=2, LBPOOL..=BUFSZD);
        let mut POOLI = StackArray2D::<i32, 32>::new(1..=2, LBPOOL..=BUFSZI);
        let mut HEADC: i32 = 0;
        let mut HEADD: i32 = 0;
        let mut HEADI: i32 = 0;
        let mut USEDC: i32 = 0;
        let mut USEDD: i32 = 0;
        let mut USEDI: i32 = 0;
        let mut NEXT: i32 = 0;
        let mut NODE: i32 = 0;
        let mut UNIT: i32 = 0;
        let mut WRUNIT: i32 = 0;
        let mut PASS1: bool = false;

        PASS1 = true;
        USEDC = 0;
        USEDD = 0;
        USEDI = 0;
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::C(b" "), BUFSZC as usize))
                .chain([]);

            RCBUFC
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::D(0.0), BUFNVD as usize))
                .chain([]);

            RCBUFD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_f64());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFNVI as usize))
                .chain([]);

            RCBUFI
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSZC as usize))
                .chain([]);

            RNBUFC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSZD as usize))
                .chain([]);

            RNBUFD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSZI as usize))
                .chain([]);

            RNBUFI
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSZC as usize))
                .chain([]);

            HNBUFC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSZD as usize))
                .chain([]);

            HNBUFD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::I(0), BUFSZI as usize))
                .chain([]);

            HNBUFI
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), BUFSZC as usize))
                .chain([]);

            UPBUFC
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), BUFSZD as usize))
                .chain([]);

            UPBUFD
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = []
                .into_iter()
                .chain(std::iter::repeat_n(Val::L(false), BUFSZI as usize))
                .chain([]);

            UPBUFI
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_bool());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        HEADC = 0;
        HEADD = 0;
        HEADI = 0;
        UNIT = -1;
        WRUNIT = -1;

        Self {
            RCBUFC,
            RCBUFD,
            RCBUFI,
            RNBUFC,
            RNBUFD,
            RNBUFI,
            HNBUFC,
            HNBUFD,
            HNBUFI,
            UPBUFC,
            UPBUFD,
            UPBUFI,
            POOLC,
            POOLD,
            POOLI,
            HEADC,
            HEADD,
            HEADI,
            USEDC,
            USEDD,
            USEDI,
            NEXT,
            NODE,
            UNIT,
            WRUNIT,
            PASS1,
        }
    }
}

/// DAS, read/write records
///
/// Read and write DAS physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINTS
///  --------  ---  --------------------------------------------------
///  HANDLE     I   RRD, RRI, RRC, WRD, WRI, WRC, URD, URI, URC
///  RECNO      I   RRD, RRI, RRC, WRD, WRI, WRC, URD, URI, URC
///  RECC       I   WRC
///  RECD       I   WRD
///  RECI       I   WRI
///  FIRST      I   RRD, RRI, RRC, URD, URI, URC
///  LAST       I   RRD, RRI, RRC, URD, URI, URC
///  DATAD      O   RRD, URD
///  DATAI      O   RRI, URI
///  DATAC      O   RRC, URC
///  NWD        P   RRD, WRD, URD
///  NWI        P   RRI, WRI, URI
///  NWC        P   RRC, WRC, URC
///  BUFSZD     P   RRD, WRD, URD
///  BUFSZI     P   RRI, WRI, URI
///  BUFSZC     P   RRC, WRC, URC
/// ```
///
/// # Detailed Input
///
/// ```text
///  See the entry points for a discussion of their inputs.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See the entry points for a discussion of their outputs.
/// ```
///
/// # Parameters
///
/// ```text
///  NWD      is the number of DPs in a single DAS record
///           containing DPs.
///
///  NWI      is the number of integers in a single DAS record
///           containing integers.
///
///  NWC      is the number of characters in a single DAS record
///           containing characters.
///
///  BUFSZD,
///  BUFSZI,
///  BUFSZC   are, respectively, the number of records in the
///           data buffers for double precision, integer, and
///           character records.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called directly, the error
///      SPICE(BOGUSENTRY) is signaled.
///
///  2)  See the entry points for discussions of their exceptions.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in the headers of
///  the entry points for a description of files accessed by this
///  set of routines.
/// ```
///
/// # Particulars
///
/// ```text
///  This suite of routines provides buffered read and write access to
///  DAS files. The purpose of this feature is to increase the
///  performance of application programs that access DAS files: in
///  particular, repeated reads from or writes to a given record
///  should be relatively fast, because the contents of the most
///  recently accessed records are buffered in memory. Thus DASRWR
///  and its entry points act as a miniature virtual memory system for
///  DAS files.
///
///  These routines are intended primarily for use by other SPICELIB
///  routines; users' application programs will not normally need to
///  call these routines. Writing to a DAS file with these routines
///  demands a particularly circumspect approach: it's quite easy to
///  end up with something other than a DAS file if one misuses the
///  routines.
///
///  The entry points of DASRWR support writing, reading, and updating
///  the records in a DAS file. The distinction between writing and
///  updating is that any record may be written (as long as the record
///  belongs to a file open for writing), but only existing records
///  may be updated.  `Writing' a record sets the values of all of
///  the elements of the record, while a subrange of the elements of an
///  existing record may be `updated'.
///
///  For each of these three operations, there are three DAS routines,
///  one for each supported data type. The names of the routines are
///
///     -- For writing:     DASWRC,  DASWRD,  DASWRI
///     -- For updating:    DASURC,  DASURD,  DASURI
///     -- For reading:     DASRRC,  DASRRD,  DASRRI
///
///  Users should note that, unlike in the case of SPICELIB's DAF
///  routines, the DAS routines buffer data that is written as well
///  as data that is read. Consequently a DAS file does not
///  necessarily yet contain, at any moment, all of the data that
///  has been written to it by the DASWRx or DASURx routines. The
///  written data that is buffered is written out when the need
///  to buffer additional data requires it, and also when the user
///  commands the closure of a file that has been written. So, at
///  the time a DAS file is closed, the contents of the physical file
///  do reflect what has been `written' to the file by the DASWRx and
///  DASURx entry points.
///
///  At any time, an application program can force the DAS system to
///  write to a DAS file any buffered records maintained for that
///  file. The entry point DASWBR (DAS, write buffered records)
///  provides this capability.
///
///  DASRWR contains three record buffers: one of character type,
///  one of double precision type, and one of integer type. Each
///  buffer has enough room for an integer number of records. The
///  sizes of the buffers are parameterized and can be increased if
///  necessary. When contemplating the revision of the buffer
///  sizes selected by NAIF, SPICELIB users should take note of the
///  following points:
///
///     -- Changing values of parameters in NAIF subroutines may cause
///        a maintenance burden for the users of the modified NAIF
///        code, since any changes made to a SPICELIB routine will have
///        to be made to any new version of that routine released by
///        NAIF in a later version of SPICELIB.
///
///     -- The effect of buffer size on the speed with which an
///        application executes is highly dependent on the specific
///        application. In some cases, increasing the buffer sizes
///        may slow the application down.
/// ```
///
/// # Examples
///
/// ```text
///  See the entry points for examples specific to those routines.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 07-OCT-2021 (NJB) (JDR)
///
///         Added initializers for record buffers.
///
///         Edited the headers of DASRWR and all its entry points to comply
///         with NAIF standard.
///
///         Cleaned up the $Revisions history.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration and
///         reading of non-native files.
///
/// -    SPICELIB Version 1.1.1, 10-FEB-2014 (BVS)
///
///         Added description of NWD, NWI, and NWC to the $Parameters
///         and $Brief_I/O sections of the header.
///
/// -    SPICELIB Version 1.1.0, 17-NOV-1995 (NJB)
///
///         Made modifications to the DASRRx routines to enhance
///         efficiency. Removed references to the function RETURN.
///
///         Removed weird spaces from ENTRY statements.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header for each entry point.
///         This was done in order to minimize documentation changes if the
///         DAS open routines ever change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration and
///         reading of non-native files.
///
/// -    SPICELIB Version 1.1.0, 17-NOV-1995 (NJB)
///
///         Made modifications to the DASRRx routines to enhance
///         efficiency. Removed references to the function RETURN.
/// ```
pub fn dasrwr(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    recc: &str,
    recd: &[f64; 128],
    reci: &[i32; 256],
    first: i32,
    last: i32,
    datad: &[f64],
    datai: &[i32],
    datac: &str,
) -> crate::Result<()> {
    DASRWR(
        handle,
        recno,
        recc.as_bytes(),
        recd,
        reci,
        first,
        last,
        datad,
        datai,
        datac.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRWR ( DAS, read/write records )
pub fn DASRWR(
    HANDLE: i32,
    RECNO: i32,
    RECC: &[u8],
    RECD: &[f64],
    RECI: &[i32],
    FIRST: i32,
    LAST: i32,
    DATAD: &[f64],
    DATAI: &[i32],
    DATAC: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
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
    // The data structure maintained by this set of routines consists
    // of three record buffers, one each for use with records of double
    // precision, integer, and character data types.
    //
    // Each buffer consists of five parallel arrays; the arrays contain:
    //
    //    -- data records
    //    -- Fortran record numbers
    //    -- file handles
    //    -- Update flags
    //
    // In addition, for each buffer there is a doubly linked list that
    // points to the buffer and keeps track of the order in which the
    // records in the buffer were accessed.  The three linked lists are
    // maintained in a doubly linked list pool structure.  The logical
    // structure of each buffer is illustrated below.  All of the array
    // elements in the same row are associated with the data record in
    // that row.
    //
    //
    //
    // Linked          Record       Record   Handles   Update
    //  List           buffer       Numbers            Flags
    //
    //  +---+      +------------+    +---+    +---+    +---+
    //  |   | ---> |            |    |   |    |   |    |   |
    //  +---+      +------------+    +---+    +---+    +---+
    //  |   | ---> |            |    |   |    |   |    |   |
    //  +---+      +------------+    +---+    +---+    +---+
    //    .              .             .        .        .
    //    .              .             .        .        .
    //    .              .             .        .        .
    //  +---+      +------------+    +---+    +---+    +---+
    //  |   | ---> |            |    |   |    |   |    |   |
    //  +---+      +------------+    +---+    +---+    +---+
    //
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
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DASRWR", ctx)?;
    }

    //
    // Never come here.
    //
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;

    CHKOUT(b"DASRWR", ctx)?;
    Ok(())
}

/// DAS, read record, double precision
///
/// Read DAS double precision physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  FIRST,
///  LAST       I   First and last indices of range within record.
///  DATAD      O   Double precision data read from record.
///  BUFSZD     P   Number of records in the DP record buffer.
///  NWD        P   Number of DP in a single DAS DP record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an open DAS file.
///
///  RECNO    is the number of a record in a DAS file.
///
///  FIRST,
///  LAST     are the first and last indices of a range of
///           double precision numbers to be read from the
///           indicated record. The record contains NWD
///           double precision numbers; these have indices
///           ranging from 1 to NWD.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATAD    is a double precision array containing the
///           elements FIRST through LAST of the specified
///           record. The record element FIRST is placed
///           in DATAD(1), the record element FIRST+1 is placed
///           in DATAD(2), and so on; the record element LAST is
///           placed in DATAD(LAST-FIRST+1).
/// ```
///
/// # Parameters
///
/// ```text
///  NWD      is the number of DPs in a single DAS record
///           containing DPs.
///
///  BUFSZD   is the number of records in the double precision
///           record buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The output argument
///      DATAD will not be modified.
///
///  2)  If a read operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      output argument DATAD will not be modified.
///
///  3)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      output argument DATAD will not be modified. This routine may
///      write out updated, buffered records in order to make room in
///      the double precision buffer for a newly read record. Note that
///      the file written to may be different than the file designated
///      by HANDLE if multiple DAS files are open for writing.
///
///  4)  If FIRST or LAST is not in the range [1, NWD], the error
///      SPICE(INDEXOUTOFRANGE) is signaled. The output argument
///      DATAD will not be modified.
///
///  5)  If FIRST > LAST, this routine will return without modifying
///      the output argument DATAD.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to read from a DAS file that is open for
///  reading or for writing. Any buffered double precision record
///  can be read with this routine. In particular, records that have
///  been written to the DAS double precision record buffer but have
///  not yet been written out to the DAS file they're intended to go
///  to ARE visible to this routine.
///
///  This routine should be used to read only records that contain
///  double precision data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Read the 10th through 100th d.p. numbers from record number 9
///      in a DAS file designated by HANDLE.
///
///          CALL DASRRD ( HANDLE, 9, 10, 100, DATAD )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration and
///         reading of non-native files.
///
/// -    SPICELIB Version 1.1.1, 10-FEB-2014 (BVS)
///
///         Added description of NWD to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.1.0, 03-NOV-1995 (NJB)
///
///         Made modifications to enhance efficiency. Removed references
///         to the function RETURN.
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.1.0, 03-NOV-1995 (NJB)
///
///         Made modifications to enhance efficiency. Removed references
///         to the function RETURN. For buffered reads, MOVED is not
///         called when a single word is to be read.
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dasrrd(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    first: i32,
    last: i32,
    datad: &mut [f64],
) -> crate::Result<()> {
    DASRRD(handle, recno, first, last, datad, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRRD ( DAS, read record, double precision )
pub fn DASRRD(
    HANDLE: i32,
    RECNO: i32,
    FIRST: i32,
    LAST: i32,
    DATAD: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DATAD = DummyArrayMut::new(DATAD, 1..);

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Check FIRST and LAST.  Use discovery check-in.
    //
    if ((((FIRST < 1) || (FIRST > NWD)) || (LAST < 1)) || (LAST > NWD)) {
        CHKIN(b"DASRRD", ctx)?;
        SETMSG(b"Array indices FIRST and LAST were #,  #; allowed range for both is [#, #]. File was #, record number was #.", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", 1, ctx);
        ERRINT(b"#", NWD, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASRRD", ctx)?;
        return Ok(());
    }

    //
    // There's nothing to do if LAST < FIRST.  (We're not checked in at
    // this point.)
    //
    if (LAST < FIRST) {
        return Ok(());
    }

    //
    // See whether record number RECNO in file HANDLE is buffered.  We'll
    // search through the list of buffered records starting at the head
    // of the list.  If we find the desired record, transfer the
    // requested data to the array DATAD and return without further ado.
    //
    save.NODE = save.HEADD;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFD[save.NODE]) && (RECNO == save.RNBUFD[save.NODE])) {
            //
            // Found it.  Move this record to the head of the list.
            // Update our head pointer as required.
            //
            if (save.NODE != save.HEADD) {
                LNKXSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADD, save.POOLD.as_slice_mut(), ctx)?;

                save.HEADD = save.NODE;
            }

            //
            // Don't forget to return the requested data.
            //
            if (FIRST == LAST) {
                DATAD[1] = save.RCBUFD[[FIRST, save.NODE]];
            } else {
                MOVED(
                    save.RCBUFD.subarray([FIRST, save.NODE]),
                    ((LAST - FIRST) + 1),
                    DATAD.as_slice_mut(),
                );
            }

            //
            // We haven't checked in, so don't check out.
            //
            return Ok(());
        }

        save.NODE = save.POOLD[[FORWRD, save.NODE]];
    }

    //
    // The record wasn't buffered.  We need to allocate entries to
    // hold the record contents.  If the buffer isn't full, just
    // select a free set of entries.  If the buffer is full, use
    // the set of entries at the tail of the list.
    //
    // Since we're now going to do a file read, it doesn't slow
    // us down much to check in, comparatively speaking.
    //
    CHKIN(b"DASRRD", ctx)?;

    if (save.USEDD == BUFSZD) {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADD, save.POOLD.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;

        //
        // If the allocated buffer entry was updated, write it out.
        //
        if save.UPBUFD[save.NODE] {
            //
            // We'll need a logical unit in order to write to the file.
            //
            ZZDDHHLU(save.HNBUFD[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DASRRD", ctx)?;
                return Ok(());
            }

            DASIOD(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFD[save.NODE],
                save.RCBUFD.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASRRD", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // Allocate a new set of buffer entries, but don't link
        // them into the list yet.
        //
        LNKAN(save.POOLD.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDD = (save.USEDD + 1);
    }

    //
    // Try to read the record.
    //
    ZZDASGRD(HANDLE, RECNO, save.RCBUFD.subarray_mut([1, save.NODE]), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASRRD", ctx)?;
        return Ok(());
    }

    //
    // The read was successful.  Link the node pointing to the buffer
    // entries for this record in before the current head of the
    // list, thus putting them at the head.
    //
    // Set the file handle, record number, and update flag for
    // this record.
    //
    LNKILB(save.NODE, save.HEADD, save.POOLD.as_slice_mut(), ctx)?;

    save.HNBUFD[save.NODE] = HANDLE;
    save.RNBUFD[save.NODE] = RECNO;
    save.UPBUFD[save.NODE] = false;
    save.HEADD = save.NODE;

    //
    // Don't forget to return the requested data.
    //
    MOVED(
        save.RCBUFD.subarray([FIRST, save.NODE]),
        ((LAST - FIRST) + 1),
        DATAD.as_slice_mut(),
    );

    CHKOUT(b"DASRRD", ctx)?;
    Ok(())
}

/// DAS, read record, integer
///
/// Read DAS integer physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  FIRST,
///  LAST       I   First and last indices of range within record.
///  DATAI      O   Integer data read from record.
///  BUFSZI     P   Number of records in the integer record buffer.
///  NWI        P   Number of integers in a single DAS integer record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an open DAS file.
///
///  RECNO    is the number of a record in a DAS file.
///
///  FIRST,
///  LAST     are the first and last indices of a range of
///           integers to be read from the indicated record.
///           The record contains NWI integers; these have
///           indices ranging from 1 to NWI.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATAI    is an integer array containing the elements FIRST
///           through LAST of the specified record. The record
///           element FIRST is placed in DATAI(1), the record
///           element FIRST+1 is placed in DATAI(2), and so on;
///           the record element LAST is placed in
///           DATAI(LAST-FIRST+1).
/// ```
///
/// # Parameters
///
/// ```text
///  NWI      is the number of integers in a single DAS record
///           containing integers.
///
///  BUFSZI   is the number of records in the integer record
///           buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled
///      by a routine in the call tree of this routine. The
///      output argument DATAI will not be modified.
///
///  2)  If a read operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine.
///      The output argument DATAI will not be modified.
///
///  3)  If a write operation attempted by this routine fails, an
///      error is signaled by a routine in the call tree of this
///      routine. The output argument DATAI will not be modified. This
///      routine may write out updated, buffered records in order to
///      make room in the integer buffer for a newly read record. Note
///      that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
///
///  4)  If FIRST or LAST is not in the range [1, NWI], the error
///      SPICE(INDEXOUTOFRANGE) is signaled. The output argument
///      DATAI will not be modified.
///
///  5)  If FIRST > LAST, this routine will return without modifying
///      the output argument DATAI.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to read from a DAS file that is open for
///  reading or writing. Any buffered integer record can be read with
///  this routine. In particular, records that have been written to
///  the DAS integer record buffer but have not yet been written out
///  to the DAS file they're intended to go to ARE visible to this
///  routine.
///
///  This routine should be used to read only records that contain
///  integer data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Read the 10th through 100th integers from record number 9
///      in a DAS file designated by HANDLE.
///
///          CALL DASRRI ( HANDLE, 9, 10, 100, DATAI )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Updated the header to comply with NAIF standard. Cleaned up
///         the $Revisions history.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration and
///         reading of non-native files.
///
/// -    SPICELIB Version 1.1.1, 10-FEB-2014 (BVS)
///
///         Added description of NWI to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.1.0, 03-NOV-1995 (NJB)
///
///         Made modifications to enhance efficiency. Removed references
///         to the function RETURN.
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration and
///         reading of non-native files.
///
/// -    SPICELIB Version 1.1.0, 03-NOV-1995 (NJB)
///
///         Made modifications to enhance efficiency. Removed references
///         to the function RETURN. For buffered reads, MOVEI is not
///         called when a single word is to be read.
/// ```
pub fn dasrri(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    first: i32,
    last: i32,
    datai: &mut [i32],
) -> crate::Result<()> {
    DASRRI(handle, recno, first, last, datai, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRRI ( DAS, read record, integer )
pub fn DASRRI(
    HANDLE: i32,
    RECNO: i32,
    FIRST: i32,
    LAST: i32,
    DATAI: &mut [i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut DATAI = DummyArrayMut::new(DATAI, 1..);

    //
    // Non-standard SPICE error handling.
    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Check FIRST and LAST.  Use discovery check-in.
    //
    if ((((FIRST < 1) || (FIRST > NWI)) || (LAST < 1)) || (LAST > NWI)) {
        CHKIN(b"DASRRI", ctx)?;
        SETMSG(b"Array indices FIRST and LAST were #,  #; allowed range for both is [#, #]. File was #, record number was #.", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", 1, ctx);
        ERRINT(b"#", NWI, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASRRI", ctx)?;
        return Ok(());
    }

    //
    // There's nothing to do if LAST < FIRST.  (We're not checked in at
    // this point.)
    //
    if (LAST < FIRST) {
        return Ok(());
    }

    //
    // See whether record number RECNO in file HANDLE is buffered.  We'll
    // search through the list of buffered records starting at the head
    // of the list.  If we find the desired record, transfer the
    // requested data to the array DATAI and return without further ado.
    //
    save.NODE = save.HEADI;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFI[save.NODE]) && (RECNO == save.RNBUFI[save.NODE])) {
            //
            //
            // Found it.  Move this record to the head of the list.
            // Update our head pointer as required.
            //
            if (save.NODE != save.HEADI) {
                LNKXSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADI, save.POOLI.as_slice_mut(), ctx)?;

                save.HEADI = save.NODE;
            }

            //
            // Don't forget to return the requested data.
            //
            if (FIRST == LAST) {
                DATAI[1] = save.RCBUFI[[FIRST, save.NODE]];
            } else {
                MOVEI(
                    save.RCBUFI.subarray([FIRST, save.NODE]),
                    ((LAST - FIRST) + 1),
                    DATAI.as_slice_mut(),
                );
            }

            //
            // We haven't checked in, so don't check out.
            //
            return Ok(());
        }

        save.NODE = save.POOLI[[FORWRD, save.NODE]];
    }

    //
    // The record wasn't buffered.  We need to allocate entries to
    // hold the record contents.  If the buffer isn't full, just
    // select a free set of entries.  If the buffer is full, use
    // the set of entries at the tail of the list.
    //
    // Since we're now going to do a file read, it doesn't slow
    // us down much to check in, comparatively speaking.
    //
    CHKIN(b"DASRRI", ctx)?;

    if (save.USEDI == BUFSZI) {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADI, save.POOLI.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;

        //
        // If the allocated buffer entry was updated, write it out.
        //
        if save.UPBUFI[save.NODE] {
            ZZDDHHLU(save.HNBUFI[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOI(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFI[save.NODE],
                save.RCBUFI.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASRRI", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // Allocate a new set of buffer entries, but don't link
        // them into the list yet.
        //
        LNKAN(save.POOLI.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDI = (save.USEDI + 1);
    }

    //
    // Try to read the record.
    //
    ZZDASGRI(HANDLE, RECNO, save.RCBUFI.subarray_mut([1, save.NODE]), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASRRI", ctx)?;
        return Ok(());
    }

    //
    // The read was successful.  Link the node pointing to the buffer
    // entries for this record in before the current head of the
    // list, thus putting them at the head.
    //
    // Set the file handle, record number, and update flag for
    // this record.
    //
    LNKILB(save.NODE, save.HEADI, save.POOLI.as_slice_mut(), ctx)?;

    save.HNBUFI[save.NODE] = HANDLE;
    save.RNBUFI[save.NODE] = RECNO;
    save.UPBUFI[save.NODE] = false;
    save.HEADI = save.NODE;

    //
    // Don't forget to return the requested data.
    //
    MOVEI(
        save.RCBUFI.subarray([FIRST, save.NODE]),
        ((LAST - FIRST) + 1),
        DATAI.as_slice_mut(),
    );

    CHKOUT(b"DASRRI", ctx)?;
    Ok(())
}

/// DAS, read record, character
///
/// Read DAS character physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  FIRST,
///  LAST       I   First and last indices of range within record.
///  DATAC      O   Character data read from record.
///  BUFSZC     P   Number of records in the character record buffer.
///  NWC        P   Number of characters in a single DAS char. record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of an open DAS file.
///
///  RECNO    is the number of a record in a DAS file.
///
///  FIRST,
///  LAST     are the first and last indices of a range of
///           characters to be read from the indicated record.
///           The record contains NWC characters; these have
///           indices ranging from 1 to NWC.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DATAC    is a character string containing the elements
///           FIRST through LAST of the specified record. The
///           record element FIRST is placed in DATAC(1:1), the
///           record element FIRST+1 is placed in DATAC(2:2),
///           and so on; the record element LAST is placed in
///           DATAC( LAST-FIRST+1 : LAST-FIRST+1 ).
/// ```
///
/// # Parameters
///
/// ```text
///  NWC      is the number of characters in a single DAS record
///           containing characters.
///
///  BUFSZC   is the number of records in the character record
///           buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is
///      signaled by a routine in the call tree of this routine. The
///      output argument DATAC will not be modified.
///
///  2)  If a read operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine.
///      The output argument DATAC will not be modified.
///
///  3)  If a write operation attempted by this routine fails, an
///      error is signaled by a routine in the call tree of this
///      routine. The output argument DATAC will not be modified. This
///      routine may write out updated, buffered records in order to
///      make room in the character buffer for a newly read record.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
///
///  4)  If FIRST or LAST is not in the range [1, NWC], the error
///      SPICE(INDEXOUTOFRANGE) is signaled. The output argument
///      DATAC will not be modified.
///
///  5)  If FIRST > LAST, this routine will return without modifying
///      the output argument DATAC.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to read from a DAS file that is open for
///  reading or writing. Any buffered character record can be read
///  with this routine. In particular, records that have been
///  written to the DAS character record buffer but have not yet been
///  written out to the DAS file they're intended to go to ARE
///  visible to this routine.
///
///  This routine should be used to read only records that contain
///  character data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Read the 10th through 100th characters from record number 9
///      in a DAS file designated by HANDLE.
///
///          CALL DASRRC ( HANDLE, 9, 10, 100, DATAC )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Updated the header to comply with NAIF standard. Cleaned up
///         the $Revisions history.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.1.1, 10-FEB-2014 (BVS)
///
///         Added description of NWC to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.1.0, 09-NOV-1995 (NJB)
///
///         Made modifications to enhance efficiency. Removed references
///         to the function RETURN.
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.1.0, 09-NOV-1995 (NJB)
///
///         Made modifications to enhance efficiency. Removed references
///         to the function RETURN.
/// ```
pub fn dasrrc(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    first: i32,
    last: i32,
    datac: &mut str,
) -> crate::Result<()> {
    DASRRC(
        handle,
        recno,
        first,
        last,
        fstr::StrBytes::new(datac).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASRRC ( DAS, read record, character )
pub fn DASRRC(
    HANDLE: i32,
    RECNO: i32,
    FIRST: i32,
    LAST: i32,
    DATAC: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Check FIRST and LAST.  Use discovery check-in.
    //
    if ((((FIRST < 1) || (FIRST > NWC)) || (LAST < 1)) || (LAST > NWC)) {
        CHKIN(b"DASRRC", ctx)?;
        SETMSG(b"Array indices FIRST and LAST were #,  #; allowed range for both is [#, #]. File was #, record number was #.", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", 1, ctx);
        ERRINT(b"#", NWC, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASRRC", ctx)?;
        return Ok(());
    }

    //
    // There's nothing to do if LAST < FIRST.  (We're not checked in at
    // this point.)
    //
    if (LAST < FIRST) {
        return Ok(());
    }

    //
    // See whether record number RECNO in file HANDLE is buffered.  We'll
    // search through the list of buffered records starting at the head
    // of the list.  If we find the desired record, transfer the
    // requested data to the array DATAC and return without further ado.
    //
    save.NODE = save.HEADC;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFC[save.NODE]) && (RECNO == save.RNBUFC[save.NODE])) {
            //
            //
            // Found it.  Move this record to the head of the list.
            // Update our head pointer as required.
            //
            if (save.NODE != save.HEADC) {
                LNKXSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADC, save.POOLC.as_slice_mut(), ctx)?;

                save.HEADC = save.NODE;
            }

            //
            // Don't forget to return the requested data.
            //
            fstr::assign(
                DATAC,
                fstr::substr(save.RCBUFC.get(save.NODE), FIRST..=LAST),
            );

            //
            // We haven't checked in, so don't check out.
            //
            return Ok(());
        }

        save.NODE = save.POOLC[[FORWRD, save.NODE]];
    }

    //
    // The record wasn't buffered.  We need to allocate entries to
    // hold the record contents.  If the buffer isn't full, just
    // select a free set of entries.  If the buffer is full, use
    // the set of entries at the tail of the list.
    //
    // Since we're now going to do a file read, it doesn't slow
    // us down much to check in, comparatively speaking.
    //
    CHKIN(b"DASRRC", ctx)?;

    if (save.USEDC == BUFSZC) {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADC, save.POOLC.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;

        //
        // If the allocated buffer entry was updated, write it out.
        //
        if save.UPBUFC[save.NODE] {
            ZZDDHHLU(save.HNBUFC[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOC(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFC[save.NODE],
                &mut save.RCBUFC[save.NODE],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASRRC", ctx)?;
                return Ok(());
            }
        }
    } else {
        //
        // Allocate a new set of buffer entries, but don't link
        // them into the list yet.
        //
        LNKAN(save.POOLC.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDC = (save.USEDC + 1);
    }

    //
    // Try to read the record.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut save.UNIT, ctx)?;

    DASIOC(b"READ", save.UNIT, RECNO, &mut save.RCBUFC[save.NODE], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASRRC", ctx)?;
        return Ok(());
    }

    //
    // The read was successful.  Link the node pointing to the buffer
    // entries for this record in before the current head of the
    // list, thus putting them at the head.
    //
    // Set the file handle, record number, and update flag for
    // this record.
    //
    LNKILB(save.NODE, save.HEADC, save.POOLC.as_slice_mut(), ctx)?;

    save.HNBUFC[save.NODE] = HANDLE;
    save.RNBUFC[save.NODE] = RECNO;
    save.UPBUFC[save.NODE] = false;
    save.HEADC = save.NODE;

    //
    // Don't forget to return the requested data.
    //
    fstr::assign(
        DATAC,
        fstr::substr(save.RCBUFC.get(save.NODE), FIRST..=LAST),
    );

    CHKOUT(b"DASRRC", ctx)?;
    Ok(())
}

/// DAS, write record, double precision
///
/// Write DAS double precision physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  RECD       I   Double precision data to be written to record.
///  BUFSZD     P   Number of records in the DP record buffer.
///  NWD        P   Number of DP in a single DAS DP record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
///
///  RECNO    is the number of a record in a DAS file.
///
///  RECD     is an array of NWD double precision numbers. The
///           contents of this array are to be written to the
///           physical file record having number RECNO.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  NWD      is the number of DPs in a single DAS record
///           containing DPs.
///
///  BUFSZD   is the number of records in the double precision
///           record buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The DAS file
///      designated by HANDLE will not be modified.
///
///  2)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      status of the DAS file written to is uncertain in this case.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to write to only DAS files that are open
///  for writing. Records written via this routine will always be
///  buffered immediately, but may not be written to the file until
///  they are cleared from the double precision buffer to make room
///  for other records, or until they are explicitly forced to to be
///  written via a call to DASWBR. In any case, at the moment this
///  routine returns, the data supplied on input may be read back by
///  DASRRD or updated by DASURD.
///
///  Closing a DAS file via DASCLS forces any remaining updated data
///  records buffered by this routine to be written to the file.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Write an array of NWD double precision numbers to the 9th
///      record in a DAS file designated by HANDLE.
///
///         DOUBLE PRECISION        RECD
///
///                      .
///                      .
///                      .
///
///         DO I = 1, NWD
///            RECD(I) = DBLE(I)
///         END DO
///
///         CALL DASWRD ( HANDLE, 9, RECD )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-2014 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.3, 10-FEB-2014 (BVS)
///
///         Added description of NWD to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn daswrd(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    recd: &[f64; 128],
) -> crate::Result<()> {
    DASWRD(handle, recno, recd, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASWRD ( DAS, write record, double precision )
pub fn DASWRD(HANDLE: i32, RECNO: i32, RECD: &[f64], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RECD = DummyArray::new(RECD, 1..=NWD);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASWRD", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASWRD", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // See whether double precision record number RECNO from file HANDLE
    // is buffered.  We'll search through the list of buffered records
    // starting at the head of the list.  If the record is already
    // buffered, we'll update the buffer entry, but we'll defer writing
    // the record out until we need to free a record, or until the
    // d.p. buffer is flushed, whichever comes first.
    //
    save.NODE = save.HEADD;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFD[save.NODE]) && (RECNO == save.RNBUFD[save.NODE])) {
            //
            // Found it.  Update the buffered record.
            //
            MOVED(
                RECD.as_slice(),
                NWD,
                save.RCBUFD.subarray_mut([1, save.NODE]),
            );

            //
            // Set the update flag, indicating that this buffer entry
            // has been modified.
            //
            save.UPBUFD[save.NODE] = true;

            //
            // Put the information about this record at the head of the
            // active list, if it is not already there.
            //
            if (save.NODE != save.HEADD) {
                LNKXSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADD, save.POOLD.as_slice_mut(), ctx)?;
                save.HEADD = save.NODE;
            }

            CHKOUT(b"DASWRD", ctx)?;
            return Ok(());
        }

        save.NODE = save.POOLD[[FORWRD, save.NODE]];
    }

    //
    // The record we're writing to is not buffered.  We'll allocate
    // a buffer entry.  If the record buffer is full, we'll
    // commandeer the least recently accessed record.  Before using
    // this record, we'll write its contents out to the corresponding
    // file, if the record has been updated.
    //
    if (save.USEDD < BUFSZD) {
        //
        // There's a free buffer entry available.  Just allocate it.
        //
        LNKAN(save.POOLD.as_slice_mut(), &mut save.NODE, ctx)?;

        save.USEDD = (save.USEDD + 1);
    } else {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADD, save.POOLD.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;

        //
        // If the allocated record was updated, write it out.
        //
        if save.UPBUFD[save.NODE] {
            ZZDDHHLU(save.HNBUFD[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOD(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFD[save.NODE],
                save.RCBUFD.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASWRD", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now update the allocated buffer entry with the input data.
    //
    MOVED(
        RECD.as_slice(),
        NWD,
        save.RCBUFD.subarray_mut([1, save.NODE]),
    );

    //
    // Set the update flag, indicating that this buffer entry
    // has been modified. Also set the handle and record number
    // entries.
    //
    save.UPBUFD[save.NODE] = true;
    save.HNBUFD[save.NODE] = HANDLE;
    save.RNBUFD[save.NODE] = RECNO;

    //
    // Link this buffer entry to the head of the list.
    //
    LNKILB(save.NODE, save.HEADD, save.POOLD.as_slice_mut(), ctx)?;

    save.HEADD = save.NODE;

    CHKOUT(b"DASWRD", ctx)?;
    Ok(())
}

/// DAS, write record, integer
///
/// Write DAS integer physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  RECI       I   Integer data to be written to record.
///  BUFSZI     P   Number of records in the integer record buffer.
///  NWI        P   Number of integers in a single DAS integer record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
///
///  RECNO    is the number of a record in a DAS file.
///
///  RECI     is an array of NWI integers. The contents of this
///           array are to be written to the physical file
///           record having number RECNO.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  NWI      is the number of integers in a single DAS record
///           containing integers.
///
///  BUFSZI   is the number of records in the integer record
///           buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The DAS file
///      designated by HANDLE will not be modified.
///
///  2)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      status of the DAS file written to is uncertain in this case.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to write to only DAS files that are open
///  for writing. Records written via this routine will always be
///  buffered immediately, but may not be written to the file until
///  they are cleared from the integer buffer to make room for other
///  records, or until they are explicitly forced to to be written via
///  a call to DASWBR. In any case, at the moment this routine
///  returns, the data supplied on input may be read back by DASRRI
///  or updated by DASURI.
///
///  Closing a DAS file via DASCLS forces any remaining updated data
///  records buffered by this routine to be written to the file.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Write an array of NWI integers to the 9th record in a DAS
///      file designated by HANDLE.
///
///         INTEGER                RECI ( NWI )
///                      .
///                      .
///                      .
///
///         DO I = 1, NWI
///            RECI(I) = I
///         END DO
///
///         CALL DASWRI ( HANDLE, 9, RECI )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-2014 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.3, 10-FEB-2014 (BVS)
///
///         Added description of NWI to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn daswri(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    reci: &[i32; 256],
) -> crate::Result<()> {
    DASWRI(handle, recno, reci, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASWRI ( DAS, write record, integer )
pub fn DASWRI(HANDLE: i32, RECNO: i32, RECI: &[i32], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let RECI = DummyArray::new(RECI, 1..=NWI);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASWRI", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASWRI", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // See whether integer record number RECNO from file HANDLE is
    // buffered.  We'll search through the list of buffered records
    // starting at the head of the list.  If the record is already
    // buffered, we'll update the buffer entry, but we'll defer writing
    // the record out until we need to free a record, or until the
    // integer buffer is flushed, whichever comes first.
    //
    save.NODE = save.HEADI;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFI[save.NODE]) && (RECNO == save.RNBUFI[save.NODE])) {
            //
            // Found it.  Update the buffered record.
            //
            MOVEI(
                RECI.as_slice(),
                NWI,
                save.RCBUFI.subarray_mut([1, save.NODE]),
            );

            //
            // Set the update flag, indicating that this buffer entry
            // has been modified.
            //
            save.UPBUFI[save.NODE] = true;

            //
            // Put the information about this record at the head of the
            // active list, if it is not already there.
            //
            if (save.NODE != save.HEADI) {
                LNKXSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADI, save.POOLI.as_slice_mut(), ctx)?;
                save.HEADI = save.NODE;
            }

            CHKOUT(b"DASWRI", ctx)?;
            return Ok(());
        }

        save.NODE = save.POOLI[[FORWRD, save.NODE]];
    }

    //
    // The record we're writing to is not buffered.  We'll allocate
    // a buffer entry.  If the record buffer is full, we'll
    // commandeer the least recently accessed record.  Before using
    // this record, we'll write its contents out to the corresponding
    // file, if the record has been updated.
    //
    if (save.USEDI < BUFSZI) {
        //
        // There's a free buffer entry available.  Just allocate it.
        //
        LNKAN(save.POOLI.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDI = (save.USEDI + 1);
    } else {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADI, save.POOLI.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;

        //
        // If the allocated record was updated, write it out.
        //
        if save.UPBUFI[save.NODE] {
            ZZDDHHLU(save.HNBUFI[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOI(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFI[save.NODE],
                save.RCBUFI.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASWRI", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now update the allocated buffer entry with the input data.
    //
    MOVEI(
        RECI.as_slice(),
        NWI,
        save.RCBUFI.subarray_mut([1, save.NODE]),
    );

    //
    // Set the update flag, indicating that this buffer entry
    // has been modified.  Also set the handle and record number
    // entries.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut save.UNIT, ctx)?;

    save.UPBUFI[save.NODE] = true;
    save.HNBUFI[save.NODE] = HANDLE;
    save.RNBUFI[save.NODE] = RECNO;

    //
    // Link this buffer entry to the head of the list.
    //
    LNKILB(save.NODE, save.HEADI, save.POOLI.as_slice_mut(), ctx)?;

    save.HEADI = save.NODE;

    CHKOUT(b"DASWRI", ctx)?;
    Ok(())
}

/// DAS, write record, character
///
/// Write DAS character physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  RECC       I   Character data to be written to record.
///  BUFSZC     P   Number of records in the character record buffer.
///  NWC        P   Number of characters in a single DAS char. record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
///
///  RECNO    is the number of a record in a DAS file.
///
///  RECC     is a string of length NWC. The contents of this
///           string are to be written to the physical file
///           record having number RECNO.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  NWC      is the number of characters in a single DAS record
///           containing characters.
///
///  BUFSZC   is the number of records in the character record
///           buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The DAS file
///      designated by HANDLE will not be modified.
///
///  2)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      status of the DAS file written to is uncertain in this case.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to write to only DAS files that are open
///  for writing. Records written via this routine will always be
///  buffered immediately, but may not be written to the file until
///  they are cleared from the character buffer to make room for other
///  records, or until they are explicitly forced to to be written via
///  a call to DASWBR. In any case, at the moment this routine
///  returns, the data supplied on input may be read back by DASRRC
///  or updated by DASURC.
///
///  Closing a DAS file via DASCLS forces any remaining updated data
///  records buffered by this routine to be written to the file.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Write a string of NWC characters to the 9th record in a DAS
///      file designated by HANDLE.
///
///         CHARACTER*(NWC)           RECC
///
///                      .
///                      .
///                      .
///
///         RECC = 'This example string is blank-padded on the '    //
///        .       'right.  All of the trailing blanks will be '    //
///        .       'written to the DAS file by the following call.'
///
///         CALL DASWRC ( HANDLE, 9, RECC )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-2014 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.3, 10-FEB-2014 (BVS)
///
///         Added description of NWC to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn daswrc(ctx: &mut SpiceContext, handle: i32, recno: i32, recc: &str) -> crate::Result<()> {
    DASWRC(handle, recno, recc.as_bytes(), ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASWRC ( DAS, write record, character )
pub fn DASWRC(HANDLE: i32, RECNO: i32, RECC: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASWRC", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASWRC", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // See whether character record number RECNO from file HANDLE is
    // buffered.  We'll search through the list of buffered records
    // starting at the head of the list.  If the record is already
    // buffered, we'll update the buffer entry, but we'll defer writing
    // the record out until we need to free a record, or until the
    // character buffer is flushed, whichever comes first.
    //
    save.NODE = save.HEADC;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFC[save.NODE]) && (RECNO == save.RNBUFC[save.NODE])) {
            //
            // Found it.  Update the buffered record.
            //
            fstr::assign(save.RCBUFC.get_mut(save.NODE), RECC);

            //
            // Set the update flag, indicating that this buffer entry
            // has been modified.
            //
            save.UPBUFC[save.NODE] = true;

            //
            // Put the information about this record at the head of the
            // active list, if it is not already there.
            //
            if (save.NODE != save.HEADC) {
                LNKXSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADC, save.POOLC.as_slice_mut(), ctx)?;
                save.HEADC = save.NODE;
            }

            CHKOUT(b"DASWRC", ctx)?;
            return Ok(());
        }

        save.NODE = save.POOLC[[FORWRD, save.NODE]];
    }

    //
    // The record we're writing to is not buffered.  We'll allocate
    // a buffer entry.  If the record buffer is full, we'll
    // commandeer the least recently accessed record.  Before using
    // this record, we'll write its contents out to the corresponding
    // file, if the record has been updated.
    //
    if (save.USEDC < BUFSZC) {
        //
        // There's a free buffer entry available.  Just allocate it.
        //
        LNKAN(save.POOLC.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDC = (save.USEDC + 1);
    } else {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADC, save.POOLC.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;

        //
        // If the allocated record was updated, write it out.
        //
        if save.UPBUFC[save.NODE] {
            ZZDDHHLU(save.HNBUFC[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOC(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFC[save.NODE],
                &mut save.RCBUFC[save.NODE],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASWRC", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now update the allocated buffer entry with the input data.
    //
    fstr::assign(save.RCBUFC.get_mut(save.NODE), RECC);

    //
    // Set the update flag, indicating that this buffer entry
    // has been modified.  Also set the handle and record number
    // entries.
    //
    save.UPBUFC[save.NODE] = true;
    save.HNBUFC[save.NODE] = HANDLE;
    save.RNBUFC[save.NODE] = RECNO;

    //
    // Link this buffer entry to the head of the list.
    //
    LNKILB(save.NODE, save.HEADC, save.POOLC.as_slice_mut(), ctx)?;

    save.HEADC = save.NODE;

    CHKOUT(b"DASWRC", ctx)?;
    Ok(())
}

/// DAS, update record, double precision
///
/// Update DAS double precision physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  FIRST,
///  LAST       I   First and last indices of range within record.
///  DATAD      I   Double precision data to write to record.
///  BUFSZD     P   Number of records in the DP record buffer.
///  NWD        P   Number of DPs in a single DAS DP record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
///
///  RECNO    is the number of a record in a DAS file.
///
///  FIRST,
///  LAST     are the first and last indices of a range of
///           elements to be updated in the indicated record.
///           The record contains NWD double precision numbers;
///           these have indices ranging from 1 to NWD.
///
///  DATAD    is a double precision array to be written to
///           elements FIRST through LAST of the specified
///           record. The array element DATAD(1) is placed in
///           record element FIRST, the array element DATAD(2)
///           is placed in record element FIRST+1, and so on;
///           the array element DATAD(LAST-FIRST+1) is placed in
///           the record element LAST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  NWD      is the number of DPs in a single DAS record
///           containing DPs.
///
///  BUFSZD   is the number of records in the double precision
///           record buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine may be used to update only records that have
///      already been written by DASWRD or that already exist in the
///      file designated by HANDLE. Attempting to update a record
///      that hasn't yet been written will cause the read operation
///      performed by this routine to fail.
///
///      If a read operation attempted by this routine fails for this
///      or any other reason, an error is signaled by a routine in the
///      call tree of this routine. The indicated record will not be
///      modified.
///
///  2)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      status of the DAS file written to is uncertain in this case.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
///
///  3)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The indicated record
///      will not be modified.
///
///  4)  If FIRST or LAST is not in the range [1, NWD], the error
///      SPICE(INDEXOUTOFRANGE) is signaled. The indicated
///      record will not be modified.
///
///  5)  If FIRST > LAST, this routine will return without modifying
///      the indicated record.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to update any existing record in a DAS
///  file that is open for writing, or any record that has been
///  `written' by DASWRD, whether or not that record has yet been
///  physically written to the file it belongs to. Records that have
///  never been written cannot be updated.
///
///  Because the DAS system buffers records that are written, multiple
///  updates of parts of a record can be made without incurring a
///  large number of file reads and writes.
///
///  This routine should be used to update only records that contain
///  double precision data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Update the 10th through 100th d.p. numbers in record number 9
///      in a DAS file designated by HANDLE.
///
///          DOUBLE PRECISION      DATAD ( 100 )
///
///                      .
///                      .
///                      .
///
///          DO I = 1, 91
///             DATAD  =  DBLE(I)
///          END DO
///
///          CALL DASURD ( HANDLE, 9, 10, 100, DATAD )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.3, 10-FEB-2014 (BVS)
///
///         Added description of NWD to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dasurd(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    first: i32,
    last: i32,
    datad: &[f64],
) -> crate::Result<()> {
    DASURD(handle, recno, first, last, datad, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASURD ( DAS, update record, double precision )
pub fn DASURD(
    HANDLE: i32,
    RECNO: i32,
    FIRST: i32,
    LAST: i32,
    DATAD: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATAD = DummyArray::new(DATAD, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASURD", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASURD", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // If FIRST or LAST are out of range, no dice.
    //
    if ((((FIRST < 1) || (FIRST > NWD)) || (LAST < 1)) || (LAST > NWD)) {
        SETMSG(b"Array indices FIRST and LAST were #,  #; allowed range for both is [#, #]. File was #, record number was #.", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", 1, ctx);
        ERRINT(b"#", NWD, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASURD", ctx)?;
        return Ok(());
    }

    //
    // There's nothing to do if LAST < FIRST.
    //
    if (LAST < FIRST) {
        CHKOUT(b"DASURD", ctx)?;
        return Ok(());
    }

    //
    // See whether double precision record number RECNO from file HANDLE
    // is buffered.  We'll search through the list of buffered records
    // starting at the head of the list.  If the record is already
    // buffered, we'll update the buffer entry, but we'll defer writing
    // the record out until we need to free a record, or until the
    // d.p. buffer is flushed, whichever comes first.
    //
    save.NODE = save.HEADD;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFD[save.NODE]) && (RECNO == save.RNBUFD[save.NODE])) {
            //
            // Found it.  Update the buffered record.
            //
            MOVED(
                DATAD.as_slice(),
                ((LAST - FIRST) + 1),
                save.RCBUFD.subarray_mut([FIRST, save.NODE]),
            );

            //
            // Set the update flag, indicating that this buffer entry
            // has been modified.
            //
            save.UPBUFD[save.NODE] = true;

            //
            // Put the information about this record at the head of the
            // active list, if it is not already there.
            //
            if (save.NODE != save.HEADD) {
                LNKXSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADD, save.POOLD.as_slice_mut(), ctx)?;
                save.HEADD = save.NODE;
            }

            CHKOUT(b"DASURD", ctx)?;
            return Ok(());
        }

        save.NODE = save.POOLD[[FORWRD, save.NODE]];
    }

    //
    // The record we're writing to is not buffered.  In order to
    // update this record, we'll need to read it first.  But before
    // we do that, we'll need to allocate a buffer entry.  If the record
    // buffer is full, we'll commandeer the least recently accessed
    // record.  Before using this record, we'll write its contents out
    // to the corresponding file, if the record has been updated.
    //
    if (save.USEDD < BUFSZD) {
        //
        // There's a free buffer entry available.  Just allocate it.
        //
        LNKAN(save.POOLD.as_slice_mut(), &mut save.NODE, ctx)?;

        save.USEDD = (save.USEDD + 1);
    } else {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADD, save.POOLD.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;

        //
        // If the allocated record was updated, write it out.
        //
        if save.UPBUFD[save.NODE] {
            ZZDDHHLU(save.HNBUFD[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOD(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFD[save.NODE],
                save.RCBUFD.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASURD", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now try to read the record we're going to update.
    //
    ZZDASGRD(HANDLE, RECNO, save.RCBUFD.subarray_mut([1, save.NODE]), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASURD", ctx)?;
        return Ok(());
    }

    //
    // The read was successful, so set the record number, handle,
    // and update flag for this buffer entry, and link these buffer
    // entries in before the current head of the list, thus putting
    // them at the head.
    //
    // Update the head pointer.
    //
    LNKILB(save.NODE, save.HEADD, save.POOLD.as_slice_mut(), ctx)?;

    save.HNBUFD[save.NODE] = HANDLE;
    save.RNBUFD[save.NODE] = RECNO;
    save.UPBUFD[save.NODE] = true;
    save.HEADD = save.NODE;

    //
    // At long last, make the requested update.  Note that we don't
    // have to write the record back to the file; that will get done
    // automatically before or at the time the file is closed.
    //
    MOVED(
        DATAD.as_slice(),
        ((LAST - FIRST) + 1),
        save.RCBUFD.subarray_mut([FIRST, save.NODE]),
    );

    CHKOUT(b"DASURD", ctx)?;
    Ok(())
}

/// DAS, update record, integer
///
/// Update DAS integer physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  FIRST,
///  LAST       I   First and last indices of range within record.
///  DATAI      I   Integer data to write to record.
///  BUFSZI     P   Number of records in the integer record buffer.
///  NWI        P   Number of integers in a single DAS integer record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
///
///  RECNO    is the number of a record in a DAS file.
///
///  FIRST,
///  LAST     are the first and last indices of a range of
///           elements to be updated in the indicated record.
///           The record contains NWI integers; these have
///           indices ranging from 1 to NWI.
///
///  DATAI    is an integer array to be written to elements FIRST
///           through LAST of the specified record. The array
///           element DATAI(1) is placed in record element FIRST,
///           the array element DATAI(2) is placed in record
///           element FIRST+1, and so on; the array element
///           DATAI(LAST-FIRST+1) is placed in the record element
///           LAST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  NWI      is the number of integers in a single DAS record
///           containing integers.
///
///  BUFSZI   is the number of records in the integer record
///           buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine may be used to update only records that have
///      already been written by DASWRI or that already exist in the
///      file designated by HANDLE. Attempting to update a record
///      that hasn't yet been written will cause the read operation
///      performed by this routine to fail.
///
///      If a read operation attempted by this routine fails for this
///      or any other reason, an error is signaled by a routine in the
///      call tree of this routine. The indicated record will not be
///      modified.
///
///  2)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      status of the DAS file written to is uncertain in this case.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
///
///  3)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The indicated record
///      will not be modified.
///
///  4)  If FIRST or LAST is not in the range [1, NWI], the error
///      SPICE(INDEXOUTOFRANGE) is signaled. The indicated
///      record will not be modified.
///
///  5)  If FIRST > LAST, this routine will return without modifying
///      the indicated record.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to update any existing record in a DAS
///  file that is open for writing, or any record that has been
///  `written' by DASWRI, whether or not that record has yet been
///  physically written to the file it belongs to. Records that have
///  never been written cannot be updated.
///
///  Because the DAS system buffers records that are written, multiple
///  updates of parts of a record can be made without incurring a
///  large number of file reads and writes.
///
///  This routine should be used to update only records that contain
///  integer data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Update the 10th through 100th integers in record number 9
///      in a DAS file designated by HANDLE.
///
///          INTEGER               DATAI ( 100 )
///
///                      .
///                      .
///                      .
///
///          DO I = 1, 91
///             DATAI  =  I
///          END DO
///
///          CALL DASURI ( HANDLE, 9, 10, 100, DATAI )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.3, 10-FEB-2014 (BVS)
///
///         Added description of NWI to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dasuri(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    first: i32,
    last: i32,
    datai: &[i32],
) -> crate::Result<()> {
    DASURI(handle, recno, first, last, datai, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASURI ( DAS, update record, integer )
pub fn DASURI(
    HANDLE: i32,
    RECNO: i32,
    FIRST: i32,
    LAST: i32,
    DATAI: &[i32],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DATAI = DummyArray::new(DATAI, 1..);

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASURI", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASURI", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // If FIRST or LAST are out of range, no dice.
    //
    if ((((FIRST < 1) || (FIRST > NWI)) || (LAST < 1)) || (LAST > NWI)) {
        SETMSG(b"Array indices FIRST and LAST were #,  #; allowed range for both is [#, #]. File was #, record number was #.", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", 1, ctx);
        ERRINT(b"#", NWI, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASURI", ctx)?;
        return Ok(());
    }

    //
    // There's nothing to do if LAST < FIRST.
    //
    if (LAST < FIRST) {
        CHKOUT(b"DASURI", ctx)?;
        return Ok(());
    }

    //
    // See whether integer record number RECNO from file HANDLE is
    // buffered.  We'll search through the list of buffered records
    // starting at the head of the list.  If the record is already
    // buffered, we'll update the buffer entry, but we'll defer writing
    // the record out until we need to free a record, or until the
    // integer buffer is flushed, whichever comes first.
    //
    save.NODE = save.HEADI;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFI[save.NODE]) && (RECNO == save.RNBUFI[save.NODE])) {
            //
            // Found it.  Update the buffered record.
            //
            MOVEI(
                DATAI.as_slice(),
                ((LAST - FIRST) + 1),
                save.RCBUFI.subarray_mut([FIRST, save.NODE]),
            );

            //
            // Set the update flag, indicating that this buffer entry
            // has been modified.
            //
            save.UPBUFI[save.NODE] = true;

            //
            // Put the information about this record at the head of the
            // active list, if it is not already there.
            //
            if (save.NODE != save.HEADI) {
                LNKXSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADI, save.POOLI.as_slice_mut(), ctx)?;
                save.HEADI = save.NODE;
            }

            CHKOUT(b"DASURI", ctx)?;
            return Ok(());
        }

        save.NODE = save.POOLI[[FORWRD, save.NODE]];
    }

    //
    // The record we're writing to is not buffered.  We'll allocate
    // a buffer entry.  If the record buffer is full, we'll
    // commandeer the least recently accessed record.  Before using
    // this record, we'll write its contents out to the corresponding
    // file, if the record has been updated.
    //
    if (save.USEDI < BUFSZI) {
        //
        // There's a free buffer entry available.  Just allocate it.
        //
        LNKAN(save.POOLI.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDI = (save.USEDI + 1);
    } else {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADI, save.POOLI.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;

        //
        // If the allocated record was updated, write it out.
        //
        if save.UPBUFI[save.NODE] {
            ZZDDHHLU(save.HNBUFI[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOI(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFI[save.NODE],
                save.RCBUFI.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASURI", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now try to read the record we're going to update.
    //
    ZZDASGRI(HANDLE, RECNO, save.RCBUFI.subarray_mut([1, save.NODE]), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASURI", ctx)?;
        return Ok(());
    }

    //
    // The read was successful, so set the record number, handle,
    // and update flag for this buffer entry, and link these buffer
    // entries in before the current head of the list, thus putting
    // them at the head.
    //
    // Update the head pointer.
    //
    LNKILB(save.NODE, save.HEADI, save.POOLI.as_slice_mut(), ctx)?;

    save.HNBUFI[save.NODE] = HANDLE;
    save.RNBUFI[save.NODE] = RECNO;
    save.UPBUFI[save.NODE] = true;
    save.HEADI = save.NODE;

    //
    // At long last, make the requested update.  Note that we don't
    // have to write the record back to the file; that will get done
    // automatically before or at the time the file is closed.
    //
    MOVEI(
        DATAI.as_slice(),
        ((LAST - FIRST) + 1),
        save.RCBUFI.subarray_mut([FIRST, save.NODE]),
    );

    CHKOUT(b"DASURI", ctx)?;
    Ok(())
}

/// DAS, update record, character
///
/// Update DAS character physical records.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
///  RECNO      I   Record number.
///  FIRST,
///  LAST       I   First and last indices of range within record.
///  DATAC      I   Character data to write to record.
///  BUFSZC     P   Number of records in the character record buffer.
///  NWC        P   Number of characters in a single DAS char. record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
///
///  RECNO    is the number of a record in a DAS file.
///
///  FIRST,
///  LAST     are the first and last indices of a range of
///           elements to be updated in the indicated record.
///           The record contains NWC characters; these have
///           indices ranging from 1 to NWC.
///
///  DATAC    is a character string to be written to elements
///           FIRST through LAST of the specified record. The
///           character DATAC(1:1) is placed in record element
///           FIRST, the character DATAC(2) is placed in record
///           element FIRST+1, and so on; the character
///           DATAC(LAST-FIRST+1) is placed in the record element
///           LAST.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None. See $Particulars for a description of the action of this
///  routine.
/// ```
///
/// # Parameters
///
/// ```text
///  NWC      is the number of characters in a single DAS record
///           containing characters.
///
///  BUFSZC   is the number of records in the character record
///           buffer.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine may be used to update only records that have
///      already been written by DASWRC or that already exist in the
///      file designated by HANDLE. Attempting to update a record
///      that hasn't yet been written will cause the read operation
///      performed by this routine to fail.
///
///      If a read operation attempted by this routine fails for this
///      or any other reason, an error is signaled by a routine in the
///      call tree of this routine. The indicated record will not be
///      modified.
///
///  2)  If a write operation attempted by this routine fails, an error
///      is signaled by a routine in the call tree of this routine. The
///      status of the DAS file written to is uncertain in this case.
///      Note that the file written to may be different than the file
///      designated by HANDLE if multiple DAS files are open for
///      writing.
///
///  3)  If the input file handle is invalid, an error is signaled by a
///      routine in the call tree of this routine. The indicated record
///      will not be modified.
///
///  4)  If FIRST or LAST is not in the range [1, NWC], the error
///      SPICE(INDEXOUTOFRANGE) is signaled. The indicated
///      record will not be modified.
///
///  5)  If FIRST > LAST, this routine will return without modifying
///      the indicated record.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Routines outside of SPICELIB will normally have no need to call
///  this routine.
///
///  This routine can be used to update any existing record in a DAS
///  file that is open for writing, or any record that has been
///  `written' by DASWRC, whether or not that record has yet been
///  physically written to the file it belongs to. Records that have
///  never been written cannot be updated.
///
///  Because the DAS system buffers records that are written, multiple
///  updates of parts of a record can be made without incurring a
///  large number of file reads and writes.
///
///  Any buffered character record can be updated with this routine.
///  In particular, records that have been written to the DAS character
///  record buffer but have not yet been written out to the DAS file
///  they're intended to go to ARE visible to this routine.
///
///  This routine should be used to update only records that contain
///  character data.
/// ```
///
/// # Examples
///
/// ```text
///  1)  Update the 10th through 100th characters in record number 9
///      in a DAS file designated by HANDLE.
///
///          CHARACTER*(100)       DATAC
///
///                      .
///                      .
///                      .
///
///          DATAC = 'The first 91 characters of this string, '      //
///         .        'including trailing blanks, will be written '   //
///         .        'to the indicated DAS file.'
///
///          CALL DASURC ( HANDLE, 9, 10, 100, DATAC )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 22-FEB-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 05-FEB-2015 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.3, 10-FEB-2014 (BVS)
///
///         Added description of NWC to the $Parameters and $Brief_I/O
///         sections of the header.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dasurc(
    ctx: &mut SpiceContext,
    handle: i32,
    recno: i32,
    first: i32,
    last: i32,
    datac: &str,
) -> crate::Result<()> {
    DASURC(
        handle,
        recno,
        first,
        last,
        datac.as_bytes(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASURC ( DAS, update record, character )
pub fn DASURC(
    HANDLE: i32,
    RECNO: i32,
    FIRST: i32,
    LAST: i32,
    DATAC: &[u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASURC", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASURC", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // If FIRST or LAST are out of range, no dice.
    //
    if ((((FIRST < 1) || (FIRST > NWC)) || (LAST < 1)) || (LAST > NWC)) {
        SETMSG(b"String indices FIRST and LAST were #,  #; allowed range for both is [#, #]. File was #, record number was #.", ctx);
        ERRINT(b"#", FIRST, ctx);
        ERRINT(b"#", LAST, ctx);
        ERRINT(b"#", 1, ctx);
        ERRINT(b"#", NWC, ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        ERRINT(b"#", RECNO, ctx);
        SIGERR(b"SPICE(INDEXOUTOFRANGE)", ctx)?;
        CHKOUT(b"DASURC", ctx)?;
        return Ok(());
    }

    //
    // There's nothing to do if LAST < FIRST.
    //
    if (LAST < FIRST) {
        CHKOUT(b"DASURC", ctx)?;
        return Ok(());
    }

    //
    // See whether character record number RECNO from file HANDLE is
    // buffered.  We'll search through the list of buffered records
    // starting at the head of the list.  If the record is already
    // buffered, we'll update the buffer entry, but we'll defer writing
    // the record out until we need to free a record, or until the
    // character buffer is flushed, whichever comes first.
    //
    save.NODE = save.HEADC;

    while (save.NODE > 0) {
        if ((HANDLE == save.HNBUFC[save.NODE]) && (RECNO == save.RNBUFC[save.NODE])) {
            //
            // Found it.  Update the buffered record.
            //
            fstr::assign(
                fstr::substr_mut(save.RCBUFC.get_mut(save.NODE), FIRST..=LAST),
                DATAC,
            );

            //
            // Set the update flag, indicating that this buffer entry
            // has been modified.
            //
            save.UPBUFC[save.NODE] = true;

            //
            // Put the information about this record at the head of the
            // active list, if it is not already there.
            //
            if (save.NODE != save.HEADC) {
                LNKXSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;
                LNKILB(save.NODE, save.HEADC, save.POOLC.as_slice_mut(), ctx)?;
                save.HEADC = save.NODE;
            }

            CHKOUT(b"DASURC", ctx)?;
            return Ok(());
        }

        save.NODE = save.POOLC[[FORWRD, save.NODE]];
    }

    //
    // The record we're writing to is not buffered.  We'll allocate
    // a buffer entry.  If the record buffer is full, we'll
    // commandeer the least recently accessed record.  Before using
    // this record, we'll write its contents out to the corresponding
    // file, if the record has been updated.
    //
    if (save.USEDC < BUFSZC) {
        //
        // There's a free buffer entry available.  Just allocate it.
        //
        LNKAN(save.POOLC.as_slice_mut(), &mut save.NODE, ctx)?;
        save.USEDC = (save.USEDC + 1);
    } else {
        //
        // Grab the buffer entry at the tail end of the list.
        //
        save.NODE = LNKTL(save.HEADC, save.POOLC.as_slice(), ctx)?;

        LNKXSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;

        //
        // If the allocated record was updated, write it out.
        //
        if save.UPBUFC[save.NODE] {
            ZZDDHHLU(save.HNBUFC[save.NODE], b"DAS", false, &mut save.WRUNIT, ctx)?;

            DASIOC(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFC[save.NODE],
                &mut save.RCBUFC[save.NODE],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASURC", ctx)?;
                return Ok(());
            }
        }
    }

    //
    // Now try to read the record we're going to update.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut save.UNIT, ctx)?;

    DASIOC(b"READ", save.UNIT, RECNO, &mut save.RCBUFC[save.NODE], ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASURC", ctx)?;
        return Ok(());
    }

    //
    // The read was successful, so set the record number, handle,
    // and update flag for this buffer entry, and link these buffer
    // entries in before the current head of the list, thus putting
    // them at the head.
    //
    // Update the head pointer.
    //
    LNKILB(save.NODE, save.HEADC, save.POOLC.as_slice_mut(), ctx)?;

    save.HNBUFC[save.NODE] = HANDLE;
    save.RNBUFC[save.NODE] = RECNO;
    save.UPBUFC[save.NODE] = true;
    save.HEADC = save.NODE;

    //
    // At long last, make the requested update.  Note that we don't
    // have to write the record back to the file; that will get done
    // automatically before or at the time the file is closed.
    //
    fstr::assign(
        fstr::substr_mut(save.RCBUFC.get_mut(save.NODE), FIRST..=LAST),
        DATAC,
    );

    CHKOUT(b"DASURC", ctx)?;
    Ok(())
}

/// DAS, write buffered records
///
/// Write out all buffered records of a specified DAS file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  HANDLE     I   Handle of DAS file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAS file opened for writing.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  See $Particulars for a description of the action of this routine.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input file handle is invalid, an error is signaled
///      by a routine in the call tree of this routine. The indicated
///      file will not be modified.
///
///  2)  If a write operation attempted by this routine fails, an
///      error is signaled by a routine in the call tree of this
///      routine. The status of the DAS file written to is uncertain
///      in this case.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument HANDLE in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine writes buffered records out to the DAS file to which
///  they correspond.
///
///  Because the DAS system buffers records that are written as well as
///  those that are read, data supplied to the DAS add data (DASADC,
///  DASADD, DASADI) and DAS update data (DASUDC, DASUDD, DASUDI)
///  routines on input has not necessarily been physically written to
///  the DAS file specified by the caller of those routines, at the
///  time those routines return. Before closing a DAS file that has
///  been opened for writing, the DAS system must write out to the file
///  any updated records present in the DAS buffers. The SPICELIB
///  routine DASCLS uses this routine to perform this function. The
///  routines DASAC and DASDC, through the use of the SPICELIB routines
///  DASACR and DASRCR, which respectively add comment records to or
///  delete comment records from a DAS file, use this routine to ensure
///  that the SPICELIB routine DASRWR record buffers don't become out
///  of sync with the file they operate upon.
///
///  In addition, this routine can be used by application programs
///  that create or update DAS files. The reason for calling this
///  routine directly would be to provide a measure of safety when
///  writing a very large file: if the file creation or update were
///  interrupted, the amount of work lost due to the loss of buffered,
///  unwritten records could be reduced.
///
///  However, routines outside of SPICELIB will generally not need to
///  call this routine directly.
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
///  1) Write a DAS file by adding data to it over multiple passes.
///     Avoid spending time on file segregation between writes.
///
///     Each pass opens the file, adds character, double precision,
///     and integer data to the file, writes out buffered data by
///     calling DASWBR, and closes the file without segregating the
///     data by calling DASLLC.
///
///     The program also checks the file: after the final write,
///     the program reads the data and compares it to expected values.
///
///     Note that most user-oriented applications should segregate a
///     DAS file after writing it, since this greatly enhances file
///     reading efficiency. The technique demonstrated here may be
///     useful for cases in which a file will be written via many
///     small data additions, and in which the file is read between
///     write operations.
///
///
///     Example code begins here.
///
///
///           PROGRAM DASWBR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters
///     C
///           INTEGER               FILSIZ
///           PARAMETER           ( FILSIZ = 255 )
///
///           INTEGER               FTYPLN
///           PARAMETER           ( FTYPLN = 3 )
///
///           INTEGER               CHRLEN
///           PARAMETER           ( CHRLEN = 50 )
///
///           INTEGER               IBUFSZ
///           PARAMETER           ( IBUFSZ = 20 )
///
///           INTEGER               DBUFSZ
///           PARAMETER           ( DBUFSZ = 30 )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(CHRLEN)    CHRBUF
///           CHARACTER*(FILSIZ)    FNAME
///           CHARACTER*(FTYPLN)    FTYPE
///           CHARACTER*(CHRLEN)    XCHRBF
///
///           DOUBLE PRECISION      DPBUF  ( DBUFSZ )
///           DOUBLE PRECISION      XDPBUF ( DBUFSZ )
///
///           INTEGER               FIRSTC
///           INTEGER               FIRSTD
///           INTEGER               FIRSTI
///           INTEGER               HANDLE
///           INTEGER               I
///           INTEGER               INTBUF ( IBUFSZ )
///           INTEGER               J
///           INTEGER               LASTC
///           INTEGER               LASTD
///           INTEGER               LASTI
///           INTEGER               NCALL
///           INTEGER               NCOMR
///           INTEGER               NPASS
///           INTEGER               PASSNO
///           INTEGER               XINTBF ( IBUFSZ )
///
///
///     C
///     C     Initial values
///     C
///           DATA                  FNAME  / 'daswbr_ex1.das' /
///           DATA                  FTYPE  / 'ANG' /
///           DATA                  NCALL  / 1000  /
///           DATA                  NCOMR  / 10    /
///           DATA                  NPASS  / 3     /
///
///     C
///     C     Open a new DAS file. We'll allocate NCOMR records
///     C     for comments. The file type is not one of the standard
///     C     types recognized by SPICE; however it can be used to
///     C     ensure the database file is of the correct type.
///     C
///     C     We'll use the file name as the internal file name.
///     C
///           CALL DASONW ( FNAME, FTYPE, FNAME, NCOMR, HANDLE )
///
///     C
///     C     Add data of character, integer, and double precision
///     C     types to the file in interleaved fashion. We'll add to
///     C     the file over NPASS "passes," in each of which we close
///     C     the file after writing.
///     C
///           DO PASSNO = 1, NPASS
///
///              IF ( PASSNO .GT. 1 ) THEN
///
///                 WRITE (*,*) 'Opening file for write access...'
///
///                 CALL DASOPW( FNAME, HANDLE )
///
///              END IF
///
///              DO I = 1, NCALL
///     C
///     C           Add string data to the file.
///     C
///                 CHRBUF = 'Character value #'
///                 CALL REPMI( CHRBUF, '#', I, CHRBUF )
///
///                 CALL DASADC ( HANDLE, CHRLEN, 1, CHRLEN, CHRBUF )
///
///     C
///     C           Add double precision data to the file.
///     C
///                 DO J = 1, DBUFSZ
///                    DPBUF(J) = DBLE( 100000000*PASSNO + 100*I + J )
///                 END DO
///
///                 CALL DASADD ( HANDLE, DBUFSZ, DPBUF )
///
///     C
///     C           Add integer data to the file.
///     C
///                 DO J = 1, IBUFSZ
///                    INTBUF(J) = 100000000*PASSNO  +  100 * I  +  J
///                 END DO
///
///                 CALL DASADI ( HANDLE, IBUFSZ, INTBUF )
///
///              END DO
///
///     C
///     C        Write buffered data to the file.
///     C
///              WRITE (*,*) 'Writing buffered data...'
///              CALL DASWBR ( HANDLE )
///
///     C
///     C        Close the file without segregating it.
///     C
///              WRITE (*,*) 'Closing DAS file...'
///              CALL DASLLC ( HANDLE )
///
///           END DO
///
///           WRITE (*,*) 'File write is done.'
///
///     C
///     C     Check file contents.
///     C
///           CALL DASOPR( FNAME, HANDLE )
///
///     C
///     C     Read data from the file; compare to expected values.
///     C
///     C     Initialize end addresses.
///     C
///           LASTC = 0
///           LASTD = 0
///           LASTI = 0
///
///           DO PASSNO = 1, NPASS
///
///              DO I = 1, NCALL
///
///     C
///     C           Check string data.
///     C
///                 XCHRBF = 'Character value #'
///                 CALL REPMI( XCHRBF, '#', I, XCHRBF )
///
///                 FIRSTC = LASTC + 1
///                 LASTC  = LASTC + CHRLEN
///
///                 CALL DASRDC ( HANDLE, FIRSTC, LASTC,
///          .                    1,      CHRLEN, CHRBUF )
///
///                 IF ( CHRBUF .NE. XCHRBF ) THEN
///                    WRITE (*,*) 'Character data mismatch: '
///                    WRITE (*,*) 'PASS     = ', PASSNO
///                    WRITE (*,*) 'I        = ', I
///                    WRITE (*,*) 'Expected = ', XCHRBF
///                    WRITE (*,*) 'Actual   = ', CHRBUF
///                    STOP
///                 END IF
///
///     C
///     C           Check double precision data.
///     C
///                 DO J = 1, DBUFSZ
///                    XDPBUF(J) = DBLE(   100000000*PASSNO
///          .                           + 100*I + J        )
///                 END DO
///
///                 FIRSTD = LASTD + 1
///                 LASTD  = LASTD + DBUFSZ
///
///                 CALL DASRDD ( HANDLE, FIRSTD, LASTD, DPBUF )
///
///                 DO J = 1, DBUFSZ
///
///                    IF ( DPBUF(J) .NE. XDPBUF(J) ) THEN
///
///                       WRITE (*,*)
///          .                   'Double precision data mismatch: '
///                       WRITE (*,*) 'PASS     = ', PASSNO
///                       WRITE (*,*) 'I        = ', I
///                       WRITE (*,*) 'J        = ', J
///                       WRITE (*,*) 'Expected = ', XDPBUF(J)
///                       WRITE (*,*) 'Actual   = ', DPBUF(J)
///                       STOP
///
///                    END IF
///
///                 END DO
///
///     C
///     C           Check integer data.
///     C
///                 DO J = 1, IBUFSZ
///                    XINTBF(J) = 100000000*PASSNO  +  100 * I  +  J
///                 END DO
///
///                 FIRSTI = LASTI + 1
///                 LASTI  = LASTI + IBUFSZ
///
///                 CALL DASRDI ( HANDLE, FIRSTI, LASTI, INTBUF )
///
///                 DO J = 1, IBUFSZ
///
///                    IF ( INTBUF(J) .NE. XINTBF(J) ) THEN
///
///                       WRITE (*,*) 'Integer data mismatch: '
///                       WRITE (*,*) 'PASS     = ', PASSNO
///                       WRITE (*,*) 'I        = ', I
///                       WRITE (*,*) 'J        = ', J
///                       WRITE (*,*) 'Expected = ', XINTBF(J)
///                       WRITE (*,*) 'Actual   = ', INTBUF(J)
///                       STOP
///
///                    END IF
///
///                 END DO
///
///              END DO
///
///           END DO
///
///           WRITE (*,*) 'File check is done.'
///
///     C
///     C     Close the file.
///     C
///           CALL DASCLS ( HANDLE )
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Writing buffered data...
///      Closing DAS file...
///      Opening file for write access...
///      Writing buffered data...
///      Closing DAS file...
///      Opening file for write access...
///      Writing buffered data...
///      Closing DAS file...
///      File write is done.
///      File check is done.
///
///
///     Note that after run completion, a new DAS file exists in the
///     output directory.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.0.1, 19-MAY-2021 (NJB) (JDR)
///
///         Edited the header to comply with NAIF standard. Added complete
///         code example.
///
///         Updated $Particulars in order to provide information about the
///         high level APIs that actually use this routine.
///
/// -    SPICELIB Version 2.0.0, 30-JUL-2014 (NJB)
///
///         Upgraded to support handle manager integration.
///
/// -    SPICELIB Version 1.0.2, 03-NOV-1995 (NJB)
///
///         Removed weird spaces from ENTRY statement.
///
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 1.0.1, 28-OCT-1993 (KRG)
///
///         Removed references to specific DAS file open routines in the
///         $Detailed_Input section of the header. This was done in order
///         to minimize documentation changes if the DAS open routines ever
///         change.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn daswbr(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    DASWBR(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASWBR ( DAS, write buffered records )
pub fn DASWBR(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"DASWBR", ctx)?;

    //
    // Check that the file is open for writing.  Signal an error if not.
    //
    DASSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASWBR", ctx)?;
        return Ok(());
    }

    //
    // If it hasn't been done yet, initialize the pointer list pools.
    //
    if save.PASS1 {
        LNKINI(BUFSZD, save.POOLD.as_slice_mut(), ctx)?;
        LNKINI(BUFSZI, save.POOLI.as_slice_mut(), ctx)?;
        LNKINI(BUFSZC, save.POOLC.as_slice_mut(), ctx)?;

        save.PASS1 = false;
    }

    //
    // Obtain a logical unit for HANDLE.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut save.WRUNIT, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DASWBR", ctx)?;
        return Ok(());
    }

    //
    // For each buffer, find the records belonging to this file, and
    // write them out to the file.
    //
    // Double precision records first.
    //
    save.NODE = save.HEADD;

    while (save.NODE > 0) {
        if (HANDLE == save.HNBUFD[save.NODE]) {
            //
            // This record belongs to the file of interest, so write the
            // the record out.
            //
            DASIOD(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFD[save.NODE],
                save.RCBUFD.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASWBR", ctx)?;
                return Ok(());
            }

            //
            // The record is no longer in use; return it to the
            // free list.  But grab the successor first.  Update
            // the head of the list, if the node we're freeing is
            // the head node.  Decrement the number of used d.p.
            // buffer elements.
            //
            save.NEXT = save.POOLD[[FORWRD, save.NODE]];

            if (save.NODE == save.HEADD) {
                save.HEADD = save.NEXT;
            }

            LNKFSL(save.NODE, save.NODE, save.POOLD.as_slice_mut(), ctx)?;

            save.NODE = save.NEXT;
            save.USEDD = (save.USEDD - 1);
        } else {
            //
            // Just get the next node.
            //
            save.NODE = save.POOLD[[FORWRD, save.NODE]];
        }
    }

    //
    // Next, integer records.
    //
    save.NODE = save.HEADI;

    while (save.NODE > 0) {
        if (HANDLE == save.HNBUFI[save.NODE]) {
            //
            // This record belongs to the file of interest, so write the
            // the record out.
            //
            DASIOI(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFI[save.NODE],
                save.RCBUFI.subarray_mut([1, save.NODE]),
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASWBR", ctx)?;
                return Ok(());
            }

            //
            // The record is no longer in use; return it to the
            // free list.  But grab the successor first.  Update
            // the head of the list, if the node we're freeing is
            // the head node.  Decrement the number of used integer
            // buffer elements.
            //
            save.NEXT = save.POOLI[[FORWRD, save.NODE]];

            if (save.NODE == save.HEADI) {
                save.HEADI = save.NEXT;
            }

            LNKFSL(save.NODE, save.NODE, save.POOLI.as_slice_mut(), ctx)?;

            save.NODE = save.NEXT;
            save.USEDI = (save.USEDI - 1);
        } else {
            //
            // Just get the next node.
            //
            save.NODE = save.POOLI[[FORWRD, save.NODE]];
        }
    }

    //
    // And last, character records.
    //
    save.NODE = save.HEADC;

    while (save.NODE > 0) {
        if (HANDLE == save.HNBUFC[save.NODE]) {
            //
            // This record belongs to the file of interest, so write the
            // the record out.
            //
            DASIOC(
                b"WRITE",
                save.WRUNIT,
                save.RNBUFC[save.NODE],
                &mut save.RCBUFC[save.NODE],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"DASWBR", ctx)?;
                return Ok(());
            }

            //
            // The record is no longer in use; return it to the
            // free list.  But grab the successor first.  Update
            // the head of the list, if the node we're freeing is
            // the head node.  Decrement the number of used character
            // buffer elements.
            //
            save.NEXT = save.POOLC[[FORWRD, save.NODE]];

            if (save.NODE == save.HEADC) {
                save.HEADC = save.NEXT;
            }

            LNKFSL(save.NODE, save.NODE, save.POOLC.as_slice_mut(), ctx)?;

            save.NODE = save.NEXT;
            save.USEDC = (save.USEDC - 1);
        } else {
            //
            // Just get the next node.
            //
            save.NODE = save.POOLC[[FORWRD, save.NODE]];
        }
    }

    CHKOUT(b"DASWBR", ctx)?;
    Ok(())
}
