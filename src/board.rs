static TRANSFORM: [[u16; 16]; 65536] = [[0; 16]; 65536];
static UNDO_TRANSFORM: [[u16; 16]; 65536] = [[0; 16]; 65536];

#[derive(Debug)]
struct Board {
  horiz: [u16; 8],
  verti: [u16; 8],
  diagr: [u16; 8],
  diagl: [u16; 8]
};

impl Board {
  fn place_piece (&mut self, location: usize) {
    let mov_h: u8 = (location / 8);
    let mov_v: u8 = (location % 8);

    self.horiz[mov_h] = TRANSFORM[self.horiz[mov_h]][location];
  }
}

