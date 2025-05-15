//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const FPRINT: i32 = 32;
const LPRINT: i32 = 126;
const DIRSIZ: i32 = 100;
const IFNLEN: i32 = 60;
const FTSIZE: i32 = 20;
const IMPLE: i32 = 0;
const IMPCLS: i32 = 1;
const EXPLT: i32 = 2;
const EXPLE: i32 = 3;
const EXPCLS: i32 = 4;
const MNIDXT: i32 = 0;
const MXIDXT: i32 = 4;
const CONBAS: i32 = 1;
const NCON: i32 = (CONBAS + 1);
const RDRBAS: i32 = (NCON + 1);
const NRDR: i32 = (RDRBAS + 1);
const RDRTYP: i32 = (NRDR + 1);
const REFBAS: i32 = (RDRTYP + 1);
const NREF: i32 = (REFBAS + 1);
const PDRBAS: i32 = (NREF + 1);
const NPDR: i32 = (PDRBAS + 1);
const PDRTYP: i32 = (NPDR + 1);
const PKTBAS: i32 = (PDRTYP + 1);
const NPKT: i32 = (PKTBAS + 1);
const RSVBAS: i32 = (NPKT + 1);
const NRSV: i32 = (RSVBAS + 1);
const PKTSZ: i32 = (NRSV + 1);
const PKTOFF: i32 = (PKTSZ + 1);
const NMETA: i32 = (PKTOFF + 1);
const MXMETA: i32 = NMETA;
const MNMETA: i32 = 15;

struct SaveVars {
    INDEX: i32,
    LSTHAN: i32,
    NUMFXD: i32,
    NUMVAR: i32,
    EXPLCT: bool,
    FXDSEG: bool,
    FTREFS: StackArray2D<f64, 40>,
    FTBADR: StackArray<i32, 20>,
    FTHAN: StackArray<i32, 20>,
    FTITYP: StackArray<i32, 20>,
    FTMXSZ: StackArray<i32, 20>,
    FTNCON: StackArray<i32, 20>,
    FTNPKT: StackArray<i32, 20>,
    FTNREF: StackArray<i32, 20>,
    FTNRES: StackArray<i32, 20>,
    FTOFF: StackArray<i32, 20>,
    FTPKSZ: StackArray<i32, 20>,
    NFT: i32,
    FTFIXD: StackArray<bool, 20>,
    FTEXPL: StackArray<bool, 20>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut INDEX: i32 = 0;
        let mut LSTHAN: i32 = 0;
        let mut NUMFXD: i32 = 0;
        let mut NUMVAR: i32 = 0;
        let mut EXPLCT: bool = false;
        let mut FXDSEG: bool = false;
        let mut FTREFS = StackArray2D::<f64, 40>::new(1..=2, 1..=FTSIZE);
        let mut FTBADR = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTHAN = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTITYP = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTMXSZ = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTNCON = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTNPKT = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTNREF = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTNRES = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTOFF = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut FTPKSZ = StackArray::<i32, 20>::new(1..=FTSIZE);
        let mut NFT: i32 = 0;
        let mut FTFIXD = StackArray::<bool, 20>::new(1..=FTSIZE);
        let mut FTEXPL = StackArray::<bool, 20>::new(1..=FTSIZE);

        NUMFXD = 0;
        NUMVAR = 0;
        NFT = 0;

        Self {
            INDEX,
            LSTHAN,
            NUMFXD,
            NUMVAR,
            EXPLCT,
            FXDSEG,
            FTREFS,
            FTBADR,
            FTHAN,
            FTITYP,
            FTMXSZ,
            FTNCON,
            FTNPKT,
            FTNREF,
            FTNRES,
            FTOFF,
            FTPKSZ,
            NFT,
            FTFIXD,
            FTEXPL,
        }
    }
}

