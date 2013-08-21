pub fn decode_u8(ch:char) -> Option<u8> {
  match ch {
    '0' => Some(0),
    '1' => Some(1),
    '2' => Some(2),
    '3' => Some(3),
    '4' => Some(4),
    '5' => Some(5),
    '6' => Some(6),
    '7' => Some(7),
    '8' => Some(8),
    '9' => Some(9),

    'b' => Some(10),
    'c' => Some(11),
    'd' => Some(12),
    'e' => Some(13),
    'f' => Some(14),
    'g' => Some(15),
    'h' => Some(16),
    'j' => Some(17),
    'k' => Some(18),
    'm' => Some(19),
    'n' => Some(20),
    'p' => Some(21),
    'q' => Some(22),
    'r' => Some(23),
    's' => Some(24),
    't' => Some(25),
    'u' => Some(26),
    'v' => Some(27),
    'w' => Some(28),
    'x' => Some(29),
    'y' => Some(30),
    'z' => Some(31),
    _   => None
  }
}

pub fn decode_tuple(ch:char) -> Option<(bool, bool, bool, bool, bool)> {
  match decode_u8(ch) {
    None     => None,
    Some(ch) => Some((
                is_bit_set(ch, BIT5),
                is_bit_set(ch, BIT4),
                is_bit_set(ch, BIT3),
                is_bit_set(ch, BIT2),
                is_bit_set(ch, BIT1)
                ))
  }
}

pub fn encode(val:u8) -> Option<char> {
  match val {
    0   => Some('0'),
    1   => Some('1'),
    2   => Some('2'),
    3   => Some('3'),
    4   => Some('4'),
    5   => Some('5'),
    6   => Some('6'),
    7   => Some('7'),
    8   => Some('8'),
    9   => Some('9'),
    10  => Some('b'),
    11  => Some('c'),
    12  => Some('d'),
    13  => Some('e'),
    14  => Some('f'),
    15  => Some('g'),
    16  => Some('h'),
    17  => Some('j'),
    18  => Some('k'),
    19  => Some('m'),
    20  => Some('n'),
    21  => Some('p'),
    22  => Some('q'),
    23  => Some('r'),
    24  => Some('s'),
    25  => Some('t'),
    26  => Some('u'),
    27  => Some('v'),
    28  => Some('w'),
    29  => Some('x'),
    30  => Some('y'),
    31  => Some('z'),
    _   => None
  }
}

static BIT1: u8 = 1 << 0;
static BIT2: u8 = 1 << 1;
static BIT3: u8 = 1 << 2;
static BIT4: u8 = 1 << 3;
static BIT5: u8 = 1 << 4;


fn is_bit_set(byte:u8, position:u8) -> bool {
  (byte & position) > 0
}
