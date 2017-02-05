static MAKE_TRANSFORM: [[u64; 16]; 26244] = [[u64; 16]; 26244];
static UNDO_TRANSFORM: [[u64; 16]; 26244] = [[u64; 16]; 26244];

#[derive(Debug)]
struct Board {
  white: u64,
  black: u64
};

impl Board {


  pub fn eval () {
    return white - black;
  }
}