/// Generic segments: Sequential writer.
///
/// This is the umbrella routine for managing the sequential writing
/// of generic segments to DAF files. It should never be called
/// directly, it provides the mechanism whereby data are shared by
/// its entry points.
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
///  HANDLE    I    Handle of a DAF file opened with write access.
///  DESCR     I    Descriptor for a generic segment.
///  SEGID     I    Identifier for a generic segment.
///  NCONST    I    Number of constant values in a generic segment.
///  CONST     I    Array of constant values for a generic segment.
///  NPKTS     I    Number of data packets to write to a segment.
///  PKTSIZ    I    Size of fixed size packets or sizes of variable
///                 size packets.
///  PKTDAT    I    Array of packet data.
///  NREFS     I    Number of reference values.
///  REFDAT    I    Reference data.
///  IDXTYP    I    Index type for the reference values.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened with write access.
///           This is the handle of the file in which a generic segment
///           will be started, or the handle of a file in which a
///           generic segment is currently being written.
///
///  DESCR    is the descriptor for the generic segment that is being
///           written. This is the packed form of the DAF double
///           precision and integer summaries which contains ND double
///           precision numbers and NI integers, respectively.
///
///  SEGID    is an identifier for the generic segment that is being
///           written. This is a character string containing at most
///           NC printing ASCII characters where
///
///                             /  ND + ( NI + 1 )  \
///                  NC =  8 *  | ----------------- |
///                             \         2         /
///
///            SEGID may be blank.
///
///  NCONST   is the number of constant values to be placed in the
///           generic segment.
///
///  CONST    is an array of NCONST constant values for the generic
///           segment.
///
///  NPKTS    is the number of data packets to write to a generic
///           segment.
///
///  PKTSIZ   is the size of fixed size packets or sizes of variable
///           size packets.
///
///           The size of a packet is the number of double precision
///           numbers it contains.
///
///           When writing a segment with fixed size packets, only
///           the first element of the array, PKTSIZ(1), is used, and
///           it should contain the size of the fixed size packets. In
///           this instance, the calling program need not declare this
///           variable as an array of one integer; it may be declared
///           as an integer variable.
///
///           When writing a segment with variable size packets,
///           there must be an element in the array PKTSIZ for each of
///           the data packets.
///
///  PKTDAT   is a singly dimensioned array containing the double
///           precision data for the fixed or variable size data
///           packets to be added to the generic segment associated
///           with HANDLE.
///
///           For fixed size data packets, PKTDAT will have the
///           following structure:
///
///              Packet #  Range of locations for the packet data.
///              --------  ------------------------------------------
///
///                 1      PKTDAT(1)              to PKTDAT(PS)
///                 2      PKTDAT(PS+1)           to PKTDAT(2*PS)
///                 3      PKTDAT(2*PS+1)         to PKTDAT(3*PS)
///                 4      PKTDAT(3*PS+1)         to PKTDAT(4*PS)
///
///                                         .
///                                         .
///                                         .
///
///                NPKTS   PKTDAT((NPKTS-1)*PS+1) to PKTDAT(NPKTS*PS)
///
///           where PS = PKTSIZ(1).
///
///           For variable size data packets, PKTDAT will have the
///           following structure:
///
///              Packet #  Range of locations for the packet data.
///              --------  ------------------------------------------
///
///                 1      PKTDAT(1)           to PKTDAT(P(1))
///                 2      PKTDAT(P(1)+1)      to PKTDAT(P(2))
///                 3      PKTDAT(P(2)+1)      to PKTDAT(P(3))
///                 4      PKTDAT(P(3)+1)      to PKTDAT(P(4))
///
///                                         .
///                                         .
///                                         .
///
///                NPKTS   PKTDAT(P(NPKTS-1)+1) to PKTDAT(P(NPKTS))
///
///                            I
///                           ---
///           where P(I) =    >   PKTSIZ(K).
///                           ---
///                          K = 1
///
///  NREFS    is the number of reference values.
///
///           For implicitly indexed packets, NREFS must have a value
///           of two (2).
///
///           When writing packets to a generic segment which uses an
///           implicit index type, the value specified by NREFS is
///           used only on the first call to SGWFPK or SGWVPK. On all
///           subsequent calls to these subroutines for a particular
///           implicitly indexed generic segment, the value of NREFS
///           is ignored.
///
///           For explicitly indexed packets, NREFS must be equal to
///           NPKTS; there should be a reference value for each data
///           packet being written to the generic segment.
///
///           When writing packets to a segment which uses an explicit
///           index type, the value specified by NREFS is used on
///           every call to SGWFPK or SGWVPK and it must always be
///           equal to NPKTS.
///
///  REFDAT   is the reference data values.
///
///           For implicitly indexed packets, there must be two (2)
///           values. The values represent a starting value, which
///           will have an index of 1, and a step size between
///           reference values, which are used to compute an index and
///           a reference value associated with a specified key value.
///
///           In order to avoid, or at least minimize, numerical
///           difficulties associated with computing index values for
///           generic segments with implicit index types, the value of
///           the step size must be an integer, i.e., DINT(REFDAT(2))
///           must equal REFDAT(2). In this case, we also recommend
///           that REFDAT(1) be an integer, although this is not
///           enforced.
///
///           When writing packets to a generic segment which uses an
///           implicit index type, the values specified by REFDAT are
///           used only on the first call to SGWFPK or SGWVPK. On all
///           subsequent calls to these subroutines for a particular
///           implicitly indexed generic segment REFDAT is ignored.
///
///           For explicitly indexed packets, there must be NPKTS
///           reference values and the values must be in increasing
///           order:
///
///              REFDAT(I) < REFDAT(I+1), I = 1, NPKTS-1
///
///           When writing packets to a segment which uses an explicit
///           index type, the values specified by REFDAT are used on
///           every call to SGWFPK or SGWVPK. On all calls to these
///           subroutines after the first, the value of REFDAT(1) must
///           be strictly greater than than the value of REFDAT(NPKTS)
///           from the previous call. This preserves the ordering of
///           the reference values for the entire segment.
///
///  IDXTYP   is the index type to use for the reference values.
///
///           Two forms of indexing are provided:
///
///              1) An implicit form of indexing based on using two
///                 values, a starting value, which will have an index
///                 of 1, and a step size between reference values,
///                 which are used to compute an index and a reference
///                 value associated with a specified key value. See
///                 the descriptions of the implicit types below for
///                 the particular formula used in each case.
///
///              2) An explicit form of indexing based on a reference
///                 value for each data packet.
///
///           See the chapter on Generic segments in the DAF required
///           or the include file 'sgparam.inc' for more details
///           about the index types that are available.
/// ```
///
/// # Detailed Output
///
/// ```text
///  None.
///
///  The data passed to the various entry points of this subroutine are
///  used to construct a generic segment in one or more DAF files, with
///  the current file specified by the input argument HANDLE.
/// ```
///
/// # Parameters
///
/// ```text
///  The entry points in this subroutine make use of parameters defined
///  in the file 'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this subroutine is called directly rather than through one
///      of its entry points, the error SPICE(BOGUSENTRY) is signaled.
///
///  2)  See the individual entry points for descriptions of their
///      exceptions.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section above.
/// ```
///
/// # Particulars
///
/// ```text
///  This is the umbrella routine for managing the sequential writing
///  of generic segments to DAF files. It should never be called
///  directly, but provides the mechanism whereby data are shared by
///  its entry points. The entry points included in this subroutine
///  are:
///
///  SGBWFS ( HANDLE, DESCR, SEGID, NCONST, CONST, PKTSIZ, IDXTYP )
///     Begin writing a generic segment with fixed size packets.
///
///  SGBWVS ( HANDLE, DESCR, SEGID, NCONST, CONST, IDXTYP )
///     Begin writing a generic segment with variable size packets.
///
///  SGWFPK ( HANDLE, NPKTS, PKTDAT, NREFS, REFDAT )
///     Write fixed size packets to a generic segment started by
///     calling SGBWFS.
///
///  SGWVPK ( HANDLE, NPKTS, PKTSIZ, PKTDAT, NREFS, REFDAT )
///     Write variable size packets to a generic segment started by
///     calling SGBWVS.
///
///  SGWES ( HANDLE )
///     End a generic segment.
///
///  A DAF generic segment contains several logical data partitions:
///
///     1) A partition for constant values to be associated with each
///        data packet in the segment.
///
///     2) A partition for the data packets.
///
///     3) A partition for reference values.
///
///     4) A partition for a packet directory, if the segment contains
///        variable sized packets.
///
///     5) A partition for a reference value directory.
///
///     6) A reserved partition that is not currently used. This
///        partition is only for the use of the NAIF group at the Jet
///        Propulsion Laboratory (JPL).
///
///     7) A partition for the meta data which describes the locations
///        and sizes of other partitions as well as providing some
///        additional descriptive information about the generic
///        segment.
///
///              +============================+
///              |         Constants          |
///              +============================+
///              |          Packet 1          |
///              |----------------------------|
///              |          Packet 2          |
///              |----------------------------|
///              |              .             |
///              |              .             |
///              |              .             |
///              |----------------------------|
///              |          Packet N          |
///              +============================+
///              |      Reference Values      |
///              +============================+
///              |      Packet Directory      |
///              +============================+
///              |    Reference  Directory    |
///              +============================+
///              |       Reserved  Area       |
///              +============================+
///              |     Segment Meta Data      |
///              +----------------------------+
///
///  Only the placement of the meta data at the end of a generic
///  segment is required. The other data partitions may occur in any
///  order in the generic segment because the meta data will contain
///  pointers to their appropriate locations within the generic
///  segment.
///
///  The meta data for a generic segment should only be obtained
///  through use of the subroutine SGMETA. The meta data should not be
///  written through any mechanism other than the ending of a generic
///  segment begun by SGBWFS or SGBWVS using SGWES.
///
///  The entry points of this subroutine when used together provide the
///  following capabilities:
///
///     1) The ability to write a generic segment with fixed size data
///        packets to a DAF.
///
///     2) the ability to write a generic segment with variable size
///        data packets to a DAF.
///
///     3) The ability to write generic segments to multiple files.
///        Only a single generic segment may be written to a particular
///        file at any time, but several files may each have a generic
///        segment being written to them at the same time.
///
///  Packets may be placed into a generic segment one at a time or N at
///  at time, depending upon the whim of the programmer, limitations
///  of the computing equipment (memory), or requirements placed upon
///  the software that will write a generic segment.
///
///  Packets are retrieved from a generic segment by an index which may
///  be obtained by using the subroutine SGFRVI (generic segments fetch
///  reference value and index).
/// ```
///
/// # Examples
///
/// ```text
///  In examples 1 and 3, we make use of the fictitious subroutines
///
///     GET_FIX_PKT ( PACKET, REF, DONE )
///
///  and
///
///     GET_VAR_PKT ( PACKET, SIZE, REF, DONE )
///
///  where
///
///     DONE   is a logical flag indicating whether there is more data
///            available. DONE = .TRUE. implies there is no more data.
///            DONE = .FALSE. implies there is more data available.
///
///     PACKET is a double precision array of an appropriate size to
///            hold all of the data returned.
///
///     REF    is a double precision reference value that will be used
///            to create an index for the data packets in the segment.
///            The values of this variable are always increasing, e.g.,
///            the value of REF on the second call to GET_FIX_PKT or
///            GET_VAR_PKT will be greater than the value on the first
///            call to the subroutine.
///
///     SIZE   is an integer for the size of the variable size data
///            packet that is returned.
///
///  These subroutines return a fixed size data packet and a variable
///  size data packet, respectively. We make use of these fictitious
///  subroutines in the examples to avoid adding unnecessary or
///  distracting complications.
///
///  You may think of these subroutines as methods for acquiring data
///  from a "black-box" process. In the first case, the data is always
///  returned in fixed size blocks from a black-box that fills a local
///  buffer with data and always returned the entire buffer when data
///  is requested, e.g., an instrument that measures the concentrations
///  of carbon dioxide, sulfur dioxide, ozone, and other constituents
///  of the air. In the second case, the data is returned in variably
///  sized blocks from a black-box, e.g., an algorithm which integrates
///  a function using polynomials of varying degree; different numbers
///  of coefficients are required for polynomials of differing degrees.
///
///  In examples 2 and 4, we make use of the fictitious subroutines
///
///     GET_FIX_PKTS ( NPKTS, PKTS, REFS, DONE )
///
///  and
///
///     GET_VAR_PKTS ( NPKTS, PKTS, SIZES, REFS, DONE )
///
///  where
///
///     DONE   is a logical flag indicating whether there is more data
///            available. DONE = .TRUE. implies there is no more data.
///            DONE = .FALSE. implies there is more data available;
///
///     NPKTS  is the number of data packets returned in the array
///            PKTS.
///
///     PKTS   is a double precision array containing NPKTS data
///            packets, either fixed size or variable size, and is of
///            an appropriate size to hold all of the data returned.
///            See the description of PKTDAT above for the exact manner
///            in which fixed size packets and variable size packets
///            are stored in an array.
///
///     REFS   is a double precision array which contains NPKTS
///            reference values that will be used to create an index
///            for the data packets in the segment. The values of this
///            variable are always increasing, e.g., the first value of
///            REFS on the second call to GET_FIX_PKTS or GET_VAR_PKTS
///            will be greater than the last value of REFS on the first
///            call to the subroutine.
///
///     SIZES  is an array of integers containing the sizes of each of
///            the variable size data packets that is returned in PKTS.
///
///  These subroutines return arrays containing one or more fixed size
///  data packets and one or more variable size data packets,
///  respectively. We make use of these fictitious subroutines in the
///  examples to avoid adding unnecessary or distracting complications.
///
///  For each example, we provide a simple code fragment that
///  demonstrates the use of the entry points to create generic
///  segments. We assume that all of the relevant variables are defined
///  at the time that the entry points are invoked. These code
///  fragments are for illustrative purposes; they do not necessarily
///  conform to what would be considered good programming practice.
///
///  Example 1-A: Adding fixed size packets one at a time.
///
///     For this example, we make no assumptions about the reference
///     values returned by GET_VAR_PKT other than they are increasing.
///     Having no other information about the reference values, we must
///     use an explicit indexing method to store the packets.
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a fixed size segment. To do this, we
///     C     need:
///     C
///     C        HANDLE -- The handle of a DAF opened with write
///     C                  access.
///     C        DESCR  -- The packed descriptor for the segment that
///     C                  we want to create.
///     C        SEGID  -- A short character string that provides an
///     C                  identifier for the segment.
///     C        NCONST -- The number of constant values to be
///     C                  associated with all of the packets in the
///     C                  segment.
///     C        CONST  -- An array of constant values to be associated
///     C                  with all of the packets in a segment.
///     C        PKTSIZ -- The size of the packets that will be stored
///     C                  in this segment, i.e., the number of double
///     C                  precision numbers necessary to store a
///     C                  complete data packet.
///     C        EXPCLS -- The type of indexing scheme that we will use
///     C                  for searching the segment to obtain a data
///     C                  packet. In this case, we are going to use an
///     C                  explicit index, which requires a reference
///     C                  value for each data packet, and when
///     C                  searching for a data packet we will choose
///     C                  the packet with a reference value closest to
///     C                  the requested value. See the include file
///     C                  'sgparam.inc' for the value of EXPCLS.
///     C
///           CALL SGBWFS ( HANDLE, DESCR,  SEGID,  NCONST,
///          .              CONST,  PKTSIZ, EXPCLS          )
///     C
///     C     We loop until done, obtaining a fixed size packet
///     C     and writing it to the generic segment in the file.
///     C
///           DONE = .FALSE.
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get a fixed size packet and a reference value.
///     C
///              CALL GET_FIX_PKT ( PACKET, REF, DONE )
///     C
///     C        Write the packet to the segment, unless we're done.
///     C
///              IF ( .NOT. DONE ) THEN
///
///                 CALL SGWFPK ( HANDLE, 1, PACKET, 1, REF )
///
///              END IF
///
///           END DO
///     C
///     C     End the segment and move on to other things.
///     C
///           CALL SGWES ( HANDLE )
///                             .
///                             .
///                             .
///
///  Example 1-B: Adding fixed size packets with uniformly spaced
///               reference values.
///
///     In the previous example, we made no assumptions about the
///     reference values other than that they were increasing. We now
///     will assume that the reference values are also equally spaced
///     and that we have a priori values for a beginning reference
///     value, BEGIN_REF, and a step size, STEP_SIZE, that is the
///     difference between two consecutive reference values. We have
///
///        BEGIN_REF <= REF <= BEGIN_REF + (N-1) * STEP_SIZE
///
///     where BEGIN_REF equals the first reference value returned by
///     GET_FIX_PKT and BEGIN_REF + (N-1) * STEP_SIZE equals the last
///     reference value returned. Under these assumptions we can use an
///     implicit index for the data packets which will provide a more
///     space efficient method for putting the data packets into a
///     generic segment. We repeat the example under these assumptions
///     using an implicit indexing method. Nothing else has changed.
///
///     The index for a data packet in the implicitly indexed generic
///     segment we create is computed from the formula:
///
///                       /          VALUE - REFDAT(1)    \
///         INDEX = IDINT | 1.5 + ----------------------- |
///                       \              REFDAT(2)        /
///
///     where the index for the data packet associated with VALUE is
///     desired.
///
///     The reference value associated with this index is:
///
///         REF   =  REFDAT(1) + REFDAT*(INDEX - 1)
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a fixed size segment. To do this, we
///     C     need:
///     C
///     C        HANDLE -- The handle of a DAF opened with write
///     C                  access.
///     C        DESCR  -- The packed descriptor for the segment that
///     C                  we want to create.
///     C        SEGID  -- A short character string that provides an
///     C                  identifier for the segment.
///     C        NCONST -- The number of constant values to be
///     C                  associated with all of the packets in the
///     C                  segment.
///     C        CONST  -- An array of constant values to be associated
///     C                  with all of the packets in a segment.
///     C        PKTSIZ -- The size of the packets that will be stored
///     C                  in this segment, i.e., the number of double
///     C                  precision numbers necessary to store a
///     C                  complete data packet.
///     C        IMPCLS -- The type of indexing scheme that we will use
///     C                  for searching the segment to obtain a data
///     C                  packet. In this case, we are going to use
///     C                  an implicit index, which requires beginning
///     C                  and ending times which bound all reference
///     C                  values, and when searching for a data packet
///     C                  we will choose the packet whose index is
///     C                  computed by the formula above. See the
///     C                  include file 'sgparam.inc' for the value
///     C                  of IMPCLS
///     C
///           CALL SGBWFS ( HANDLE, DESCR,  SEGID,  NCONST,
///          .              CONST,  PKTSIZ, IMPCLS          )
///     C
///     C     Set the beginning and ending reference values for the
///     C     implicit indexing method.
///     C
///           REFS(1) = BEGIN_REF
///           REFS(2) = STEP_SIZE
///     C
///     C     Get the first data packet and put it in the generic
///     C     segment. At the same time, we write the bounds used for
///     C     the implicit indexing. We ignore the value of REF since
///     C     the reference values are equally spaced and we are using
///     C     an implicit indexing method. We do not check DONE here
///     C     because we assume that there is at least one data packet.
///     C
///           CALL GET_FIX_PKT ( PACKET, REF, DONE )
///
///           CALL SGWFPK ( HANDLE, 1, PACKET, 2, REFS )
///     C
///     C     We loop until done, obtaining a fixed size packet
///     C     and writing it to the generic segment in the file.
///     C
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get a fixed size packet and a reference value.
///     C
///              CALL GET_FIX_PKT ( PACKET, REF, DONE )
///     C
///     C        Write the packet to the segment, unless we're done.
///     C        Because this segment is implicitly indexed, the last
///     C        two calling arguments are only used in the first call
///     C        to SGWFPK above. they are ignored in all subsequent
///     C        calls, so we may pass "dummy" arguments.
///     C
///              IF ( .NOT. DONE ) THEN
///
///                 CALL SGWFPK ( HANDLE, 1, PACKET, DUM1, DUM2 )
///
///              END IF
///
///           END DO
///     C
///     C     End the segment and move on to other things.
///     C
///           CALL SGWES ( HANDLE )
///                             .
///                             .
///                             .
///
///  Example 2: Adding fixed size packets more efficiently.
///
///     It is possible to add more than one fixed size data packet to a
///     generic segment at one time. Doing this will usually prove to
///     be a more efficient way of adding the data packets, provided
///     there is sufficient storage to hold more than one data packet
///     available. This example demonstrates this capability.
///
///     For this example, we make no assumptions about the reference
///     values returned by GET_FIX_PKTS other than they are increasing.
///     Having no other information about the reference values, we must
///     use an explicit indexing method to store the packets.
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a fixed size segment. To do this, we
///     C     need:
///     C
///     C        HANDLE -- The handle of a DAF opened with write
///     C                  access.
///     C        DESCR  -- The packed descriptor for the segment that
///     C                  we want to create.
///     C        SEGID  -- A short character string that provides an
///     C                  identifier for the segment.
///     C        NCONST -- The number of constant values to be
///     C                  associated with all of the packets in the
///     C                  segment.
///     C        CONST  -- An array of constant values to be associated
///     C                  with all of the packets in a segment.
///     C        PKTSIZ -- The size of the packets that will be stored
///     C                  in this segment, i.e., the number of double
///     C                  precision numbers necessary to store a
///     C                  complete data packet.
///     C        EXPCLS -- The type of indexing scheme that we will use
///     C                  for searching the segment to obtain a data
///     C                  packet. In this case, we are going to use an
///     C                  explicit index, which requires a reference
///     C                  value for each data packet, and when
///     C                  searching for a data packet we will choose
///     C                  the packet with a reference value closest to
///     C                  the requested value. See the include file
///     C                  'sgparam.inc' for the value of EXPCLS
///     C
///           CALL SGBWFS ( HANDLE, DESCR,  SEGID,  NCONST,
///          .              CONST,  PKTSIZ, EXPCLS          )
///     C
///     C     We loop until done, obtaining a fixed size packet
///     C     and writing it to the generic segment in the file.
///     C
///           DONE = .FALSE.
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get a collection of fixed size packet and associated
///     C        array of increasing reference values.
///     C
///              CALL GET_FIX_PKTS ( NPKTS, PKTS, REFS, DONE )
///     C
///     C        Write the packets to the segment if we have any. Since
///     C        we are using an explicit index, the number of
///     C        reference values is the same as the number of data
///     C        packets.
///     C
///              IF ( .NOT. DONE ) THEN
///
///                 CALL SGWFPK ( HANDLE, NPKTS, PKTS, NPKTS, REFS )
///
///              END IF
///
///           END DO
///     C
///     C     End the segment and move on to other things.
///     C
///           CALL SGWES ( HANDLE )
///                             .
///                             .
///                             .
///
///     If we are using an implicit indexing method, multiple data
///     packets may be added with one call to SGWFPK as in the above
///     example for an explicit index, with the exception that there
///     are only two reference values, and they are specified on the
///     first call to SGWFPK, as in Example 1-B.
///
///  Example 3-A: Adding variable size packets one at a time.
///
///     For this example, we make no assumptions about the reference
///     values returned by GET_VAR_PKT other than they are increasing.
///     Having no other information about the reference values, we must
///     use an explicit indexing method to store the packets.
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a variable size segment. To do this, we
///     C     need:
///     C
///     C        HANDLE -- The handle of a DAF opened with write
///     C                  access.
///     C        DESCR  -- The packed descriptor for the segment that
///     C                  we want to create.
///     C        SEGID  -- A short character string that provides an
///     C                  identifier for the segment.
///     C        NCONST -- The number of constant values to be
///     C                  associated with all of the packets in the
///     C                  segment.
///     C        CONST  -- An array of constant values to be associated
///     C                  with all of the packets in a segment.
///     C        EXPCLS -- The type of indexing scheme that we will use
///     C                  for searching the segment to obtain a data
///     C                  packet. In this case, we are going to use an
///     C                  explicit index, which requires a reference
///     C                  value for each data packet, and when
///     C                  searching for a data packet we will choose
///     C                  the packet with a reference value closest to
///     C                  the requested value. See the include file
///     C                  'sgparam.inc' for the value of EXPCLS.
///     C
///           CALL SGBVFS ( HANDLE, DESCR, SEGID,
///          .              NCONST, CONST, EXPCLS )
///     C
///     C     We loop until done, obtaining a variable size packet
///     C     and writing it to the generic segment in the file.
///     C
///           DONE = .FALSE.
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get a variable size packet and a reference value.
///     C
///              CALL GET_VAR_PKT ( PACKET, SIZE, REF, DONE )
///     C
///     C        Write the packet to the segment, unless we're done.
///     C
///              IF ( .NOT. DONE ) THEN
///
///                 CALL SGWVPK ( HANDLE, 1, SIZE, PACKET, 1, REF )
///
///              END IF
///
///           END DO
///     C
///     C     End the segment and move on to other things.
///     C
///           CALL SGWES ( HANDLE )
///                             .
///                             .
///                             .
///
///  Example 3-B: Adding variable size packets one at a time with
///               uniformly spaced reference values.
///
///     In the previous example, we made no assumptions about the
///     reference values other than that they were increasing. We now
///     will assume that the reference values are also equally spaced
///     and that we have a priori values for a beginning reference
///     value, BEGIN_REF, and a step size, STEP_SIZE, that is the
///     difference between two consecutive reference values. We have
///
///        BEGIN_REF <= REF <= BEGIN_REF + (N-1) * STEP_SIZE
///
///     where BEGIN_REF equals the first reference value returned by
///     GET_VAR_PKT and BEGIN_REF + (N-1) * STEP_SIZE equals the last
///     reference value returned. Putting all of this together means
///     that we can use an implicit index for the data packets which
///     will provide a more space efficient method for putting the data
///     packets into a generic segment. We repeat the example under
///     these assumptions using an implicit indexing method. Nothing
///     else has changed.
///
///     The index for a data packet in the implicitly indexed generic
///     segment we create is computed from the formula:
///
///                       /          VALUE - REFDAT(1)    \
///         INDEX = IDINT | 1.5 + ----------------------- |
///                       \              REFDAT(2)        /
///
///     where the index for the data packet associated with VALUE is
///     desired.
///
///     The reference value associated with this index is:
///
///         REF   =  REFDAT(1) + REFDAT*(INDEX - 1)
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a variable size segment. To do this, we
///     C     need:
///     C
///     C        HANDLE -- The handle of a DAF opened with write
///     C                  access.
///     C        DESCR  -- The packed descriptor for the segment that
///     C                  we want to create.
///     C        SEGID  -- A short character string that provides an
///     C                  identifier for the segment.
///     C        NCONST -- The number of constant values to be
///     C                  associated with all of the packets in the
///     C                  segment.
///     C        CONST  -- An array of constant values to be associated
///     C                  with all of the packets in a segment.
///     C        IMPCLS -- The type of indexing scheme that we will use
///     C                  for searching the segment to obtain a data
///     C                  packet. In this case, we are going to use
///     C                  an implicit index, which requires beginning
///     C                  and ending times which bound all reference
///     C                  values, and when searching for a data packet
///     C                  we will choose the packet whose index is
///     C                  computed by the formula above. See the
///     C                  include file 'sgparam.inc' for the value of
///     C                  IMPCLS.
///     C
///           CALL SGBWVS ( HANDLE, DESCR,  SEGID,  NCONST,
///          .              CONST,  IMPCLS                   )
///     C
///     C     Set the beginning and ending reference values for the
///     C     implicit indexing method.
///     C
///           REFS(1) = BEGIN_REF
///           REFS(2) = STEP_SIZE
///     C
///     C     Get the first data packet and put it in the generic
///     C     segment. At the same time, we write the bounds used for
///     C     the implicit indexing. We ignore the value of REF since
///     C     the reference values are equally spaced and we are using
///     C     an implicit indexing method. We do not check DONE here
///     C     because we assume that there is at least one data packet.
///     C
///           CALL GET_VAR_PKT ( PACKET, SIZE, REF, DONE )
///
///           CALL SGWVPK ( HANDLE, 1, SIZE, PACKET, 2, REFS )
///     C
///     C     We loop until done, obtaining a fixed size packet
///     C     and writing it to the generic segment in the file.
///     C
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get a variable size packet and a unique reference
///     C        value.
///     C
///              CALL GET_VAR_PKT ( PACKET, SIZE, REF, DONE )
///     C
///     C        Write the packet to the segment, unless we're done.
///     C        Because this segment is implicitly indexed, the last
///     C        two calling arguments are only used in the first call
///     C        to SGWFPK above. they are ignored in all subsequent
///     C        calls, so we may pass "dummy" arguments.
///     C
///              IF ( .NOT. DONE ) THEN
///
///                 CALL SGVFPK ( HANDLE, 1, SIZE, PACKET, DUM1, DUM2 )
///
///              END IF
///
///           END DO
///     C
///     C     End the segment and move on to other things.
///     C
///           CALL SGWES ( HANDLE )
///                             .
///                             .
///                             .
///
///  Example 4: Adding variable size packets more efficiently.
///
///     It is possible to add more than one variable size data packet
///     to a generic segment at one time. Doing this will usually prove
///     to be a more efficient way of adding the data packets, provided
///     there is sufficient storage to hold more than one data packet
///     available. This example demonstrates this capability.
///
///     For this example, we make no assumptions about the reference
///     values returned by GET_VAR_PKTS other than they are increasing.
///     Having no other information about the reference values, we must
///     use an explicit indexing method to store the packets.
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a variable size segment. To do this, we
///     C     need:
///     C
///     C        HANDLE -- The handle of a DAF opened with write
///     C                  access.
///     C        DESCR  -- The packed descriptor for the segment that
///     C                  we want to create.
///     C        SEGID  -- A short character string that provides an
///     C                  identifier for the segment.
///     C        NCONST -- The number of constant values to be
///     C                  associated with all of the packets in the
///     C                  segment.
///     C        CONST  -- An array of constant values to be associated
///     C                  with all of the packets in a segment.
///     C        EXPCLS -- The type of indexing scheme that we will use
///     C                  for searching the segment to obtain a data
///     C                  packet. In this case, we are going to use an
///     C                  explicit index, which requires a reference
///     C                  value for each data packet, and when
///     C                  searching for a data packet we will choose
///     C                  the packet with a reference value closest to
///     C                  the requested value. See the include file
///     C                  sgparam.inc for the value of EXPCLS.
///     C
///           CALL SGBWVS ( HANDLE, DESCR,  SEGID,
///     C    .              NCONST, CONST, EXPCLS  )
///     C
///     C     We loop until done, obtaining a fixed size packet
///     C     and writing it to the generic segment in the file.
///     C
///           DONE = .FALSE.
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get a collection of variable size packets and an
///     C        array of increasing reference values.
///     C
///              GET_VAR_PKTS ( NPKTS, PKTS, SIZES, REFS, DONE )
///     C
///     C        Write the packets to the segment if we have any. Since
///     C        we are using an explicit index, the number of
///     C        reference values is the same as the number of data
///     C        packets.
///     C
///              IF ( NPKTS .GT. 0 ) THEN
///
///                 CALL SGWVPK ( HANDLE, NPKTS, SIZES,
///          .                    PKTS,   NPKTS, REFS   )
///
///              END IF
///
///           END DO
///     C
///     C     End the segment and move on to other things.
///     C
///           CALL SGWES ( HANDLE )
///                             .
///                             .
///                             .
///
///     If we are using an implicit indexing method, multiple data
///     packets may be added with one call to SGWVPK as in the above
///     example for an explicit index, with the exception that there
///     are only two reference values, and they are specified on the
///     first call to SGWVPK, as in Example 3-B.
///
///  Example 5: Adding packets to multiple files.
///
///     It is possible to write multiple generic segments to different
///     DAFs at the same time. Only one generic segment may be written
///     to a particular DAF at any given time, however.
///
///     For this example we assume that we have previously opened four
///     DAF files, having the handles HANDL1, HANDL2, HANDL3, HANDL4.
///     We will be writing fixed size data packets to the DAFs
///     associated with handles HANDL2 and HANDL3, with packet sizes of
///     21 and 53, respectively. We will be writing variable size data
///     packets to the DAFs associated with handles HANDL1 and HANDL4.
///     We will be writing individual data packets to the files
///     associated with handles HANDL2 and HANDL4, and one or more data
///     packets to the files associated with handles HANDL1 and HANDL3.
///     On each trip through the loop in the example below, we will add
///     data to any of the segments whose status flags are not set. We
///     are done with the loop below when we have finished each of the
///     segments, as indicated by its status flag.
///
///     For this example, we make no assumptions about the reference
///     values returned by the GET_*_* subroutines other than they are
///     increasing. Having no other information about the reference
///     values, we must use an explicit indexing method to store the
///     packets.
///
///                             .
///                             .
///                             .
///     C
///     C     First we begin a generic segment of the appropriate type
///     C     in each of the files. segment. To do this, we need:
///     C
///     C        HANDL1, HANDL2, HANDL3, HANDL4 --
///     C
///     C           The handles of a DAFs opened with write access to
///     C           which we wish to add a new generic segment.
///     C
///     C        DESCR1, DESCR2, DESCR3, DESCR4  --
///     C
///     C           The packed descriptors for the segments that
///     C           we want to create.
///     C
///     C        SEGID1, SEGID2, SEGID3, SEGID4 --
///     C
///     C           A short character string that provides an
///     C           identifier for each of the segments we will be
///     C           creating.
///     C
///     C        NCON1, NCON2, NCON3, NCON4 --
///     C
///     C           The number of constant values to be associated with
///     C           all of the packets in each the segments we will be
///     C           creating.
///     C
///     C
///     C        CONST1, CONST2, CONST3, CONST4 --
///     C
///     C           An array of constant values to be associated with
///     C           all of the packets in each of the segments that we
///     C           are creating.
///     C
///     C        IDXT1, IDXT2, IDXT3, IDXT4 --
///     C
///     C          The type of indexing scheme that we will use for
///     C          searching each of the segments to obtain a data
///     C          packet. In this example, each of the generic
///     C          segments will use an explicit index, which requires
///     C          a reference value for each data packet. When
///     C          searching for a data packet we will choose the
///     C          packet with a reference value closest to the
///     C          requested value.
///     C
///     C            IDXT1 = EXPCLS
///     C            IDXT2 = EXPCLS
///     C            IDXT3 = EXPCLS
///     C            IDXT4 = EXPCLS
///     C
///           CALL SGBWVS ( HANDL1, DESCR1, SEGID1,
///          .              NCON1,  CONST1, IDXT1   )
///           CALL SGBWFS ( HANDL2, DESCR2, SEGID2, 21,
///          .              NCON2,  CONST2, IDXT2   )
///           CALL SGBWFS ( HANDL3, DESCR3, SEGID3, 53,
///          .              NCON3,  CONST3, IDXT3   )
///           CALL SGBWVS ( HANDL4, DESCR4, SEGID4,
///          .              NCON4,  CONST4, IDXT4   )
///     C
///     C     We loop until done, obtaining data packets and writing
///     C     them to the generic segments in the appropriate DAFs.
///     C
///     C     We keep track of a status flag, DONE1, DONE2, DONE3,
///     C     DONE4, for each of the segments we are writing. When we
///     C     have finished writing all of the segments, we exit the
///     C     loop.
///     C
///           DONE  = .FALSE.
///           DONE1 = .FALSE.
///           DONE2 = .FALSE.
///           DONE3 = .FALSE.
///           DONE4 = .FALSE.
///
///           DO WHILE ( .NOT. DONE )
///     C
///     C        Get data packets and reference values for HANDL1 and
///     C        write them to the generic segment in that file.
///     C
///              IF ( .NOT. DONE1 ) THEN
///                 GET_VAR_PKTS ( NPKTS, PKTS, SIZES, REFS, DONE1 )
///
///                 IF ( NPKTS .GT. 0 ) THEN
///                    CALL SGWVPK ( HANDL1, NPKTS, SIZES,
///          .                       PKTS,   NPKTS, REFS   )
///                 END IF
///              END IF
///     C
///     C        Get a data packet and reference value for HANDL2 and
///     C        write it to the generic segment in that file.
///     C
///              IF ( .NOT. DONE2 ) THEN
///                 CALL GET_FIX_PKT ( PACKET, REF, DONE2 )
///
///                 IF ( .NOT. DONE2 ) THEN
///                    CALL SGWFPK ( HANDL2, 1, PACKET, 1, REF )
///                 END IF
///              END IF
///     C
///     C        Get data packets and reference values for HANDL3 and
///     C        write them to the generic segment in that file.
///     C
///              IF ( .NOT. DONE3 ) THEN
///                 CALL GET_FIX_PKTS ( NPKTS, PKTS, REFS, DONE3 )
///
///                 IF ( NPKTS .GT. 0 ) THEN
///                    CALL SGWFPK ( HANDL3, NPKTS, PKTS, NPKTS, REFS )
///                 END IF
///              END IF
///     C
///     C        Get a data packet and reference value for HANDL4 and
///     C        write it to the generic segment in that file.
///     C
///              IF ( .NOT. DONE4 ) THEN
///                 GET_VAR_PKT ( PACKET, SIZE, REF, DONE4 )
///
///                 IF ( .NOT. DONE4 ) THEN
///                    CALL SGWVPK ( HANDL4, 1, SIZES, PKTS, 1, REFS )
///                 END IF
///              END IF
///     C
///     C        Set the DONE flag.
///     C
///              DONE = DONE1 .AND. DONE2 .AND. DONE3 .AND. DONE4
///
///           END DO
///     C
///     C     End the segments and move on to other things.
///     C
///           CALL SGWES ( HANDL1 )
///           CALL SGWES ( HANDL2 )
///           CALL SGWES ( HANDL3 )
///           CALL SGWES ( HANDL4 )
///                             .
///                             .
///                             .
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  See the individual entry points for any restrictions they may
///      have.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.1, 27-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.2.0, 07-SEP-2001 (EDW)
///
///         Replaced DAFRDA calls with DAFGDA.
///         Removed DAFHLU calls; replaced ERRFN calls with ERRHAN.
///
/// -    SPICELIB Version 1.1.0, 30-JUL-1996 (KRG) (NJB)
///
///         Fixed an annoying little bug in the variable segments code
///         when ending a segment. Rather than storing an appropriate
///         offset from the beginning of the segment as the packet
///         address in the packet directory, the absolute address, the
///         DAF address, was stored. This bug has been fixed.
///
///         See SGWES for the details of the changes.
///
/// -    SPICELIB Version 1.0.0, 03-APR-1995 (KRG) (WLT)
/// ```
pub fn sgseqw(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    segid: &str,
    nconst: i32,
    const_: &[f64],
    npkts: i32,
    pktsiz: &[i32],
    pktdat: &[f64],
    nrefs: i32,
    refdat: &[f64],
    idxtyp: i32,
) -> crate::Result<()> {
    SGSEQW(
        handle,
        descr,
        segid.as_bytes(),
        nconst,
        const_,
        npkts,
        pktsiz,
        pktdat,
        nrefs,
        refdat,
        idxtyp,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGSEQW ( Generic segments: Sequential writer. )
pub fn SGSEQW(
    HANDLE: i32,
    DESCR: &[f64],
    SEGID: &[u8],
    NCONST: i32,
    CONST: &[f64],
    NPKTS: i32,
    PKTSIZ: &[i32],
    PKTDAT: &[f64],
    NREFS: i32,
    REFDAT: &[f64],
    IDXTYP: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //
    // FPRINT is the integer value of the first printable ASCII
    // character.
    //
    // LPRINT is the integer value of the last printable ASCII character.
    //

    //
    // The number of reference values it takes to get a reference
    // directory value.
    //
    //
    // The length of a DAF internal filename.
    //
    //
    // The file table size. This needs to be the same as the file table
    // size in DAFAH.
    //
    //
    // Include the mnemonic values for the generic segment declarations
    // and the meta data information.
    //

    //
    // Local variables
    //
    // Variables with the name DUMMY* are used as place holders when
    // calling various subroutines. Their values are not used in any of
    // the entry points of this subroutine.
    //

    //
    // File table declarations. The file table is used to keep track of
    // the vital statistics for each of the generic segments being
    // written.
    //

    //
    // Saved values.
    //
    //
    // Save the file table.
    //
    //
    // Initial values
    //
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }
    //
    // Signal an error if this routine is called directly.
    //
    CHKIN(b"SGSEQW", ctx)?;
    SETMSG(b"This routine should never be called directly. It exists as an umbrella routine to maintain all of the variables for the generic segment sequential writing entry points.", ctx);
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"SGSEQW", ctx)?;
    Ok(())
}

