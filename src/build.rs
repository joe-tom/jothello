
use std::fmt;

static TRANSFORM: [u32; 6561] = [0; 6561];

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
  for black in 0..256 {
    for white in 0..256 {

      // Go through each square
      for start in 0..8 {
        num | (1 << start);

      }
    }
  }
}









