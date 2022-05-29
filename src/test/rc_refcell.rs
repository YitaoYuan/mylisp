use std::rc::Rc;
use std::cell::*;
use std::borrow::*;
fn main()  // 编译错误
{ 
  let a : Rc<RefCell<i32>> = Rc::new(RefCell::new(0));
  let c = &a;
  let b = c.clone();
  //a = ();
  let x : &RefCell<i32> = a.borrow();
  let y : RefMut<i32> = x.borrow_mut();
  drop(y);
  //*y = ();
  *(a.borrow() as &RefCell<i32>).borrow_mut() += 1;
  *(b.borrow() as &RefCell<i32>).borrow_mut() += 1;
  //println!("{}", *(b.borrow().borrow()) as i32);
  println!("{}", *(Borrow::<RefCell<i32>>::borrow(&a).borrow()) as i32);
  println!("OK");
}