/// Generic segments: Begin a fixed size segment.
///
/// Begin writing a generic segment that will contain fixed size data
/// packets.
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
///  HANDLE    I    Handle of a DAF file opened with write access.
///  DESCR     I    Descriptor for a generic segment.
///  SEGID     I    Identifier for a generic segment.
///  NCONST    I    Number of constant values in a generic segment.
///  CONST     I    Array of constant values for a generic segment.
///  PKTSIZ    I    Size of the data packets.
///  IDXTYP    I    Index type for the reference values.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened with write access.
///           This is the handle of the file in which a generic segment
///           will be written.
///
///  DESCR    is the descriptor for a segment that is being written.
///           This is the packed form of the DAF double precision and
///           integer summaries which contain ND double precision
///           numbers and NI integers.
///
///  SEGID    is an identifier for a segment that is being written.
///           This is a character string containing at most NC printing
///           ASCII characters where
///
///                             /  ND + ( NI + 1 )  \
///                  NC =  8 *  | ----------------- |
///                             \         2         /
///
///            SEGID may be blank.
///
///  NCONST   is the number of constant values to be placed in a
///           segment.
///
///  CONST    is an array of NCONST constant values for a segment.
///
///  PKTSIZ   is the size of fixed size packets. The size of a packet
///           is the number of double precision numbers contained in
///           the data packet.
///
///  IDXTYP   is the index type to use for the reference values.
///
///           Two forms of indexing are provided:
///
///              1) An implicit form of indexing based on using two
///                 values, a starting value, which will have an index
///                 of 1, and a step size between reference values,
///                 which are used to compute an index and a reference
///                 value associated with a specified key value. See
///                 the descriptions of the implicit types below for
///                 the particular formula used in each case.
///
///              2) An explicit form of indexing based on a reference
///                 value for each data packet.
///
///           See the chapter on generic segments in the DAF required
///           or the include file 'sgparam.inc' for more details
///           about the index types that are available.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called more than once for a particular file
///      and segment, the error SPICE(CALLEDOUTOFORDER) is signaled.
///
///  2)  If the length of the segment identifier, SEGID, is greater
///      than NC, as determined from the ND and NI values for a
///      particular DAF file, the error SPICE(SEGIDTOOLONG) is
///      signaled.
///
///  3)  If the segment identifier contains nonprinting characters, the
///      error SPICE(NONPRINTINGCHARS) is signaled.
///
///  4)  If the number of constant values, NCONST, is negative, the
///      error SPICE(NUMCONSTANTSNEG) is signaled.
///
///  5)  If the packet size, PKTSIZ, is not positive, the error
///      SPICE(NONPOSPACKETSIZE) is signaled.
///
///  6)  If the index type for the reference values is not recognized,
///      the error SPICE(UNKNOWNINDEXTYPE) is signaled.
///
///  7)  If the file table is full, the error SPICE(FILETABLEFULL) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  Begin writing a generic segment for fixed size data packets to
///  the DAF file associated with HANDLE.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section in the header for the main subroutine.
///  It contains examples which demonstrate the use of the entry points
///  in the generic segments sequential writer. The entry points which
///  comprise the generic segments sequential writer must be used
///  together in the proper manner. Rather than repeating the examples
///  for each entry point they are provided in a single location.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1995 (KRG) (WLT)
/// ```
pub fn sgbwfs(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    segid: &str,
    nconst: i32,
    const_: &[f64],
    pktsiz: &[i32],
    idxtyp: i32,
) -> crate::Result<()> {
    SGBWFS(
        handle,
        descr,
        segid.as_bytes(),
        nconst,
        const_,
        pktsiz,
        idxtyp,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGBWFS ( Generic segments: Begin a fixed size segment. )
pub fn SGBWFS(
    HANDLE: i32,
    DESCR: &[f64],
    SEGID: &[u8],
    NCONST: i32,
    CONST: &[f64],
    PKTSIZ: &[i32],
    IDXTYP: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..);
    let CONST = DummyArray::new(CONST, 1..);
    let PKTSIZ = DummyArray::new(PKTSIZ, 1..);
    let mut DUMMY1 = [b' '; IFNLEN as usize];
    let mut BEGADR: i32 = 0;
    let mut DUMMY2: i32 = 0;
    let mut DUMMY3: i32 = 0;
    let mut ICH: i32 = 0;
    let mut NC: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut SIDLEN: i32 = 0;

    //
    // SPICELIB functions
    //
    // INTEGER               LASTNB
    // INTEGER               ISRCHI
    //
    // LOGICAL               FAILED
    // LOGICAL               RETURN
    //
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGBWFS", ctx)?;
    //
    // We need to do some sanity checks on our input arguments before we
    // should attempt to write anything to the file. So, let's start with
    // that.
    //
    // Check to see if the file attached to the handle is open for
    // writing. If not, an error is signaled.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }
    //
    // Check to see if the handle is currently in the file table. If it
    // is, we've got a problem. This routine may only be called once for
    // each segment that is to contain fixed size packets, and it places
    // a handle in the file table. If the handle is currently in the
    // file table a segment has already been started by this routine or
    // SGBWVS. In either case, we cannot continue, so we signal an error.
    //
    if (save.NFT > 0) {
        save.INDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

        if (save.INDEX != 0) {
            SETMSG(b"A segment is already being written to the file \'#\'. A new segment cannot be started for this file until the current segment is finished. ", ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(CALLEDOUTOFORDER)", ctx)?;
            CHKOUT(b"SGBWFS", ctx)?;
            return Ok(());
        }
    }
    //
    // Get the ND and NI values from the DAF file. We need these to know
    // the size of the descriptor and the length of the segment ID. The
    // length of the segment ID is determined by the following formula
    // using integer division:
    //
    //                             /  ND + ( NI + 1 )  \
    //                  NC =  8 *  | ----------------- |
    //                             \         2         /
    //
    DAFHSF(HANDLE, &mut ND, &mut NI, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }

    NC = (8 * (ND + ((NI + 1) / 2)));
    //
    // Get the length of the segment ID. Leading blanks are considered to
    // be important. A blank segment ID is OK too.
    //
    SIDLEN = LASTNB(SEGID);
    //
    // Check the segment ID to see if it is OK. Its length must be less
    // than NC and it must consist of only printing ASCII characters.
    //
    if (SIDLEN > NC) {
        SETMSG(b"Segment identifier contains more than # characters.", ctx);
        ERRINT(b"#", NC, ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }

    for I in 1..=SIDLEN {
        ICH = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((ICH < FPRINT) || (ICH > LPRINT)) {
            SETMSG(
                b"The segment identifier contains  a nonprinting character at location #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(NONPRINTINGCHARS)", ctx)?;
            CHKOUT(b"SGBWFS", ctx)?;
            return Ok(());
        }
    }
    //
    // Check to see if the number of constants is negative. This is all
    // we can do here, we cannot check the constant values.
    //
    if (NCONST < 0) {
        SETMSG(b"The number of constants specified was #. This number must be non-negative. Perhaps the variable was not properlyinitialized. ", ctx);
        ERRINT(b"#", NCONST, ctx);
        SIGERR(b"SPICE(NUMCONSTANTSNEG) ", ctx)?;
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }
    //
    // Check to see that the packet size is OK. It should be positive.
    //
    if (PKTSIZ[1] <= 0) {
        SETMSG(b"The size of the data packets must be positive. It was specified as #. Perhaps the input variable was not properly initialized. ", ctx);
        ERRINT(b"#", *PKTSIZ.first(), ctx);
        SIGERR(b"SPICE(NONPOSPACKETSIZE)", ctx)?;
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }
    //
    // Check to see if the index type is one that we recognize.
    //
    if ((IDXTYP < MNIDXT) || (IDXTYP > MXIDXT)) {
        SETMSG(b"The index type specified was #.  This is not a valid index type. Valid types are in the range from # to #.", ctx);
        ERRINT(b"#", IDXTYP, ctx);
        ERRINT(b"#", MNIDXT, ctx);
        ERRINT(b"#", MXIDXT, ctx);
        SIGERR(b"SPICE(UNKNOWNINDEXTYPE)", ctx)?;
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }
    //
    // Check to see whether we still have room in the file table.
    //
    if (save.NFT == FTSIZE) {
        SETMSG(b"There are already # files being written by generic segment writing routines. No more files may be written by the generic segment writers until one of those currently being written is closed via a call to SGWES.", ctx);
        ERRINT(b"#", save.NFT, ctx);
        SIGERR(b"SPICE(FILETABLEFULL)", ctx)?;
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }
    //
    // Set the flag which indicate whether this index type is an
    // explicit type or an implicit type.
    //
    save.EXPLCT = (((IDXTYP == EXPLT) || (IDXTYP == EXPLE)) || (IDXTYP == EXPCLS));
    //
    // At this point, we know that the input data is OK, in so far as we
    // can validate it, and we have room in the file table. So we proceed
    // with starting a segment for fixed size packets.
    //
    // Set the flag that indicate that this segment is a fixed size
    // segment.
    //
    save.FXDSEG = true;
    //
    // Get the address for the beginning of the array that we are going
    // to create. We have to get this by reading the file record.
    //
    DAFRFR(
        HANDLE,
        &mut ND,
        &mut NI,
        &mut DUMMY1,
        &mut DUMMY2,
        &mut DUMMY3,
        &mut BEGADR,
        ctx,
    )?;
    //
    // Begin a new segment in the DAF file.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGBWFS", ctx)?;
        return Ok(());
    }
    //
    // Write out the constants to the new segment, if there are any
    // constants.
    //
    if (NCONST > 0) {
        DAFADA(CONST.as_slice(), NCONST, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGBWFS", ctx)?;
            return Ok(());
        }
    }
    //
    // Store the information for this file and segment in the file table.
    //
    save.NFT = (save.NFT + 1);

    save.FTITYP[save.NFT] = IDXTYP;
    save.FTPKSZ[save.NFT] = PKTSIZ[1];
    save.FTMXSZ[save.NFT] = 0;

    save.FTNCON[save.NFT] = NCONST;
    save.FTNPKT[save.NFT] = 0;
    save.FTNREF[save.NFT] = 0;
    save.FTNRES[save.NFT] = 0;

    save.FTEXPL[save.NFT] = save.EXPLCT;

    save.FTFIXD[save.NFT] = save.FXDSEG;

    save.FTHAN[save.NFT] = HANDLE;
    save.FTBADR[save.NFT] = BEGADR;

    save.FTREFS[[1, save.NFT]] = 0.0;
    save.FTREFS[[2, save.NFT]] = 0.0;

    if save.EXPLCT {
        save.FTOFF[save.NFT] = 1;
    } else {
        save.FTOFF[save.NFT] = 0;
    }

    save.LSTHAN = HANDLE;
    save.INDEX = save.NFT;
    save.NUMFXD = (save.NUMFXD + 1);

    CHKOUT(b"SGBWFS", ctx)?;
    Ok(())
}

/// Generic segments: Begin a variable size segment.
///
/// Begin writing a generic segment that will contain variable size
/// data packets.
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
///  HANDLE    I    Handle of a DAF file opened with write access.
///  DESCR     I    Descriptor for a segment.
///  SEGID     I    Identifier for a segment.
///  NCONST    I    Number of constant values in a segment.
///  CONST     I    Array of constant values for a segment.
///  IDXTYP    I    Index type for the reference values.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened with write access.
///           This is the handle of the file in which a generic segment
///           will be written.
///
///  DESCR    is the descriptor for a segment that is being written.
///           This is the packed form of the DAF double precision and
///           integer summaries which contain ND double precision
///           numbers and NI integers.
///
///  SEGID    is an identifier for a segment that is being written.
///           This is a character string containing at most NC printing
///           ASCII characters where
///
///                             /  ND + ( NI + 1 )  \
///                  NC =  8 *  | ----------------- |
///                             \         2         /
///
///            SEGID may be blank.
///
///  NCONST   is the number of constant values to be placed in a
///           segment.
///
///  CONST    is an array of NCONST constant values for a segment.
///
///  IDXTYP   is the index type to use for the reference values.
///
///           Two forms of indexing are provided:
///
///              1) An implicit form of indexing based on using two
///                 values, a starting value, which will have an index
///                 of 1, and a step size between reference values,
///                 which are used to compute an index and a reference
///                 value associated with a specified key value. See
///                 the descriptions of the implicit types below for
///                 the particular formula used in each case.
///
///              2) An explicit form of indexing based on a reference
///                 value for each data packet.
///
///           See the chapter on generic segments in the DAF required
///           or the include file 'sgparam.inc' for more details
///           about the index types that are available.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called more than once for a particular file
///      and segment, the error SPICE(CALLEDOUTOFORDER) is signaled.
///
///  2)  If the length of the segment identifier, SEGID, is greater
///      than NC, as determined from the ND and NI values for a
///      particular DAF file, the error SPICE(SEGIDTOOLONG) is
///      signaled.
///
///  3)  If the segment identifier contains nonprinting characters, the
///      error SPICE(NONPRINTINGCHARS) is signaled.
///
///  4)  If the number of constant values, NCONST, is negative, the
///      error SPICE(NUMCONSTANTSNEG) is signaled.
///
///  5)  If the index type for the reference values is not recognized,
///      the error SPICE(UNKNOWNINDEXTYPE) is signaled.
///
///  6)  If the file table is full, the error SPICE(FILETABLEFULL) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  Begin writing a generic segment for variable size data packets to
///  the DAF file associated with HANDLE.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section in the header for the main subroutine.
///  It contains examples which demonstrate the use of the entry points
///  in the generic segments sequential writer. The entry points which
///  comprise the generic segments sequential writer must be used
///  together in the proper manner. Rather than repeating the examples
///  for each entry point they are provided in a single location.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1995 (KRG) (WLT)
/// ```
pub fn sgbwvs(
    ctx: &mut SpiceContext,
    handle: i32,
    descr: &[f64],
    segid: &str,
    nconst: i32,
    const_: &[f64],
    idxtyp: i32,
) -> crate::Result<()> {
    SGBWVS(
        handle,
        descr,
        segid.as_bytes(),
        nconst,
        const_,
        idxtyp,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGBWVS ( Generic segments: Begin a variable size segment. )
pub fn SGBWVS(
    HANDLE: i32,
    DESCR: &[f64],
    SEGID: &[u8],
    NCONST: i32,
    CONST: &[f64],
    IDXTYP: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let DESCR = DummyArray::new(DESCR, 1..);
    let CONST = DummyArray::new(CONST, 1..);
    let mut DUMMY1 = [b' '; IFNLEN as usize];
    let mut BEGADR: i32 = 0;
    let mut DUMMY2: i32 = 0;
    let mut DUMMY3: i32 = 0;
    let mut ICH: i32 = 0;
    let mut NC: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut SIDLEN: i32 = 0;

    //
    // SPICELIB functions
    //
    // INTEGER               LASTNB
    // INTEGER               ISRCHI
    //
    // LOGICAL               FAILED
    // LOGICAL               RETURN
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGBWVS", ctx)?;
    //
    // We need to do some sanity checks on our input arguments before we
    // should attempt to write anything to the file. So, let's start with
    // that.
    //
    // Check to see if the file attached to the handle is open for
    // writing. If not, an error is signaled.
    //
    DAFSIH(HANDLE, b"WRITE", ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }
    //
    // Check to see if the handle is currently in the file table. If it
    // is, we've got a problem. This routine may only be called once for
    // each segment that is to contain variable size packets, and it
    // places a handle into the file table. If the handle is currently in
    // the file table a segment has already been started by this routine
    // or SGBWFS. In either case, we cannot continue, so we signal an
    // error.
    //
    if (save.NFT > 0) {
        save.INDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

        if (save.INDEX != 0) {
            SETMSG(b"A segment is already being written to the file \'#\'. A new segment cannot be started for this file until the current segment is finished. ", ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(CALLEDOUTOFORDER)", ctx)?;
            CHKOUT(b"SGBWVS", ctx)?;
            return Ok(());
        }
    }
    //
    // Get the ND and NI values from the DAF file. We need these to know
    // the size of the descriptor and the length of the segment ID. The
    // length of the segment ID is determined by the following formula
    // using integer division:
    //
    //                             /  ND + ( NI + 1 )  \
    //                  NC =  8 *  | ----------------- |
    //                             \         2         /
    //
    DAFHSF(HANDLE, &mut ND, &mut NI, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }

    NC = (8 * (ND + ((NI + 1) / 2)));
    //
    // Get the length of the segment ID. Leading blanks are considered to
    // be important. A blank segment ID is OK too.
    //
    SIDLEN = LASTNB(SEGID);
    //
    // Check the segment ID to see if it is OK. Its length must be less
    // than NC and it must consist of only printing ASCII characters.
    //
    if (SIDLEN > NC) {
        SETMSG(b"Segment identifier contains more than # characters.", ctx);
        ERRINT(b"#", NC, ctx);
        SIGERR(b"SPICE(SEGIDTOOLONG)", ctx)?;
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }

    for I in 1..=SIDLEN {
        ICH = intrinsics::ICHAR(fstr::substr(SEGID, I..=I));

        if ((ICH < FPRINT) || (ICH > LPRINT)) {
            SETMSG(
                b"The segment identifier contains  a nonprinting character at location #.",
                ctx,
            );
            ERRINT(b"#", I, ctx);
            SIGERR(b"SPICE(NONPRINTINGCHARS)", ctx)?;
            CHKOUT(b"SGBWVS", ctx)?;
            return Ok(());
        }
    }
    //
    // Check to see if the number of constants is negative. This is all
    // we can do here, we cannot check the constant values.
    //
    if (NCONST < 0) {
        SETMSG(b"The number of constants specified was #. This number must be non-negative. Perhaps the variable was not initialized. ", ctx);
        ERRINT(b"#", NCONST, ctx);
        SIGERR(b"SPICE(NUMCONSTANTSNEG) ", ctx)?;
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }

    //
    // Check to see if the index type is one that we recognize.
    //
    if ((IDXTYP < MNIDXT) || (IDXTYP > MXIDXT)) {
        SETMSG(b"The index type specified was #.  This is not a valid index type. Valid types are in the range from # to #.", ctx);
        ERRINT(b"#", IDXTYP, ctx);
        ERRINT(b"#", MNIDXT, ctx);
        ERRINT(b"#", MXIDXT, ctx);
        SIGERR(b"SPICE(UNKNOWNINDEXTYPE)", ctx)?;
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }
    //
    // Check to see if there is room in the file table.
    //
    if (save.NFT == FTSIZE) {
        SETMSG(b"There are already # files being written by generic segment writing routines. No more files may be written by the generic segment writers until one of those currently being written is closed via a call to SGWES. ", ctx);
        ERRINT(b"#", save.NFT, ctx);
        SIGERR(b"SPICE(FILETABLEFULL)", ctx)?;
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }

    //
    // Set the flag which indicate whether this index type is an
    // explicit type or an implicit type.
    //
    save.EXPLCT = (((IDXTYP == EXPLT) || (IDXTYP == EXPLE)) || (IDXTYP == EXPCLS));
    //
    // At this point, we know that the input data is OK, in so far as we
    // can validate it and that there is room in the file table. So we
    // proceed with starting a segment for fixed size packets.
    //
    // Set the flag that indicate that this segment is a variable size
    // segment.
    //
    save.FXDSEG = false;
    //
    // Get the address for the beginning of the array that we are going
    // to create. We have to get this by reading the file record.
    //
    DAFRFR(
        HANDLE,
        &mut ND,
        &mut NI,
        &mut DUMMY1,
        &mut DUMMY2,
        &mut DUMMY3,
        &mut BEGADR,
        ctx,
    )?;
    //
    // Begin a new segment in the DAF file.
    //
    DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGBWVS", ctx)?;
        return Ok(());
    }
    //
    // Write out the constants to the new segment, if there are any
    // constants.
    //
    if (NCONST > 0) {
        DAFADA(CONST.as_slice(), NCONST, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGBWVS", ctx)?;
            return Ok(());
        }
    }
    //
    // Save the information for this file and segment in the file table.
    //
    save.NFT = (save.NFT + 1);

    save.FTITYP[save.NFT] = IDXTYP;
    save.FTPKSZ[save.NFT] = 0;
    save.FTMXSZ[save.NFT] = 0;

    save.FTNCON[save.NFT] = NCONST;
    save.FTNPKT[save.NFT] = 0;
    save.FTNREF[save.NFT] = 0;
    save.FTNRES[save.NFT] = 0;

    save.FTEXPL[save.NFT] = save.EXPLCT;

    save.FTFIXD[save.NFT] = save.FXDSEG;

    save.FTHAN[save.NFT] = HANDLE;
    save.FTBADR[save.NFT] = BEGADR;

    save.FTREFS[[1, save.NFT]] = 0.0;
    save.FTREFS[[2, save.NFT]] = 0.0;

    if save.EXPLCT {
        save.FTOFF[save.NFT] = 2;
    } else {
        save.FTOFF[save.NFT] = 1;
    }

    save.LSTHAN = HANDLE;
    save.INDEX = save.NFT;
    save.NUMVAR = (save.NUMVAR + 1);

    CHKOUT(b"SGBWVS", ctx)?;
    Ok(())
}

/// Generic segments: Write fixed size packets.
///
/// Write one or more fixed size data packets to the generic segment
/// currently being written to the DAF file associated with HANDLE.
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
///  HANDLE    I    Handle of a DAF file opened with write access.
///  NPKTS     I    Number of data packets to write to a segment.
///  PKTDAT    I    Array of packet data.
///  NREFS     I    Number of reference values.
///  REFDAT    I    Reference data.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened with write access.
///           This is the handle of a file in which a generic segment
///           has been started and is currently being written.
///
///  NPKTS    is the number of data packets to write to a segment.
///
///  PKTDAT   is a singly dimensioned array containing the fixed size
///           data packets to be added to the segment associated with
///           HANDLE.
///
///           For fixed size data packets, PKTDAT will have the
///           following structure:
///
///              Packet #  Range of Locations
///              --------  ------------------------------------------
///
///                 1      PKTDAT(1)              to PKTDAT(PS)
///                 2      PKTDAT(PS+1)           to PKTDAT(2*PS)
///                 3      PKTDAT(2*PS+1)         to PKTDAT(3*PS)
///                 4      PKTDAT(3*PS+1)         to PKTDAT(4*PS)
///
///                                         .
///                                         .
///                                         .
///
///                NPKTS   PKTDAT((NPKTS-1)*PS+1) to PKTDAT(NPKTS*PS)
///
///           where PS = PKTSIZ.
///
///  NREFS    is the number of reference values.
///
///           For implicitly indexed packets, NREFS must have a value
///           of two (2).
///
///           When writing packets to a segment which uses an implicit
///           index type, the value specified by NREFS is used only on
///           the first call to SGWFPK. On all subsequent calls to
///           these subroutines for a particular implicitly indexed
///           segment, the value of NREFS is ignored.
///
///           For explicitly indexed packets, NREFS must be equal to
///           NPKTS, i.e., there should ba a reference value for each
///           data packet being written to the segment.
///
///           When writing packets to a segment which uses an explicit
///           index type, the value specified by NREFS is used on
///           every call to SGWFPK and it must be equal to NPKTS.
///
///  REFDAT   is the reference data values.
///
///           For implicitly indexed packets, there must be two (2)
///           values. The reference values represent a starting
///           reference value and a step size between consecutive
///           reference values, respectively.
///
///           In order to avoid, or at least minimize, numerical
///           difficulties associated with computing index values for
///           generic segments with implicit index types, the value of
///           the step size must be an integer, i.e., DINT(REFDAT(2))
///           must equal REFDAT(2).
///
///           When writing packets to a segment which uses an implicit
///           index type, the values specified by REFDAT are used only
///           on the first call to SGWFPK. On all subsequent calls to
///           this subroutine for a particular implicitly indexed
///           segment, REFDAT is ignored.
///
///           For explicitly indexed packets, there must be NPKTS
///           reference values and the values must be in increasing
///           order:
///
///              REFDAT(I) < REFDAT(I+1), I = 1, NPKTS-1
///
///           When writing packets to a segment which uses an explicit
///           index type, the values specified by REFDAT are used on
///           every call to SGWFPK. On all calls to these subroutines
///           after the first, the value of REFDAT(1) must be greater
///           than than the value of REFDAT(NPKTS) from the previous
///           call. This preserves the ordering of the reference
///           values for the entire segment.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there are no generic segments with fixed packet sizes
///      currently being written, the error SPICE(CALLEDOUTOFORDER) is
///      signaled.
///
///  2)  If there is not a generic segment with fixed packet size being
///      written to the file associated with HANDLE, the error
///      SPICE(SEGMENTNOTFOUND) is signaled.
///
///  3)  If the type of generic segment being written to this file is
///      not a fixed packet size generic segment, the error
///      SPICE(SEGTYPECONFLICT) is signaled.
///
///  4)  If the number of packets to be written to the generic segment
///      is not positive, the error SPICE(NUMPACKETSNOTPOS) is
///      signaled.
///
///  5)  If an explicitly indexed generic segment is being written and
///      the number of reference values, NREFS, is not equal to the
///      number of data packets being written, NPKTS, the error
///      SPICE(INCOMPATIBLENUMREF) is signaled.
///
///  6)  If an explicitly indexed generic segment is being written and
///      the reference values are not in increasing order, the error
///      SPICE(UNORDEREDREFS) is signaled.
///
///  7)  If an explicitly indexed generic segment is being written and
///      the first reference value on the second or later additions
///      of packets to the generic segment is not greater than the last
///      reference value from the previous addition of packets, the
///      error SPICE(UNORDEREDREFS) is signaled.
///
///  8)  If an implicitly indexed generic segment is being written and
///      the number of reference values, NREFS, is not equal to two (2)
///      on the first call to this subroutine for a particular segment,
///      the error SPICE(INCOMPATIBLENUMREF) is signaled.
///
///  9)  If an implicitly indexed generic segment is being written and
///      the second reference value, the step size used for indexing,
///      is not integral, i.e., DINT(REFDAT(2)) .NE. REFDAT(2), the
///      error SPICE(REFVALNOTINTEGER) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will write one or more fixed size data packets to a
///  generic segment in the DAF file associated with HANDLE. The
///  generic segment must have been started by a call to SGBWFS.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section in the header for the main subroutine.
///  It contains examples which demonstrate the use of the entry points
///  in the generic segments sequential writer. The entry points which
///  comprise the generic segments sequential writer must be used
///  together in the proper manner. Rather than repeating the examples
///  for each entry point they are provided in a single location.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1995 (KRG) (WLT)
/// ```
pub fn sgwfpk(
    ctx: &mut SpiceContext,
    handle: i32,
    npkts: i32,
    pktdat: &[f64],
    nrefs: i32,
    refdat: &[f64],
) -> crate::Result<()> {
    SGWFPK(handle, npkts, pktdat, nrefs, refdat, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGWFPK ( Generic segments: Write fixed size packets. )
pub fn SGWFPK(
    HANDLE: i32,
    NPKTS: i32,
    PKTDAT: &[f64],
    NREFS: i32,
    REFDAT: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PKTDAT = DummyArray::new(PKTDAT, 1..);
    let REFDAT = DummyArray::new(REFDAT, 1..);

    //
    // SPICELIB functions
    //
    // INTEGER               LASTNB
    // INTEGER               ISRCHI
    //
    // LOGICAL               FAILED
    // LOGICAL               RETURN
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGWFPK", ctx)?;
    //
    // Check to see if this is the first time here. If it is, we have
    // been called out of order, so signal an error.
    //
    if (save.NUMFXD == 0) {
        SETMSG(b"No segment with fixed size packets is currently being written. This routine has been called out of order. The routine SGBWFS must be called before his routine may be called.", ctx);
        SIGERR(b"SPICE(CALLEDOUTOFORDER)", ctx)?;
        CHKOUT(b"SGWFPK", ctx)?;
        return Ok(());
    }
    //
    // Check to see if the last handle used is the same as the current
    // handle. This saves us a table lookup to get the appropriate index
    // into the file table to restore the information for that handle.
    //
    if (HANDLE != save.LSTHAN) {
        save.INDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

        if (save.INDEX == 0) {
            SETMSG(b"No segment with fixed size packets is associated with the file \'#\'. In order to write fixed size packets to a file the routine SGBWFS must be called to begin the segment.", ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(SEGMENTNOTFOUND)", ctx)?;
            CHKOUT(b"SGWFPK", ctx)?;
            return Ok(());
        }

        save.EXPLCT = save.FTEXPL[save.INDEX];
        save.FXDSEG = save.FTFIXD[save.INDEX];
        save.LSTHAN = HANDLE;

        DAFCAD(HANDLE, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGWFPK", ctx)?;
            return Ok(());
        }
    }
    //
    // Check to see if the segment being written is a fixed size packet
    // segment or a variable size packet segment. If the latter, then
    // this is the wrong routine.
    //
    if !save.FXDSEG {
        SETMSG(b"The segment being written to the file  \'#\' is a variable packet size segment, not a fixed packet size segment.  The routine SGWVPK may be used to write variable size packets.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(SEGTYPECONFLICT)", ctx)?;
        CHKOUT(b"SGWFPK", ctx)?;
        return Ok(());
    }
    //
    // At this point, we have a good file handle, an index into the file
    // table, and we know that we are working with a fixed packet size
    // segment. So, what we need to do now is verify the input arguments.
    //
    // Check the number of packets to be sure that it is positive.
    //
    if (NPKTS <= 0) {
        SETMSG(b"The number of packets to store is not positive.  The value supplied was #. Perhaps this packet count was uninitialized.", ctx);
        ERRINT(b"#", NPKTS, ctx);
        SIGERR(b"SPICE(NUMPACKETSNOTPOS)", ctx)?;
        CHKOUT(b"SGWFPK", ctx)?;
        return Ok(());
    }

    //
    // Now we get to some of the more interesting bits. We now need to
    // differentiate between the explicitly indexed types and the
    // implicitly indexed types, because they have different
    // characteristics and assumptions about how they are stored.
    //
    if save.EXPLCT {
        //
        // For explicitly indexed packets the number of reference values
        // must be equal to the number of packets. The references must
        // also be in increasing order.
        //
        if (NREFS != NPKTS) {
            SETMSG(b"The number of reference values supplied, #, is not compatible with explicitly indexed packets. Explicitly indexed packets require the number of reference values to equal the number of packets, in this case, #.", ctx);
            ERRINT(b"#", NREFS, ctx);
            ERRINT(b"#", NPKTS, ctx);
            SIGERR(b"SPICE(INCOMPATIBLENUMREF)", ctx)?;
            CHKOUT(b"SGWFPK", ctx)?;
            return Ok(());
        }
        //
        // If this is not the first time we have added data to this
        // segment, we need to be sure that all of the current reference
        // values are greater then the last reference value from the
        // previous addition of packets to the segment.
        //
        if (save.FTNPKT[save.INDEX] > 0) {
            if (save.FTREFS[[1, save.INDEX]] >= REFDAT[1]) {
                SETMSG(b"Reference values are out of order. The offending value, #, was found to be out of order. The reference values for explicitly indexed packets must be in increasing order, and the first reference value is less than or equal to the last reference value, #, from the previous addition of packets.", ctx);
                ERRDP(b"#", REFDAT[1], ctx);
                ERRDP(b"#", save.FTREFS[[1, save.INDEX]], ctx);
                SIGERR(b"SPICE(UNORDEREDREFS)", ctx)?;
                CHKOUT(b"SGWFPK", ctx)?;
                return Ok(());
            }
        }

        for I in 2..=NREFS {
            if (REFDAT[(I - 1)] >= REFDAT[I]) {
                SETMSG(b"Reference values are out of order. The offending value, #, was found to be out of order for index #. The reference values for explicitly indexed packets must be in increasing order.", ctx);
                ERRDP(b"#", REFDAT[(I - 1)], ctx);
                ERRINT(b"#", (I - 1), ctx);
                SIGERR(b"SPICE(UNORDEREDREFS)", ctx)?;
                CHKOUT(b"SGWFPK", ctx)?;
                return Ok(());
            }
        }
        //
        // Add the packets preceded by their reference values to the
        // segment. We put the reference values with the packets so that
        // we do not need to open a scratch file. We will use them to
        // construct a reference directory after all of the packets have
        // been added to the segment.
        //
        for I in 1..=NPKTS {
            DAFADA(REFDAT.subarray(I), 1, ctx)?;
            DAFADA(
                PKTDAT.subarray((((I - 1) * save.FTPKSZ[save.INDEX]) + 1)),
                save.FTPKSZ[save.INDEX],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SGWFPK", ctx)?;
                return Ok(());
            }
        }
        //
        // Save the last reference value in the file table so that we
        // can use it to verify that the next addition does not violate
        // the increasing order of the reference values.
        //
        save.FTREFS[[1, save.INDEX]] = REFDAT[NREFS];
        //
        // Update the counts for the number of packets, the number of
        // references.
        //
        save.FTNPKT[save.INDEX] = (save.FTNPKT[save.INDEX] + NPKTS);
        save.FTNREF[save.INDEX] = (save.FTNREF[save.INDEX] + NREFS);
    } else {
        //
        // For implicitly indexed packets the number of reference values
        // must be two (2), and the second reference value must be an
        // integer, i.e., DINT(REFDAT(2)) .eq. REFDAT(2). The number of
        // reference values and the integrality of the second reference
        // value are checked only on the first call to add variable length
        // data packets to a generic segment. In all subsequent calls,
        // these arguments are ignored.
        //

        if (save.FTNPKT[save.INDEX] == 0) {
            if (NREFS != 2) {
                SETMSG(b"The number of reference values supplied, #, is not compatible with implicitly indexed packets. Implicitly indexed packets require the number of reference values to be two (2).", ctx);
                ERRINT(b"#", NREFS, ctx);
                SIGERR(b"SPICE(INCOMPATIBLENUMREF)", ctx)?;
                CHKOUT(b"SGWFPK", ctx)?;
                return Ok(());
            }

            if (f64::trunc(REFDAT[2]) != REFDAT[2]) {
                SETMSG(
                    b"For implicitly indexed packets the step size must be an integer.",
                    ctx,
                );
                SIGERR(b"SPICE(REFVALNOTINTEGER)", ctx)?;
                CHKOUT(b"SGWFPK", ctx)?;
                return Ok(());
            }
        }
        //
        // Add the packets to the segment.
        //
        DAFADA(PKTDAT.as_slice(), (save.FTPKSZ[save.INDEX] * NPKTS), ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGWFPK", ctx)?;
            return Ok(());
        }
        //
        // Save the last reference values and the number of reference
        // values in the file table. We only do this on the first time
        // through the routine.
        //
        if (save.FTNPKT[save.INDEX] == 0) {
            save.FTNREF[save.INDEX] = NREFS;
            save.FTREFS[[1, save.INDEX]] = REFDAT[1];
            save.FTREFS[[2, save.INDEX]] = REFDAT[2];
        }
        //
        // Update the count for the number of packets.
        //
        save.FTNPKT[save.INDEX] = (save.FTNPKT[save.INDEX] + NPKTS);
    }

    CHKOUT(b"SGWFPK", ctx)?;
    Ok(())
}

/// Generic segment: Write variable size packets.
///
/// Write one or more variable size data packets to the generic
/// segment currently being written to the DAF file associated with
/// HANDLE.
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
///  HANDLE    I    Handle of a DAF file opened with write access.
///  NPKTS     I    Number of data packets to write to a segment.
///  PKTSIZ    I    Array of sizes of variable size packets.
///  PKTDAT    I    Array of packet data.
///  NREFS     I    Number of reference values.
///  REFDAT    I    Reference data.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened with write access.
///           This is the handle of a file in which a generic segment
///           has been started and is currently being written.
///
///  NPKTS    is the number of data packets to write to a segment.
///
///  PKTSIZ   is the sizes of variable size packets.
///
///           By the size of a packet we mean the number of double
///           precision numbers contained in a data packet.
///
///           When writing a segment with variable size packets,
///           there must be an element in the array PKTSIZ for each of
///           the variable size data packets.
///
///  PKTDAT   is a singly dimensioned array containing the variable
///           size data packets to be added to the generic segment
///           associated with HANDLE.
///
///           For variable size data packets, PKTDAT will have the
///           following structure:
///
///              Packet #  Range of Locations
///              --------  ------------------------------------------
///
///                 1      PKTDAT(1)           to PKTDAT(P(1))
///                 2      PKTDAT(P(1)+1)      to PKTDAT(P(2))
///                 3      PKTDAT(P(2)+1)      to PKTDAT(P(3))
///                 4      PKTDAT(P(3)+1)      to PKTDAT(P(4))
///
///                                         .
///                                         .
///                                         .
///
///                NPKTS   PKTDAT(P(NPKTS-1)+1) to PKTDAT(P(NPKTS))
///
///                            I
///                           ---
///           where P(I) =    >   PKTSIZ(K).
///                           ---
///                          K = 1
///
///  NREFS    is the number of reference values.
///
///           For implicitly indexed packets, NREFS must have a value
///           of two (2).
///
///           When writing packets to a segment which uses an implicit
///           index type, the value specified by NREFS is used only on
///           the first call to SGWVPK. On all subsequent calls to
///           these subroutines for a particular implicitly indexed
///           segment, the value of NREFS is ignored.
///
///           For explicitly indexed packets, NREFS must be equal to
///           NPKTS, i.e., there should be a reference value for each
///           data packet being written to the segment.
///
///           When writing packets to a segment which uses an explicit
///           index type, the value specified by NREFS is used on
///           every call to SGWVPK and it must be equal to NPKTS.
///
///  REFDAT   is the reference data values.
///
///           For implicitly indexed packets, there must be two (2)
///           values. The reference values represent a starting
///           reference value and a step size between consecutive
///           reference values, respectively.
///
///           In order to avoid, or at least minimize, numerical
///           difficulties associated with computing index values for
///           generic segments with implicit index types, the value of
///           the step size must be an integer, i.e., DINT(REFDAT(2))
///           must equal REFDAT(2).
///
///           When writing packets to a segment which uses an implicit
///           index type, the values specified by REFDAT are used only
///           on the first call to SGWVPK. On all subsequent calls to
///           this subroutine for a particular implicitly indexed
///           segment, REFDAT is ignored.
///
///           For explicitly indexed packets, there must be NPKTS
///           reference values and the values must be in increasing
///           order:
///
///              REFDAT(I) < REFDAT(I+1), I = 1, NPKTS-1
///
///           When writing packets to a segment which uses an explicit
///           index type, the values specified by REFDAT are used on
///           every call to SGWVPK. On all calls to this subroutine
///           after the first, the value of REFDAT(1) must be greater
///           than than the value of REFDAT(NPKTS) from the previous
///           call. This preserves the ordering of the reference
///           values for the entire segment.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there are no generic segments with variable packet sizes
///      currently being written, the error SPICE(CALLEDOUTOFORDER)
///      is signaled.
///
///  2)  If there is not a generic segment with variable packet size
///      being written to the file associated with HANDLE, the error
///      SPICE(SEGMENTNOTFOUND) is signaled.
///
///  3)  If the type of generic segment being written to this file is
///      not a variable packet size generic segment, the error
///      SPICE(SEGTYPECONFLICT) is signaled.
///
///  4)  If the number of packets to be written to the generic segment
///      is not positive, the error SPICE(NUMPACKETSNOTPOS) is
///      signaled.
///
///  5)  If an explicitly indexed generic segment is being written and
///      the number of reference values, NREFS, is not equal to the
///      number of data packets being written, NPKTS, the error
///      SPICE(INCOMPATIBLENUMREF) is signaled.
///
///  6)  If an explicitly indexed generic segment is being written and
///      the reference values are not in increasing order, the error
///      SPICE(UNORDEREDREFS) is signaled.
///
///  7)  If an explicitly indexed generic segment is being written and
///      the first reference value on the second or later additions
///      of packets to the generic segment is not greater than the last
///      reference value from the previous addition of packets, the
///      error SPICE(UNORDEREDREFS) is signaled.
///
///  8)  If an explicitly indexed generic segment is being written and
///      one or more of the packet sizes is not positive, the error
///      SPICE(NONPOSPACKETSIZE) is signaled.
///
///  9)  If an implicitly indexed generic segment is being written and
///      the number of reference values, NREFS, is not equal to two (2)
///      on the first call to this subroutine for a particular segment,
///      the error SPICE(INCOMPATIBLENUMREF) is signaled.
///
///  10) If an implicitly indexed generic segment is being written and
///      the second reference value, the step size used for indexing,
///      is not integral, i.e., DINT(REFDAT(2)) .NE. REFDAT(2), the
///      error SPICE(REFVALNOTINTEGER) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will write one or more variable size data packets to
///  a generic segment in the DAF file associated with HANDLE. The
///  generic segment must have been started by a call to SGBWVS.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section in the header for the main subroutine.
///  It contains examples which demonstrate the use of the entry points
///  in the generic segments sequential writer. The entry points which
///  comprise the generic segments sequential writer must be used
///  together in the proper manner. Rather than repeating the examples
///  for each entry point they are provided in a single location.
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
/// -    SPICELIB Version 1.0.1, 27-OCT-2021 (JDR) (NJB)
///
///         Edited the header to comply with NAIF standard. Corrected
///         typos in comments.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1995 (KRG) (WLT)
/// ```
pub fn sgwvpk(
    ctx: &mut SpiceContext,
    handle: i32,
    npkts: i32,
    pktsiz: &[i32],
    pktdat: &[f64],
    nrefs: i32,
    refdat: &[f64],
) -> crate::Result<()> {
    SGWVPK(
        handle,
        npkts,
        pktsiz,
        pktdat,
        nrefs,
        refdat,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGWVPK ( Generic segment: Write variable size packets. )
pub fn SGWVPK(
    HANDLE: i32,
    NPKTS: i32,
    PKTSIZ: &[i32],
    PKTDAT: &[f64],
    NREFS: i32,
    REFDAT: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let PKTSIZ = DummyArray::new(PKTSIZ, 1..);
    let PKTDAT = DummyArray::new(PKTDAT, 1..);
    let REFDAT = DummyArray::new(REFDAT, 1..);
    let mut DPKSIZ: f64 = 0.0;
    let mut PKTPOS: i32 = 0;

    //
    // SPICELIB functions
    //
    // INTEGER               LASTNB
    // INTEGER               ISRCHI
    //
    // LOGICAL               FAILED
    // LOGICAL               RETURN
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGWVPK", ctx)?;
    //
    // Check to see if this is the first time here. If it is, we have
    // been called out of order, so signal an error.
    //
    if (save.NUMVAR == 0) {
        SETMSG(b"No segment with variable size packets is currently being written. This routine has been called out of order. The routine SGBWVS must be called before his routine may be called.", ctx);
        SIGERR(b"SPICE(CALLEDOUTOFORDER)", ctx)?;
        CHKOUT(b"SGWVPK", ctx)?;
        return Ok(());
    }
    //
    // Check to see if the last handle used is the same as the current
    // handle. This saves us a table lookup to get the appropriate index
    // into the file table to restore the information for that handle.
    //
    if (HANDLE != save.LSTHAN) {
        save.INDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

        if (save.INDEX == 0) {
            SETMSG(b"No segment with variable size packets is associated with the file \'#\'. In order to write variable size packets to a file the routine SGBWVS must be called to begin the segment.", ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(SEGMENTNOTFOUND)", ctx)?;
            CHKOUT(b"SGWVPK", ctx)?;
            return Ok(());
        }

        save.EXPLCT = save.FTEXPL[save.INDEX];
        save.FXDSEG = save.FTFIXD[save.INDEX];
        save.LSTHAN = HANDLE;

        DAFCAD(HANDLE, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGWVPK", ctx)?;
            return Ok(());
        }
    }
    //
    // Check to see if the segment being written is a fixed size packet
    // segment or a variable size packet segment. If the former, then
    // this is the wrong routine.
    //
    if save.FXDSEG {
        SETMSG(b"The segment being written to the file  \'#\' is a fixed packet size segment, not a variable packet size segment.  The routine SGWFPK may be used to write fixed size packets.", ctx);
        ERRHAN(b"#", HANDLE, ctx)?;
        SIGERR(b"SPICE(SEGTYPECONFLICT)", ctx)?;
        CHKOUT(b"SGWVPK", ctx)?;
        return Ok(());
    }
    //
    // At this point, we have a good file handle, an index into the file
    // table, and we know that we are working with a variable packet
    // size segment. So, what we need to do now is verify the input
    // arguments.
    //
    // Check the number of packets to be sure that it is positive.
    //
    if (NPKTS <= 0) {
        SETMSG(b"The number of packets to store is not positive.  The value supplied was #. Perhaps this packet count was uninitialized.", ctx);
        ERRINT(b"#", NPKTS, ctx);
        SIGERR(b"SPICE(NUMPACKETSNOTPOS)", ctx)?;
        CHKOUT(b"SGWVPK", ctx)?;
        return Ok(());
    }

    //
    // Now we get to some of the more interesting bits. We now need to
    // differentiate between the explicitly indexed types and the
    // implicitly indexed types, because they have different
    // characteristics and assumptions about how they are stored.
    //
    if save.EXPLCT {
        //
        // For explicitly indexed packets the number of reference values
        // must be equal to the number of packets. The references must
        // also be in increasing order.
        //
        if (NREFS != NPKTS) {
            SETMSG(b"The number of reference values supplied, #, is not compatible with explicitly indexed packets. Explicitly indexed packets require the number of reference values to equal the number of packets, in this case, #.", ctx);
            ERRINT(b"#", NREFS, ctx);
            ERRINT(b"#", NPKTS, ctx);
            SIGERR(b"SPICE(INCOMPATIBLENUMREF)", ctx)?;
            CHKOUT(b"SGWVPK", ctx)?;
            return Ok(());
        }
        //
        // If this is not the first time we have added data to this
        // segment, we need to be sure that all of the current reference
        // values are greater then the last reference value from the
        // previous addition of packets to the segment.
        //
        if (save.FTNPKT[save.INDEX] > 0) {
            if (save.FTREFS[[1, save.INDEX]] >= REFDAT[1]) {
                SETMSG(b"Reference values are out of order. The offending value, #, was found The reference values for explicitly to be out of order. indexed packets must be in increasing order, and the first reference value is less than or equal to the last reference value, #, from the previous addition of packets.", ctx);
                ERRDP(b"#", REFDAT[1], ctx);
                ERRDP(b"#", save.FTREFS[[1, save.INDEX]], ctx);
                SIGERR(b"SPICE(UNORDEREDREFS)", ctx)?;
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }
        }

        for I in 2..=NREFS {
            if (REFDAT[(I - 1)] >= REFDAT[I]) {
                SETMSG(b"Reference values are out of order. The offending value, #, was found to be out of order for index #. The reference values for explicitly indexed packets must be in increasing order.", ctx);
                ERRDP(b"#", REFDAT[(I - 1)], ctx);
                ERRINT(b"#", (I - 1), ctx);
                SIGERR(b"SPICE(UNORDEREDREFS)", ctx)?;
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }
        }
        //
        // Check the packet size to be sure that it is positive.
        //
        for I in 1..=NPKTS {
            if (PKTSIZ[I] <= 0) {
                SETMSG(b"The packet size for packet # was not positive. It had a value of #. All packet sizes must be greater then zero.", ctx);
                ERRINT(b"#", I, ctx);
                ERRINT(b"#", PKTSIZ[I], ctx);
                SIGERR(b"SPICE(NONPOSPACKETSIZE)", ctx)?;
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }
        }
        //
        // Add the packets preceded by their reference values and sizes to
        // the segment. We put the reference values with the packets so
        // that we do not need to open a scratch file. We will use them to
        // construct a reference directory after all of the packets have
        // been added to the segment.
        //
        PKTPOS = 1;

        for I in 1..=NPKTS {
            DPKSIZ = (PKTSIZ[I] as f64);

            DAFADA(REFDAT.subarray(I), 1, ctx)?;
            DAFADA(&[DPKSIZ], 1, ctx)?;
            DAFADA(PKTDAT.subarray(PKTPOS), PKTSIZ[I], ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }

            PKTPOS = (PKTPOS + PKTSIZ[I]);
            save.FTPKSZ[save.INDEX] = (save.FTPKSZ[save.INDEX] + PKTSIZ[I]);
            //
            // Remember the maximum packet size encountered.
            //
            if (PKTSIZ[I] > save.FTMXSZ[save.INDEX]) {
                save.FTMXSZ[save.INDEX] = PKTSIZ[I];
            }
        }
        //
        // Save the last reference value in the file table so that we
        // can use it to verify that the next addition does not violate
        // the increasing order of the reference values.
        //
        save.FTREFS[[1, save.INDEX]] = REFDAT[NREFS];
        //
        // Update the counts for the number of packets, the number of
        // references.
        //
        save.FTNPKT[save.INDEX] = (save.FTNPKT[save.INDEX] + NPKTS);
        save.FTNREF[save.INDEX] = (save.FTNREF[save.INDEX] + NREFS);
    } else {
        //
        // For implicitly indexed packets the number of reference values
        // must be two (2), and the second reference value must be an
        // integer, i.e., DINT(REFDAT(2)) .eq. REFDAT(2). The number of
        // reference values and the integrality of the second reference
        // value are checked only on the first call to add variable length
        // data packets to a generic segment. In all subsequent calls,
        // these arguments are ignored.
        //
        if (save.FTNPKT[save.INDEX] == 0) {
            if (NREFS != 2) {
                SETMSG(b"The number of reference values supplied, #, is not compatible with implicitly indexed packets. Implicitly indexed packets require the number of reference values to be two (2).", ctx);
                ERRINT(b"#", NREFS, ctx);
                SIGERR(b"SPICE(INCOMPATIBLENUMREF)", ctx)?;
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }

            if (f64::trunc(REFDAT[2]) != REFDAT[2]) {
                SETMSG(
                    b"For implicitly indexed packets the step size must be an integer.",
                    ctx,
                );
                SIGERR(b"SPICE(REFVALNOTINTEGER)", ctx)?;
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }
        }

        //
        // Add the packets to the segment preceded by the size of the
        // packet.
        //
        PKTPOS = 1;

        for I in 1..=NPKTS {
            DPKSIZ = (PKTSIZ[I] as f64);

            DAFADA(&[DPKSIZ], 1, ctx)?;
            DAFADA(PKTDAT.subarray(PKTPOS), PKTSIZ[I], ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGWVPK", ctx)?;
                return Ok(());
            }

            PKTPOS = (PKTPOS + PKTSIZ[I]);
            save.FTPKSZ[save.INDEX] = (save.FTPKSZ[save.INDEX] + PKTSIZ[I]);
        }
        //
        // Save the reference values and the number of reference values
        // in the file table. We only do this on the first time through
        // the routine.
        //
        if (save.FTNPKT[save.INDEX] == 0) {
            save.FTNREF[save.INDEX] = NREFS;
            save.FTREFS[[1, save.INDEX]] = REFDAT[1];
            save.FTREFS[[2, save.INDEX]] = REFDAT[2];
        }
        //
        // Update the counts for the number of packets.
        //
        save.FTNPKT[save.INDEX] = (save.FTNPKT[save.INDEX] + NPKTS);
    }

    CHKOUT(b"SGWVPK", ctx)?;
    Ok(())
}

/// Generic segments: End a segment.
///
/// End the generic segment in the DAF file associated with HANDLE.
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
///  HANDLE    I    Handle of a DAF file opened with write access.
/// ```
///
/// # Detailed Input
///
/// ```text
///  HANDLE   is the handle of a DAF file opened with write access.
///           This is the handle of the file which contains the generic
///           segment that we wish to end.
/// ```
///
/// # Parameters
///
/// ```text
///  This subroutine makes use of parameters defined in the file
///  'sgparam.inc'.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If there are no generic segments currently being written, the
///      error SPICE(CALLEDOUTOFORDER) is signaled.
///
///  2)  If there is no generic segment being written to the file
///      associated with HANDLE, the error SPICE(SEGMENTNOTFOUND) is
///      signaled.
/// ```
///
/// # Files
///
/// ```text
///  See HANDLE in the $Detailed_Input section.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine will end the generic segment started by a call to
///  either SGBWFS or SGBWVS that is currently being written to the DAF
///  file associated with HANDLE.
/// ```
///
/// # Examples
///
/// ```text
///  See the $Examples section in the header for the main subroutine.
///  It contains examples which demonstrate the use of the entry points
///  in the generic segments sequential writer. The entry points which
///  comprise the generic segments sequential writer must be used
///  together in the proper manner. Rather than repeating the examples
///  for each entry point they are provided in a single location.
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
/// -    SPICELIB Version 1.1.1, 03-JUN-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 1.1.0, 30-JUL-1996 (KRG) (NJB)
///
///         Fixed an annoying little bug in the variable segments code
///         when ending a segment. Rather than storing an appropriate
///         offset from the beginning of the segment as the packet
///         address in the packet directory, the absolute address, the
///         DAF address, was stored. This bug has been fixed.
///
///         The address calculations, see the variable MYADDR, were fixed.
///         This involved initializing the variable outside of the loop
///         that scans through the packet data and then incrementing this
///         variable in the same way as PKTADR.
///
///         The changes were made in two places, for the explicitly indexed
///         case and for the implicitly indexed case.
///
/// -    SPICELIB Version 1.0.0, 05-APR-1995 (KRG) (WLT)
/// ```
pub fn sgwes(ctx: &mut SpiceContext, handle: i32) -> crate::Result<()> {
    SGWES(handle, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure SGWES ( Generic segments: End a segment. )
pub fn SGWES(HANDLE: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut XMETA = StackArray::<f64, 17>::new(1..=MXMETA);
    let mut MYREF: f64 = 0.0;
    let mut MYADDR: f64 = 0.0;
    let mut MYSIZE: f64 = 0.0;
    let mut META = StackArray::<i32, 17>::new(1..=MXMETA);
    let mut PKTADR: i32 = 0;
    let mut REFADR: i32 = 0;
    let mut SIZE: i32 = 0;

    //
    // SPICELIB functions
    //
    // INTEGER               LASTNB
    // INTEGER               ISRCHI
    //
    // LOGICAL               FAILED
    // LOGICAL               RETURN
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"SGWES", ctx)?;
    //
    // Check to see if we have any fixed or variable segments being
    // written.
    //
    if (save.NFT == 0) {
        SETMSG(b"No segment is currently being written. This routine has been called out of order. One of the routines SGBWFS or SGBWVS must be called before his routine may be called.", ctx);
        SIGERR(b"SPICE(CALLEDOUTOFORDER)", ctx)?;
        CHKOUT(b"SGWES", ctx)?;
        return Ok(());
    }
    //
    // Check to see if the last handle used is the same as the current
    // handle. This saves us a table lookup to get the appropriate index
    // into the file table to restore the information for that handle.
    //
    if (HANDLE != save.LSTHAN) {
        save.INDEX = ISRCHI(HANDLE, save.NFT, save.FTHAN.as_slice());

        if (save.INDEX == 0) {
            SETMSG(b"No segment is associated with the file \'#\'. In order to write packets to a segment one of the routines SGBWFS or SGBWVS must be called to begin a segment.", ctx);
            ERRHAN(b"#", HANDLE, ctx)?;
            SIGERR(b"SPICE(SEGMENTNOTFOUND)", ctx)?;
            CHKOUT(b"SGWES", ctx)?;
            return Ok(());
        }

        save.EXPLCT = save.FTEXPL[save.INDEX];
        save.FXDSEG = save.FTFIXD[save.INDEX];
        save.LSTHAN = HANDLE;

        DAFCAD(HANDLE, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"SGWES", ctx)?;
            return Ok(());
        }
    }
    //
    // We need to do different things depending on whether the reference
    // values are implicitly or explicitly defined. We will also need to
    // treat the cases of fixed size packets and variable size packets
    // differently.
    //
    if save.EXPLCT {
        //
        // We have an explicit segment.
        //
        if save.FXDSEG {
            //
            // We need to do a little bit of work to finish this case off.
            // We know that we do not need a list of packet starting
            // addresses or a packet directory, but we do need to store in
            // a contiguous block the references and a reference directory
            // if the number of references is greater than DIRSIZ.
            //
            // We need to do the following things:
            //
            // 1) Initialize the offset of the packet data from the
            //    beginning of the packet, set the size of the packet, and
            //    set the beginning address of the packet data area in the
            //    segment.
            //
            SIZE = (save.FTOFF[save.INDEX] + save.FTPKSZ[save.INDEX]);
            REFADR = (save.FTBADR[save.INDEX] + save.FTNCON[save.INDEX]);
            //
            // 2) Collect all of the references stored with the packets
            //    when they were written, and copy them into the
            //    reference area.
            //
            for I in 1..=save.FTNPKT[save.INDEX] {
                DAFGDA(
                    HANDLE,
                    REFADR,
                    REFADR,
                    std::slice::from_mut(&mut MYREF),
                    ctx,
                )?;
                DAFADA(&[MYREF], 1, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGWES", ctx)?;
                    return Ok(());
                }

                REFADR = (REFADR + SIZE);
            }
            //
            // 3) Create a reference directory if the number of
            //    references is greater than DIRSIZ.
            //
            if (save.FTNREF[save.INDEX] > DIRSIZ) {
                REFADR = (save.FTBADR[save.INDEX] + save.FTNCON[save.INDEX]);
                REFADR = (((REFADR + (save.FTNPKT[save.INDEX] * SIZE)) + DIRSIZ) - 1);

                for I in 1..=((save.FTNREF[save.INDEX] - 1) / DIRSIZ) {
                    DAFGDA(
                        HANDLE,
                        REFADR,
                        REFADR,
                        std::slice::from_mut(&mut MYREF),
                        ctx,
                    )?;
                    DAFADA(&[MYREF], 1, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"SGWES", ctx)?;
                        return Ok(());
                    }

                    REFADR = (REFADR + DIRSIZ);
                }
            }
            //
            // 4) Construct the meta data for the segment.
            //
            SIZE = ((save.FTOFF[save.INDEX] + save.FTPKSZ[save.INDEX]) * save.FTNPKT[save.INDEX]);

            META[CONBAS] = 0;
            META[NCON] = save.FTNCON[save.INDEX];
            META[PKTBAS] = (META[CONBAS] + META[NCON]);
            META[NPKT] = save.FTNPKT[save.INDEX];
            META[PKTOFF] = save.FTOFF[save.INDEX];
            META[PDRBAS] = 0;
            META[NPDR] = 0;
            META[PDRTYP] = 0;
            META[REFBAS] = (META[PKTBAS] + SIZE);
            META[NREF] = save.FTNREF[save.INDEX];
            META[RDRBAS] = (META[REFBAS] + META[NREF]);
            META[NRDR] = ((save.FTNREF[save.INDEX] - 1) / DIRSIZ);
            META[RDRTYP] = save.FTITYP[save.INDEX];
            META[RSVBAS] = 0;
            META[NRSV] = 0;
            META[PKTSZ] = save.FTPKSZ[save.INDEX];
            META[NMETA] = MXMETA;
        } else {
            //
            // We need to do a little bit of work to finish this case off.
            // We know that we need a packet directory and we need to store
            // in a contiguous block the references and a reference
            // directory if the number of references is greater than
            // DIRSIZ.
            //
            // We need to do the following things:
            //
            // 1) Set the beginning address of the packet data area in the
            //    segment and initialize the address of the first data
            //    packet.
            //
            PKTADR = ((save.FTBADR[save.INDEX] + save.FTNCON[save.INDEX]) + save.FTOFF[save.INDEX]);
            MYADDR = ((save.FTOFF[save.INDEX] + 1) as f64);
            //
            // 2) Create a packet directory. The packet directory consists
            //    of the beginning addresses for each of the packets and a
            //    fake beginning for an extra packet so that we can easily
            //    compute the size of the last packet.
            //
            for I in 1..=save.FTNPKT[save.INDEX] {
                DAFGDA(
                    HANDLE,
                    (PKTADR - 1),
                    (PKTADR - 1),
                    std::slice::from_mut(&mut MYSIZE),
                    ctx,
                )?;
                DAFADA(&[MYADDR], 1, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGWES", ctx)?;
                    return Ok(());
                }

                SIZE = (MYSIZE as i32);
                PKTADR = ((PKTADR + SIZE) + save.FTOFF[save.INDEX]);
                MYADDR = (MYADDR + ((SIZE + save.FTOFF[save.INDEX]) as f64));
            }
            //
            // Put in the fake beginning for an extra packet. PKTADR should
            // contain the proper value.
            //
            MYADDR = (MYADDR as f64);

            DAFADA(&[MYADDR], 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGWES", ctx)?;
                return Ok(());
            }

            //
            // 3) Collect all of the references, stored with the packets
            //    when they were written, and copy them into the
            //    reference area.
            //
            REFADR = (save.FTBADR[save.INDEX] + save.FTNCON[save.INDEX]);

            for I in 1..=save.FTNPKT[save.INDEX] {
                DAFGDA(
                    HANDLE,
                    REFADR,
                    REFADR,
                    std::slice::from_mut(&mut MYREF),
                    ctx,
                )?;
                DAFGDA(
                    HANDLE,
                    (REFADR + 1),
                    (REFADR + 1),
                    std::slice::from_mut(&mut MYSIZE),
                    ctx,
                )?;
                DAFADA(&[MYREF], 1, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGWES", ctx)?;
                    return Ok(());
                }

                SIZE = (MYSIZE as i32);
                REFADR = ((REFADR + SIZE) + save.FTOFF[save.INDEX]);
            }
            //
            // 3) Create a reference directory if the number of
            //    references is greater than DIRSIZ. Note that we have one
            //    more packet directory item than we have data packets.
            //    This allows us to compute the size of the last data
            //    packet.
            //
            if (save.FTNREF[save.INDEX] > DIRSIZ) {
                REFADR = (save.FTBADR[save.INDEX] + save.FTNCON[save.INDEX]);
                REFADR = (REFADR + save.FTPKSZ[save.INDEX]);
                REFADR = (REFADR + (save.FTOFF[save.INDEX] * save.FTNPKT[save.INDEX]));
                REFADR = ((REFADR + save.FTNPKT[save.INDEX]) + 1);
                REFADR = ((REFADR + DIRSIZ) - 1);

                for I in 1..=((save.FTNREF[save.INDEX] - 1) / DIRSIZ) {
                    DAFGDA(
                        HANDLE,
                        REFADR,
                        REFADR,
                        std::slice::from_mut(&mut MYREF),
                        ctx,
                    )?;
                    DAFADA(&[MYREF], 1, ctx)?;

                    if FAILED(ctx) {
                        CHKOUT(b"SGWES", ctx)?;
                        return Ok(());
                    }

                    REFADR = (REFADR + DIRSIZ);
                }
            }
            //
            // 4) Construct the meta data for the segment.
            //
            META[CONBAS] = 0;
            META[NCON] = save.FTNCON[save.INDEX];
            META[PKTBAS] = (META[CONBAS] + META[NCON]);
            META[NPKT] = save.FTNPKT[save.INDEX];
            META[PKTOFF] = save.FTOFF[save.INDEX];
            META[PDRBAS] = ((META[PKTBAS] + save.FTPKSZ[save.INDEX])
                + (save.FTOFF[save.INDEX] * save.FTNPKT[save.INDEX]));
            META[NPDR] = (save.FTNPKT[save.INDEX] + 1);
            META[PDRTYP] = 1;
            META[REFBAS] = (META[PDRBAS] + META[NPDR]);
            META[NREF] = save.FTNREF[save.INDEX];
            META[RDRBAS] = (META[REFBAS] + META[NREF]);
            META[NRDR] = ((save.FTNREF[save.INDEX] - 1) / DIRSIZ);
            META[RDRTYP] = save.FTITYP[save.INDEX];
            META[RSVBAS] = 0;
            META[NRSV] = 0;
            META[PKTSZ] = save.FTMXSZ[save.INDEX];
            META[NMETA] = MXMETA;
        }
    } else {
        //
        // We have an implicitly indexed segment.
        //
        if save.FXDSEG {
            //
            // There is no packet directory, so we just write the reference
            // values. There is no reference directory either, because
            // implicitly indexed packets only have two (2) reference
            // values.
            //
            DAFADA(
                save.FTREFS.subarray([1, save.INDEX]),
                save.FTNREF[save.INDEX],
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"SGWES", ctx)?;
                return Ok(());
            }
            //
            // Now we need to construct the meta data for this segment. We
            // will write it to the file a bit later.
            //
            SIZE = ((save.FTOFF[save.INDEX] + save.FTPKSZ[save.INDEX]) * save.FTNPKT[save.INDEX]);

            META[CONBAS] = 0;
            META[NCON] = save.FTNCON[save.INDEX];
            META[PKTBAS] = (META[CONBAS] + META[NCON]);
            META[NPKT] = save.FTNPKT[save.INDEX];
            META[PKTOFF] = save.FTOFF[save.INDEX];
            META[PDRBAS] = 0;
            META[NPDR] = 0;
            META[PDRTYP] = 0;
            META[REFBAS] = (META[PKTBAS] + SIZE);
            META[NREF] = save.FTNREF[save.INDEX];
            META[RDRBAS] = (META[REFBAS] + META[NREF]);
            META[NRDR] = 0;
            META[RDRTYP] = save.FTITYP[save.INDEX];
            META[RSVBAS] = 0;
            META[NRSV] = 0;
            META[PKTSZ] = save.FTPKSZ[save.INDEX];
            META[NMETA] = MXMETA;
        } else {
            //
            // We need to do a little bit of work to finish this case off.
            // We know that we need a packet directory, but we do not need
            // a reference directory.
            //
            // We need to do the following things:
            //
            // 1) Set the beginning address of the packet data area in the
            //    segment and initialize the address of the first data
            //    packet.
            //
            PKTADR = ((save.FTBADR[save.INDEX] + save.FTNCON[save.INDEX]) + save.FTOFF[save.INDEX]);
            MYADDR = ((save.FTOFF[save.INDEX] + 1) as f64);
            //
            // 2) Create a packet directory. The packet directory consists
            //    of the beginning addresses for each of the packets and a
            //    fake beginning for an extra packet so that we can easily
            //    compute the size of the last packet.
            //
            for I in 1..=save.FTNPKT[save.INDEX] {
                DAFGDA(
                    HANDLE,
                    (PKTADR - 1),
                    (PKTADR - 1),
                    std::slice::from_mut(&mut MYSIZE),
                    ctx,
                )?;
                DAFADA(&[MYADDR], 1, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"SGWES", ctx)?;
                    return Ok(());
                }

                SIZE = (MYSIZE as i32);
                PKTADR = ((PKTADR + SIZE) + save.FTOFF[save.INDEX]);
                MYADDR = (MYADDR + ((SIZE + save.FTOFF[save.INDEX]) as f64));
            }
            //
            // Put in the fake beginning for an extra packet. PKTADR should
            // contain the proper value.
            //
            MYADDR = ((PKTADR - save.FTBADR[save.INDEX]) as f64);

            DAFADA(&[MYADDR], 1, ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"SGWES", ctx)?;
                return Ok(());
            }
            //
            // 3) Construct the meta data for the segment.
            //
            META[CONBAS] = 0;
            META[NCON] = save.FTNCON[save.INDEX];
            META[PKTBAS] = (META[CONBAS] + META[NCON]);
            META[NPKT] = save.FTNPKT[save.INDEX];
            META[PKTOFF] = save.FTOFF[save.INDEX];
            META[PDRBAS] = ((META[PKTBAS] + save.FTPKSZ[save.INDEX])
                + (save.FTOFF[save.INDEX] * save.FTNPKT[save.INDEX]));
            META[NPDR] = (save.FTNPKT[save.INDEX] + 1);
            META[PDRTYP] = 1;
            META[REFBAS] = (META[PDRBAS] + META[NPDR]);
            META[NREF] = save.FTNREF[save.INDEX];
            META[RDRBAS] = (META[REFBAS] + META[NREF]);
            META[NRDR] = 0;
            META[RDRTYP] = save.FTITYP[save.INDEX];
            META[RSVBAS] = 0;
            META[NRSV] = 0;
            META[PKTSZ] = save.FTMXSZ[save.INDEX];
            META[NMETA] = MXMETA;
        }
    }
    //
    // Write the meta data to the segment and end the segment.
    //
    for I in 1..=MXMETA {
        XMETA[I] = (META[I] as f64);
    }

    DAFADA(XMETA.as_slice(), MXMETA, ctx)?;
    //
    // End the segment.
    //
    DAFENA(ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"SGWES", ctx)?;
        return Ok(());
    }
    //
    // Now we need to clean up after ourselves, removing the information
    // for the segment we just ended from the file table.
    //
    save.NFT = (save.NFT - 1);

    for I in save.INDEX..=save.NFT {
        save.FTBADR[I] = save.FTBADR[(I + 1)];
        save.FTHAN[I] = save.FTHAN[(I + 1)];
        save.FTITYP[I] = save.FTITYP[(I + 1)];
        save.FTNCON[I] = save.FTNCON[(I + 1)];
        save.FTNPKT[I] = save.FTNPKT[(I + 1)];
        save.FTNREF[I] = save.FTNREF[(I + 1)];
        save.FTNRES[I] = save.FTNRES[(I + 1)];
        save.FTOFF[I] = save.FTOFF[(I + 1)];
        save.FTPKSZ[I] = save.FTPKSZ[(I + 1)];
        save.FTFIXD[I] = save.FTFIXD[(I + 1)];
        save.FTEXPL[I] = save.FTEXPL[(I + 1)];
    }

    if save.FXDSEG {
        save.NUMFXD = (save.NUMFXD - 1);
    } else {
        save.NUMVAR = (save.NUMVAR - 1);
    }

    CHKOUT(b"SGWES", ctx)?;
    Ok(())
}
