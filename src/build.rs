
use std::fmt;

// Enumeration for stone colour type, in array lookup.
static WHITE: usize = 0;
static BLACK: usize = 1;

// Consider adding an array specifically for diagonals.
static TRANSFORM: [[u32; 8]; 6561] = [[0; 8]; 6561];

// Lookup table for binary to ternary transfromation.
static BT: [u32; 256] = [0; 256];
static BT2: [u32; 256] = [0; 256];



pub fn build_all () {
  build_bin_to_trin();
  build_transform();
}

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
      if (white&black) != 0 {
        continue;
      }
      // Initialize a taken number.
      let taken = white | black;

      // Initialize stone arrays.
      let mut white_arr: Vec<u8> = vec![];
      let mut black_arr: Vec<u8> = vec![];

      // Iterate through the bits to create bit vectors.
      for i in 0..8 {
        white_arr.push((white >> i) & 1);
        black_arr.push((black >> i) & 1);
      }

      // Find empty states. If the states are empty, go through!
      for i in 0..8 {
        if ((taken >> i) & 1) == 0 {
          black(white_arr.clone(), black_arr.clone(), i);
          white(white_arr.clone(), black_arr.clone(), i);
        }
      }
    }
  }
}


fn black (bit_arr_opp: Vec<u8>, bit_arr_you: Vec<u8>, point: u8) {
  let mut i = 0;
  
}

fn white (bit_arr_you: Vec<u8>, bit_arr_opp: Vec<u8>, point: u8) {
  if point == 0 {

  } else if point == 7 {

  } else {
    
  }
}