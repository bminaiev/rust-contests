#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Shift {
    pub dx: i32,
    pub dy: i32,
}

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

pub fn shift_by_char(c: u8) -> Shift {
    match c {
        b'S' => SHIFT_DOWN,
        b'N' => SHIFT_UP,
        b'E' => SHIFT_RIGHT,
        b'W' => SHIFT_LEFT,
        _ => panic!("Unexpected direction!"),
    }
}
