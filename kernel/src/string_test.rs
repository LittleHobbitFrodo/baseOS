
use ministd::{test_only, testing, String, Vec};
use ministd::{print, println};



#[testing(STRING)]
fn with_capacity() {
    let mut s: String = String::with_capacity(10);

    // The String contains no chars, even though it has capacity for more
    assert_eq!(s.len(), 0);

    // These are all done without reallocating...
    let cap = s.capacity();
    for _ in 0..10 {
        s.push(b'a');
    }

    assert_eq!(s.capacity(), cap);

    // ...but this may make the string reallocate
    s.push(b'a');

}

#[testing(STRING)]
fn from_raw_parts() {
    unsafe {
        let s: String = String::from("hello");

        // Prevent automatically dropping the String's data
        let mut s = ministd::mem::ManuallyDrop::new(s);

        let ptr = s.as_mut_ptr();
        let len = s.len();
        let capacity = s.capacity();

        let s = String::from_raw_parts(ptr, len, capacity);

        let ss: String = String::from("hello");
        assert_eq!(ss, s);
    }
}

#[testing(STRING)]
fn into_bytes() {
    let s: String = String::from("hello");
    let bytes = s.into_bytes();

    assert_eq!(bytes, [104u8, 101, 108, 108, 111]);
}

#[testing(STRING)]
fn as_str() {
    let s: String = String::from("foo");

    assert_eq!("foo", s.as_str());
}


#[testing(STRING)]
fn as_mut_str() {
    let mut s: String = String::from("foobar");
    let s_mut_str = s.as_mut_str();

    s_mut_str.make_ascii_uppercase();

    assert_eq!("FOOBAR", s_mut_str);
}

#[testing(STRING)]
fn push_str() {
    let mut s: String = String::from("foo");

    s.push_str("bar");

    assert_eq!("foobar", s);
}

#[testing(STRING)]
fn extend_from_within() {
    let mut string: String = String::from("abcde");

    string.extend_from_within(2..);
    assert_eq!(string, "abcdecde");

    string.extend_from_within(..2);
    assert_eq!(string, "abcdecdeab");

    string.extend_from_within(4..8);
    assert_eq!(string, "abcdecdeabecde");

}

#[testing(STRING)]
pub fn capacity() {
    let s: String = String::with_capacity(10);

    assert!(s.capacity() >= 10);
}



#[testing(STRING)]
pub fn reserve() {
    let mut s: String = String::with_capacity(10);
    s.push(b'a');
    s.push(b'b');

    // s now has a length of 2 and a capacity of at least 10
    let capacity = s.capacity();
    assert_eq!(2, s.len());
    assert!(capacity >= 10);

    // Since we already have at least an extra 8 capacity, calling this...
    s.reserve(8);

    // ... doesn't actually increase.
    assert_eq!(capacity, s.capacity());

}

#[testing(STRING)]
fn reserve_exact() {

    let mut s: String = String::new();

    s.reserve_exact(10);

    assert!(s.capacity() >= 10);

}

#[testing(STRING)]
fn shrink_to_fit() {
    let mut s: String = String::from("foo");

    s.reserve(100);
    assert!(s.capacity() >= 100);

    s.shrink_to_fit();
    assert_eq!(4, s.capacity());
        //  4 because the size is aligned to STEP

}

#[testing(STRING)]
fn shrink_to() {
    let mut s: String = String::from("foo");
    s.reserve(100);
    assert!(s.capacity() >= 100);

    s.shrink_to(10);
    assert!(s.capacity() >= 10);
    s.shrink_to(0);
    assert!(s.capacity() >= 3);
}

#[testing(STRING)]
fn push() {
    let mut s: String = String::from("abc");

    s.push(b'1');
    s.push(b'2');
    s.push(b'3');

    assert_eq!("abc123", s);
}

#[testing(STRING)]
fn as_bytes() {
    let s: String = String::from("hello");

    assert_eq!([104, 101, 108, 108, 111], s.as_bytes());
}

#[testing(STRING)]
fn truncate() {
    let mut s: String = String::from("hello");

    s.truncate(2);

    assert_eq!("he", s);
}

#[testing(STRING)]
fn pop() {
    let mut s: String = String::from("abc");

    assert_eq!(s.pop(), Some(b'c'));
    assert_eq!(s.pop(), Some(b'b'));
    assert_eq!(s.pop(), Some(b'a'));

    assert_eq!(s.pop(), None);
}


#[testing(STRING)]
fn remove() {
    let mut s: String = String::from("abc");

    s.remove(0);
    assert_eq!("bc", s);
    s.remove(1);
    assert_eq!("b", s);
    s.remove(0);
    assert_eq!("", "");
}

#[testing(String)]
fn retain() {
    let mut s: String = String::from("f_o_ob_ar");

    s.retain(|c| c != b'_');

    assert_eq!(s, "foobar");
}

#[testing(STRING)]
fn insert() {
    let mut s: String = String::with_capacity(3);

    s.insert(0, b'f');
    s.insert(1, b'o');
    s.insert(2, b'o');

    assert_eq!("foo", s);
}

#[testing(STRING)]
fn insert_str() {
    let mut s: String = String::from("bar");

    s.insert_str(0, "foo");

    assert_eq!("foobar", s);

}

#[testing(STRING)]
fn as_mut_vec() {
    let mut s: String = String::from("hello");

    unsafe {
        let vec = s.as_mut_vec();
        assert_eq!(&[104, 101, 108, 108, 111][..], unsafe { vec.as_slice_unchecked() } );

        vec.reverse();
    }
    assert_eq!(s, "olleh");
}

#[testing(STRING)]
fn len() {
    let a: String = String::from("foo");
    assert_eq!(a.len(), 3);

    let fancy_f: String = String::from("ƒoo");
    assert_eq!(fancy_f.len(), 4);
    assert_eq!(fancy_f.chars().count(), 3);
}

#[testing(STRING)]
fn empty() {
    let mut v: String = String::new();
    assert!(v.is_empty());

    v.push(b'a');
    assert!(!v.is_empty());
}

#[testing(STRING)]
fn split_off() {
    let mut hello: String = String::from("Hello, World!");
    let world = hello.split_off(7);
    assert_eq!(hello, "Hello, ");
    assert_eq!(world, "World!");
}

#[testing(STRING)]
fn clear() {
    let mut s: String = String::from("foo");

    s.clear();

    assert!(s.is_empty());
    assert_eq!(0, s.len());
}



