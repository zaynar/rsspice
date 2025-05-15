//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NTITLE: i32 = 28;
const WDSIZE: i32 = 32;
const MSGSIZ: i32 = 400;

struct SaveVars {
    ENGLSH: ActualCharArray,
    FRENCH: ActualCharArray,
    GERMAN: ActualCharArray,
    RUSSAN: ActualCharArray,
    LANG: Vec<u8>,
    TITLE: ActualCharArray,
    IORDER: StackArray<i32, 28>,
    ITEM: i32,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut ENGLSH = ActualCharArray::new(MSGSIZ, 1..=NTITLE);
        let mut FRENCH = ActualCharArray::new(MSGSIZ, 1..=NTITLE);
        let mut GERMAN = ActualCharArray::new(MSGSIZ, 1..=NTITLE);
        let mut RUSSAN = ActualCharArray::new(MSGSIZ, 1..=NTITLE);
        let mut LANG = vec![b' '; WDSIZE as usize];
        let mut TITLE = ActualCharArray::new(WDSIZE, 1..=NTITLE);
        let mut IORDER = StackArray::<i32, 28>::new(1..=NTITLE);
        let mut ITEM: i32 = 0;
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            ENGLSH,
            FRENCH,
            GERMAN,
            RUSSAN,
            LANG,
            TITLE,
            IORDER,
            ITEM,
            FIRST,
        }
    }
}

