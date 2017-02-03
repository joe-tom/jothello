

#[derive(Debug)]
struct Board {
  horiz: [u16; 8]
};

impl Board {
  fn place_piece (&mut self, location: usize) {
    let mov_h = HORIZ[location];
    let mov_v = VERTI[location];
    let mov_r = DIAGR[location];
    let mov_l = DIAGL[location];

    self.horiz[mov_h] = TRANSFORM[self.horiz[mov_h]][location][piece];
  }
}