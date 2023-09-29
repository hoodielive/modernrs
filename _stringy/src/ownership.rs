fn ownership_stuff() {
    let v = vec![1, 2, 3];
    // let v2 = v;

    println!("{:?}", v);

    let foo = |v: Vec<i32>| ();
    foo(v);

    // println!("{:?}", v);

    let u = 1;
    let u2 = u;
}