//$ Disclaimer
//
//     THIS SOFTWARE AND ANY RELATED MATERIALS WERE CREATED BY THE
//     CALIFORNIA INSTITUTE OF TECHNOLOGY (CALTECH) UNDER A U.S.
//     GOVERNMENT CONTRACT WITH THE NATIONAL AERONAUTICS AND SPACE
//     ADMINISTRATION (NASA). THE SOFTWARE IS TECHNOLOGY AND SOFTWARE
//     PUBLICLY AVAILABLE UNDER U.S. EXPORT LAWS AND IS PROVIDED "AS-IS"
//     TO THE RECIPIENT WITHOUT WARRANTY OF ANY KIND, INCLUDING ANY
//     WARRANTIES OF PERFORMANCE OR MERCHANTABILITY OR FITNESS FOR A
//     PARTICULAR USE OR PURPOSE (AS SET FORTH IN UNITED STATES UCC
//     SECTIONS 2312-2313) OR FOR ANY PURPOSE WHATSOEVER, FOR THE
//     SOFTWARE AND RELATED MATERIALS, HOWEVER USED.
//
//     IN NO EVENT SHALL CALTECH, ITS JET PROPULSION LABORATORY, OR NASA
//     BE LIABLE FOR ANY DAMAGES AND/OR COSTS, INCLUDING, BUT NOT
//     LIMITED TO, INCIDENTAL OR CONSEQUENTIAL DAMAGES OF ANY KIND,
//     INCLUDING ECONOMIC DAMAGE OR INJURY TO PROPERTY AND LOST PROFITS,
//     REGARDLESS OF WHETHER CALTECH, JPL, OR NASA BE ADVISED, HAVE
//     REASON TO KNOW, OR, IN FACT, SHALL KNOW OF THE POSSIBILITY.
//
//     RECIPIENT BEARS ALL RISK RELATING TO QUALITY AND PERFORMANCE OF
//     THE SOFTWARE AND ANY RELATED MATERIALS, AND AGREES TO INDEMNIFY
//     CALTECH AND NASA FOR ALL THIRD-PARTY CLAIMS RESULTING FROM THE
//     ACTIONS OF RECIPIENT IN THE USE OF THE SOFTWARE.
//
pub fn TRNLAT(PHRASE: &[u8], MESSGE: &mut [u8], ctx: &mut Context) {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //

    if save.FIRST {
        save.FIRST = false;

        fstr::assign(save.TITLE.get_mut(1), b"ERRFLAG");

        fstr::assign(save.ENGLSH.get_mut(1), b"-Oops!-");
        fstr::assign(save.FRENCH.get_mut(1), b"--%-Sacre^Bleu!!-%^^:");
        fstr::assign(save.GERMAN.get_mut(1), b"--%-Achtung!!-%^^:");
        fstr::assign(save.RUSSAN.get_mut(1), b"--%-ERROR-%^^:");

        fstr::assign(save.TITLE.get_mut(2), b"EXIT");

        fstr::assign(save.ENGLSH.get_mut(2), b"EXIT");
        fstr::assign(save.FRENCH.get_mut(2), b"SORTIE");
        fstr::assign(save.GERMAN.get_mut(2), b"EXIT");
        fstr::assign(save.RUSSAN.get_mut(2), b"EXIT");

        fstr::assign(save.TITLE.get_mut(3), b"START");

        fstr::assign(save.ENGLSH.get_mut(3), b"START");
        fstr::assign(save.FRENCH.get_mut(3), b"COMMENCER");
        fstr::assign(save.GERMAN.get_mut(3), b"START");
        fstr::assign(save.RUSSAN.get_mut(3), b"START");

        fstr::assign(save.TITLE.get_mut(4), b"STOP");

        fstr::assign(save.ENGLSH.get_mut(4), b"STOP");
        fstr::assign(save.FRENCH.get_mut(4), b"ARETE");
        fstr::assign(save.GERMAN.get_mut(4), b"STOP");
        fstr::assign(save.RUSSAN.get_mut(4), b"STOP");

        fstr::assign(save.TITLE.get_mut(5), b"DEMO");

        fstr::assign(save.ENGLSH.get_mut(5), b"DEMO");
        fstr::assign(save.FRENCH.get_mut(5), b"MONTRER");
        fstr::assign(save.GERMAN.get_mut(5), b"DEMO");
        fstr::assign(save.RUSSAN.get_mut(5), b"DEMO");

        fstr::assign(save.TITLE.get_mut(6), b"PAUSE");

        fstr::assign(save.ENGLSH.get_mut(6), b"PAUSE");
        fstr::assign(save.FRENCH.get_mut(6), b"PAUSE");
        fstr::assign(save.GERMAN.get_mut(6), b"PAUSE");
        fstr::assign(save.RUSSAN.get_mut(6), b"PAUSE");

        fstr::assign(save.TITLE.get_mut(7), b"WAIT");

        fstr::assign(save.ENGLSH.get_mut(7), b"WAIT");
        fstr::assign(save.FRENCH.get_mut(7), b"ATTENDRE");
        fstr::assign(save.GERMAN.get_mut(7), b"WAIT");
        fstr::assign(save.RUSSAN.get_mut(7), b"WAIT");

        fstr::assign(save.TITLE.get_mut(8), b"QUIT");

        fstr::assign(save.ENGLSH.get_mut(8), b"QUIT");
        fstr::assign(save.FRENCH.get_mut(8), b"ARETE");
        fstr::assign(save.GERMAN.get_mut(8), b"QUIT");
        fstr::assign(save.RUSSAN.get_mut(8), b"QUIT");

        fstr::assign(save.TITLE.get_mut(9), b"DEFPROMPT");

        fstr::assign(save.ENGLSH.get_mut(9), b"Yes? >");
        fstr::assign(save.FRENCH.get_mut(9), b"Oui? >");
        fstr::assign(save.GERMAN.get_mut(9), b"Ja? >");
        fstr::assign(save.RUSSAN.get_mut(9), b"Dah? >");

        fstr::assign(save.TITLE.get_mut(10), b"MISSINGFILELONG");

        fstr::assign(
            save.ENGLSH.get_mut(10),
            b"No command sequence file was specified in the START command. ",
        );
        fstr::assign(
            save.FRENCH.get_mut(10),
            b"Le fichier command sequence n\'est pas present dans le command \"COMMENCER\". ",
        );

        fstr::assign(
            save.GERMAN.get_mut(10),
            b"No command sequence file was specified in the START command. ",
        );
        fstr::assign(
            save.RUSSAN.get_mut(10),
            b"No command sequence file was specified in the START command. ",
        );

        fstr::assign(save.TITLE.get_mut(11), b"MISSINGFILESHORT");

        fstr::assign(save.ENGLSH.get_mut(11), b"Missing_File_Name");
        fstr::assign(save.FRENCH.get_mut(11), b"Nom_de_fichier_abscent");
        fstr::assign(save.GERMAN.get_mut(11), b"Missing_File_Name");
        fstr::assign(save.RUSSAN.get_mut(11), b"Missing_File_Name");

        fstr::assign(save.TITLE.get_mut(12), b"UNABLETOSTART");

        fstr::assign(save.ENGLSH.get_mut(12), b"Unable_To_Start_File");
        fstr::assign(save.FRENCH.get_mut(12), b"Unable_To_Start_File");
        fstr::assign(save.GERMAN.get_mut(12), b"Unable_To_Start_File");
        fstr::assign(save.RUSSAN.get_mut(12), b"Unable_To_Start_File");

        fstr::assign(save.TITLE.get_mut(13), b"COMBUFFULLLNG");

        fstr::assign(save.ENGLSH.get_mut(13), b"The designer of the program has inadvertantly exceeded the internal command buffer.  Please keep your session log and report this problem to NAIF. ");
        fstr::assign(save.FRENCH.get_mut(13), b"The designer of the program has inadvertantly exceeded the internal command buffer.  Please keep your session log and report this problem to NAIF. ");
        fstr::assign(save.GERMAN.get_mut(13), b"The designer of the program has inadvertantly exceeded the internal command buffer.  Please keep your session log and report this problem to NAIF. ");
        fstr::assign(save.RUSSAN.get_mut(13), b"The designer of the program has inadvertantly exceeded the internal command buffer.  Please keep your session log and report this problem to NAIF. ");

        fstr::assign(save.TITLE.get_mut(14), b"COMBUFFULLSHT");

        fstr::assign(save.ENGLSH.get_mut(14), b"Command_Buffer_Full");
        fstr::assign(save.FRENCH.get_mut(14), b"Command_Buffer_Full");
        fstr::assign(save.GERMAN.get_mut(14), b"Command_Buffer_Full");
        fstr::assign(save.RUSSAN.get_mut(14), b"Command_Buffer_Full");

        fstr::assign(save.TITLE.get_mut(15), b"NESTINGTOODEEP");

        fstr::assign(save.ENGLSH.get_mut(15), b"The command sequence contained in # could not be started. There are already # command sequences files that have been started without resolution. This is the limit on the number of active command sequence files that can be active at any time. ");
        fstr::assign(save.FRENCH.get_mut(15), b"The command sequence contained in # could not be started. There are already # command sequences files that have been started without resolution. This is the limit on the number of active command sequence files that can be active at any time. ");
        fstr::assign(save.GERMAN.get_mut(15), b"The command sequence contained in # could not be started. There are already # command sequences files that have been started without resolution. This is the limit on the number of active command sequence files that can be active at any time. ");
        fstr::assign(save.RUSSAN.get_mut(15), b"The command sequence contained in # could not be started. There are already # command sequences files that have been started without resolution. This is the limit on the number of active command sequence files that can be active at any time. ");

        fstr::assign(save.TITLE.get_mut(16), b"NOLOGUNITSFREE");

        fstr::assign(save.ENGLSH.get_mut(16), b"The command sequence contained in # could not be started. There are no FORTRAN logical units available that can be attached to the file. A possible cause for this problem is that there are too many files already in use by the program. ");
        fstr::assign(save.FRENCH.get_mut(16), b"The command sequence contained in # could not be started. There are no FORTRAN logical units available that can be attached to the file. A possible cause for this problem is that there are too many files already in use by the program. ");
        fstr::assign(save.GERMAN.get_mut(16), b"The command sequence contained in # could not be started. There are no FORTRAN logical units available that can be attached to the file. A possible cause for this problem is that there are too many files already in use by the program. ");
        fstr::assign(save.RUSSAN.get_mut(16), b"The command sequence contained in # could not be started. There are no FORTRAN logical units available that can be attached to the file. A possible cause for this problem is that there are too many files already in use by the program. ");

        fstr::assign(save.TITLE.get_mut(17), b"FILENOTEXIST");

        fstr::assign(
            save.ENGLSH.get_mut(17),
            b"The file \"#\" could not be started. It doesn\'t exist. ",
        );
        fstr::assign(
            save.FRENCH.get_mut(17),
            b"The file \"#\" could not be started. It doesn\'t exist. ",
        );
        fstr::assign(
            save.GERMAN.get_mut(17),
            b"The file \"#\" could not be started. It doesn\'t exist. ",
        );
        fstr::assign(
            save.RUSSAN.get_mut(17),
            b"The file \"#\" could not be started. It doesn\'t exist. ",
        );

        fstr::assign(save.TITLE.get_mut(18), b"COMFILEOPENERROR");

        fstr::assign(save.ENGLSH.get_mut(18), b"The command sequence contained in # could not be started. An error occurred while attempting to open the file. ");
        fstr::assign(save.FRENCH.get_mut(18), b"The command sequence contained in # could not be started. An error occurred while attempting to open the file. ");
        fstr::assign(save.GERMAN.get_mut(18), b"The command sequence contained in # could not be started. An error occurred while attempting to open the file. ");
        fstr::assign(save.RUSSAN.get_mut(18), b"The command sequence contained in # could not be started. An error occurred while attempting to open the file. ");

        fstr::assign(save.TITLE.get_mut(19), b"LOGFILWRITTENTO");

        fstr::assign(
            save.ENGLSH.get_mut(19),
            b"The log file has been written to: ",
        );
        fstr::assign(
            save.FRENCH.get_mut(19),
            b"Le fichier de log s\'est ecrivee : ",
        );
        fstr::assign(
            save.GERMAN.get_mut(19),
            b"Das logenfile hass bin written to: ",
        );
        fstr::assign(
            save.RUSSAN.get_mut(19),
            b"The log file has been written to: ",
        );

        fstr::assign(save.TITLE.get_mut(20), b"SAVFILWRITTENTO");

        fstr::assign(
            save.ENGLSH.get_mut(20),
            b"The save file has been written to: ",
        );
        fstr::assign(
            save.FRENCH.get_mut(20),
            b"Le fichier de garde s\'est ecrivee : ",
        );
        fstr::assign(
            save.GERMAN.get_mut(20),
            b"Das savenfile hass bin written to: ",
        );
        fstr::assign(
            save.RUSSAN.get_mut(20),
            b"The save file has been written to: ",
        );

        fstr::assign(save.TITLE.get_mut(21), b"UNABLETOWRITETOFILE");

        fstr::assign(save.ENGLSH.get_mut(21), b"I was unable to write to the file: /cr/cr(3:3) # /cr/cr(-3;-3) The value of IOSTAT that was returned as a diagnosis of the problem was: /cr/cr(3:3) # /cr/cr(-3;-3) This file is now closed. No further attempts will be made to write to it. ");
        fstr::assign(save.FRENCH.get_mut(21), b"I was unable to write to the file: /cr/cr(3:3) # /cr/cr(-3;-3) The value of IOSTAT that was returned as a diagnosis of the problem was: /cr/cr(3:3) # /cr/cr(-3;-3) This file is now closed. No further attempts will be made to write to it. ");
        fstr::assign(save.GERMAN.get_mut(21), b"I was unable to write to the file: /cr/cr(3:3) # /cr/cr(-3;-3) The value of IOSTAT that was returned as a diagnosis of the problem was: /cr/cr(3:3) # /cr/cr(-3;-3) This file is now closed. No further attempts will be made to write to it. ");
        fstr::assign(save.RUSSAN.get_mut(21), b"I was unable to write to the file: /cr/cr(3:3) # /cr/cr(-3;-3) The value of IOSTAT that was returned as a diagnosis of the problem was: /cr/cr(3:3) # /cr/cr(-3;-3) This file is now closed. No further attempts will be made to write to it. ");

        fstr::assign(save.TITLE.get_mut(22), b"WARNING");

        fstr::assign(save.ENGLSH.get_mut(22), b"Warning:");
        fstr::assign(save.FRENCH.get_mut(22), b"Attention: ");
        fstr::assign(save.GERMAN.get_mut(22), b"Achtung: ");
        fstr::assign(save.RUSSAN.get_mut(22), b"Hey!! ");

        fstr::assign(save.TITLE.get_mut(23), b"CANNOTOPENLOG");

        fstr::assign(save.ENGLSH.get_mut(23), b"An error occurred while attempting to open the log file. It will not be possible to keep a log of this session. No further attempts to log commands will be attempted. /cr/cr The cause of the failure to open the log file was diagnosed to be: /cr/cr(3:3) ");
        fstr::assign(save.FRENCH.get_mut(23), b"An error occurred while attempting to open zee log file. It will not be possible to keep a log of this session. No further attempts to log commands will be attempted. /cr/cr Zee cause of zee failure to open zee log file was diagnosed to be: /cr/cr(3:3) ");
        fstr::assign(save.GERMAN.get_mut(23), b"An error occurred while attempting to open the log file. It will not be possible to keep a log of this session. No further attempts to log commands will be attempted. /cr/cr The cause of the failure to open the log file was diagnosed to be: /cr/cr(3:3) ");
        fstr::assign(save.GERMAN.get_mut(23), b"An error occurred while attempting to open the log file. It will not be possible to keep a log of this session. No further attempts to log commands will be attempted. /cr/cr The cause of the failure to open the log file was diagnosed to be: /cr/cr(3:3) ");

        fstr::assign(save.TITLE.get_mut(24), b"NOMOREDIAGNOSTICS");

        fstr::assign(
            save.ENGLSH.get_mut(24),
            b"Sorry, no further diagnostics are available.",
        );
        fstr::assign(
            save.FRENCH.get_mut(24),
            b"Mon ami, I am so sorry. I can say no more about zee error I reported earlier.",
        );
        fstr::assign(
            save.GERMAN.get_mut(24),
            b"No further diagnostics are available.",
        );
        fstr::assign(
            save.RUSSAN.get_mut(24),
            b"Sorry, no further diagnostics are available.",
        );

        fstr::assign(save.TITLE.get_mut(25), b"DONT");

        fstr::assign(save.ENGLSH.get_mut(25), b"NO");
        fstr::assign(save.FRENCH.get_mut(25), b"NO");
        fstr::assign(save.GERMAN.get_mut(25), b"NEIN");
        fstr::assign(save.RUSSAN.get_mut(25), b"NYET");

        fstr::assign(save.TITLE.get_mut(26), b"ECHO");

        fstr::assign(save.ENGLSH.get_mut(26), b"ECHO");
        fstr::assign(save.FRENCH.get_mut(26), b"ECHO");
        fstr::assign(save.GERMAN.get_mut(26), b"ECHO");
        fstr::assign(save.RUSSAN.get_mut(26), b"ECHO");

        fstr::assign(save.TITLE.get_mut(27), b"ERRFILWRITTENTO");

        fstr::assign(
            save.ENGLSH.get_mut(27),
            b"The error file has been written to: ",
        );
        fstr::assign(
            save.FRENCH.get_mut(27),
            b"The error file has been written to: ",
        );
        fstr::assign(
            save.GERMAN.get_mut(27),
            b"The error file has been written to: ",
        );
        fstr::assign(
            save.RUSSAN.get_mut(27),
            b"The error file has been written to: ",
        );

        fstr::assign(save.TITLE.get_mut(28), b"ERRFILWRITEFAIL");

        fstr::assign(
            save.ENGLSH.get_mut(28),
            b"WARNING--Unable to create the errorfile: ",
        );
        fstr::assign(
            save.FRENCH.get_mut(28),
            b"WARNING--Unable to create the errorfile: ",
        );
        fstr::assign(
            save.GERMAN.get_mut(28),
            b"WARNING--Unable to create the errorfile: ",
        );
        fstr::assign(
            save.RUSSAN.get_mut(28),
            b"WARNING--Unable to create the errorfile: ",
        );

        spicelib::ORDERC(save.TITLE.as_arg(), NTITLE, save.IORDER.as_slice_mut());
        spicelib::REORDC(save.IORDER.as_slice_mut(), NTITLE, save.TITLE.as_arg_mut());
        spicelib::REORDC(save.IORDER.as_slice_mut(), NTITLE, save.ENGLSH.as_arg_mut());
        spicelib::REORDC(save.IORDER.as_slice_mut(), NTITLE, save.FRENCH.as_arg_mut());
        spicelib::REORDC(save.IORDER.as_slice_mut(), NTITLE, save.GERMAN.as_arg_mut());
        spicelib::REORDC(save.IORDER.as_slice_mut(), NTITLE, save.RUSSAN.as_arg_mut());
    }

    save.ITEM = spicelib::BSRCHC(PHRASE, NTITLE, save.TITLE.as_arg());
    GETLAN(&mut save.LANG, ctx);

    if (save.ITEM == 0) {
        fstr::assign(MESSGE, PHRASE);
    } else if fstr::eq(&save.LANG, b"FRENCH") {
        fstr::assign(MESSGE, save.FRENCH.get(save.ITEM));
    } else if fstr::eq(&save.LANG, b"GERMAN") {
        fstr::assign(MESSGE, save.GERMAN.get(save.ITEM));
    } else if fstr::eq(&save.LANG, b"RUSSIAN") {
        fstr::assign(MESSGE, save.RUSSAN.get(save.ITEM));
    } else {
        fstr::assign(MESSGE, save.ENGLSH.get(save.ITEM));
    }
}
