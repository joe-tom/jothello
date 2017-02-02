
use std::fmt;

// Enumeration for stone colour type, in array lookup.
static WHITE: usize = 0;
static BLACK: usize = 1;

// Consider adding an array specifically for diagonals.
static TRANSFORM: [[[u32; 8]; 6561] = [[0; 8]; 6561];

// Lookup table for binary to ternary transfromation.
static BT: [u32; 256] = [0; 256];
static BT2: [u32; 256] = [0; 256];



fn build_bin_to_trin () {
  for num in 0..256 {
    // Parse as ternary, the binary stuff.
    let val = u32::from_str_radix(format!("{:b}", num));

    // Update the bianry table and the 2*binary ternary table.
    BT2[num] = 2 * (BT[num] = val);
  }
}

fn build_transform () {
  // Iterate through every combination of white and black stones.
  for black in 0..256 {
    for white in 0..256 {

      // Ignore invalid positions. i.e. intersections within stones.
      if white&black {
        continue;
      }


      // Duplicate white and black.
      let new_black = black;
      let new_white = white;

      // Go through each square.
      for start in 0..8 {
        if new_white >> start & 1
      }
    }
  }
}









