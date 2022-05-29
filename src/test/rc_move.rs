use std::rc::Rc;

fn main()  
{  
  let mut rd = true;
  let mut b = Rc::new(0);// box想要分发多个引用只能通过native ref
  let mut x = b.clone();// 但是通过box创建的所有引用的生命周期都在box的生命周期内
  let mut y = b.clone();// 譬如想要在循环内像C一样将new出来的指针放在一个循环外的数据结构中，用box是无法实现的
  drop(b);        // 因此需要rc，将rc而非native ref放进数据结构就行
  println!("{}", *x.as_ref());
  println!("OK");
}

