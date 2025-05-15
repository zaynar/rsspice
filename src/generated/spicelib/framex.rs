//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const NINERT: i32 = 21;
const NNINRT: i32 = 124;
const INERTL: i32 = 1;
const PCK: i32 = (INERTL + 1);
const CK: i32 = (PCK + 1);
const TK: i32 = (CK + 1);
const DYN: i32 = (TK + 1);
const SWTCH: i32 = (DYN + 1);
const ALL: i32 = -1;
const CTRSIZ: i32 = 2;
const NON: i32 = NNINRT;
const NCOUNT: i32 = (NINERT + NON);
const ROOM: i32 = 8;
const LNSIZE: i32 = 80;
const BDNMLN: i32 = 36;
const FRNMLN: i32 = 32;
const KVNMLN: i32 = 32;
const KVBSZ: i32 = 100;
const LBPOOL: i32 = -5;
const MAXKFR: i32 = 5209;
const MAXBFR: i32 = 149;

struct SaveVars {
    DATTYP: Vec<u8>,
    LCNAME: Vec<u8>,
    LCFRAM: Vec<u8>,
    NAME: ActualCharArray,
    PNAME: Vec<u8>,
    KVBUFF: ActualCharArray,
    LOOK2: Vec<u8>,
    LOOKUP: Vec<u8>,
    LINE: ActualCharArray,
    CENTER: StackArray<i32, 145>,
    CENTRD: StackArray<i32, 145>,
    ID: i32,
    IDCODE: StackArray<i32, 145>,
    ITEM: i32,
    KVCLID: i32,
    KVCLSS: i32,
    N: i32,
    START: i32,
    TYPE: StackArray<i32, 145>,
    TYPEID: StackArray<i32, 145>,
    VALUES: StackArray<i32, 8>,
    FIRST: bool,
    FND: bool,
    GOTIT: bool,
    PULCTR: StackArray<i32, 2>,
    KNMLST: ActualArray<i32>,
    KNMPOL: ActualArray<i32>,
    KNMNMS: ActualCharArray,
    KNMIDS: ActualArray<i32>,
    KIDLST: ActualArray<i32>,
    KIDPOL: ActualArray<i32>,
    KIDIDS: ActualArray<i32>,
    KNAME: ActualCharArray,
    KCENT: ActualArray<i32>,
    KCLASS: ActualArray<i32>,
    KCLSID: ActualArray<i32>,
    LUPDTE: bool,
    LNEW: bool,
    BNMLST: StackArray<i32, 149>,
    BNMPOL: StackArray<i32, 155>,
    BNMNMS: ActualCharArray,
    BNMIDX: StackArray<i32, 149>,
    BIDLST: StackArray<i32, 149>,
    BIDPOL: StackArray<i32, 155>,
    BIDIDS: StackArray<i32, 149>,
    BIDIDX: StackArray<i32, 149>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut DATTYP = vec![b' '; 1 as usize];
        let mut LCNAME = vec![b' '; BDNMLN as usize];
        let mut LCFRAM = vec![b' '; FRNMLN as usize];
        let mut NAME = ActualCharArray::new(FRNMLN, 1..=NCOUNT);
        let mut PNAME = vec![b' '; FRNMLN as usize];
        let mut KVBUFF = ActualCharArray::new(KVNMLN, 1..=KVBSZ);
        let mut LOOK2 = vec![b' '; KVNMLN as usize];
        let mut LOOKUP = vec![b' '; KVNMLN as usize];
        let mut LINE = ActualCharArray::new(LNSIZE, 1..=ROOM);
        let mut CENTER = StackArray::<i32, 145>::new(1..=NCOUNT);
        let mut CENTRD = StackArray::<i32, 145>::new(1..=NCOUNT);
        let mut ID: i32 = 0;
        let mut IDCODE = StackArray::<i32, 145>::new(1..=NCOUNT);
        let mut ITEM: i32 = 0;
        let mut KVCLID: i32 = 0;
        let mut KVCLSS: i32 = 0;
        let mut N: i32 = 0;
        let mut START: i32 = 0;
        let mut TYPE = StackArray::<i32, 145>::new(1..=NCOUNT);
        let mut TYPEID = StackArray::<i32, 145>::new(1..=NCOUNT);
        let mut VALUES = StackArray::<i32, 8>::new(1..=ROOM);
        let mut FIRST: bool = false;
        let mut FND: bool = false;
        let mut GOTIT: bool = false;
        let mut PULCTR = StackArray::<i32, 2>::new(1..=CTRSIZ);
        let mut KNMLST = ActualArray::<i32>::new(1..=MAXKFR);
        let mut KNMPOL = ActualArray::<i32>::new(LBPOOL..=MAXKFR);
        let mut KNMNMS = ActualCharArray::new(FRNMLN, 1..=MAXKFR);
        let mut KNMIDS = ActualArray::<i32>::new(1..=MAXKFR);
        let mut KIDLST = ActualArray::<i32>::new(1..=MAXKFR);
        let mut KIDPOL = ActualArray::<i32>::new(LBPOOL..=MAXKFR);
        let mut KIDIDS = ActualArray::<i32>::new(1..=MAXKFR);
        let mut KNAME = ActualCharArray::new(FRNMLN, 1..=MAXKFR);
        let mut KCENT = ActualArray::<i32>::new(1..=MAXKFR);
        let mut KCLASS = ActualArray::<i32>::new(1..=MAXKFR);
        let mut KCLSID = ActualArray::<i32>::new(1..=MAXKFR);
        let mut LUPDTE: bool = false;
        let mut LNEW: bool = false;
        let mut BNMLST = StackArray::<i32, 149>::new(1..=MAXBFR);
        let mut BNMPOL = StackArray::<i32, 155>::new(LBPOOL..=MAXBFR);
        let mut BNMNMS = ActualCharArray::new(FRNMLN, 1..=MAXBFR);
        let mut BNMIDX = StackArray::<i32, 149>::new(1..=MAXBFR);
        let mut BIDLST = StackArray::<i32, 149>::new(1..=MAXBFR);
        let mut BIDPOL = StackArray::<i32, 155>::new(LBPOOL..=MAXBFR);
        let mut BIDIDS = StackArray::<i32, 149>::new(1..=MAXBFR);
        let mut BIDIDX = StackArray::<i32, 149>::new(1..=MAXBFR);

        FIRST = true;

        Self {
            DATTYP,
            LCNAME,
            LCFRAM,
            NAME,
            PNAME,
            KVBUFF,
            LOOK2,
            LOOKUP,
            LINE,
            CENTER,
            CENTRD,
            ID,
            IDCODE,
            ITEM,
            KVCLID,
            KVCLSS,
            N,
            START,
            TYPE,
            TYPEID,
            VALUES,
            FIRST,
            FND,
            GOTIT,
            PULCTR,
            KNMLST,
            KNMPOL,
            KNMNMS,
            KNMIDS,
            KIDLST,
            KIDPOL,
            KIDIDS,
            KNAME,
            KCENT,
            KCLASS,
            KCLSID,
            LUPDTE,
            LNEW,
            BNMLST,
            BNMPOL,
            BNMNMS,
            BNMIDX,
            BIDLST,
            BIDPOL,
            BIDIDS,
            BIDIDX,
        }
    }
}

