
fn main()  
{  
  let mut rd = true;
  let mut b = Box::new(0);
  let mut x = & 0;
  let mut y = &mut 0;
  let mut i = & 0;
  if rd {
    x = b.as_ref();
  }
  else {
    y = b.as_mut();
  }
  if rd {
    i = b.as_ref();
  }
  if rd {
    println!("{}", x);
  }
  else {
    println!("{}", y);
  }
  println!("OK");
}

