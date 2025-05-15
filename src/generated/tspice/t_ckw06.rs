//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const MAXDEG: i32 = 23;
const ITRUE: i32 = 1;
const IFALSE: i32 = -1;
const C06TP0: i32 = 0;
const C06TP1: i32 = (C06TP0 + 1);
const C06TP2: i32 = (C06TP1 + 1);
const C06TP3: i32 = (C06TP2 + 1);
const C06NST: i32 = 4;
const C06PS0: i32 = 8;
const C06PS1: i32 = 4;
const C06PS2: i32 = 14;
const C06PS3: i32 = 7;
const C06MXZ: i32 = C06PS2;
const C06MNZ: i32 = C06PS1;
const MAXRSZ: i32 = (4 + ((MAXDEG + 1) * (C06PS3 + 1)));
const QSIZ: i32 = 4;
const QAVSIZ: i32 = 7;
const CK1DTP: i32 = 1;
const CK1RSZ: i32 = 8;
const CK2DTP: i32 = 2;
const CK2RSZ: i32 = 10;
const CK3DTP: i32 = 3;
const CK3RSZ: i32 = 17;
const CK4DTP: i32 = 4;
const CK4PCD: f64 = 128.0;
const CK4MXD: i32 = 18;
const CK4SFT: i32 = 10;
const CK4RSZ: i32 = (((CK4MXD + 1) * QAVSIZ) + CK4SFT);
const CK5DTP: i32 = 5;
const CK5MXD: i32 = 23;
const CK5MET: i32 = 4;
const CK5MXP: i32 = 14;
const CK5RSZ: i32 = (((CK5MXD + 1) * CK5MXP) + CK5MET);
const CK6DTP: i32 = 6;
const CK6MXD: i32 = 23;
const CK6MET: i32 = 4;
const CK6PS3: i32 = 7;
const CK6RSZ: i32 = (((CK6MXD + 1) * (CK6PS3 + 1)) + CK6MET);
const CKMRSZ: i32 = CK5RSZ;
const ND: i32 = 2;
const NI: i32 = 6;
const DSCSIZ: i32 = 5;
const DTYPE: i32 = 6;
const DIRSIZ: i32 = 100;

struct SaveVars {
    PKTSZS: StackArray<i32, 4>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut PKTSZS = StackArray::<i32, 4>::new(0..=(C06NST - 1));

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::I(C06PS0),
                Val::I(C06PS1),
                Val::I(C06PS2),
                Val::I(C06PS3),
            ]
            .into_iter();
            PKTSZS
                .iter_mut()
                .for_each(|n| *n = clist.next().unwrap().into_i32());

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self { PKTSZS }
    }
}

