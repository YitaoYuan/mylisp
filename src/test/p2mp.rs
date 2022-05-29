
fn main()  // 编译错误
{ 
  let mut n = 0; // rust中的指针与C的const声明有区别
  let x = &mut n; // C的const ptr to ptr通过两次解引用是可以修改数据的，但rust不行
  let y = & x; // compiler accquire change to "&mut x" here
  let z = & x; // C的const只管一级，而rust的const管一颗树
  println!("{} {}", y, z);
  **y += 1;
  **z += 1;
  println!("OK");
}

