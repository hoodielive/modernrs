#![allow(dead_code)]
#![allow(unused_variables)]

fn main()
{
   let mut v: Vec<i32> = Vec::new();
   v.push(1);
   v.push(2);
   v.push(3);
   v.push(4);

   println!("Time {:?}", &v);

   let third: &i32 = &v[2];
   println!("The third element is {}", third);

   let third: Option<&i32> = v.get(6);

   match third
   {
     Some(third) => println!("The third element is {}", third),
     None => println!("There is no third element."),
   }

   let mut ve = vec![1, 2, 3, 4, 5];
//   let does_not_exist = &ve[100]; // will generate error - use &ve.get(100) instead.
   let does_not_exist = &ve.get(100);

   println!("Does it exist? {:?}", does_not_exist);

   for i in &ve
   {
     println!("{}", i);
   }

   for x in &mut ve
   {
     *x += 50;
     println!("{}", x);
   }
}
