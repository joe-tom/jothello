pub fn build_all () {

}



fn move_diagl () {

  for white in 0..8 {
    for black in 0.8 {

      if (white&black) != 0 {
        continue;
      }


    }
  }
}





fn diagl_transform (item: u8) -> u64 {
  let thing: u64 = ((((item as u64) >> 7) & 1) << 63)
        | ((((item as u64) >> 6) & 1) << 54)
        | ((((item as u64) >> 5) & 1) << 45)
        | ((((item as u64) >> 4) & 1) << 36)
        | ((((item as u64) >> 3) & 1) << 27)
        | ((((item as u64) >> 2) & 1) << 18)
        | ((((item as u64) >> 1) & 1) << 09)
        | ((((item as u64) >> 0) & 1) << 00);

  return thing;
}



/*
  00,01,02,03,04,05,06,07,
  08,09,10,11,12,13,14,15,
  16,17,18,19,20,21,22,23,
  24,25,26,27,28,29,30,31,    
  32,33,34,35,36,37,38,39,
  40,41,42,43,44,45,46,47,
  48,49,50,51,52,53,54,55,
  56,57,58,59,60,61,62,63


  63,62,61,60,59,58,57,56,
  55,54,53,52,51,50,49,48,
  47,46,45,44,43,42,41,40,
  39,38,37,36,35,34,33,32,
  31,30,29,28,27,26,25,24,
  23,22,21,20,19,18,17,16,
  15,14,13,12,11,10,09,08,
  07,06,05,04,03,02,01,00
*/























