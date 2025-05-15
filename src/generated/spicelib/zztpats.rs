//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const COUNT: i32 = 231;
const WDSIZE: i32 = 32;

struct SaveVars {
    MYKNWN: ActualCharArray,
    MYMNNG: ActualCharArray,
    FIRST: bool,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut MYKNWN = ActualCharArray::new(WDSIZE, 1..=COUNT);
        let mut MYMNNG = ActualCharArray::new(WDSIZE, 1..=COUNT);
        let mut FIRST: bool = false;

        FIRST = true;

        Self {
            MYKNWN,
            MYMNNG,
            FIRST,
        }
    }
}

//$Procedure ZZTPATS (Private, Time --- Time Patterns)
pub fn ZZTPATS(
    ROOM: i32,
    NKNOWN: &mut i32,
    KNOWN: CharArrayMut,
    MEANNG: CharArrayMut,
    ctx: &mut Context,
) -> bool {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    let mut KNOWN = DummyCharArrayMut::new(KNOWN, None, 1..);
    let mut MEANNG = DummyCharArrayMut::new(MEANNG, None, 1..);
    let mut ZZTPATS: bool = false;
    let mut ORDVEC = StackArray::<i32, 231>::new(1..=COUNT);

    if save.FIRST {
        save.FIRST = false;

        fstr::assign(save.MYKNWN.get_mut(1), b"Y-i-it");
        fstr::assign(save.MYMNNG.get_mut(1), b"Y*m*D*");

        fstr::assign(save.MYKNWN.get_mut(2), b"Y-i-iti:i");
        fstr::assign(save.MYMNNG.get_mut(2), b"Y*m*D*H*M");

        fstr::assign(save.MYKNWN.get_mut(3), b"Y-i-iti:i:i");
        fstr::assign(save.MYMNNG.get_mut(3), b"Y*m*D*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(4), b"Y-i-iti:i:n");
        fstr::assign(save.MYMNNG.get_mut(4), b"Y*m*D*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(5), b"Y-i-iti:n");
        fstr::assign(save.MYMNNG.get_mut(5), b"Y*m*D*H*M");

        fstr::assign(save.MYKNWN.get_mut(6), b"Y-i/");
        fstr::assign(save.MYMNNG.get_mut(6), b"Y*y*");

        fstr::assign(save.MYKNWN.get_mut(7), b"Y-i/i:i");
        fstr::assign(save.MYMNNG.get_mut(7), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(8), b"Y-i/i:i:i");
        fstr::assign(save.MYMNNG.get_mut(8), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(9), b"Y-i/i:i:n");
        fstr::assign(save.MYMNNG.get_mut(9), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(10), b"Y-i/i:n");
        fstr::assign(save.MYMNNG.get_mut(10), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(11), b"Y-id");
        fstr::assign(save.MYMNNG.get_mut(11), b"Y*y*");

        fstr::assign(save.MYKNWN.get_mut(12), b"Y-idi:i");
        fstr::assign(save.MYMNNG.get_mut(12), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(13), b"Y-idi:i:i");
        fstr::assign(save.MYMNNG.get_mut(13), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(14), b"Y-idi:i:n");
        fstr::assign(save.MYMNNG.get_mut(14), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(15), b"Y-idi:n");
        fstr::assign(save.MYMNNG.get_mut(15), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(16), b"Y-it");
        fstr::assign(save.MYMNNG.get_mut(16), b"Y*y*");

        fstr::assign(save.MYKNWN.get_mut(17), b"Y-iti:i");
        fstr::assign(save.MYMNNG.get_mut(17), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(18), b"Y-iti:i:i");
        fstr::assign(save.MYMNNG.get_mut(18), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(19), b"Y-iti:i:n");
        fstr::assign(save.MYMNNG.get_mut(19), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(20), b"Y-iti:n");
        fstr::assign(save.MYMNNG.get_mut(20), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(21), b"Yid");
        fstr::assign(save.MYMNNG.get_mut(21), b"Yy*");

        fstr::assign(save.MYKNWN.get_mut(22), b"Yidi:i");
        fstr::assign(save.MYMNNG.get_mut(22), b"Yy*H*M");

        fstr::assign(save.MYKNWN.get_mut(23), b"Yidi:i:i");
        fstr::assign(save.MYMNNG.get_mut(23), b"Yy*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(24), b"Yidi:i:n");
        fstr::assign(save.MYMNNG.get_mut(24), b"Yy*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(25), b"Yidi:n");
        fstr::assign(save.MYMNNG.get_mut(25), b"Yy*H*M");

        fstr::assign(save.MYKNWN.get_mut(26), b"Yii");
        fstr::assign(save.MYMNNG.get_mut(26), b"YmD");

        fstr::assign(save.MYKNWN.get_mut(27), b"Yiii");
        fstr::assign(save.MYMNNG.get_mut(27), b"YmDH");

        fstr::assign(save.MYKNWN.get_mut(28), b"Yiii:i");
        fstr::assign(save.MYMNNG.get_mut(28), b"YmDH*M");

        fstr::assign(save.MYKNWN.get_mut(29), b"Yiii:i:i");
        fstr::assign(save.MYMNNG.get_mut(29), b"YmDH*M*S");

        fstr::assign(save.MYKNWN.get_mut(30), b"Yiii:i:n");
        fstr::assign(save.MYMNNG.get_mut(30), b"YmDH*M*S");

        fstr::assign(save.MYKNWN.get_mut(31), b"Yiii:n");
        fstr::assign(save.MYMNNG.get_mut(31), b"YmDH*M");

        fstr::assign(save.MYKNWN.get_mut(32), b"Yiiii");
        fstr::assign(save.MYMNNG.get_mut(32), b"YmDHM");

        fstr::assign(save.MYKNWN.get_mut(33), b"Yiiiii");
        fstr::assign(save.MYMNNG.get_mut(33), b"YmDHMS");

        fstr::assign(save.MYKNWN.get_mut(34), b"Yiiiin");
        fstr::assign(save.MYMNNG.get_mut(34), b"YmDHMS");

        fstr::assign(save.MYKNWN.get_mut(35), b"Yiiin");
        fstr::assign(save.MYMNNG.get_mut(35), b"YmDHM");

        fstr::assign(save.MYKNWN.get_mut(36), b"Yiin");
        fstr::assign(save.MYMNNG.get_mut(36), b"YmDH");

        fstr::assign(save.MYKNWN.get_mut(37), b"Yim");
        fstr::assign(save.MYMNNG.get_mut(37), b"YDm");

        fstr::assign(save.MYKNWN.get_mut(38), b"Yimi");
        fstr::assign(save.MYMNNG.get_mut(38), b"YDmH");

        fstr::assign(save.MYKNWN.get_mut(39), b"Yimi:i");
        fstr::assign(save.MYMNNG.get_mut(39), b"YDmH*M");

        fstr::assign(save.MYKNWN.get_mut(40), b"Yimi:i:i");
        fstr::assign(save.MYMNNG.get_mut(40), b"YDmH*M*S");

        fstr::assign(save.MYKNWN.get_mut(41), b"Yimi:i:n");
        fstr::assign(save.MYMNNG.get_mut(41), b"YDmH*M*S");

        fstr::assign(save.MYKNWN.get_mut(42), b"Yimi:n");
        fstr::assign(save.MYMNNG.get_mut(42), b"YDmH*M");

        fstr::assign(save.MYKNWN.get_mut(43), b"Yimn");
        fstr::assign(save.MYMNNG.get_mut(43), b"YDmH");

        fstr::assign(save.MYKNWN.get_mut(44), b"Yin");
        fstr::assign(save.MYMNNG.get_mut(44), b"YmD");

        fstr::assign(save.MYKNWN.get_mut(45), b"Ymi");
        fstr::assign(save.MYMNNG.get_mut(45), b"YmD");

        fstr::assign(save.MYKNWN.get_mut(46), b"Ymii");
        fstr::assign(save.MYMNNG.get_mut(46), b"YmDH");

        fstr::assign(save.MYKNWN.get_mut(47), b"Ymii:i");
        fstr::assign(save.MYMNNG.get_mut(47), b"YmDH*M");

        fstr::assign(save.MYKNWN.get_mut(48), b"Ymii:i:i");
        fstr::assign(save.MYMNNG.get_mut(48), b"YmDH*M*S");

        fstr::assign(save.MYKNWN.get_mut(49), b"Ymii:i:n");
        fstr::assign(save.MYMNNG.get_mut(49), b"YmDH*M*S");

        fstr::assign(save.MYKNWN.get_mut(50), b"Ymii:n");
        fstr::assign(save.MYMNNG.get_mut(50), b"YmDH*M");

        fstr::assign(save.MYKNWN.get_mut(51), b"Ymin");
        fstr::assign(save.MYMNNG.get_mut(51), b"YmDH");

        fstr::assign(save.MYKNWN.get_mut(52), b"Ymn");
        fstr::assign(save.MYMNNG.get_mut(52), b"YmD");

        fstr::assign(save.MYKNWN.get_mut(53), b"Ynm");
        fstr::assign(save.MYMNNG.get_mut(53), b"YDm");

        fstr::assign(save.MYKNWN.get_mut(54), b"i-Y/");
        fstr::assign(save.MYMNNG.get_mut(54), b"y*Y*");

        fstr::assign(save.MYKNWN.get_mut(55), b"i-Y/i:i");
        fstr::assign(save.MYMNNG.get_mut(55), b"y*Y*H*M");

        fstr::assign(save.MYKNWN.get_mut(56), b"i-Y/i:i:i");
        fstr::assign(save.MYMNNG.get_mut(56), b"y*Y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(57), b"i-Y/i:i:n");
        fstr::assign(save.MYMNNG.get_mut(57), b"y*Y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(58), b"i-Y/i:n");
        fstr::assign(save.MYMNNG.get_mut(58), b"y*Y*H*M");

        fstr::assign(save.MYKNWN.get_mut(59), b"i-Yd");
        fstr::assign(save.MYMNNG.get_mut(59), b"y*Y*");

        fstr::assign(save.MYKNWN.get_mut(60), b"i-Ydi:i");
        fstr::assign(save.MYMNNG.get_mut(60), b"y*Y*H*M");

        fstr::assign(save.MYKNWN.get_mut(61), b"i-Ydi:i:i");
        fstr::assign(save.MYMNNG.get_mut(61), b"y*Y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(62), b"i-Ydi:i:n");
        fstr::assign(save.MYMNNG.get_mut(62), b"y*Y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(63), b"i-Ydi:n");
        fstr::assign(save.MYMNNG.get_mut(63), b"y*Y*H*M");

        fstr::assign(save.MYKNWN.get_mut(64), b"i-i-it");
        fstr::assign(save.MYMNNG.get_mut(64), b"Y*m*D*");

        fstr::assign(save.MYKNWN.get_mut(65), b"i-i-iti:i");
        fstr::assign(save.MYMNNG.get_mut(65), b"Y*m*D*H*M");

        fstr::assign(save.MYKNWN.get_mut(66), b"i-i-iti:i:i");
        fstr::assign(save.MYMNNG.get_mut(66), b"Y*m*D*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(67), b"i-i-iti:i:n");
        fstr::assign(save.MYMNNG.get_mut(67), b"Y*m*D*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(68), b"i-i-iti:n");
        fstr::assign(save.MYMNNG.get_mut(68), b"Y*m*D*H*M");

        fstr::assign(save.MYKNWN.get_mut(69), b"i-i/i:i");
        fstr::assign(save.MYMNNG.get_mut(69), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(70), b"i-i/i:i:i");
        fstr::assign(save.MYMNNG.get_mut(70), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(71), b"i-i/i:i:n");
        fstr::assign(save.MYMNNG.get_mut(71), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(72), b"i-i/i:n");
        fstr::assign(save.MYMNNG.get_mut(72), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(73), b"i-idi:i");
        fstr::assign(save.MYMNNG.get_mut(73), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(74), b"i-idi:i:i");
        fstr::assign(save.MYMNNG.get_mut(74), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(75), b"i-idi:i:n");
        fstr::assign(save.MYMNNG.get_mut(75), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(76), b"i-idi:n");
        fstr::assign(save.MYMNNG.get_mut(76), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(77), b"i-it");
        fstr::assign(save.MYMNNG.get_mut(77), b"Y*y*");

        fstr::assign(save.MYKNWN.get_mut(78), b"i-iti:i");
        fstr::assign(save.MYMNNG.get_mut(78), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(79), b"i-iti:i:i");
        fstr::assign(save.MYMNNG.get_mut(79), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(80), b"i-iti:i:n");
        fstr::assign(save.MYMNNG.get_mut(80), b"Y*y*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(81), b"i-iti:n");
        fstr::assign(save.MYMNNG.get_mut(81), b"Y*y*H*M");

        fstr::assign(save.MYKNWN.get_mut(82), b"i:i:iimY");
        fstr::assign(save.MYMNNG.get_mut(82), b"H*M*SDmY");

        fstr::assign(save.MYKNWN.get_mut(83), b"i:i:imiY");
        fstr::assign(save.MYMNNG.get_mut(83), b"H*M*SmDY");

        fstr::assign(save.MYKNWN.get_mut(84), b"i:i:nimY");
        fstr::assign(save.MYMNNG.get_mut(84), b"H*M*SDmY");

        fstr::assign(save.MYKNWN.get_mut(85), b"i:i:nmiY");
        fstr::assign(save.MYMNNG.get_mut(85), b"H*M*SmDY");

        fstr::assign(save.MYKNWN.get_mut(86), b"i:iimY");
        fstr::assign(save.MYMNNG.get_mut(86), b"H*MDmY");

        fstr::assign(save.MYKNWN.get_mut(87), b"i:imiY");
        fstr::assign(save.MYMNNG.get_mut(87), b"H*MmDY");

        fstr::assign(save.MYKNWN.get_mut(88), b"i:nimY");
        fstr::assign(save.MYMNNG.get_mut(88), b"H*MDmY");

        fstr::assign(save.MYKNWN.get_mut(89), b"i:nmiY");
        fstr::assign(save.MYMNNG.get_mut(89), b"H*MmDY");

        fstr::assign(save.MYKNWN.get_mut(90), b"iYd");
        fstr::assign(save.MYMNNG.get_mut(90), b"yY*");

        fstr::assign(save.MYKNWN.get_mut(91), b"iYdi:i");
        fstr::assign(save.MYMNNG.get_mut(91), b"yY*H*M");

        fstr::assign(save.MYKNWN.get_mut(92), b"iYdi:i:i");
        fstr::assign(save.MYMNNG.get_mut(92), b"yY*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(93), b"iYdi:i:n");
        fstr::assign(save.MYMNNG.get_mut(93), b"yY*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(94), b"iYdi:n");
        fstr::assign(save.MYMNNG.get_mut(94), b"yY*H*M");

        fstr::assign(save.MYKNWN.get_mut(95), b"iiY");
        fstr::assign(save.MYMNNG.get_mut(95), b"mDY");

        fstr::assign(save.MYKNWN.get_mut(96), b"iiYi");
        fstr::assign(save.MYMNNG.get_mut(96), b"mDYH");

        fstr::assign(save.MYKNWN.get_mut(97), b"iiYi:i");
        fstr::assign(save.MYMNNG.get_mut(97), b"mDYH*M");

        fstr::assign(save.MYKNWN.get_mut(98), b"iiYi:i:i");
        fstr::assign(save.MYMNNG.get_mut(98), b"mDYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(99), b"iiYi:i:n");
        fstr::assign(save.MYMNNG.get_mut(99), b"mDYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(100), b"iiYi:n");
        fstr::assign(save.MYMNNG.get_mut(100), b"mDYH*M");

        fstr::assign(save.MYKNWN.get_mut(101), b"iiYn");
        fstr::assign(save.MYMNNG.get_mut(101), b"mDYH");

        fstr::assign(save.MYKNWN.get_mut(102), b"iid");
        fstr::assign(save.MYMNNG.get_mut(102), b"Yy*");

        fstr::assign(save.MYKNWN.get_mut(103), b"iidi:i");
        fstr::assign(save.MYMNNG.get_mut(103), b"Yy*H*M");

        fstr::assign(save.MYKNWN.get_mut(104), b"iidi:i:i");
        fstr::assign(save.MYMNNG.get_mut(104), b"Yy*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(105), b"iidi:i:n");
        fstr::assign(save.MYMNNG.get_mut(105), b"Yy*H*M*S");

        fstr::assign(save.MYKNWN.get_mut(106), b"iidi:n");
        fstr::assign(save.MYMNNG.get_mut(106), b"Yy*H*M");

        fstr::assign(save.MYKNWN.get_mut(107), b"iim");
        fstr::assign(save.MYMNNG.get_mut(107), b"YDm");

        fstr::assign(save.MYKNWN.get_mut(108), b"iimi");
        fstr::assign(save.MYMNNG.get_mut(108), b"YDmH");

        fstr::assign(save.MYKNWN.get_mut(109), b"iimi:i");
        fstr::assign(save.MYMNNG.get_mut(109), b"YDmH*M");

        fstr::assign(save.MYKNWN.get_mut(110), b"iimi:i:i");
        fstr::assign(save.MYMNNG.get_mut(110), b"YDmH*M*S");

        fstr::assign(save.MYKNWN.get_mut(111), b"iimi:i:n");
        fstr::assign(save.MYMNNG.get_mut(111), b"YDmH*M*S");

        fstr::assign(save.MYKNWN.get_mut(112), b"iimi:n");
        fstr::assign(save.MYMNNG.get_mut(112), b"YDmH*M");

        fstr::assign(save.MYKNWN.get_mut(113), b"iimii");
        fstr::assign(save.MYMNNG.get_mut(113), b"YDmHM");

        fstr::assign(save.MYKNWN.get_mut(114), b"iimiii");
        fstr::assign(save.MYMNNG.get_mut(114), b"YDmHMS");

        fstr::assign(save.MYKNWN.get_mut(115), b"iimiin");
        fstr::assign(save.MYMNNG.get_mut(115), b"YDmHMS");

        fstr::assign(save.MYKNWN.get_mut(116), b"iimin");
        fstr::assign(save.MYMNNG.get_mut(116), b"YDmHM");

        fstr::assign(save.MYKNWN.get_mut(117), b"iimn");
        fstr::assign(save.MYMNNG.get_mut(117), b"YDmH");

        fstr::assign(save.MYKNWN.get_mut(118), b"imY");
        fstr::assign(save.MYMNNG.get_mut(118), b"DmY");

        fstr::assign(save.MYKNWN.get_mut(119), b"imYi");
        fstr::assign(save.MYMNNG.get_mut(119), b"DmYH");

        fstr::assign(save.MYKNWN.get_mut(120), b"imYi:i");
        fstr::assign(save.MYMNNG.get_mut(120), b"DmYH*M");

        fstr::assign(save.MYKNWN.get_mut(121), b"imYi:i:i");
        fstr::assign(save.MYMNNG.get_mut(121), b"DmYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(122), b"imYi:i:n");
        fstr::assign(save.MYMNNG.get_mut(122), b"DmYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(123), b"imYi:n");
        fstr::assign(save.MYMNNG.get_mut(123), b"DmYH*M");

        fstr::assign(save.MYKNWN.get_mut(124), b"imYn");
        fstr::assign(save.MYMNNG.get_mut(124), b"DmYH");

        fstr::assign(save.MYKNWN.get_mut(125), b"imi");
        fstr::assign(save.MYMNNG.get_mut(125), b"YmD");

        fstr::assign(save.MYKNWN.get_mut(126), b"imi:i:iY");
        fstr::assign(save.MYMNNG.get_mut(126), b"DmH*M*SY");

        fstr::assign(save.MYKNWN.get_mut(127), b"imi:i:nY");
        fstr::assign(save.MYMNNG.get_mut(127), b"DmH*M*SY");

        fstr::assign(save.MYKNWN.get_mut(128), b"imi:iY");
        fstr::assign(save.MYMNNG.get_mut(128), b"DmH*MY");

        fstr::assign(save.MYKNWN.get_mut(129), b"imi:nY");
        fstr::assign(save.MYMNNG.get_mut(129), b"DmH*MY");

        fstr::assign(save.MYKNWN.get_mut(130), b"imii");
        fstr::assign(save.MYMNNG.get_mut(130), b"YmDH");

        fstr::assign(save.MYKNWN.get_mut(131), b"imii:i");
        fstr::assign(save.MYMNNG.get_mut(131), b"YmDH*M");

        fstr::assign(save.MYKNWN.get_mut(132), b"imii:i:i");
        fstr::assign(save.MYMNNG.get_mut(132), b"YmDH*M*S");

        fstr::assign(save.MYKNWN.get_mut(133), b"imii:i:n");
        fstr::assign(save.MYMNNG.get_mut(133), b"YmDH*M*S");

        fstr::assign(save.MYKNWN.get_mut(134), b"imii:n");
        fstr::assign(save.MYMNNG.get_mut(134), b"YmDH*M");

        fstr::assign(save.MYKNWN.get_mut(135), b"imiii");
        fstr::assign(save.MYMNNG.get_mut(135), b"YmDHM");

        fstr::assign(save.MYKNWN.get_mut(136), b"imiiii");
        fstr::assign(save.MYMNNG.get_mut(136), b"YmDHMS");

        fstr::assign(save.MYKNWN.get_mut(137), b"imiiin");
        fstr::assign(save.MYMNNG.get_mut(137), b"YmDHMS");

        fstr::assign(save.MYKNWN.get_mut(138), b"imiin");
        fstr::assign(save.MYMNNG.get_mut(138), b"YmDHM");

        fstr::assign(save.MYKNWN.get_mut(139), b"imin");
        fstr::assign(save.MYMNNG.get_mut(139), b"YmDH");

        fstr::assign(save.MYKNWN.get_mut(140), b"imn");
        fstr::assign(save.MYMNNG.get_mut(140), b"YmD");

        fstr::assign(save.MYKNWN.get_mut(141), b"inY");
        fstr::assign(save.MYMNNG.get_mut(141), b"mDY");

        fstr::assign(save.MYKNWN.get_mut(142), b"inm");
        fstr::assign(save.MYMNNG.get_mut(142), b"YDm");

        fstr::assign(save.MYKNWN.get_mut(143), b"miY");
        fstr::assign(save.MYMNNG.get_mut(143), b"mDY");

        fstr::assign(save.MYKNWN.get_mut(144), b"miYi");
        fstr::assign(save.MYMNNG.get_mut(144), b"mDYH");

        fstr::assign(save.MYKNWN.get_mut(145), b"miYi:i");
        fstr::assign(save.MYMNNG.get_mut(145), b"mDYH*M");

        fstr::assign(save.MYKNWN.get_mut(146), b"miYi:i:i");
        fstr::assign(save.MYMNNG.get_mut(146), b"mDYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(147), b"miYi:i:n");
        fstr::assign(save.MYMNNG.get_mut(147), b"mDYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(148), b"miYi:n");
        fstr::assign(save.MYMNNG.get_mut(148), b"mDYH*M");

        fstr::assign(save.MYKNWN.get_mut(149), b"miYn");
        fstr::assign(save.MYMNNG.get_mut(149), b"mDYH");

        fstr::assign(save.MYKNWN.get_mut(150), b"mii");
        fstr::assign(save.MYMNNG.get_mut(150), b"mDY");

        fstr::assign(save.MYKNWN.get_mut(151), b"mii:i:iY");
        fstr::assign(save.MYMNNG.get_mut(151), b"mDH*M*SY");

        fstr::assign(save.MYKNWN.get_mut(152), b"mii:i:nY");
        fstr::assign(save.MYMNNG.get_mut(152), b"mDH*M*SY");

        fstr::assign(save.MYKNWN.get_mut(153), b"mii:iY");
        fstr::assign(save.MYMNNG.get_mut(153), b"mDH*MY");

        fstr::assign(save.MYKNWN.get_mut(154), b"mii:nY");
        fstr::assign(save.MYMNNG.get_mut(154), b"mDH*MY");

        fstr::assign(save.MYKNWN.get_mut(155), b"miii");
        fstr::assign(save.MYMNNG.get_mut(155), b"mDYH");

        fstr::assign(save.MYKNWN.get_mut(156), b"miii:i");
        fstr::assign(save.MYMNNG.get_mut(156), b"mDYH*M");

        fstr::assign(save.MYKNWN.get_mut(157), b"miii:i:i");
        fstr::assign(save.MYMNNG.get_mut(157), b"mDYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(158), b"miii:i:n");
        fstr::assign(save.MYMNNG.get_mut(158), b"mDYH*M*S");

        fstr::assign(save.MYKNWN.get_mut(159), b"miii:n");
        fstr::assign(save.MYMNNG.get_mut(159), b"mDYH*M");

        fstr::assign(save.MYKNWN.get_mut(160), b"miiii");
        fstr::assign(save.MYMNNG.get_mut(160), b"mDYHM");

        fstr::assign(save.MYKNWN.get_mut(161), b"miiiii");
        fstr::assign(save.MYMNNG.get_mut(161), b"mDYHMS");

        fstr::assign(save.MYKNWN.get_mut(162), b"miiiin");
        fstr::assign(save.MYMNNG.get_mut(162), b"mDYHMS");

        fstr::assign(save.MYKNWN.get_mut(163), b"miiin");
        fstr::assign(save.MYMNNG.get_mut(163), b"mDYHM");

        fstr::assign(save.MYKNWN.get_mut(164), b"miin");
        fstr::assign(save.MYMNNG.get_mut(164), b"mDYH");

        fstr::assign(save.MYKNWN.get_mut(165), b"mnY");
        fstr::assign(save.MYMNNG.get_mut(165), b"mDY");

        fstr::assign(save.MYKNWN.get_mut(166), b"mni");
        fstr::assign(save.MYMNNG.get_mut(166), b"mDY");

        fstr::assign(save.MYKNWN.get_mut(167), b"nmY");
        fstr::assign(save.MYMNNG.get_mut(167), b"DmY");

        fstr::assign(save.MYKNWN.get_mut(168), b"i/i/i");
        fstr::assign(save.MYMNNG.get_mut(168), b"m*D*Y");

        fstr::assign(save.MYKNWN.get_mut(169), b"i/i/ii:i");
        fstr::assign(save.MYMNNG.get_mut(169), b"m*D*YH*M");

        fstr::assign(save.MYKNWN.get_mut(170), b"i/i/ii:n");
        fstr::assign(save.MYMNNG.get_mut(170), b"m*D*YH*M");

        fstr::assign(save.MYKNWN.get_mut(171), b"i/i/ii:i:n");
        fstr::assign(save.MYMNNG.get_mut(171), b"m*D*YH*M*S");

        fstr::assign(save.MYKNWN.get_mut(172), b"i/i/ii:i:i");
        fstr::assign(save.MYMNNG.get_mut(172), b"m*D*YH*M*S");

        fstr::assign(save.MYKNWN.get_mut(173), b"i/i/Y");
        fstr::assign(save.MYMNNG.get_mut(173), b"m*D*Y");

        fstr::assign(save.MYKNWN.get_mut(174), b"i/i/Yi:i");
        fstr::assign(save.MYMNNG.get_mut(174), b"m*D*YH*M");

        fstr::assign(save.MYKNWN.get_mut(175), b"i/i/ii:n");
        fstr::assign(save.MYMNNG.get_mut(175), b"m*D*YH*M");

        fstr::assign(save.MYKNWN.get_mut(176), b"i/i/Yi:i:n");
        fstr::assign(save.MYMNNG.get_mut(176), b"m*D*YH*M*S");

        fstr::assign(save.MYKNWN.get_mut(177), b"i/i/Yi:i:i");
        fstr::assign(save.MYMNNG.get_mut(177), b"m*D*YH*M*S");

        fstr::assign(save.MYKNWN.get_mut(178), b"Y-i-iti");
        fstr::assign(save.MYMNNG.get_mut(178), b"Y*m*D*H");

        fstr::assign(save.MYKNWN.get_mut(179), b"Y-iti");
        fstr::assign(save.MYMNNG.get_mut(179), b"Y*y*H");

        fstr::assign(save.MYKNWN.get_mut(180), b"Y-i-itn");
        fstr::assign(save.MYMNNG.get_mut(180), b"Y*m*D*H");

        fstr::assign(save.MYKNWN.get_mut(181), b"Y-itn");
        fstr::assign(save.MYMNNG.get_mut(181), b"Y*y*H");

        fstr::assign(save.MYKNWN.get_mut(182), b"i-i-iti");
        fstr::assign(save.MYMNNG.get_mut(182), b"Y*m*D*H");

        fstr::assign(save.MYKNWN.get_mut(183), b"i-i-itn");
        fstr::assign(save.MYMNNG.get_mut(183), b"Y*m*D*H");

        fstr::assign(save.MYKNWN.get_mut(184), b"i-iti");
        fstr::assign(save.MYMNNG.get_mut(184), b"Y*y*H");

        fstr::assign(save.MYKNWN.get_mut(185), b"i-itn");
        fstr::assign(save.MYMNNG.get_mut(185), b"Y*y*H");

        fstr::assign(save.MYKNWN.get_mut(186), b"i:ii/i/i");
        fstr::assign(save.MYMNNG.get_mut(186), b"H*Mm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(187), b"i:ni/i/i");
        fstr::assign(save.MYMNNG.get_mut(187), b"H*Mm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(188), b"i:i:ii/i/i");
        fstr::assign(save.MYMNNG.get_mut(188), b"H*M*Sm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(189), b"i:i:ni/i/i");
        fstr::assign(save.MYMNNG.get_mut(189), b"H*M*Sm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(190), b"i:ii/i/Y");
        fstr::assign(save.MYMNNG.get_mut(190), b"H*Mm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(191), b"i:ni/i/Y");
        fstr::assign(save.MYMNNG.get_mut(191), b"H*Mm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(192), b"i:i:ii/i/Y");
        fstr::assign(save.MYMNNG.get_mut(192), b"H*M*Sm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(193), b"i:i:ni/i/Y");
        fstr::assign(save.MYMNNG.get_mut(193), b"H*M*Sm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(194), b"i:ii-i-Y");
        fstr::assign(save.MYMNNG.get_mut(194), b"H*Mm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(195), b"i:ni-i-Y");
        fstr::assign(save.MYMNNG.get_mut(195), b"H*Mm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(196), b"i:i:ii-i-Y");
        fstr::assign(save.MYMNNG.get_mut(196), b"H*M*Sm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(197), b"i:i:ni-i-Y");
        fstr::assign(save.MYMNNG.get_mut(197), b"H*M*Sm*D*Y");

        fstr::assign(save.MYKNWN.get_mut(198), b"i/i/Y/i:n");
        fstr::assign(save.MYMNNG.get_mut(198), b"m*D*Y*H*M");

        fstr::assign(save.MYKNWN.get_mut(199), b"i-i-Y");
        fstr::assign(save.MYMNNG.get_mut(199), b"m*D*Y");

        fstr::assign(save.MYKNWN.get_mut(200), b"i-i-Yi:n");
        fstr::assign(save.MYMNNG.get_mut(200), b"m*D*YH*M");

        fstr::assign(save.MYKNWN.get_mut(201), b"i-i-Yi:i:n");
        fstr::assign(save.MYMNNG.get_mut(201), b"m*D*YH*M*S");

        fstr::assign(save.MYKNWN.get_mut(202), b"i-i-Yi:i:i");
        fstr::assign(save.MYMNNG.get_mut(202), b"m*D*YH*M*S");

        fstr::assign(save.MYKNWN.get_mut(203), b"i-i-Yi:i");
        fstr::assign(save.MYMNNG.get_mut(203), b"m*D*YH*M");

        //
        // ISO patterns allowing the 'Z' suffix, only a suffix.
        //
        fstr::assign(save.MYKNWN.get_mut(204), b"Y-i-itx");
        fstr::assign(save.MYMNNG.get_mut(204), b"Y*m*D**");

        fstr::assign(save.MYKNWN.get_mut(205), b"Y-i-iti:ix");
        fstr::assign(save.MYMNNG.get_mut(205), b"Y*m*D*H*M*");

        fstr::assign(save.MYKNWN.get_mut(206), b"Y-i-iti:i:ix");
        fstr::assign(save.MYMNNG.get_mut(206), b"Y*m*D*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(207), b"Y-i-iti:i:nx");
        fstr::assign(save.MYMNNG.get_mut(207), b"Y*m*D*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(208), b"Y-i-iti:nx");
        fstr::assign(save.MYMNNG.get_mut(208), b"Y*m*D*H*M*");

        fstr::assign(save.MYKNWN.get_mut(209), b"Y-itx");
        fstr::assign(save.MYMNNG.get_mut(209), b"Y*y**");

        fstr::assign(save.MYKNWN.get_mut(210), b"Y-iti:ix");
        fstr::assign(save.MYMNNG.get_mut(210), b"Y*y*H*M*");

        fstr::assign(save.MYKNWN.get_mut(211), b"Y-iti:i:ix");
        fstr::assign(save.MYMNNG.get_mut(211), b"Y*y*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(212), b"Y-iti:i:nx");
        fstr::assign(save.MYMNNG.get_mut(212), b"Y*y*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(213), b"Y-iti:nx");
        fstr::assign(save.MYMNNG.get_mut(213), b"Y*y*H*M*");

        fstr::assign(save.MYKNWN.get_mut(214), b"i-i-itx");
        fstr::assign(save.MYMNNG.get_mut(214), b"Y*m*D**");

        fstr::assign(save.MYKNWN.get_mut(215), b"i-i-iti:ix");
        fstr::assign(save.MYMNNG.get_mut(215), b"Y*m*D*H*M*");

        fstr::assign(save.MYKNWN.get_mut(216), b"i-i-iti:i:ix");
        fstr::assign(save.MYMNNG.get_mut(216), b"Y*m*D*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(217), b"i-i-iti:i:nx");
        fstr::assign(save.MYMNNG.get_mut(217), b"Y*m*D*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(218), b"i-i-iti:nx");
        fstr::assign(save.MYMNNG.get_mut(218), b"Y*m*D*H*M*");

        fstr::assign(save.MYKNWN.get_mut(219), b"i-itx");
        fstr::assign(save.MYMNNG.get_mut(219), b"Y*y**");

        fstr::assign(save.MYKNWN.get_mut(220), b"i-iti:ix");
        fstr::assign(save.MYMNNG.get_mut(220), b"Y*y*H*M*");

        fstr::assign(save.MYKNWN.get_mut(221), b"i-iti:i:ix");
        fstr::assign(save.MYMNNG.get_mut(221), b"Y*y*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(222), b"i-iti:i:nx");
        fstr::assign(save.MYMNNG.get_mut(222), b"Y*y*H*M*S*");

        fstr::assign(save.MYKNWN.get_mut(223), b"i-iti:nx");
        fstr::assign(save.MYMNNG.get_mut(223), b"Y*y*H*M*");

        fstr::assign(save.MYKNWN.get_mut(224), b"Y-i-itix");
        fstr::assign(save.MYMNNG.get_mut(224), b"Y*m*D*H*");

        fstr::assign(save.MYKNWN.get_mut(225), b"Y-itix");
        fstr::assign(save.MYMNNG.get_mut(225), b"Y*y*H*");

        fstr::assign(save.MYKNWN.get_mut(226), b"Y-i-itnx");
        fstr::assign(save.MYMNNG.get_mut(226), b"Y*m*D*H*");

        fstr::assign(save.MYKNWN.get_mut(227), b"Y-itnx");
        fstr::assign(save.MYMNNG.get_mut(227), b"Y*y*H*");

        fstr::assign(save.MYKNWN.get_mut(228), b"i-i-itix");
        fstr::assign(save.MYMNNG.get_mut(228), b"Y*m*D*H*");

        fstr::assign(save.MYKNWN.get_mut(229), b"i-i-itnx");
        fstr::assign(save.MYMNNG.get_mut(229), b"Y*m*D*H*");

        fstr::assign(save.MYKNWN.get_mut(230), b"i-itix");
        fstr::assign(save.MYMNNG.get_mut(230), b"Y*y*H*");

        fstr::assign(save.MYKNWN.get_mut(231), b"i-itnx");
        fstr::assign(save.MYMNNG.get_mut(231), b"Y*y*H*");
    }

    //
    // Copy as many patterns and meanings as the input arrays allow.
    //
    *NKNOWN = intrinsics::MIN0(&[COUNT, ROOM]);
    for I in 1..=*NKNOWN {
        fstr::assign(KNOWN.get_mut(I), save.MYKNWN.get(I));
        fstr::assign(MEANNG.get_mut(I), save.MYMNNG.get(I));
    }

    //
    // Make sure everything is in the proper order.
    //
    ORDERC(KNOWN.as_arg(), *NKNOWN, ORDVEC.as_slice_mut());
    REORDC(ORDVEC.as_slice_mut(), *NKNOWN, KNOWN.as_arg_mut());
    REORDC(ORDVEC.as_slice_mut(), *NKNOWN, MEANNG.as_arg_mut());

    //
    // If there wasn't sufficient room to get all of the patterns
    // and meanings, return FALSE.
    //
    if (COUNT > ROOM) {
        ZZTPATS = false;
        return ZZTPATS;
    }

    ZZTPATS = true;

    ZZTPATS
}