//$Procedure      T_CKW06 ( CK, Write segment, type 6 )
pub fn T_CKW06(
    HANDLE: i32,
    INST: i32,
    REF: &[u8],
    AVFLAG: bool,
    FIRST: f64,
    LAST: f64,
    SEGID: &[u8],
    NINTVL: i32,
    NPKTS: &[i32],
    SUBTPS: &[i32],
    DEGRES: &[i32],
    PACKTS: &[f64],
    RATES: &[f64],
    SCLKDP: &[f64],
    IVLBDS: &[f64],
    SELLST: bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let NPKTS = DummyArray::new(NPKTS, 1..);
    let SUBTPS = DummyArray::new(SUBTPS, 1..);
    let DEGRES = DummyArray::new(DEGRES, 1..);
    let PACKTS = DummyArray::new(PACKTS, 1..);
    let RATES = DummyArray::new(RATES, 1..);
    let SCLKDP = DummyArray::new(SCLKDP, 1..);
    let IVLBDS = DummyArray::new(IVLBDS, 1..);
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=DSCSIZ);
    let mut BEPIX: i32 = 0;
    let mut EEPIX: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut ISEL: i32 = 0;
    let mut K: i32 = 0;
    let mut MINISZ: i32 = 0;
    let mut NDIR: i32 = 0;
    let mut PKTBEG: i32 = 0;
    let mut PKTDSZ: i32 = 0;
    let mut PKTEND: i32 = 0;
    let mut PKTSIZ: i32 = 0;
    let mut REFCOD: i32 = 0;
    let mut SEGBEG: i32 = 0;
    let mut SEGEND: i32 = 0;
    let mut SUBTYP: i32 = 0;
    let mut WINSIZ: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Packet structure parameters
    //

    //
    // Local variables
    //

    //
    // Saved values
    //

    //
    // Initial values
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_CKW06", ctx)?;

    //
    // Check the input data before writing to the file.
    //
    // This order of operations entails some redundant
    // calculations, but it allows for rapid error
    // detection.
    //
    // Initialize the mini-segment packet array indices,
    // and those of the mini-segment epoch array as well.
    //
    PKTBEG = 0;
    PKTEND = 0;

    BEPIX = 0;
    EEPIX = 0;

    //
    // If we made it this far, we're ready to start writing the segment.
    //
    // The type 6 segment structure is eloquently described by this
    // diagram from the CK Required Reading:
    //
    //    +--------------------------------+
    //    | Interval 1 mini-segment        |
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval N mini-segment        |
    //    +--------------------------------+
    //    | Interval 1 start time          |
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval N start time          |
    //    +--------------------------------+
    //    | Interval N stop time           |
    //    +--------------------------------+
    //    | Interval start 100             | (First interval directory)
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval start (N/100)*100     | (Last interval directory)
    //    +--------------------------------+
    //    | Interval 1 start pointer       |
    //    +--------------------------------+
    //          .
    //          .
    //          .
    //    +--------------------------------+
    //    | Interval N start pointer       |
    //    +--------------------------------+
    //    | Interval N stop pointer + 1    |
    //    +--------------------------------+
    //    | Boundary choice flag           |
    //    +--------------------------------+
    //    | Number of intervals            |
    //    +--------------------------------+
    //
    // CK type 6 mini-segments have the following structure:
    //
    //    +-----------------------+
    //    | Packet 1              |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Packet M              |
    //    +-----------------------+
    //    | Epoch 1               |
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch M               |
    //    +-----------------------+
    //    | Epoch 100             | (First time tag directory)
    //    +-----------------------+
    //                .
    //                .
    //                .
    //    +-----------------------+
    //    | Epoch ((M-1)/100)*100 | (Last time tag directory)
    //    +-----------------------+
    //    | Clock rate (sec/tick) |
    //    +-----------------------+
    //    | Subtype code          |
    //    +-----------------------+
    //    | Window size           |
    //    +-----------------------+
    //    | Number of packets     |
    //    +-----------------------+
    //
    // Note that the set of parameters at the end of a mini-segment does
    // not contain an interpolation interval count. This is because,
    // unlike a CK type 5 segment, a CK type 6 segment can contain at
    // most one gap. If present, the gap is located at the end of
    // mini-segment's interpolation interval.
    //
    //
    // Get the NAIF integer code for the reference frame.
    //
    spicelib::NAMFRM(REF, &mut REFCOD, ctx)?;

    // Create the segment descriptor. We don't use CKPDS because
    // that routine doesn't allow creation of a singleton segment.
    //
    IC[1] = INST;
    IC[2] = REFCOD;
    IC[3] = DTYPE;

    if AVFLAG {
        IC[4] = 1;
    } else {
        IC[4] = 0;
    }

    DC[1] = FIRST;
    DC[2] = LAST;

    spicelib::DAFPS(ND, NI, DC.as_slice(), IC.as_slice(), DESCR.as_slice_mut());

    //
    // Begin a new segment.
    //
    spicelib::DAFBNA(HANDLE, DESCR.as_slice(), SEGID, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_CKW06", ctx)?;
        return Ok(());
    }

    //
    // Re-initialize the mini-segment packet array indices,
    // and those of the mini-segment epoch array as well.
    //
    PKTBEG = 0;
    PKTEND = 0;

    BEPIX = 0;
    EEPIX = 0;

    //
    // Write data for each mini-segment to the file.
    //
    for I in 1..=NINTVL {
        //
        // Set the packet size, which is a function of the subtype.
        //
        SUBTYP = spicelib::BRCKTI(SUBTPS[I], C06TP0, C06TP3);

        if (SUBTYP != SUBTPS[I]) {
            SUBTYP = 1;
        }

        PKTSIZ = save.PKTSZS[SUBTYP];

        if spicelib::ODD(SUBTYP) {
            WINSIZ = (DEGRES[I] + 1);
        } else {
            WINSIZ = ((DEGRES[I] + 1) / 2);
        }

        //
        // Now that we have the packet size, we can compute
        // mini-segment packet index range. We'll let PKTDSZ
        // be the total count of packet data entries for this
        // mini-segment.
        //
        PKTDSZ = (NPKTS[I] * PKTSIZ);

        PKTBEG = (PKTEND + 1);
        PKTEND = ((PKTBEG - 1) + PKTDSZ);

        //
        // At this point, we're read to start writing the
        // current mini-segment to the file. Start with the
        // packet data.
        //
        spicelib::DAFADA(PACKTS.subarray(PKTBEG), PKTDSZ, ctx)?;

        //
        // Write the epochs for this mini-segment.
        //
        BEPIX = (EEPIX + 1);
        EEPIX = ((BEPIX - 1) + NPKTS[I]);

        spicelib::DAFADA(SCLKDP.subarray(BEPIX), NPKTS[I], ctx)?;

        //
        // Compute the number of epoch directories for the
        // current mini-segment.
        //
        NDIR = ((NPKTS[I] - 1) / DIRSIZ);

        //
        // Write the epoch directories to the segment.
        //
        for J in 1..=NDIR {
            K = ((BEPIX - 1) + (J * DIRSIZ));

            spicelib::DAFADA(SCLKDP.subarray(K), 1, ctx)?;
        }

        //
        // Write the mini-segment's SCLK rate, subtype, window size, and
        // packet count to the segment.
        //
        spicelib::DAFADA(RATES.subarray(I), 1, ctx)?;
        spicelib::DAFADA(&[(SUBTPS[I] as f64)], 1, ctx)?;
        spicelib::DAFADA(&[(WINSIZ as f64)], 1, ctx)?;
        spicelib::DAFADA(&[(NPKTS[I] as f64)], 1, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_CKW06", ctx)?;
            return Ok(());
        }
    }

    //
    // We've finished writing the mini-segments.
    //
    // Next write the interpolation interval bounds.
    //
    spicelib::DAFADA(IVLBDS.as_slice(), (NINTVL + 1), ctx)?;

    //
    // Create and write directories for the interval
    // bounds.
    //
    // The directory count is the interval bound count
    // (N+1), minus 1, divided by the directory size.
    //
    NDIR = (NINTVL / DIRSIZ);

    for I in 1..=NDIR {
        spicelib::DAFADA(IVLBDS.subarray((DIRSIZ * I)), 1, ctx)?;
    }

    //
    // Now we compute and write the start/stop pointers
    // for each mini-segment.
    //
    // The pointers are relative to the DAF address
    // preceding the segment. For example, a pointer
    // to the first DAF address in the segment has
    // value 1.
    //
    SEGEND = 0;

    for I in 1..=NINTVL {
        //
        // Set the packet size, which is a function of the subtype. Also
        // set the window size. First check the subtype, which will be
        // used as an array index.
        //
        SUBTYP = spicelib::BRCKTI(SUBTPS[I], C06TP0, C06TP3);

        if (SUBTYP != SUBTPS[I]) {
            SUBTYP = 1;
        }

        PKTSIZ = save.PKTSZS[SUBTYP];

        //
        // In order to compute the end pointer of the current
        // mini-segment, we must compute the size, in terms
        // of DAF addresses, of this mini-segment. The formula
        // for the size is
        //
        //     size =     n_packets * packet_size
        //             +  n_epochs
        //             +  n_epoch_directories
        //             +  4
        //
        //          =     n_packets * ( packet_size + 1 )
        //             +  ( n_packets - 1 ) / DIRSIZ
        //             +  4
        //
        MINISZ = (((NPKTS[I] * (PKTSIZ + 1)) + ((NPKTS[I] - 1) / DIRSIZ)) + 4);

        SEGBEG = (SEGEND + 1);
        SEGEND = ((SEGBEG + MINISZ) - 1);

        //
        // Write the mini-segment begin pointer.
        //
        // After the loop terminates, the final end pointer, incremented
        // by 1, will be written.
        //
        spicelib::DAFADA(&[(SEGBEG as f64)], 1, ctx)?;
    }

    //
    // Write the last mini-segment end pointer, incremented by one.
    // SEGEND was computed on the last iteration of the above loop.
    //
    spicelib::DAFADA(&[((SEGEND + 1) as f64)], 1, ctx)?;

    //
    // Write out the interval selection flag. The input
    // boolean value is represented by a numeric constant.
    //
    if SELLST {
        ISEL = ITRUE;
    } else {
        ISEL = IFALSE;
    }

    spicelib::DAFADA(&[(ISEL as f64)], 1, ctx)?;

    //
    // Write the mini-segment/interpolation interval count.
    //
    spicelib::DAFADA(&[(NINTVL as f64)], 1, ctx)?;

    //
    // End the segment.
    //
    spicelib::DAFENA(ctx)?;

    spicelib::CHKOUT(b"T_CKW06", ctx)?;
    Ok(())
}
