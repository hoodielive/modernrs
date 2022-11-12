pub mod my_mod 
{
    pub fn create_box()
    {
        // Allocate an integer on the heap.
        let _box1 = Box::new(3i32);

        // _box1 is destroyed here, and memory gets freed.
    }
}

