
fn main()  
{ 
  let mut n = 0;
  let x = & n;
  let y = &mut x; 
  println!("OK");
}

