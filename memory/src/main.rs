fn main() {
    let s1 = String::from("Hello");
    println!("s1: {}", s1);

    let s2 = s1;
    println!("s2: {}", s2);
    // s1 has been moved to s2, s1 now doesn't point to the String and is considered uninitialized
    // println!("s1: {}", s1); would now give compile time error
    

    // can't just pass in s2 and return len, when passing in, it gets moved to the function, so if 
    // we don't return it disapears
    let (s3, len) = find_len_weird(s2);
    println!("no reference : {}, {}", s3, len);

    // instead pass by reference, so it doesn't take ownership
    let len2 = find_len(&s3);
    println!("reference: {}, {}", s3, len2);

    // if we want to pass by reference and change it, pass by mut reference
    let mut s4 = String::from("change");
    println!("before : {}", s4);
    change_str(&mut s4);
    println!("after : {}", s4);

    // can not have immutable and mutable reference to same memory
    let mut s5 = String::from("my string");
    let ref1 = &s5;
    let ref2 = &s5;
    // let ref3 = &mut s5; this line would break here, can't mut data that has immutable references

    println!("references to same string : {}, {}", ref1, ref2);
    // can do it here because the scope of ref1 and ref2 end on the println, aren't used again
    let _ref3 = &mut s5;

    // println!("references to same string : {}, {}", ref1, ref2);  here would break it again, would extend the scope

    // cannot have multiple mutable references in same scope
    // let ref4 = &mut s5;  just adding this line doesn't break it, because ref3 scope is only its line
    // println!("mutable references to same string : {}, {}", ref3, ref4);  this would break because ref3 and ref4 in same scope

    // string slices, pointer to string and length of the slice
    let whole = String::from("hello world");
    let hello = &whole[..5];  // same as [0..5], takes first 5 letters
    let world = &whole[6..];  // same as [6..11], takes last 5 letters
    let _literal_hello = "hello";  // string slice to hello in memory

    println!("{}, {}", hello, world);

    let a = [1, 2, 3, 4];
    let slice_a = &a[1..3]; // array slice of type &[i32] containing [2, 3]
    println!("{}, {}", slice_a[0], slice_a[1]);

}
// the passed in string is moved into st
// Moving
fn find_len_weird(st : String) -> (String, usize) {
    let length = st.len();
    return (st, length);
}

// st is essentially a pointer to the passed in string, immutable by default
// Borrowing
fn find_len(st : &String) -> usize {
    return st.len();
}

// change string via mut reference
fn change_str(st: &mut String) {
    st.push_str(", changgee");
}

/* breaks because function returns reference to nothing, s is deallocated after function completes
fn dangle() -> &String {
    let s = String::from("hi");
    &s
}
*/