use std::io::Result;

/**
 * plane_samba function
 * 
 * @param arg_rhythmtext: String
 * 
 * return None
 */
pub fn plane_samba(rhythmtext: String, charicon: String) {
  let split_rhythm: Vec<char> = rhythmtext.chars().collect();

  // check rhythmtext length
  if 2 != Vec::len(&split_rhythm) {
    panic!("I can't get on such rhythm (pointed out)");
  }

  // ホホホイ　ホホホイ ホホホイ　ホイ
  let first_clause: String = 
    format!("{}{}{}{}", 
      split_rhythm[0], 
      split_rhythm[0], 
      split_rhythm[0], 
      split_rhythm[1]);
  
  let last_clause: String = 
    format!("{}{}{}", 
      first_clause,
      split_rhythm[0], 
      split_rhythm[1]);

  // put samba rhythm
  println!("{}{}   {}{}   {}{}", 
    first_clause, charicon, 
    first_clause, charicon, 
    last_clause, charicon);
}


/**
 * puertorico_samba function
 * 
 * @param arg_rhythmtext: String
 * 
 * return None
 */
pub fn puertorico_samba(rhythmtext: String, charicon: String) {
  let split_rhythm: Vec<char> = rhythmtext.chars().collect();
  
  // check rhythmtext length
  if 2 != Vec::len(&split_rhythm) {
    panic!("I can't get on such rhythm (pointed out)");
  }

  // ホイホイ　ホホホホイ ホイホイホホイホホイ　ホイ
  let first_clause: String = 
    format!("{}{}{}{}", 
      split_rhythm[0], 
      split_rhythm[1],
      split_rhythm[0], 
      split_rhythm[1]);
  
  let second_clause: String = 
    format!("{}{}{}{}{}", 
      split_rhythm[0], 
      split_rhythm[0], 
      split_rhythm[0], 
      split_rhythm[0],
      split_rhythm[1]);

  let hoi_clause: String = 
    format!("{}{}", 
      split_rhythm[0], 
      split_rhythm[1]);

  let theard_clause: String = 
    format!("{}{}{}{}{}{}{}{}{}", 
      hoi_clause,
      hoi_clause,
      hoi_clause,
      split_rhythm[0], 
      split_rhythm[0], 
      split_rhythm[1],
      split_rhythm[0], 
      split_rhythm[0], 
      split_rhythm[1]);

  // put samba rhythm
  println!("{}{}   {}{}   {}{}   {}{}", 
    first_clause, charicon, 
    second_clause, charicon, 
    theard_clause, charicon, 
    hoi_clause, charicon);
}