/// FRAMe EXpert
///
/// This is an umbrella routine for the entry points available for
/// manipulating different reference frames. It should not be called
/// directly.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  ENTRY POINT
///  --------  ---  --------------------------------------------------
///  CNAME      I   CNMFRM
///  FRNAME    I-O  NAMFRM, FRMNAM, CCIFRM
///  FRCODE    I-O  NAMFRM, FRMNAM, FRINFO, CIDFRM, CCIFRM
///  CENT      I-O  FRINFO, CIDFRM, CCIFRM
///  FRCLSS    I-O  FRINFO, CCIFRM
///  CLSSID    I-O  FRINFO, CCIFRM
///  FOUND      O   FRINFO
/// ```
///
/// # Detailed Input
///
/// ```text
///  See individual entry points for details concerning inputs.
/// ```
///
/// # Detailed Output
///
/// ```text
///  See individual entry points for details concerning inputs.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If this routine is called directly, the error
///      SPICE(BOGUSENTRY) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This is an umbrella routine that comprises the SPICE
///  interface to the reference frame transformation software.
///
///  There are 6 entry points.
///
///  NAMFRM  converts string to the ID codes used by low level
///          SPICE software
///
///  FRMNAM  converts frame ID codes to the more familiar names
///          used to describe various reference frames.
///
///  FRINFO  returns the center associated with a reference frame.
///
///  CIDFRM  given the ID code of an object, returns the bodyfixed
///          frame associated with it.
///
///  CNMFRM  given the name of an object, returns the bodyfixed
///          frame associated with it.
///
///  CCIFRM  given a frame's class and class ID, returns
///          the frame's ID code, name, and center.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you needed to transform between two reference
///  frames on the basis of their names and that you wanted to
///  correct for light time to the center of the second frame
///  as seen from an observer with ID code OBS.
///
///  The code fragment below illustrates how you could use the
///  entry points gathered in this routine to retrieve the
///  state transformation matrix.
///
///
///     First convert names to frame ID codes.
///
///     CHARACTER*(32)        NAME1
///     CHARACTER*(32)        NAME2
///
///     INTEGER               FRAME1
///     INTEGER               FRAME2
///     INTEGER               CENT
///     INTEGER               OBS
///
///     DOUBLE PRECISION      ET
///     DOUBLE PRECISION      LT
///
///     DOUBLE PRECISION      STATE ( 6 )
///     DOUBLE PRECISION      XFORM ( 6, 6 )
///
///
///     First we use the entry points NAMFRM to convert the frame
///     names to ID codes.
///
///     CALL NAMFRM ( NAME1, FRAME1 )
///     CALL NAMFRM ( NAME2, FRAME2 )
///
///     Next we determine the center of the second frame
///
///     CALL FRINFO ( FRAME2, CENT, FRCLSS, CLSSID, FOUND )
///
///     Determine the light time to the center of the second frame.
///
///     CALL SPKGEO ( CENT,  ET, 'J2000',  OBS, STATE, LT )
///
///     Finally get the state transformation from FRAME1 to FRAME2
///     at time ET - LT
///
///     CALL FRMCHG ( FRAME1, FRAME2, ET-LT, XFORM )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 5.3.0, 26-AUG-2021 (JDR)
///
///         Changed the argument name CLASS to FRCLSS in FRINFO and CCIFRM
///         entry points for consistency with other routines.
///
///         Edited the header of the umbrella routine and all its entry
///         points to comply with NAIF standard. Updated the $Examples
///         section of CCIFRM entry point.
///
///         Updated frame name length from 26 to 32 in example code.
///
///         Increased MAXBFR from 127 to 149 to accommodate additional
///         built-in PCK based frames.
///
/// -    SPICELIB Version 5.2.1, 02-FEB-2017 (BVS)
///
///         Shortened one of permuted index entries in CCIFRM.
///
/// -    SPICELIB Version 5.2.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
///         Bug fix: CCIFRM entry point logic was corrected to examine the
///         built-in frames before looking at the kernel POOL frames.
///
/// -    SPICELIB Version 5.1.1, 09-FEB-2011 (NJB)
///
///         Bug fix: corrected logic in entry point CIDFRM for
///         object-frame association for case where the assigned frame
///         value is denoted by a frame code.
///
///         Fixed typo in FRAMEX header.
///
/// -    SPICELIB Version 5.0.1, 17-MAR-2009 (EDW)
///
///         Entry point NAMFRM: Typo correction in $Required_Reading,
///         changed FRAME to FRAMES.
///
/// -    SPICELIB Version 5.0.0, 05-NOV-2007 (NJB)
///
///         Entry point CCIFRM (map frame class and class ID
///         to frame ID code, name, and center) has been added.
///
/// -    SPICELIB Version 4.0.0, 13-SEP-2005 (NJB)
///
///         Entry point FRINFO is no longer error-free. Various frame
///         definition errors that were previously ignored are now
///         diagnosed.
///
///         Entry point FRINFO has been updated to support specification
///         of frame center by name or ID code. Previously only ID codes
///         could be used to identify frame centers.
///
/// -    SPICELIB Version 3.2.0, 20-DEC-2004 (BVS)
///
///         Added parameter incorporating maximum body name length and set
///         it to the same value as MAXL from zzbodtrn.inc. Used this
///         parameter to declare local variable that holds frame center
///         name (LCNAME).
///
///         In FRINFO entry: removed special handling of the frame IDs
///         less than -999. If they cannot be ``resolved'' using kernel
///         pool keywords, the frame is NOT declared CK-based with center
///         ID derived by dividing frame ID by a 1000 and class ID
///         assigned the frame ID anymore. In the current practice with
///         multitude of TK frames with IDs set instrument IDs this
///         default behavior is simply not valid.
///
/// -    SPICELIB Version 3.1.0, 28-NOV-2002 (NJB)
///
///         Bug fix: updated CNMFRM so a TK frame specified by name and
///         designated as an object's preferred frame via kernel pool
///         assignments is found, and so that the correct name of this
///         frame is returned.
///
/// -    SPICELIB Version 3.0.1, 25-JUN-1999 (WLT)
///
///         Extended documentation of entry point CNMFRM and
///         corrected example for that entry point.
///
/// -    SPICELIB Version 3.0.0, 03-JUN-1997 (WLT)
///
///         The entry points CIDFRM and CNMFRM were added so that
///         user's may determine the frame-id and name to associated
///         with a planetary object.
///
/// -    SPICELIB Version 2.0.0, 04-APR-1997 (WLT)
///
///         The routine was upgraded to reflect that a block of
///         frame ID codes have been reserved for use by the DSN.
///         ID codes 13001 to 13999 have been set aside for DSN
///         models for the orientation of the earth. These frames
///         are all PCK frames. Moreover, the PCK ID code to
///         use with these frames is simply the Frame-Code minus 10000.
///         All of these frames are centered at the earth (body 399).
///
/// -    SPICELIB Version 1.1.0, 14-OCT-1996 (WLT)
///
///         The values NINERT and NNINRT are included instead of
///         being declared locally.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (WLT)
/// ```
pub fn framex(
    ctx: &mut SpiceContext,
    cname: &str,
    frname: &str,
    frcode: i32,
    cent: i32,
    frclss: i32,
    clssid: i32,
    found: bool,
) -> crate::Result<()> {
    FRAMEX(
        cname.as_bytes(),
        frname.as_bytes(),
        frcode,
        cent,
        frclss,
        clssid,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FRAMEX ( FRAMe EXpert )
pub fn FRAMEX(
    CNAME: &[u8],
    FRNAME: &[u8],
    FRCODE: i32,
    CENT: i32,
    FRCLSS: i32,
    CLSSID: i32,
    FOUND: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    //
    // SPICELIB Functions
    //

    //
    // Local parameters
    //

    //
    // Body name length. The value BDNMLN used here must be the
    // same as the value of MAXL defined in the INCLUDE file
    //
    //    zzbodtrn.inc
    //
    // Current value of MAXL = 36.
    //

    //
    // Frame name length.
    //

    //
    // Kernel variable name length.
    //

    //
    // Kernel variable buffer size.
    //

    //
    // Local Variables
    //

    //
    // POOL state counter.
    //

    //
    // Lower bound of collision lists in hashes.
    //

    //
    // The size of ID-based hash for kernel POOL frames.
    //
    // Since defining a valid kernel POOL frame takes at least 5
    // keywords and integer hash dimension must be a prime number, this
    // size should be set to the first prime number greater than POOL's
    // MAXVAR / 5 + 1.
    //
    // For the current POOL MAXVAR set to 26003, such number is 5209.
    //

    //
    // Name-based hash for kernel pool frames. KNMLST, KNMPOL, and
    // KNMNMS provide an index in the frame ID array KNMIDS at which the
    // ID for the frame with a given name is stored.
    //

    //
    // ID-based hash for kernel pool frames. KIDLST, KIDPOL, and KIDIDS
    // provide the index in the kernel frame attribute arrays KNAME,
    // KCENT, KCLASS, and KCLSID at which the attributes of the frame
    // with a given ID are stored.
    //

    //
    // The size of hashes for built-in frames.
    //
    // Since integer hash dimension must be a prime number it cannot be
    // computed in a parameter statement from the inertial and
    // non-inertial frame counts provided in the include files. Instead
    // it should be set manually to the first prime number greater than
    // or equal to NCOUNT.
    //
    // For the current N0067 NCOUNT equal to 145 (21 inertial + 124
    // non-inertial), such number is 149.
    //

    //
    // Name-based hash for built-in frames. BNMLST, BNMPOL, and BNMNMS
    // provide the index in BNMIDX which stores the index for the frame
    // attributes in the built-in frame attributes arrays IDCODE, NAME,
    // CENTER, TYPE, and TYPEID.
    //

    //
    // ID-based hash for built-in frames. BIDLST, BIDPOL, and BIDIDS
    // provide an index in BIDIDX which stores the index for the frame
    // attributes in the built-in frame attributes arrays IDCODE, NAME,
    // CENTER, TYPE, and TYPEID.
    //

    //
    // Saved variables
    //
    // Because we need to save almost everything we save everything
    // rather than taking a chance and accidentally leaving something
    // off the list.
    //

    //
    // Initial values
    //

    CHKIN(b"FRAMEX", ctx)?;
    SETMSG(b"A call has been made to the umbrella routine FRAMEX. This routine doesn\'t do anything. It acts only as an umbrella routine for its entry points. This call probably indicates a misunderstanding in programming. ", ctx);
    SIGERR(b"SPICE(BOGUSENTRY)", ctx)?;
    CHKOUT(b"FRAMEX", ctx)?;
    Ok(())
}

/// frame NAMe to FRaMe id
///
/// Look up the frame ID code associated with a string.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FRNAME     I   The name of some reference frame
///  FRCODE     O   The SPICE ID code of the frame.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FRNAME   is a character string that stands for some
///           reference frame (either inertial or non-inertial).
///
///           Leading blanks in FRNAME are ignored. And the
///           case of the letters in FRNAME are insignificant.
///
///           Note that all legitimate frame names contain
///           26 or fewer characters.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FRCODE   is the SPICE integer code used for internal
///           representation of the named reference frame.
///
///           If the name input through FRNAME is not recognized
///           FRCODE will be returned with a value of zero.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the input name is not recognized, FRCODE will be
///      returned with a value of 0.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a low level interface routine intended primarily for
///  use within the SPK and CK systems to assist in the transformation
///  to user specified reference frames.
///
///  The routine first consults a stored list of reference frame
///  names in an attempt to determine the appropriate reference
///  frame code.
///
///  If this search is unsuccessful, the routine then examines the
///  kernel pool to determine whether or not a variable of the
///  form
///
///     'FRAME_' // FRNAME
///
///     (where leading blanks of FRNAME are ignored)
///
///  is present. If it is and the number of values associated with the
///  name is 1, this value is taken to be the frame ID code.
///
///  Note: It is NOT possible to override the default names and
///  ID codes stored locally in this routine by placing an
///  appropriately variable in the kernel pool with a different
///  ID code. The predefined values always take precedence.
///
///  Consult the FRAMES required reading document for more details
///  about constructing your own frame definitions.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you needed to find the SPICE ID code for the
///  bodyfixed reference frame for Mars as modeled by the
///  IAU cartographic working group. Use the following code
///  to perform this task.
///
///     CALL NAMFRM ( 'IAU_MARS', FRCODE )
///
///     WRITE (*,*) 'The SPICE code for the Mars bodyfixed frame is: ',
///    .             FRCODE.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.1, 02-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.1.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
/// -    SPICELIB Version 3.0.2, 17-MAR-2009 (EDW)
///
///         Typo correction in $Required_Reading, changed FRAME to FRAMES.
///
/// -    SPICELIB Version 3.0.1, 25-JUN-1999 (WLT)
///
///         Extended documentation of entry point CNMFRM and
///         corrected example for that entry point.
///
/// -    SPICELIB Version 3.0.0, 03-JUN-1997 (WLT)
///
///         The entry points CIDFRM and CNMFRM were added so that
///         user's may determine the frame-id and name to associated
///         with a planetary object.
///
/// -    SPICELIB Version 2.0.0, 04-APR-1997 (WLT)
///
///         The routine was upgraded to reflect that a block of
///         frame ID codes have been reserved for use by the DSN.
///         ID codes 13001 to 13999 have been set aside for DSN
///         models for the orientation of the earth. These frames
///         are all PCK frames. Moreover, the PCK ID code to
///         use with these frames is simply the Frame-Code minus 10000.
///         All of these frames are centered at the earth (body 399).
///
/// -    SPICELIB Version 1.1.0, 14-OCT-1996 (WLT)
///
///         The values NINERT and NNINRT are included instead of
///         being declared locally.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (WLT)
/// ```
pub fn namfrm(ctx: &mut SpiceContext, frname: &str, frcode: &mut i32) -> crate::Result<()> {
    NAMFRM(frname.as_bytes(), frcode, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure NAMFRM ( frame NAMe to FRaMe id )
pub fn NAMFRM(FRNAME: &[u8], FRCODE: &mut i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *FRCODE = 0;
    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    //
    // For efficiency, J2000 deserves special treatment.
    //
    if (fstr::eq(FRNAME, b"J2000") || fstr::eq(FRNAME, b"j2000")) {
        *FRCODE = 1;
        return Ok(());
    }

    CHKIN(b"NAMFRM", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXKFR,
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            ctx,
        )?;
        ZZHSCINI(
            MAXKFR,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Initialize built-in frame tables and hashes.
        //
        ZZFDAT(
            NCOUNT,
            MAXBFR,
            save.NAME.as_arg_mut(),
            save.IDCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.TYPE.as_slice_mut(),
            save.TYPEID.as_slice_mut(),
            save.CENTRD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"NAMFRM", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // Determine the location of the requested item in the array
    // of names.
    //
    LJUST(FRNAME, &mut save.PNAME);
    UCASE(&save.PNAME.to_vec(), &mut save.PNAME, ctx);

    ZZHSCCHK(
        save.BNMLST.as_slice(),
        save.BNMPOL.as_slice(),
        save.BNMNMS.as_arg(),
        &save.PNAME,
        &mut save.ITEM,
        ctx,
    )?;
    if (save.ITEM != 0) {
        save.ITEM = save.BNMIDX[save.ITEM];
    }

    //
    // If the name is in our hash, we can just look up its ID code in
    // the parallel array.
    //
    if (save.ITEM > 0) {
        *FRCODE = save.IDCODE[save.ITEM];
    } else {
        //
        // See if this frame is in the kernel pool frame name-based hash.
        // First reset the hash if POOL has changed.
        //
        ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

        if save.LUPDTE {
            ZZHSCINI(
                MAXKFR,
                save.KNMLST.as_slice_mut(),
                save.KNMPOL.as_slice_mut(),
                ctx,
            )?;
            ZZHSIINI(
                MAXKFR,
                save.KIDLST.as_slice_mut(),
                save.KIDPOL.as_slice_mut(),
                ctx,
            )?;
        }

        //
        // Check if this name is in the hash.
        //
        ZZHSCCHK(
            save.KNMLST.as_slice(),
            save.KNMPOL.as_slice(),
            save.KNMNMS.as_arg(),
            &save.PNAME,
            &mut save.ITEM,
            ctx,
        )?;

        if (save.ITEM != 0) {
            *FRCODE = save.KNMIDS[save.ITEM];
        } else {
            //
            // The name wasn't in the hash, see if we can find this frame
            // in the kernel pool.
            //
            PREFIX(b"FRAME_", 0, &mut save.PNAME);
            GIPOOL(
                &save.PNAME,
                1,
                ROOM,
                &mut save.N,
                save.VALUES.as_slice_mut(),
                &mut save.GOTIT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"NAMFRM", ctx)?;
                return Ok(());
            }

            if ((save.N == 1) && save.GOTIT) {
                *FRCODE = save.VALUES[1];

                //
                // If we made it to this point, we successfully mapped the
                // kernel frame name to its ID. Add this pair to the
                // name-based hash.
                //
                ZZHSCADD(
                    save.KNMLST.as_slice_mut(),
                    save.KNMPOL.as_slice_mut(),
                    save.KNMNMS.as_arg_mut(),
                    &save.PNAME,
                    &mut save.ITEM,
                    &mut save.LNEW,
                    ctx,
                )?;

                if (!FAILED(ctx) && (save.ITEM != 0)) {
                    save.KNMIDS[save.ITEM] = *FRCODE;
                }
            } else {
                *FRCODE = 0;
            }
        }
    }

    CHKOUT(b"NAMFRM", ctx)?;
    Ok(())
}

/// FRaMe id to frame NAMe
///
/// Retrieve the name of a reference frame associated with a SPICE ID
/// code.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FRCODE     I   an integer code for a reference frame
///  FRNAME     O   the name associated with the reference frame.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FRCODE   is an integer code for a reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FRNAME   is the name associated with the reference frame. It will
///           be returned left justified.
///
///           If FRCODE is not recognized as the name of a known
///           reference frame, FRNAME will be returned as a blank.
///
///           If FRNAME is not sufficiently long to hold the name, it
///           will be truncated on the right.
///
///           All reference frame names are 26 or fewer characters in
///           length. Thus declaring FRNAME to be CHARACTER*(26) will
///           ensure that the returned name will not be truncated.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If FRCODE is not recognized as the name of a known reference
///      frame, FRNAME will be returned as a blank.
///
///  2)  If FRNAME is not sufficiently long to hold the name, it will
///      be truncated on the right.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine retrieves the name of a reference frame associated
///  with a SPICE frame ID code.
///
///  The ID codes stored locally are scanned for a match with FRCODE.
///  If a match is found, the name stored locally will be returned
///  as the name for the frame.
///
///  If FRCODE is not a member of the list of internally stored
///  ID codes, the kernel pool will be examined to see if the
///  variable
///
///     FRAME_idcode_NAME
///
///  is present (where idcode is the decimal character equivalent
///  of FRCODE). If the variable is located and it has both
///  character type and dimension 1, the string value of the
///  kernel pool variable is returned as the name of the reference
///  frame.
///
///  Note that because the local information is always examined
///  first and searches of the kernel pool are performed only
///  after exhausting local information, it is not possible to
///  override the local name for any reference frame that is
///  known by this routine.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you needed to create a message concerning a reference
///  frame and wish to use the name of the frame in the message.
///  Suppose further that you have only the frame ID code at your
///  disposal. You can capture the frame name using this routine
///  as shown here.
///
///     CHARACTER*(32)        FRNAME
///
///     CALL FRMNAM ( FRCODE, FRNAME )
///
///     IF ( FRNAME .EQ. ' ' ) THEN
///        CALL INTSTR ( FRCODE, FRNAME )
///     END IF
///
///     WRITE (*,*) 'Concerning reference frame:', FRNAME
///
///     print the rest of your message.
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.1, 02-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard. Updated maximum
///         frame name length in detailed description of FRNAME argument
///         and changed frame name length from 26 to 32 in example code.
///
/// -    SPICELIB Version 3.1.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
/// -    SPICELIB Version 3.0.1, 25-JUN-1999 (WLT)
///
///         Extended documentation of entry point CNMFRM and
///         corrected example for that entry point.
///
/// -    SPICELIB Version 3.0.0, 03-JUN-1997 (WLT)
///
///         The entry points CIDFRM and CNMFRM were added so that
///         user's may determine the frame-id and name to associated
///         with a planetary object.
///
/// -    SPICELIB Version 2.0.0, 04-APR-1997 (WLT)
///
///         The routine was upgraded to reflect that a block of
///         frame ID codes have been reserved for use by the DSN.
///         ID codes 13001 to 13999 have been set aside for DSN
///         models for the orientation of the earth. These frames
///         are all PCK frames. Moreover, the PCK ID code to
///         use with these frames is simply the Frame-Code minus 10000.
///         All of these frames are centered at the earth (body 399).
///
/// -    SPICELIB Version 1.1.0, 14-OCT-1996 (WLT)
///
///         The values NINERT and NNINRT are included instead of
///         being declared locally.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (WLT)
/// ```
pub fn frmnam(ctx: &mut SpiceContext, frcode: i32, frname: &mut str) -> crate::Result<()> {
    FRMNAM(
        frcode,
        fstr::StrBytes::new(frname).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FRMNAM ( FRaMe id to frame NAMe )
pub fn FRMNAM(FRCODE: i32, FRNAME: &mut [u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // Standard SPICE error handling.
    //
    fstr::assign(FRNAME, b" ");

    if RETURN(ctx) {
        return Ok(());
    }

    //
    // For efficiency, J2000 deserves special treatment.
    //
    if (FRCODE == 1) {
        fstr::assign(FRNAME, b"J2000");
        return Ok(());
    }

    CHKIN(b"FRMNAM", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXKFR,
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            ctx,
        )?;
        ZZHSCINI(
            MAXKFR,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Initialize built-in frame tables and hashes.
        //
        ZZFDAT(
            NCOUNT,
            MAXBFR,
            save.NAME.as_arg_mut(),
            save.IDCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.TYPE.as_slice_mut(),
            save.TYPEID.as_slice_mut(),
            save.CENTRD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"FRMNAM", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    ZZHSICHK(
        save.BIDLST.as_slice(),
        save.BIDPOL.as_slice(),
        save.BIDIDS.as_slice(),
        FRCODE,
        &mut save.ITEM,
        ctx,
    )?;
    if (save.ITEM != 0) {
        save.ITEM = save.BIDIDX[save.ITEM];
    }

    if (save.ITEM != 0) {
        fstr::assign(FRNAME, save.NAME.get(save.ITEM));
    } else {
        //
        // See if this frame is in the kernel pool frame ID-based hash.
        // First reset the hash if POOL has changed.
        //
        ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

        if save.LUPDTE {
            ZZHSCINI(
                MAXKFR,
                save.KNMLST.as_slice_mut(),
                save.KNMPOL.as_slice_mut(),
                ctx,
            )?;
            ZZHSIINI(
                MAXKFR,
                save.KIDLST.as_slice_mut(),
                save.KIDPOL.as_slice_mut(),
                ctx,
            )?;
        }

        //
        // Check if this ID is in the hash.
        //
        ZZHSICHK(
            save.KIDLST.as_slice(),
            save.KIDPOL.as_slice(),
            save.KIDIDS.as_slice(),
            FRCODE,
            &mut save.ITEM,
            ctx,
        )?;

        if (save.ITEM != 0) {
            fstr::assign(FRNAME, save.KNAME.get(save.ITEM));
        } else {
            //
            // The ID wasn't in the hash, see if we can find this frame in
            // the kernel pool.
            //
            fstr::assign(&mut save.PNAME, b"FRAME_#_NAME");
            REPMI(&save.PNAME.to_vec(), b"#", FRCODE, &mut save.PNAME, ctx);

            GCPOOL(
                &save.PNAME,
                1,
                ROOM,
                &mut save.N,
                save.LINE.as_arg_mut(),
                &mut save.GOTIT,
                ctx,
            )?;

            if ((save.N == 1) && save.GOTIT) {
                LJUST(&save.LINE[1], FRNAME);

            //
            // Note that since we did not collect all needed
            // information about this frame, we will not try to add it
            // to the hash. This addition is done only by FRINFO.
            //
            } else {
                fstr::assign(FRNAME, b" ");
            }
        }
    }

    CHKOUT(b"FRMNAM", ctx)?;
    Ok(())
}

/// FRame INFOrmation
///
/// Retrieve the minimal attributes associated with a frame
/// needed for converting transformations to and from it.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FRCODE     I   the ID code for some frame
///  CENT       O   the center of the frame
///  FRCLSS     O   the class (type) of the frame
///  CLSSID     O   the ID code for the frame within its class.
///  FOUND      O   .TRUE. if the requested information is available.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FRCODE   is the ID code for some reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  CENT     is the body ID code for the center of the reference
///           frame (if such an ID code is appropriate).
///
///  FRCLSS   is the class or type of the frame. This identifies
///           which subsystem will be used to perform frame
///           transformations.
///
///  CLSSID   is the ID code used for the frame within its class.
///           This may be different from the frame ID code.
///
///  FOUND    is .TRUE. if CENT, FRCLSS and CCODE are available.
///           Otherwise, FOUND is returned with the value .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If a frame definition is encountered that does not define a
///      central body for the frame, an error is signaled by a routine
///      in the call tree of this routine.
///
///  2)  If a frame definition is encountered that does not define
///      a class for the frame, an error is signaled by a routine
///      in the call tree of this routine.
///
///  3)  If a frame definition is encountered that does not define a
///      class ID for the frame, an error is signaled by a routine in
///      the call tree of this routine.
///
///  4)  If a kernel variable defining a frame name is found, but
///      that variable has dimension greater than 1, the error
///      SPICE(INVALIDDIMENSION) is signaled.
/// ```
///
/// # Particulars
///
/// ```text
///  This is a low level routine needed by state transformation
///  software to transform states and attitudes between different
///  reference frames.
///
///  The routine first examines local "hard-coded" information about
///  reference frames to see if the requested frame belongs to this
///  set. If it does that information is returned.
///
///  If the requested information is not stored locally, the routine
///  then examines the kernel pool to see if the requested information
///  is stored there. If it is and has the expected format, the data
///  is retrieved and returned.
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
///  1) Given a frame ID, retrieve the SPICE body ID associated with
///     the frame's center, the frame class (or type of the frame),
///     and the ID used for the frame within its class.
///
///       Example code begins here.
///
///
///             PROGRAM FRINFO_EX1
///             IMPLICIT NONE
///
///       C
///       C     Local parameters.
///       C
///             INTEGER               FRCODE
///             PARAMETER           ( FRCODE = 13000 )
///
///       C
///       C     Local variables.
///       C
///             INTEGER               CENTER
///             INTEGER               CLSSID
///             INTEGER               FRCLSS
///
///             LOGICAL               FOUND
///
///       C
///       C     Retrieve the information for frame ID 13000.
///       C
///             CALL FRINFO ( FRCODE, CENTER, FRCLSS, CLSSID, FOUND )
///
///             IF ( FOUND ) THEN
///
///                WRITE(*,'(A,I6)') 'Frame center  : ', CENTER
///                WRITE(*,'(A,I6)') 'Frame class   : ', FRCLSS
///                WRITE(*,'(A,I6)') 'Frame class ID: ', CLSSID
///
///             ELSE
///
///                WRITE (*,'(A,I6)') 'There is insufficient data '
///            .                   // 'for frame ', FRCODE
///
///             END IF
///
///             END
///
///
///       When this program was executed on a Mac/Intel/gfortran/64-bit
///       platform, the output was:
///
///
///     Frame center  :    399
///     Frame class   :      2
///     Frame class ID:   3000
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
/// -    SPICELIB Version 4.2.0, 02-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example from existing fragments.
///
///         Changed the output argument name CLASS to FRCLSS for
///         consistency with other routines.
///
/// -    SPICELIB Version 4.1.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
/// -    SPICELIB Version 4.0.0, 12-SEP-2005 (NJB)
///
///         Entry point FRINFO is no longer error-free. The following
///         errors are now diagnosed:
///
///            - Invalid dimension of frame name variable
///
///            - If a valid frame name assignment is present:
///
///               + Missing frame ID code assignment
///               + Missing class assignment
///               + Missing class ID assignment
///
///         Specification of frame center by name or ID is now supported.
///         Previously only ID codes could be used to identify frame
///         centers. Various frame definition errors that were previously
///         ignored are now diagnosed.
///
/// -    SPICELIB Version 3.1.0, 20-DEC-2004 (BVS)
///
///         Removed special handling of the frame IDs less than -999. If
///         they cannot be ``resolved'' using kernel pool keywords, the
///         frame is NOT declared CK-based with center ID derived by
///         dividing frame ID by a 1000 and class ID assigned the frame ID
///         anymore. In the current practice with multitude of TK frames
///         with IDs set instrument IDs this default behavior is simply
///         not valid.
///
/// -    SPICELIB Version 3.0.1, 25-JUN-1999 (WLT)
///
///         Extended documentation of entry point CNMFRM and
///         corrected example for that entry point.
///
/// -    SPICELIB Version 3.0.0, 03-JUN-1997 (WLT)
///
///         The entry points CIDFRM and CNMFRM were added so that
///         user's may determine the frame-id and name to associated
///         with a planetary object.
///
/// -    SPICELIB Version 2.0.0, 04-APR-1997 (WLT)
///
///         The routine was upgraded to reflect that a block of
///         frame ID codes have been reserved for use by the DSN.
///         ID codes 13001 to 13999 have been set aside for DSN
///         models for the orientation of the earth. These frames
///         are all PCK frames. Moreover, the PCK ID code to
///         use with these frames is simply the Frame-Code minus 10000.
///         All of these frames are centered at the earth (body 399).
///
/// -    SPICELIB Version 1.1.0, 14-OCT-1996 (WLT)
///
///         The values NINERT and NNINRT are included instead of
///         being declared locally.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (WLT)
/// ```
pub fn frinfo(
    ctx: &mut SpiceContext,
    frcode: i32,
    cent: &mut i32,
    frclss: &mut i32,
    clssid: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    FRINFO(frcode, cent, frclss, clssid, found, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure FRINFO ( FRame INFOrmation )
pub fn FRINFO(
    FRCODE: i32,
    CENT: &mut i32,
    FRCLSS: &mut i32,
    CLSSID: &mut i32,
    FOUND: &mut bool,
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

    //
    // For efficiency, J2000 deserves special treatment.
    //
    if (FRCODE == 1) {
        *CENT = 0;
        *FRCLSS = INERTL;
        *CLSSID = 1;
        *FOUND = true;
        return Ok(());
    }

    CHKIN(b"FRINFO", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXKFR,
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            ctx,
        )?;
        ZZHSCINI(
            MAXKFR,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Initialize built-in frame tables and hashes.
        //
        ZZFDAT(
            NCOUNT,
            MAXBFR,
            save.NAME.as_arg_mut(),
            save.IDCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.TYPE.as_slice_mut(),
            save.TYPEID.as_slice_mut(),
            save.CENTRD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"FRINFO", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // No frame information has been found yet.
    //
    *FOUND = false;

    //
    // Determine the location of the requested item in the array
    // of ID codes.
    //
    ZZHSICHK(
        save.BIDLST.as_slice(),
        save.BIDPOL.as_slice(),
        save.BIDIDS.as_slice(),
        FRCODE,
        &mut save.ITEM,
        ctx,
    )?;
    if (save.ITEM != 0) {
        save.ITEM = save.BIDIDX[save.ITEM];
    }

    //
    // If the name is in our hash, we can just look up its ID code in
    // the parallel array.
    //
    if (save.ITEM > 0) {
        *CENT = save.CENTER[save.ITEM];
        *FRCLSS = save.TYPE[save.ITEM];
        *CLSSID = save.TYPEID[save.ITEM];
        *FOUND = true;
    } else {
        //
        // See if this frame is in the kernel pool frame ID-based hash.
        // First reset the hash if POOL has changed.
        //
        ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

        if save.LUPDTE {
            ZZHSCINI(
                MAXKFR,
                save.KNMLST.as_slice_mut(),
                save.KNMPOL.as_slice_mut(),
                ctx,
            )?;
            ZZHSIINI(
                MAXKFR,
                save.KIDLST.as_slice_mut(),
                save.KIDPOL.as_slice_mut(),
                ctx,
            )?;
        }

        //
        // Check if this ID is in the hash.
        //
        ZZHSICHK(
            save.KIDLST.as_slice(),
            save.KIDPOL.as_slice(),
            save.KIDIDS.as_slice(),
            FRCODE,
            &mut save.ITEM,
            ctx,
        )?;

        if (save.ITEM != 0) {
            *CENT = save.KCENT[save.ITEM];
            *FRCLSS = save.KCLASS[save.ITEM];
            *CLSSID = save.KCLSID[save.ITEM];
            *FOUND = true;
        } else {
            //
            // The ID wasn't in the hash, see if we can find this frame in
            // the kernel pool.
            //
            fstr::assign(&mut save.PNAME, b"FRAME_#_NAME");
            REPMI(&save.PNAME.to_vec(), b"#", FRCODE, &mut save.PNAME, ctx);

            GCPOOL(
                &save.PNAME,
                1,
                ROOM,
                &mut save.N,
                save.LINE.as_arg_mut(),
                &mut save.GOTIT,
                ctx,
            )?;

            if save.GOTIT {
                if (save.N > 1) {
                    //
                    // We have an array-valued variable that looks like
                    // a frame name. We consider this an error.
                    //
                    SETMSG(b"Kernel variable # is array-valued; Frame name variables must be scalar-valued.", ctx);
                    ERRCH(b"#", &save.PNAME, ctx);
                    SIGERR(b"SPICE(INVALIDDIMENSION)", ctx)?;
                    CHKOUT(b"FRINFO", ctx)?;
                    return Ok(());
                }

                LJUST(&save.LINE[1], &mut save.LCFRAM);
                //
                // Start by looking up the central body of the frame. The
                // name of the kernel variable for the body could refer to
                // the frame by name or frame ID; the body itself could be
                // specified by name or body ID.
                //
                ZZDYNBID(&save.LCFRAM, FRCODE, b"CENTER", CENT, ctx)?;

                if FAILED(ctx) {
                    CHKOUT(b"FRINFO", ctx)?;
                    return Ok(());
                }

                *FOUND = true;

                //
                // FOUND has been set to indicate whether we found the
                // frame's center. If we did, CENT has been assigned.
                //
                // Next look up the frame class and class ID.
                //
                ZZDYNVAI(
                    &save.LCFRAM,
                    FRCODE,
                    b"CLASS",
                    1,
                    &mut save.N,
                    save.VALUES.as_slice_mut(),
                    ctx,
                )?;
                *FRCLSS = save.VALUES[1];

                ZZDYNVAI(
                    &save.LCFRAM,
                    FRCODE,
                    b"CLASS_ID",
                    1,
                    &mut save.N,
                    save.VALUES.as_slice_mut(),
                    ctx,
                )?;
                *CLSSID = save.VALUES[1];

                if FAILED(ctx) {
                    CHKOUT(b"FRINFO", ctx)?;
                    return Ok(());
                }

                // If we made it to this point, we successfully collected
                // all items for this frame. Add this frame to the
                // ID-based hash.
                //
                ZZHSIADD(
                    save.KIDLST.as_slice_mut(),
                    save.KIDPOL.as_slice_mut(),
                    save.KIDIDS.as_slice_mut(),
                    FRCODE,
                    &mut save.ITEM,
                    &mut save.LNEW,
                    ctx,
                )?;

                if (!FAILED(ctx) && (save.ITEM != 0)) {
                    fstr::assign(save.KNAME.get_mut(save.ITEM), &save.LCFRAM);
                    save.KCENT[save.ITEM] = *CENT;
                    save.KCLASS[save.ITEM] = *FRCLSS;
                    save.KCLSID[save.ITEM] = *CLSSID;

                    //
                    // Also, try to add this frame to the name-based hash.
                    //
                    ZZHSCADD(
                        save.KNMLST.as_slice_mut(),
                        save.KNMPOL.as_slice_mut(),
                        save.KNMNMS.as_arg_mut(),
                        &save.LCFRAM,
                        &mut save.ITEM,
                        &mut save.LNEW,
                        ctx,
                    )?;

                    if (!FAILED(ctx) && (save.ITEM != 0)) {
                        save.KNMIDS[save.ITEM] = FRCODE;
                    }
                }
            }
        }

        //
        // In support of the DSN, NAIF has reserved a block of
        // ID codes for DSN specific frames  from 13000 to 13999.
        // These are always PCK based frames for the earth.
        // The PCK ID code is just FRCODE - 10000.
        //
        if ((!*FOUND && (FRCODE >= 13000)) && (FRCODE < 14000)) {
            *CENT = 399;
            *FRCLSS = PCK;
            *CLSSID = (FRCODE - 10000);
            *FOUND = true;
        }
    }

    CHKOUT(b"FRINFO", ctx)?;
    Ok(())
}

/// Center ID to FRaMe id and name
///
/// Retrieve frame ID code and name to associate with a frame center.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CENT       I   an object to associate a frame with.
///  FRCODE     O   the ID code of the frame associated with CENT
///  FRNAME     O   the name of the frame with ID FRCODE
///  FOUND      O   .TRUE. if the requested information is available.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CENT     is the ID code for object for which there is a
///           preferred reference frame.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FRCODE   is the frame ID code to associate with the object
///           specified by CENT.
///
///  FRNAME   is the name of the frame that should be associated
///           with the object specified by CENT. FRNAME should be
///           declared as CHARACTER*(32) to ensure that it can
///           contain the full name of the frame. If FRNAME does
///           not have enough room to hold the full name of the
///           frame, the name will be truncated on the right.
///
///  FOUND    is .TRUE. if the appropriate frame ID code and frame
///           name can be determined. Otherwise FOUND is returned
///           with the value .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If FRNAME does not have enough room to contain the frame name,
///      the name will be truncated on the right. (Declaring FRNAME to
///      be CHARACTER*(32) will ensure that the name will not be
///      truncated).
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows the user to determine the frame that should
///  be associated with a particular object. For example, if you
///  need the frame to associate with the Io, you can call CIDFRM
///  to determine the frame name and ID code for the bodyfixed frame
///  of Io.
///
///  The preferred frame to use with an object is specified via one
///  of the kernel pool variables:
///
///      OBJECT_<cent>_FRAME
///
///  where <cent> is the decimal representation of the integer CENT.
///
///  For those PCK objects that have "built-in" frame names this
///  routine returns the corresponding "IAU" frame and frame ID code.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you want to determine the state of a target
///  in the preferred reference frame of some observer. This
///  routine can be used in conjunction with SPKEZ to compute
///  the state.
///
///     CALL CIDFRM ( OBS, FRCODE, FRNAME, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///
///        WRITE (*,*) 'The bodyfixed frame for object ', OBS
///        WRITE (*,*) 'could not be identified.'
///        STOP
///
///     END IF
///
///     CALL SPKEZ ( TARG, ET, FRNAME, ABCORR, OBS, STATE, LT )
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
/// -    SPICELIB Version 3.2.1, 13-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Updated maximum frame name length from 26 to 32 in
///         $Detailed_Output and $Exceptions section.
///
/// -    SPICELIB Version 3.2.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
/// -    SPICELIB Version 3.1.1, 09-FEB-2011 (NJB)
///
///         Bug fix: corrected logic for object-frame association for case
///         where the assigned frame value is denoted by a frame code.
///
/// -    SPICELIB Version 3.0.1, 25-JUN-1999 (WLT)
///
///         Extended documentation of entry point CNMFRM and
///         corrected example for that entry point.
///
/// -    SPICELIB Version 3.0.0, 03-JUN-1997 (WLT)
///
///         The entry points CIDFRM and CNMFRM were added so that
///         user's may determine the frame-id and name to associated
///         with a planetary object.
///
/// -    SPICELIB Version 2.0.0, 04-APR-1997 (WLT)
///
///         The routine was upgraded to reflect that a block of
///         frame ID codes have been reserved for use by the DSN.
///         ID codes 13001 to 13999 have been set aside for DSN
///         models for the orientation of the earth. These frames
///         are all PCK frames. Moreover, the PCK ID code to
///         use with these frames is simply the Frame-Code minus 10000.
///         All of these frames are centered at the earth (body 399).
///
/// -    SPICELIB Version 1.1.0, 14-OCT-1996 (WLT)
///
///         The values NINERT and NNINRT are included instead of
///         being declared locally.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (WLT)
/// ```
pub fn cidfrm(
    ctx: &mut SpiceContext,
    cent: i32,
    frcode: &mut i32,
    frname: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    CIDFRM(
        cent,
        frcode,
        fstr::StrBytes::new(frname).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CIDFRM ( Center ID to FRaMe id and name )
pub fn CIDFRM(
    CENT: i32,
    FRCODE: &mut i32,
    FRNAME: &mut [u8],
    FOUND: &mut bool,
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

    CHKIN(b"CIDFRM", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXKFR,
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            ctx,
        )?;
        ZZHSCINI(
            MAXKFR,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Initialize built-in frame tables and hashes.
        //
        ZZFDAT(
            NCOUNT,
            MAXBFR,
            save.NAME.as_arg_mut(),
            save.IDCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.TYPE.as_slice_mut(),
            save.TYPEID.as_slice_mut(),
            save.CENTRD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"CIDFRM", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // First look up in the kernel pool the frame associated with this
    // center.
    //
    fstr::assign(&mut save.LOOKUP, b"OBJECT_#_FRAME");

    REPMI(&save.LOOKUP.to_vec(), b"#", CENT, &mut save.LOOKUP, ctx);
    DTPOOL(
        &save.LOOKUP,
        &mut save.GOTIT,
        &mut save.N,
        &mut save.DATTYP,
        ctx,
    )?;
    //
    // If we didn't find this object in the form OBJECT_<number>_FRAME
    // maybe it is present in the form OBJECT_<name>_FRAME. It's
    // worth a try.
    //
    if !save.GOTIT {
        //
        // See if we can get the name for this center's ID code.
        //
        BODC2N(CENT, &mut save.LCNAME, &mut save.GOTIT, ctx)?;

        if save.GOTIT {
            //
            // Construct and look up the alternative name in the
            // kernel pool.
            //
            fstr::assign(&mut save.LOOKUP, b"OBJECT_#_FRAME");

            REPMC(&save.LOOKUP.to_vec(), b"#", &save.LCNAME, &mut save.LOOKUP);
            UCASE(&save.LOOKUP.to_vec(), &mut save.LOOKUP, ctx);
            DTPOOL(
                &save.LOOKUP,
                &mut save.GOTIT,
                &mut save.N,
                &mut save.DATTYP,
                ctx,
            )?;
        }
    }
    //
    // There are two cases. The user may specify either a name
    // or ID code for the frame to use to model the orientation of
    // an object. We assume they'll opt for the character string
    // form so we test that case first.
    //
    if save.GOTIT {
        if fstr::eq(&save.DATTYP, b"C") {
            GCPOOL(
                &save.LOOKUP,
                1,
                1,
                &mut save.N,
                CharArrayMut::from_mut(&mut save.PNAME),
                &mut save.GOTIT,
                ctx,
            )?;
            //
            // We've got the name:  See if we have this in our handy hash
            // of built-in names.
            //
            ZZHSCCHK(
                save.BNMLST.as_slice(),
                save.BNMPOL.as_slice(),
                save.BNMNMS.as_arg(),
                &save.PNAME,
                &mut save.ITEM,
                ctx,
            )?;
            if (save.ITEM != 0) {
                save.ITEM = save.BNMIDX[save.ITEM];
            }

            if (save.ITEM > 0) {
                fstr::assign(FRNAME, &save.PNAME);
                *FRCODE = save.IDCODE[save.ITEM];
                *FOUND = true;
            } else {
                //
                // See if this frame is in the kernel pool frame name-based
                // hash. First reset the hash if POOL has changed.
                //
                ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

                if save.LUPDTE {
                    ZZHSCINI(
                        MAXKFR,
                        save.KNMLST.as_slice_mut(),
                        save.KNMPOL.as_slice_mut(),
                        ctx,
                    )?;
                    ZZHSIINI(
                        MAXKFR,
                        save.KIDLST.as_slice_mut(),
                        save.KIDPOL.as_slice_mut(),
                        ctx,
                    )?;
                }

                //
                // Check if this name is in the hash.
                //
                ZZHSCCHK(
                    save.KNMLST.as_slice(),
                    save.KNMPOL.as_slice(),
                    save.KNMNMS.as_arg(),
                    &save.PNAME,
                    &mut save.ITEM,
                    ctx,
                )?;

                if (save.ITEM != 0) {
                    fstr::assign(FRNAME, &save.PNAME);
                    *FRCODE = save.KNMIDS[save.ITEM];
                    *FOUND = true;
                } else {
                    //
                    // Nope. Look in the kernel pool for the data associated
                    // with this frame.
                    //
                    // Capture the frame name now, since we're going to
                    // modify PNAME.
                    //
                    fstr::assign(FRNAME, &save.PNAME);

                    PREFIX(b"FRAME_", 0, &mut save.PNAME);
                    GIPOOL(
                        &save.PNAME,
                        1,
                        ROOM,
                        &mut save.N,
                        save.VALUES.as_slice_mut(),
                        &mut save.GOTIT,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"CIDFRM", ctx)?;
                        return Ok(());
                    }

                    if ((save.N == 1) && save.GOTIT) {
                        *FRCODE = save.VALUES[1];
                        *FOUND = true;

                        //
                        // If we made it to this point, we successfully
                        // mapped the kernel frame name to its ID. Add this
                        // pair to the name-based hash.
                        //
                        ZZHSCADD(
                            save.KNMLST.as_slice_mut(),
                            save.KNMPOL.as_slice_mut(),
                            save.KNMNMS.as_arg_mut(),
                            FRNAME,
                            &mut save.ITEM,
                            &mut save.LNEW,
                            ctx,
                        )?;

                        if (!FAILED(ctx) && (save.ITEM != 0)) {
                            save.KNMIDS[save.ITEM] = *FRCODE;
                        }
                    } else {
                        *FRCODE = 0;
                        fstr::assign(FRNAME, b" ");
                        *FOUND = false;
                    }
                }
            }
        } else if fstr::eq(&save.DATTYP, b"N") {
            //
            // Ok. They decided to use the numeric form to specify
            // the frame ID. We need to figure out the name of the frame.
            // First we retrieve the frame ID they've loaded into the
            // kernel pool.
            //
            GIPOOL(
                &save.LOOKUP,
                1,
                1,
                &mut save.N,
                save.VALUES.as_slice_mut(),
                &mut save.GOTIT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"CIDFRM", ctx)?;
                return Ok(());
            }

            //
            // We've got the frame ID, see if we already know about this
            // ID code.
            //
            ZZHSICHK(
                save.BIDLST.as_slice(),
                save.BIDPOL.as_slice(),
                save.BIDIDS.as_slice(),
                save.VALUES[1],
                &mut save.ITEM,
                ctx,
            )?;
            if (save.ITEM != 0) {
                save.ITEM = save.BIDIDX[save.ITEM];
            }

            if (save.ITEM != 0) {
                //
                // Just look up the name and set the frame code.
                //
                fstr::assign(FRNAME, save.NAME.get(save.ITEM));
                *FRCODE = save.VALUES[1];
                *FOUND = true;
            } else {
                //
                // See if this frame is in the kernel pool frame ID-based
                // hash. First reset the hash if POOL has changed.
                //
                ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

                if save.LUPDTE {
                    ZZHSCINI(
                        MAXKFR,
                        save.KNMLST.as_slice_mut(),
                        save.KNMPOL.as_slice_mut(),
                        ctx,
                    )?;
                    ZZHSIINI(
                        MAXKFR,
                        save.KIDLST.as_slice_mut(),
                        save.KIDPOL.as_slice_mut(),
                        ctx,
                    )?;
                }

                //
                // Check if this ID is in the hash.
                //
                ZZHSICHK(
                    save.KIDLST.as_slice(),
                    save.KIDPOL.as_slice(),
                    save.KIDIDS.as_slice(),
                    save.VALUES[1],
                    &mut save.ITEM,
                    ctx,
                )?;

                if (save.ITEM != 0) {
                    fstr::assign(FRNAME, save.KNAME.get(save.ITEM));
                    *FRCODE = save.VALUES[1];
                    *FOUND = true;
                } else {
                    //
                    // It is not in the hash. See if it's in the kernel pool
                    // somewhere.
                    //
                    fstr::assign(&mut save.PNAME, b"FRAME_#_NAME");
                    REPMI(
                        &save.PNAME.to_vec(),
                        b"#",
                        save.VALUES[1],
                        &mut save.PNAME,
                        ctx,
                    );
                    GCPOOL(
                        &save.PNAME,
                        1,
                        ROOM,
                        &mut save.N,
                        save.LINE.as_arg_mut(),
                        &mut save.GOTIT,
                        ctx,
                    )?;

                    if ((save.N == 1) && save.GOTIT) {
                        LJUST(&save.LINE[1], FRNAME);
                        *FRCODE = save.VALUES[1];
                        *FOUND = true;

                    //
                    // Note that since we did not collect all needed
                    // information about this frame, we will not try to
                    // add it to the hash. This addition is done only by
                    // FRINFO.
                    //
                    } else {
                        *FRCODE = save.VALUES[1];
                        fstr::assign(FRNAME, b" ");
                        *FOUND = false;
                    }
                }
            }
        }
        //
        // One way or the other we've filled in the values at this
        // point. Nothing left to do but check out and return.
        //
        CHKOUT(b"CIDFRM", ctx)?;
        return Ok(());
    }
    //
    // The only way to reach this point is if the user did not
    // specify via the kernel pool a frame to use for this center.
    //
    // We have a special case for EARTH.
    //
    if (CENT == 399) {
        *FRCODE = 10013;
        fstr::assign(FRNAME, b"IAU_EARTH");
        *FOUND = true;
        CHKOUT(b"CIDFRM", ctx)?;
        return Ok(());
    }

    //
    // Determine the location of the requested item in the array
    // of centers.
    //
    save.ITEM = BSCHOI(CENT, NCOUNT, save.CENTER.as_slice(), save.CENTRD.as_slice());

    //
    // If the name is in our list, we can just look up its ID code and
    // name in the parallel array.
    //
    if (save.ITEM > 0) {
        *FRCODE = save.IDCODE[save.ITEM];
        fstr::assign(FRNAME, save.NAME.get(save.ITEM));
        *FOUND = true;
    } else {
        //
        // There's nothing we can do now. We don't know what frame
        // might be associated with this object.
        //
        fstr::assign(FRNAME, b" ");
        *FRCODE = 0;
        *FOUND = false;
    }

    CHKOUT(b"CIDFRM", ctx)?;
    Ok(())
}

/// Center NaMe to FRaMe id and name
///
/// Retrieve frame ID code and name to associate with an object.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CNAME      I   name of the object to find a frame for
///  FRCODE     O   the ID code of the frame associated with CNAME
///  FRNAME     O   the name of the frame with ID FRCODE
///  FOUND      O   .TRUE. if the requested information is available.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CNAME    is the name for object for which there is a
///           preferred reference frame
/// ```
///
/// # Detailed Output
///
/// ```text
///  FRCODE   is the frame ID code to associate with a the object
///           specified by CNAME.
///
///  FRNAME   is the name of the frame that should be associated
///           with the object specified by CNAME. FRNAME should be
///           declared as CHARACTER*(32) to ensure that it can
///           contain the full name of the frame. If FRNAME does
///           not have enough room to hold the full name of the
///           frame, the name will be truncated on the right.
///
///  FOUND    is .TRUE. if the appropriate frame ID code and frame
///           name can be determined. Otherwise FOUND is returned
///           with the value .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If FRNAME does not have enough room to contain the frame name,
///      the name will be truncated on the right. (Declaring FRNAME to
///      be CHARACTER*(32) will ensure that the name will not be
///      truncated).
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows the user to determine the frame that should
///  be associated with a particular object. For example, if you
///  need the frame to associate with the Io, you can call CNMFRM
///  to determine the frame name and ID code for the bodyfixed frame
///  of Io.
///
///  The preferred frame to use with an object is specified via one
///  of the kernel pool variables:
///
///      OBJECT_<cname>_FRAME
///
///  where <cname> is the non-blank portion of the string CNAME.
///
///  For those PCK objects that have "built-in" frame names this
///  routine returns the corresponding "IAU" frame and frame ID code.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose that you want to determine the state of a target
///  in the preferred reference frame of some observer. This
///  routine can be used in conjunction with SPKEZR to compute
///  the state.
///
///     CALL CNMFRM ( OBSNAM, FRCODE, FRNAME, FOUND )
///
///     IF ( .NOT. FOUND ) THEN
///
///        WRITE (*,*) 'The bodyfixed frame for object ', OBSNAM
///        WRITE (*,*) 'could not be identified.'
///        STOP
///
///     END IF
///
///     CALL SPKEZR ( TARGET, ET, FRNAME, ABCORR, OBSNAM, STATE, LT )
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
/// -    SPICELIB Version 3.2.1, 02-AUG-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///
///         Updated maximum frame name length from 26 to 32 in
///         $Detailed_Output and $Exceptions section.
///
/// -    SPICELIB Version 3.2.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
/// -    SPICELIB Version 3.1.0, 28-NOV-2002 (NJB)
///
///         Bug fix: updated this routine so a TK frame specified by name
///         and designated as an object's preferred frame via kernel pool
///         assignments is found, and so that the correct name of this
///         frame is returned.
///
/// -    SPICELIB Version 3.0.1, 25-JUN-1999 (WLT)
///
///         Extended documentation of entry point CNMFRM and
///         corrected example for that entry point.
///
/// -    SPICELIB Version 3.0.0, 03-JUN-1997 (WLT)
///
///         The entry points CIDFRM and CNMFRM were added so that
///         user's may determine the frame-id and name to associated
///         with a planetary object.
///
/// -    SPICELIB Version 2.0.0, 04-APR-1997 (WLT)
///
///         The routine was upgraded to reflect that a block of
///         frame ID codes have been reserved for use by the DSN.
///         ID codes 13001 to 13999 have been set aside for DSN
///         models for the orientation of the earth. These frames
///         are all PCK frames. Moreover, the PCK ID code to
///         use with these frames is simply the Frame-Code minus 10000.
///         All of these frames are centered at the earth (body 399).
///
///
/// -    SPICELIB Version 1.1.0, 14-OCT-1996 (WLT)
///
///         The values NINERT and NNINRT are included instead of
///         being declared locally.
///
/// -    SPICELIB Version 1.0.0, 18-SEP-1995 (WLT)
/// ```
pub fn cnmfrm(
    ctx: &mut SpiceContext,
    cname: &str,
    frcode: &mut i32,
    frname: &mut str,
    found: &mut bool,
) -> crate::Result<()> {
    CNMFRM(
        cname.as_bytes(),
        frcode,
        fstr::StrBytes::new(frname).as_mut(),
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CNMFRM ( Center NaMe to FRaMe id and name )
pub fn CNMFRM(
    CNAME: &[u8],
    FRCODE: &mut i32,
    FRNAME: &mut [u8],
    FOUND: &mut bool,
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

    CHKIN(b"CNMFRM", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXKFR,
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            ctx,
        )?;
        ZZHSCINI(
            MAXKFR,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Initialize built-in frame tables and hashes.
        //
        ZZFDAT(
            NCOUNT,
            MAXBFR,
            save.NAME.as_arg_mut(),
            save.IDCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.TYPE.as_slice_mut(),
            save.TYPEID.as_slice_mut(),
            save.CENTRD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"CNMFRM", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // First look up in the kernel pool the frame associated with this
    // center.
    //
    fstr::assign(&mut save.LOOKUP, b"OBJECT_#_FRAME");

    REPMC(&save.LOOKUP.to_vec(), b"#", CNAME, &mut save.LOOKUP);
    UCASE(&save.LOOKUP.to_vec(), &mut save.LOOKUP, ctx);
    DTPOOL(
        &save.LOOKUP,
        &mut save.GOTIT,
        &mut save.N,
        &mut save.DATTYP,
        ctx,
    )?;

    //
    // If we didn't find this object in the form OBJECT_<name>_FRAME
    // maybe it is present in the form OBJECT_<number>_FRAME. It's
    // worth a try.
    //
    if !save.GOTIT {
        //
        // See if we can get the name for this center's ID code.
        //
        BODN2C(CNAME, &mut save.ID, &mut save.GOTIT, ctx)?;

        if save.GOTIT {
            //
            // Construct and look up the alternative name in the
            // kernel pool.
            //
            fstr::assign(&mut save.LOOKUP, b"OBJECT_#_FRAME");

            REPMI(&save.LOOKUP.to_vec(), b"#", save.ID, &mut save.LOOKUP, ctx);
            DTPOOL(
                &save.LOOKUP,
                &mut save.GOTIT,
                &mut save.N,
                &mut save.DATTYP,
                ctx,
            )?;
        }
    }

    //
    // There are two cases. The user may specify either a name
    // or ID code for the frame to use to model the orientation of
    // an object. We assume they'll opt for the character string
    // form so we test that case first.
    //
    if save.GOTIT {
        if fstr::eq(&save.DATTYP, b"C") {
            GCPOOL(
                &save.LOOKUP,
                1,
                1,
                &mut save.N,
                CharArrayMut::from_mut(&mut save.PNAME),
                &mut save.GOTIT,
                ctx,
            )?;
            //
            // We've got the name:  See if we have this in our handy hash
            // of built-in names.
            //
            ZZHSCCHK(
                save.BNMLST.as_slice(),
                save.BNMPOL.as_slice(),
                save.BNMNMS.as_arg(),
                &save.PNAME,
                &mut save.ITEM,
                ctx,
            )?;
            if (save.ITEM != 0) {
                save.ITEM = save.BNMIDX[save.ITEM];
            }

            if (save.ITEM > 0) {
                fstr::assign(FRNAME, &save.PNAME);
                *FRCODE = save.IDCODE[save.ITEM];
                *FOUND = true;
            } else {
                //
                // See if this frame is in the kernel pool frame name-based
                // hash. First reset the hash if POOL has changed.
                //
                ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

                if save.LUPDTE {
                    ZZHSCINI(
                        MAXKFR,
                        save.KNMLST.as_slice_mut(),
                        save.KNMPOL.as_slice_mut(),
                        ctx,
                    )?;
                    ZZHSIINI(
                        MAXKFR,
                        save.KIDLST.as_slice_mut(),
                        save.KIDPOL.as_slice_mut(),
                        ctx,
                    )?;
                }

                //
                // Check if this name is in the hash.
                //
                ZZHSCCHK(
                    save.KNMLST.as_slice(),
                    save.KNMPOL.as_slice(),
                    save.KNMNMS.as_arg(),
                    &save.PNAME,
                    &mut save.ITEM,
                    ctx,
                )?;

                if (save.ITEM != 0) {
                    fstr::assign(FRNAME, &save.PNAME);
                    *FRCODE = save.KNMIDS[save.ITEM];
                    *FOUND = true;
                } else {
                    //
                    // Nope. Look in the kernel pool for the data associated
                    // with this frame.
                    //
                    // Capture the frame name now, since we're going to
                    // modify PNAME.
                    //
                    fstr::assign(FRNAME, &save.PNAME);

                    PREFIX(b"FRAME_", 0, &mut save.PNAME);
                    GIPOOL(
                        &save.PNAME,
                        1,
                        ROOM,
                        &mut save.N,
                        save.VALUES.as_slice_mut(),
                        &mut save.GOTIT,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"CNMFRM", ctx)?;
                        return Ok(());
                    }

                    if ((save.N == 1) && save.GOTIT) {
                        *FRCODE = save.VALUES[1];
                        *FOUND = true;

                        //
                        // If we made it to this point, we successfully
                        // mapped the kernel frame name to its ID. Add this
                        // pair to the name-based hash.
                        //
                        ZZHSCADD(
                            save.KNMLST.as_slice_mut(),
                            save.KNMPOL.as_slice_mut(),
                            save.KNMNMS.as_arg_mut(),
                            FRNAME,
                            &mut save.ITEM,
                            &mut save.LNEW,
                            ctx,
                        )?;

                        if (!FAILED(ctx) && (save.ITEM != 0)) {
                            save.KNMIDS[save.ITEM] = *FRCODE;
                        }
                    } else {
                        *FRCODE = 0;
                        fstr::assign(FRNAME, b" ");
                        *FOUND = false;
                    }
                }
            }
        } else if fstr::eq(&save.DATTYP, b"N") {
            //
            // Ok. They decided to use the numeric form to specify
            // the frame ID. We need to figure out the name of the frame.
            // First we retrieve the frame ID they've loaded into the
            // kernel pool.
            //
            GIPOOL(
                &save.LOOKUP,
                1,
                1,
                &mut save.N,
                save.VALUES.as_slice_mut(),
                &mut save.GOTIT,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"CNMFRM", ctx)?;
                return Ok(());
            }

            //
            // We've got the frame ID, see if we already know about this
            // ID code.
            //
            ZZHSICHK(
                save.BIDLST.as_slice(),
                save.BIDPOL.as_slice(),
                save.BIDIDS.as_slice(),
                save.VALUES[1],
                &mut save.ITEM,
                ctx,
            )?;
            if (save.ITEM != 0) {
                save.ITEM = save.BIDIDX[save.ITEM];
            }

            if (save.ITEM != 0) {
                //
                // Just look up the name and set the frame code.
                //
                fstr::assign(FRNAME, save.NAME.get(save.ITEM));
                *FRCODE = save.VALUES[1];
                *FOUND = true;
            } else {
                //
                // See if this frame is in the kernel pool frame ID-based
                // hash. First reset the hash if POOL has changed.
                //
                ZZPCTRCK(save.PULCTR.as_slice_mut(), &mut save.LUPDTE, ctx);

                if save.LUPDTE {
                    ZZHSCINI(
                        MAXKFR,
                        save.KNMLST.as_slice_mut(),
                        save.KNMPOL.as_slice_mut(),
                        ctx,
                    )?;
                    ZZHSIINI(
                        MAXKFR,
                        save.KIDLST.as_slice_mut(),
                        save.KIDPOL.as_slice_mut(),
                        ctx,
                    )?;
                }

                //
                // Check if this ID is in the hash.
                //
                ZZHSICHK(
                    save.KIDLST.as_slice(),
                    save.KIDPOL.as_slice(),
                    save.KIDIDS.as_slice(),
                    save.VALUES[1],
                    &mut save.ITEM,
                    ctx,
                )?;

                if (save.ITEM != 0) {
                    fstr::assign(FRNAME, save.KNAME.get(save.ITEM));
                    *FRCODE = save.VALUES[1];
                    *FOUND = true;
                } else {
                    //
                    // It is not in the hash. See if it's in the kernel pool
                    // somewhere.
                    //
                    fstr::assign(&mut save.PNAME, b"FRAME_#_NAME");
                    REPMI(
                        &save.PNAME.to_vec(),
                        b"#",
                        save.VALUES[1],
                        &mut save.PNAME,
                        ctx,
                    );
                    GCPOOL(
                        &save.PNAME,
                        1,
                        ROOM,
                        &mut save.N,
                        save.LINE.as_arg_mut(),
                        &mut save.GOTIT,
                        ctx,
                    )?;

                    if ((save.N == 1) && save.GOTIT) {
                        LJUST(&save.LINE[1], FRNAME);
                        *FRCODE = save.VALUES[1];
                        *FOUND = true;

                    //
                    // Note that since we did not collect all needed
                    // information about this frame, we will not try to
                    // add it to the hash. This addition is done only by
                    // FRINFO.
                    //
                    } else {
                        *FRCODE = save.VALUES[1];
                        fstr::assign(FRNAME, b" ");
                        *FOUND = false;
                    }
                }
            }
        }
        //
        // One way or the other we've filled in the values at this
        // point. Nothing left to do but check out and return.
        //
        CHKOUT(b"CNMFRM", ctx)?;
        return Ok(());
    }
    //
    // The only way to reach this point is if the user did not
    // specify via the kernel pool a frame to use for this center.
    //
    //
    fstr::assign(FRNAME, b"IAU_#");
    REPMC(&FRNAME.to_vec(), b"#", CNAME, FRNAME);
    UCASE(&FRNAME.to_vec(), FRNAME, ctx);

    //
    // Determine the location of the requested item in the array
    // of centers.
    //
    ZZHSCCHK(
        save.BNMLST.as_slice(),
        save.BNMPOL.as_slice(),
        save.BNMNMS.as_arg(),
        FRNAME,
        &mut save.ITEM,
        ctx,
    )?;
    if (save.ITEM != 0) {
        save.ITEM = save.BNMIDX[save.ITEM];
    }

    //
    // If the name is in our hash, we can just look up its ID code and
    // name in the parallel array.
    //
    if (save.ITEM > 0) {
        *FRCODE = save.IDCODE[save.ITEM];
        *FOUND = true;
    } else {
        //
        // There's nothing we can do now. We don't know what frame
        // might be associated with this object.
        //
        *FRCODE = 0;
        *FOUND = false;
    }

    CHKOUT(b"CNMFRM", ctx)?;
    Ok(())
}

/// frame Class and Class Id to FRaMe id and name
///
/// Return the frame name, frame ID, and center associated with
/// a given frame class and class ID.
///
/// # Required Reading
///
/// * [FRAMES](crate::required_reading::frames)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  FRCLSS     I   Class of frame.
///  CLSSID     I   Class ID of frame.
///  FRCODE     O   ID code of the frame identified by FRCLSS, CLSSID.
///  FRNAME     O   Name of the frame identified by FRCLSS, CLSSID.
///  CENT       O   Center of the frame identified by FRCLSS, CLSSID.
///  FOUND      O   .TRUE. if the requested information is available.
/// ```
///
/// # Detailed Input
///
/// ```text
///  FRCLSS   is the class or type of the frame. This identifies
///           which subsystem will be used to perform frame
///           transformations.
///
///  CLSSID   is the ID code used for the frame within its class.
///           This may be different from the frame ID code.
/// ```
///
/// # Detailed Output
///
/// ```text
///  FRCODE   is the frame ID code for the reference frame
///           identified by FRCLSS and CLSSID.
///
///  FRNAME   is the name of the frame identified by FRCLSS and
///           CLSSID. FRNAME should be declared as CHARACTER*(32)
///           to ensure that it can contain the full name of the
///           frame. If FRNAME does not have enough room to hold
///           the full name of the frame, the name will be
///           truncated on the right.
///
///  CENT     is the body ID code for the center of the reference
///           frame identified by FRCLSS and CLSSID.
///
///  FOUND    is .TRUE. if FRCODE, FRNAME, and CENT are available.
///           Otherwise, FOUND is returned with the value .FALSE.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine assumes that the first frame found with matching
///      class and class ID is the correct one. SPICE's frame system
///      does not diagnose the situation where there are multiple,
///      distinct frames with matching classes and class ID codes, but
///      this situation could occur if such conflicting frame
///      specifications are loaded via one or more frame kernels. The
///      user is responsible for avoiding such frame specification
///      conflicts.
///
///  2)  If FRNAME does not have room to contain the frame name, the
///      name will be truncated on the right. (Declaring FRNAME to be
///      CHARACTER*(32) will ensure that the name will not be
///      truncated).
///
///  3)  If a frame class assignment is found that associates a
///      string (as opposed to numeric) value with a frame class
///      keyword, the error SPICE(INVALIDFRAMEDEF) is signaled.
///
///  4)  If a frame class assignment is found that matches the input
///      class, but a corresponding class ID assignment is not
///      found in the kernel pool, the error SPICE(INVALIDFRAMEDEF)
///      is signaled.
///
///  5)  If a frame specification is found in the kernel pool with
///      matching frame class and class ID, but either the frame name
///      or frame ID code are not found, the error
///      SPICE(INVALIDFRAMEDEF) is signaled.
///
///  6)  If a frame specification is found in the kernel pool with
///      matching frame class and class ID, but the frame center
///      is not found, an error is signaled by a routine
///      in the call tree of this routine.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine allows the user to determine the frame associated
///  with a given frame class and class ID code. The built-in frame
///  list is searched first for a matching frame; if no match is
///  found, then the kernel POOL is searched.
///
///  Since the neither the frame class nor the class ID are primary
///  keys, searching for matching frames is a linear (and therefore
///  typically slow) process.
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
///  1) The following code example demonstrates how to find the frame
///     information about a frame by its ID using FRINFO and
///     by its class and class ID using CCIFRM.
///
///
///     Example code begins here.
///
///
///           PROGRAM CCIFRM_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               FRNLEN
///           PARAMETER           ( FRNLEN = 32 )
///
///     C
///     C     Local variables.
///     C
///           CHARACTER*(FRNLEN)    FRNAME
///
///           INTEGER               CENTR1
///           INTEGER               CENTR2
///           INTEGER               CLSS
///           INTEGER               CLSSID
///           INTEGER               FRCOD1
///           INTEGER               FRCOD2
///
///           LOGICAL               FOUND
///
///     C
///     C     Find the frame code associated with ITRF93
///     C
///           FRNAME = 'ITRF93'
///           CALL NAMFRM ( FRNAME, FRCOD1 )
///
///     C
///     C     Get the frame information.
///     C
///           CALL FRINFO ( FRCOD1, CENTR1, CLSS, CLSSID, FOUND )
///
///           IF ( .NOT. FOUND ) THEN
///
///                 WRITE(*,*) 'No info found for frame ', FRCOD1
///                 STOP
///
///           END IF
///
///           WRITE(*,'(A)')    'Frame ITRF93 info:'
///           WRITE(*,'(A,I6)') '   Frame Code: ', FRCOD1
///           WRITE(*,'(A,I6)') '   Center ID : ', CENTR1
///           WRITE(*,'(A,I6)') '   Class     : ', CLSS
///           WRITE(*,'(A,I6)') '   Class ID  : ', CLSSID
///
///     C
///     C     Return the frame name, frame ID, and center associated
///     C     with the frame CLSS and CLSSID.
///     C
///           CALL CCIFRM ( CLSS,   CLSSID, FRCOD2,
///          .              FRNAME, CENTR2, FOUND  )
///
///           IF ( .NOT. FOUND ) THEN
///
///                 WRITE(*,*) 'No info found for type ', CLSS,
///          .                 ' frame ', CLSSID
///                 STOP
///
///           END IF
///
///
///           WRITE(*,'(A,I3,A,I6,A)') 'Type', CLSS, ' frame', CLSSID,
///          .                         ' info:'
///           WRITE(*,'(2A)')   '   Frame name: ', FRNAME
///           WRITE(*,'(A,I6)') '   Frame Code: ', FRCOD2
///           WRITE(*,'(A,I6)') '   Center ID : ', CENTR2
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     Frame ITRF93 info:
///        Frame Code:  13000
///        Center ID :    399
///        Class     :      2
///        Class ID  :   3000
///     Type  2 frame  3000 info:
///        Frame name: ITRF93
///        Frame Code:  13000
///        Center ID :    399
/// ```
///
/// # Restrictions
///
/// ```text
///  1)  See item (1) in the $Exceptions section above.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.2.0, 01-OCT-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete code example from existing fragments.
///
///         Changed the input argument name CLASS to FRCLSS for
///         consistency with other routines.
///
///         Updated maximum frame name length from 26 to 32 in
///         $Detailed_Output and $Exceptions section.
///
/// -    SPICELIB Version 1.1.1, 02-FEB-2017 (BVS)
///
///         Shortened one of permuted index entries.
///
/// -    SPICELIB Version 1.1.0, 08-AUG-2012 (BVS)
///
///         The routine was updated to be more efficient by using hashes
///         instead kernel POOL look-ups for kernel POOL frames and by
///         using hashes instead of ordered array searches for built-in
///         frames.
///
///         Bug fix: CCIFRM logic was corrected to examine the built-in
///         frames before looking at the kernel POOL frames.
///
/// -    SPICELIB Version 1.0.0, 05-NOV-2007 (NJB)
/// ```
pub fn ccifrm(
    ctx: &mut SpiceContext,
    frclss: i32,
    clssid: i32,
    frcode: &mut i32,
    frname: &mut str,
    cent: &mut i32,
    found: &mut bool,
) -> crate::Result<()> {
    CCIFRM(
        frclss,
        clssid,
        frcode,
        fstr::StrBytes::new(frname).as_mut(),
        cent,
        found,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure CCIFRM ( frame Class and Class Id to FRaMe id and name )
pub fn CCIFRM(
    FRCLSS: i32,
    CLSSID: i32,
    FRCODE: &mut i32,
    FRNAME: &mut [u8],
    CENT: &mut i32,
    FOUND: &mut bool,
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

    CHKIN(b"CCIFRM", ctx)?;

    //
    // Perform any needed first pass initializations.
    //
    if save.FIRST {
        //
        // Initialize POOL state counter to the user value.
        //
        ZZCTRUIN(save.PULCTR.as_slice_mut(), ctx);

        //
        // Initialize kernel POOL frame hashes.
        //
        ZZHSIINI(
            MAXKFR,
            save.KIDLST.as_slice_mut(),
            save.KIDPOL.as_slice_mut(),
            ctx,
        )?;
        ZZHSCINI(
            MAXKFR,
            save.KNMLST.as_slice_mut(),
            save.KNMPOL.as_slice_mut(),
            ctx,
        )?;

        //
        // Initialize built-in frame tables and hashes.
        //
        ZZFDAT(
            NCOUNT,
            MAXBFR,
            save.NAME.as_arg_mut(),
            save.IDCODE.as_slice_mut(),
            save.CENTER.as_slice_mut(),
            save.TYPE.as_slice_mut(),
            save.TYPEID.as_slice_mut(),
            save.CENTRD.as_slice_mut(),
            save.BNMLST.as_slice_mut(),
            save.BNMPOL.as_slice_mut(),
            save.BNMNMS.as_arg_mut(),
            save.BNMIDX.as_slice_mut(),
            save.BIDLST.as_slice_mut(),
            save.BIDPOL.as_slice_mut(),
            save.BIDIDS.as_slice_mut(),
            save.BIDIDX.as_slice_mut(),
            ctx,
        )?;

        if FAILED(ctx) {
            CHKOUT(b"CCIFRM", ctx)?;
            return Ok(());
        }

        save.FIRST = false;
    }

    //
    // No frame found so far.
    //
    *FOUND = false;

    //
    // First try to look up from the built-in list the frame associated
    // with the input class and class ID. Unfortunately, this is a
    // linear search.
    //
    for I in 1..=NCOUNT {
        if ((save.TYPE[I] == FRCLSS) && (save.TYPEID[I] == CLSSID)) {
            //
            // We have a match. Assign the output arguments and return.
            //
            fstr::assign(FRNAME, save.NAME.get(I));
            *FRCODE = save.IDCODE[I];
            *CENT = save.CENTER[I];
            *FOUND = true;

            CHKOUT(b"CCIFRM", ctx)?;
            return Ok(());
        }
    }

    //
    // Unfortunately we did not find a frame associated with the input
    // class and class ID in the built-in list. We need to look for this
    // frame in the kernel POOL. Since neither of these input values
    // appears in a kernel variable name, we may have to look at all of
    // the frame specifications in the kernel pool. Start out by looking
    // the frame class assignments from any loaded frame specifications.
    //
    fstr::assign(&mut save.LOOKUP, b"FRAME_*_CLASS");

    save.START = 1;
    GNPOOL(
        &save.LOOKUP,
        save.START,
        KVBSZ,
        &mut save.N,
        save.KVBUFF.as_arg_mut(),
        &mut save.FND,
        ctx,
    )?;

    while (save.FND && (save.N > 0)) {
        //
        // For each kernel variable name found in the buffer, look up the
        // associated class. If the class matches the input class, look
        // up the class ID as well. Set the output arguments and return
        // if we get a complete match.
        //
        for I in 1..=save.N {
            GIPOOL(
                &save.KVBUFF[I],
                1,
                1,
                &mut save.N,
                std::slice::from_mut(&mut save.KVCLSS),
                &mut save.FND,
                ctx,
            )?;

            if FAILED(ctx) {
                CHKOUT(b"CCIFRM", ctx)?;
                return Ok(());
            }

            if !save.FND {
                SETMSG(b"Invalid frame specification found in kernel pool: frame class keyword is # but integer class was not associated with this keyword.", ctx);
                ERRCH(b"#", &save.KVBUFF[I], ctx);
                SIGERR(b"SPICE(INVALIDFRAMEDEF)", ctx)?;
                CHKOUT(b"CCIFRM", ctx)?;
                return Ok(());
            }

            if (save.KVCLSS == FRCLSS) {
                //
                // Get the class ID for the current frame.
                //
                fstr::assign(&mut save.LOOK2, save.KVBUFF.get(I));

                SUFFIX(b"_ID", 0, &mut save.LOOK2);

                GIPOOL(
                    &save.LOOK2,
                    1,
                    1,
                    &mut save.N,
                    std::slice::from_mut(&mut save.KVCLID),
                    &mut save.FND,
                    ctx,
                )?;

                if FAILED(ctx) {
                    CHKOUT(b"CCIFRM", ctx)?;
                    return Ok(());
                }

                if !save.FND {
                    SETMSG(b"Invalid frame specification found in kernel pool: frame class keyword is # but associated integer class ID assignment was not found.", ctx);
                    ERRCH(b"#", &save.KVBUFF[I], ctx);
                    SIGERR(b"SPICE(INVALIDFRAMEDEF)", ctx)?;
                    CHKOUT(b"CCIFRM", ctx)?;
                    return Ok(());
                }

                //
                // Check the class ID for the current kernel variable
                // against the input value.
                //
                if (save.KVCLID == CLSSID) {
                    //
                    // We have a match. We need to return the frame
                    // ID, frame name, and center. As long as we're
                    // looking at a valid frame specification, this is
                    // no problem.
                    //
                    // Look up the frame name first. Create the frame
                    // name keyword.
                    //
                    REPMC(&save.KVBUFF[I], b"_CLASS", b"_NAME", &mut save.LOOK2);

                    GCPOOL(
                        &save.LOOK2,
                        1,
                        1,
                        &mut save.N,
                        CharArrayMut::from_mut(FRNAME),
                        &mut save.FND,
                        ctx,
                    )?;

                    if !save.FND {
                        SETMSG(b"Invalid frame specification found in kernel pool: frame class keyword is # but associated frame name assignment was not found.", ctx);
                        ERRCH(b"#", &save.KVBUFF[I], ctx);
                        SIGERR(b"SPICE(INVALIDFRAMEDEF)", ctx)?;
                        CHKOUT(b"CCIFRM", ctx)?;
                        return Ok(());
                    }

                    //
                    // We could extract the frame ID code from KVBUFF(I), but
                    // instead we'll make sure that the ID is defined in the
                    // kernel pool.
                    //
                    fstr::assign(&mut save.LOOK2, FRNAME);
                    PREFIX(b"FRAME_", 0, &mut save.LOOK2);

                    GIPOOL(
                        &save.LOOK2,
                        1,
                        1,
                        &mut save.N,
                        std::slice::from_mut(FRCODE),
                        &mut save.FND,
                        ctx,
                    )?;

                    if FAILED(ctx) {
                        CHKOUT(b"CCIFRM", ctx)?;
                        return Ok(());
                    }

                    if !save.FND {
                        SETMSG(b"Invalid frame specification found in kernel pool: frame name is is # but associated frame ID assignment was not found.", ctx);
                        ERRCH(b"#", FRNAME, ctx);
                        SIGERR(b"SPICE(INVALIDFRAMEDEF)", ctx)?;
                        CHKOUT(b"CCIFRM", ctx)?;
                        return Ok(());
                    }

                    //
                    // Look up the frame center. Whether the frame center
                    // has been specified by name or ID code, the ID code
                    // will be returned by ZZDYNBID.
                    //
                    ZZDYNBID(FRNAME, *FRCODE, b"CENTER", CENT, ctx)?;

                    //
                    // As long as we looked up the center successfully,
                    // we're done.
                    //
                    if !FAILED(ctx) {
                        *FOUND = true;
                    }

                    //
                    // Exit here, whether or not we looked up the frame's
                    // center successfully.
                    //
                    CHKOUT(b"CCIFRM", ctx)?;
                    return Ok(());
                }
            }
            //
            // Getting to this point means we didn't have a match;
            // examine the next buffer entry.
            //
        }

        //
        // Get the next buffer full of frame class keywords.
        //
        save.START = (save.START + save.N);
        GNPOOL(
            &save.LOOKUP,
            save.START,
            KVBSZ,
            &mut save.N,
            save.KVBUFF.as_arg_mut(),
            &mut save.FND,
            ctx,
        )?;
    }

    //
    // We drop down to this point only if no matching frame was found.
    // The FOUND flag has already been set to .FALSE.
    //
    CHKOUT(b"CCIFRM", ctx)?;
    Ok(())
}
