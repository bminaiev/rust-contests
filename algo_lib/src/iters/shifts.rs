#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Shift {
    pub dx: i32,
    pub dy: i32,
}

// x goes down
// y goes right
pub const SHIFT_DOWN: Shift = Shift { dx: 1, dy: 0 };
pub const SHIFT_UP: Shift = Shift { dx: -1, dy: 0 };
pub const SHIFT_RIGHT: Shift = Shift { dx: 0, dy: 1 };
pub const SHIFT_LEFT: Shift = Shift { dx: 0, dy: -1 };

pub const SHIFTS_4: [Shift; 4] = [SHIFT_DOWN, SHIFT_LEFT, SHIFT_UP, SHIFT_RIGHT];
pub const SHIFTS_8: [Shift; 8] = [
    SHIFT_DOWN,
    SHIFT_LEFT,
    SHIFT_UP,
    SHIFT_RIGHT,
    Shift { dx: -1, dy: -1 },
    Shift { dx: -1, dy: 1 },
    Shift { dx: 1, dy: -1 },
    Shift { dx: 1, dy: 1 },
];

pub const SHIFTS_9: [Shift; 9] = [
    SHIFT_DOWN,
    SHIFT_LEFT,
    SHIFT_UP,
    SHIFT_RIGHT,
    Shift { dx: -1, dy: -1 },
    Shift { dx: -1, dy: 1 },
    Shift { dx: 1, dy: -1 },
    Shift { dx: 1, dy: 1 },
    Shift { dx: 0, dy: 0 },
];

pub fn shift_by_nswe(c: u8) -> Shift {
    match c {
        b'S' | b's' => SHIFT_DOWN,
        b'N' | b'n' => SHIFT_UP,
        b'E' | b'e' => SHIFT_RIGHT,
        b'W' | b'w' => SHIFT_LEFT,
        _ => panic!("Unexpected direction!"),
    }
}

pub fn shift_by_uldr(c: u8) -> Shift {
    match c {
        b'D' | b'd' => SHIFT_DOWN,
        b'U' | b'u' => SHIFT_UP,
        b'R' | b'r' => SHIFT_RIGHT,
        b'L' | b'l' => SHIFT_LEFT,
        _ => panic!("Unexpected direction!"),
    }
}
