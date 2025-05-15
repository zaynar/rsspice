//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
pub const NPGRPH: i32 = 20;
pub const AVESIZ: i32 = 20;
pub const LNSIZE: i32 = 132;
const PBEG: i32 = 1;
const PNXT: i32 = 2;
const MAXLIN: i32 = (AVESIZ * NPGRPH);

struct SaveVars {
    LINES: ActualCharArray,
    BEGEND: StackArray2D<i32, 40>,
    BUFFRD: i32,
    CURRNT: i32,
    N: i32,
    RANGE: i32,
    BACKUP: i32,
    BSIZE: i32,
    GETAT: i32,
    GOTTEN: i32,
    LAST: i32,
    PUTAT: i32,
    QUIT: i32,
    START: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut LINES = ActualCharArray::new(LNSIZE, 1..=MAXLIN);
        let mut BEGEND = StackArray2D::<i32, 40>::new(1..=2, 1..=NPGRPH);
        let mut BUFFRD: i32 = 0;
        let mut CURRNT: i32 = 0;
        let mut N: i32 = 0;
        let mut RANGE: i32 = 0;
        let mut BACKUP: i32 = 0;
        let mut BSIZE: i32 = 0;
        let mut GETAT: i32 = 0;
        let mut GOTTEN: i32 = 0;
        let mut LAST: i32 = 0;
        let mut PUTAT: i32 = 0;
        let mut QUIT: i32 = 0;
        let mut START: i32 = 0;
        let mut FIRST: bool = false;

        BUFFRD = 0;
        CURRNT = 1;
        FIRST = true;

        Self {
            LINES,
            BEGEND,
            BUFFRD,
            CURRNT,
            N,
            RANGE,
            BACKUP,
            BSIZE,
            GETAT,
            GOTTEN,
            LAST,
            PUTAT,
            QUIT,
            START,
            FIRST,
        }
    }
}

fn PREV(N: i32, RANGE: i32) -> i32 {
    ((N - 1) + ((((RANGE - N) + 1) / RANGE) * RANGE))
}

fn NEXT(N: i32, RANGE: i32) -> i32 {
    ((N + 1) - ((N / RANGE) * RANGE))
}

//$Procedure      PSTACK (Save paragraphs of text in a paragraph stack)
pub fn PSTACK(DEPTH: i32, LINE: &[u8], BUFFER: CharArray) {

    //
    // Local Parameters
    //

    //
    // Spicelib Functions
    //

    //
    // Local Buffers
    //

    //
    // In-line function dummy arguments
    //
    //
    // In-line functions
    //
    //
    // Local Variables
    //

    //
    // In-line functions for computing the next and previous item
    // in a circular list of items.
    //
}

//$Procedure      RSTBUF (Reset paragraph buffering)
pub fn RSTBUF(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // On the first call to the buffering routines we need to
    // initialize our buffering pointers.
    //
    if save.FIRST {
        save.FIRST = false;

        save.CURRNT = 1;
        save.BUFFRD = 1;
        save.BEGEND[[PBEG, save.CURRNT]] = 1;
        save.BEGEND[[PNXT, save.CURRNT]] = 1;

        for I in 1..=MAXLIN {
            fstr::assign(save.LINES.get_mut(I), b" ");
        }
    } else {
        //
        // Store the current buffer pointer and compute the
        // next one.
        //
        save.BUFFRD = intrinsics::MIN0(&[(save.BUFFRD + 1), NPGRPH]);
        save.LAST = save.CURRNT;
        save.RANGE = NPGRPH;
        save.CURRNT = NEXT(save.CURRNT, save.RANGE);
        //
        // Now compute the pointers to the beginning and ending of
        // data in the buffer that saves input lines.
        //
        save.RANGE = MAXLIN;
        save.BEGEND[[PBEG, save.CURRNT]] = NEXT(save.BEGEND[[PNXT, save.LAST]], save.RANGE);
        save.BEGEND[[PNXT, save.CURRNT]] = save.BEGEND[[PBEG, save.CURRNT]];
    }
}

//$Procedure      PUTBUF ( Put a line of text in the paragraph buffer )
pub fn PUTBUF(LINE: &[u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    // If things haven't already been initialized, we do so now.
    //
    if save.FIRST {
        save.FIRST = false;

        save.CURRNT = 1;
        save.BUFFRD = 1;
        save.BEGEND[[PBEG, save.CURRNT]] = 1;
        save.BEGEND[[PNXT, save.CURRNT]] = 1;

        for I in 1..=MAXLIN {
            fstr::assign(save.LINES.get_mut(I), b" ");
        }
    }

    //
    // Store the input line.
    //
    save.RANGE = MAXLIN;
    save.PUTAT = save.BEGEND[[PNXT, save.CURRNT]];
    fstr::assign(save.LINES.get_mut(save.PUTAT), LINE);
    //
    // Find out where to put the next line of input.
    //
    save.BEGEND[[PNXT, save.CURRNT]] = NEXT(save.PUTAT, save.RANGE);
}

//$Procedure      GETBUF (Get a paragraph at specified depth in a buffer)
pub fn GETBUF(DEPTH: i32, BUFFER: CharArrayMut, ctx: &mut Context) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut BUFFER = DummyCharArrayMut::new(BUFFER, None, LBCELL..);

    //
    // First empty the buffer where we will be sending the buffered
    // inputs.
    //
    save.BSIZE = spicelib::SIZEC(BUFFER.as_arg(), ctx)?;
    spicelib::SSIZEC(save.BSIZE, BUFFER.as_arg_mut(), ctx)?;
    //
    // DEPTH represents how deep we want to push down into the
    // buffer of items.  1 is the current, 2 is immediately before
    // that and so on...
    //
    save.BACKUP = intrinsics::MIN0(&[(DEPTH - 1), (save.BUFFRD - 1)]);

    if (save.BACKUP < 0) {
        //
        // This is probably a mistake, but we will not pass any
        // moral judgements on the request to get data, we simply
        // return the buffer empty.
        //
        return Ok(());
    }
    //
    // Backup from the current position the appropriate number to
    // find out where to get the buffered input lines.
    //
    save.GETAT = save.CURRNT;
    save.RANGE = NPGRPH;

    for I in 1..=save.BACKUP {
        save.GETAT = PREV(save.GETAT, save.RANGE);
    }

    save.START = save.BEGEND[[PBEG, save.GETAT]];
    save.QUIT = save.BEGEND[[PNXT, save.GETAT]];
    save.GOTTEN = 0;
    save.RANGE = MAXLIN;

    while ((save.START != save.QUIT) && (save.GOTTEN <= save.BSIZE)) {
        save.GOTTEN = (save.GOTTEN + 1);
        fstr::assign(BUFFER.get_mut(save.GOTTEN), save.LINES.get(save.START));
        save.START = NEXT(save.START, save.RANGE);
    }

    spicelib::SCARDC(save.GOTTEN, BUFFER.as_arg_mut(), ctx)?;

    Ok(())
}

//$Procedure      GETBSZ (Get current size of paragraph buffer)
pub fn GETBSZ(DEPTH: &mut i32, ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    *DEPTH = save.BUFFRD;
}

//$Procedure      DMPBUF ( Dump the last buffered paragraph )
pub fn DMPBUF(ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    save.BUFFRD = intrinsics::MAX0(&[(save.BUFFRD - 1), 0]);
    save.RANGE = NPGRPH;
    save.CURRNT = PREV(save.CURRNT, save.RANGE);
}
