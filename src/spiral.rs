//! logic to compute dot movement.
pub const CENTER: (u8, u8) = (2, 2);
const SPIRAL_LEN: usize = 49;
const START: i8 = 24;
const COORD_MASK: i8 = 0b0011_1111u8 as i8;
const CLOCKWISE_MASK: i8 = 0b1000_0000u8 as i8;
const SPIRAL_TABLE: [(u8, u8); SPIRAL_LEN] = [
    (2, 2),
    (1, 3),
    (2, 3),
    (3, 3),
    (3, 2),
    (3, 1),
    (2, 1),
    (1, 1),
    (1, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
    (0, 0),
    (0, 1),
    (1, 2),
    (1, 3),
    (2, 3),
    (3, 3),
    (3, 2),
    (3, 1),
    (2, 1),
    (1, 1),
    (2, 2),
];
const LEFT: i8 = 0;
const RIGHT: i8 = SPIRAL_LEN as i8 - 1;

/// State of the dot.
pub struct DotState(i8);

impl DotState {
    /// create a new dot at starting point.
    pub const fn new() -> Self {
        Self(START)
    }

    /// the coordinate of the dot.
    #[inline]
    pub fn px(&self) -> &(u8, u8) {
        &SPIRAL_TABLE[self.pos() as usize]
    }

    /// spiral the dot.
    ///
    /// must call `toggle_clockwise` in center_cb
    /// before calling `spiral` again.
    /// Otherwise, the code might panic.
    pub fn spiral<CB: FnMut(&mut Self)>(&mut self, mut center_cb: CB) {
        let pos = self.pos();
        if let LEFT | RIGHT = pos {
            center_cb(self)
        }
        self.next();
    }

    #[inline]
    pub fn is_left(&self) -> bool {
        self.pos() == LEFT
    }

    #[inline]
    pub fn is_clockwise(&self) -> bool {
        (self.0 & CLOCKWISE_MASK) == 0
    }

    #[inline]
    pub fn toggle_clockwise(&mut self) {
        self.0 ^= CLOCKWISE_MASK;
    }

    #[inline]
    fn next(&mut self) {
        self.0 += self.is_clockwise() as i8;
        self.0 -= !self.is_clockwise() as i8;
    }

    #[inline]
    fn pos(&self) -> i8 {
        self.0 & COORD_MASK
    }
}
