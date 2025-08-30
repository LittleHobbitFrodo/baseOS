

use ministd::{eprintln, test_only, testing, vec, String, Vec};
use ministd::{print, println};
#[test_only]
use ministd::DROP_COUNTER;

#[testing(VEC)]
fn with_capacity() {
    let mut vec: Vec<u8> = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert!(vec.capacity() >= 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert!(vec.capacity() >= 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);




}

#[testing(VEC)]
fn from_raw_parts() {
    use core::ptr;
    use core::mem;

    let v: Vec<usize> = vec![1, 2, 3];

    // Prevent running `v`'s destructor so we are in complete control
    // of the allocation.
    let mut v = mem::ManuallyDrop::new(v);

    // Pull out the various important pieces of information about `v`
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // Overwrite memory with 4, 5, 6
        for i in 0..len {
            ptr::write(p.add(i), 4 + i);
        }

        // Put everything back together into a Vec
        let rebuilt: Vec<usize> = Vec::from_raw_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
}

#[testing(VEC)]
fn from_parts() {
    use core::ptr::NonNull;
    use core::mem;

    let v: Vec<usize> = vec![1, 2, 3];

    // Prevent running `v`'s destructor so we are in complete control
    // of the allocation.
    let mut v = mem::ManuallyDrop::new(v);

    // Pull out the various important pieces of information about `v`
    let p = unsafe { NonNull::new_unchecked(v.as_mut_ptr()) };
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // Overwrite memory with 4, 5, 6
        for i in 0..len {
            p.add(i).write(4 + i);
        }

        // Put everything back together into a Vec
        let rebuilt: Vec<usize> = Vec::from_parts(p, len, cap);
        assert_eq!(rebuilt, [4, 5, 6]);
    }
}

#[testing(VEC)]
fn into_raw_parts() {
    let v: Vec<i32> = vec![-1, 0, 1];

    let (ptr, len, cap) = unsafe { v.into_raw_parts() };

    let rebuilt: Vec<i32> = unsafe {
        // We can now make changes to the components, such as
        // transmuting the raw pointer to a compatible type.
        let ptr = ptr as *mut i32;

        Vec::from_raw_parts(ptr, len, cap)
    };
    assert_eq!(rebuilt, [-1, 0, 1]);
}

#[testing(VEC)]
fn into_parts() {

    let v: Vec<i32> = vec![-1, 0, 1];

    let (ptr, len, cap) = unsafe { v.into_parts() };

    let rebuilt: Vec<u32> = unsafe {
        // We can now make changes to the components, such as
        // transmuting the raw pointer to a compatible type.
        let ptr = ptr.cast::<u32>();

        Vec::from_parts(ptr, len, cap)
    };
    assert_eq!(rebuilt, [4294967295, 0, 1]);
}

#[testing(VEC)]
fn reserve() {
    let mut vec: Vec<u32> = vec![1];
    vec.reserve(10);
    assert!(vec.capacity() >= 11);
}

#[testing(VEC)]
fn reserve_exact() {
    let mut vec: Vec<u32> = vec![1];
    vec.reserve_exact(10);
    assert!(vec.capacity() >= 11);
}

#[testing(VEC)]
fn shrink_to_fit() {
    let mut vec: Vec<usize> = Vec::with_capacity(10);
    //vec.extend([1, 2, 3]);
    vec.extend_from_slice(&[1, 2, 3]);
    assert!(vec.capacity() >= 10);
    vec.shrink_to_fit();
    assert!(vec.capacity() >= 3);
}


#[testing(VEC)]
fn shrink_to() {
    let mut vec: Vec<usize> = Vec::with_capacity(10);
    vec.extend_from_slice(&[1, 2, 3]);
    assert!(vec.capacity() >= 10);
    vec.shrink_to(4);
    assert!(vec.capacity() >= 4);

    vec.shrink_to(0);
    assert!(vec.capacity() >= 3);
}

#[testing(VEC)]
fn truncate() {
    let mut vec: Vec<usize> = vec![1, 2, 3, 4, 5];
    vec.truncate(2);
    assert_eq!(vec, [1, 2]);

    let mut vec: Vec<usize> = vec![1, 2, 3];
    vec.truncate(8);
    assert_eq!(vec, [1, 2, 3]);

    let mut vec: Vec<usize> = vec![1, 2, 3];
    vec.truncate(0);
    assert_eq!(vec, []);
}


#[testing(VEC)]
fn swap_remove() {
    let mut v: Vec<&'static str> = vec!["foo", "bar", "baz", "qux"];

    assert_eq!(v.swap_remove(1), "bar");
    assert_eq!(v, ["foo", "qux", "baz"]);

    assert_eq!(v.swap_remove(0), "foo");
    assert_eq!(v, ["baz", "qux"]);
}

#[testing(VEC)]
fn insert() {
    let mut vec: Vec<u8> = vec![b'a', b'b', b'c'];
    vec.insert(1, b'd');
    assert_eq!(vec, [b'a', b'd', b'b', b'c']);
    vec.insert(4, b'e');
    assert_eq!(vec, [b'a', b'd', b'b', b'c', b'e']);
}

#[testing(VEC)]
fn insert_mut() {
    let mut vec = vec![1, 3, 5, 9];
    let x = vec.insert_mut(3, 6);
    *x += 1;
    assert_eq!(vec, [1, 3, 5, 7, 9]);
}

#[testing(VEC)]
fn remove() {
    let mut v = vec![b'a', b'b', b'c'];
    assert_eq!(v.remove(1), b'b');
    assert_eq!(v, [b'a', b'c']);
}

#[testing(VEC)]
fn retain() {
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);
}

#[testing(VEC)]
fn retain_mut() {
    let mut vec = vec![1, 2, 3, 4];
    vec.retain_mut(|x| if *x <= 3 {
        *x += 1;
        true
    } else {
        false
    });
    assert_eq!(vec, [2, 3, 4]);
}

#[testing(VEC)]
fn push() {
    let mut vec = vec![1, 2];
    vec.push(3);
    assert_eq!(vec, [1, 2, 3]);
}

#[testing(VEC)]
fn push_mut() {
    let mut vec = vec![1, 2];
    let last = vec.push_mut(3);
    assert_eq!(*last, 3);
    assert_eq!(vec, [1, 2, 3]);

    let last = vec.push_mut(3);
    *last += 1;
    assert_eq!(vec, [1, 2, 3, 4]);
}

#[testing(VEC)]
fn pop() {
    let mut vec = vec![1, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);
}

#[testing(VEC)]
fn pop_if() {
    let mut vec = vec![1, 2, 3, 4];
    let pred = |x: &mut i32| *x % 2 == 0;

    assert_eq!(vec.pop_if(pred), Some(4));
    assert_eq!(vec, [1, 2, 3]);
    assert_eq!(vec.pop_if(pred), None);
}

#[testing(VEC)]
fn append() {
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);
}


#[testing(VEC)]
fn clear() {
    let mut v = vec![1, 2, 3];

    v.clear();

    assert!(v.is_empty());
}


#[testing(VEC)]
fn resize_with() {
    let mut vec = vec![1, 2, 3];
    vec.resize_with(5, Default::default);
    assert_eq!(vec, [1, 2, 3, 0, 0]);

    let mut vec = vec![];
    let mut p = 1;
    vec.resize_with(4, || { p *= 2; p });
    assert_eq!(vec, [2, 4, 8, 16]);
}

#[testing(VEC)]
fn resize() {
    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec!['a', 'b', 'c', 'd'];
    vec.resize(2, '_');
    assert_eq!(vec, ['a', 'b']);
}

#[testing(VEC)]
fn extend_from_slice() {
    let mut vec = vec![1];
    vec.extend_from_slice(&[2, 3, 4]);
    assert_eq!(vec, [1, 2, 3, 4]);
}

#[testing(VEC)]
fn extend_from_within() {
    let mut characters = vec!['a', 'b', 'c', 'd', 'e'];
    characters.extend_from_within(2..);
    assert_eq!(characters, ['a', 'b', 'c', 'd', 'e', 'c', 'd', 'e']);

    let mut numbers = vec![0, 1, 2, 3, 4];
    numbers.extend_from_within(..2);
    assert_eq!(numbers, [0, 1, 2, 3, 4, 0, 1]);

    let mut strings: Vec<String> = vec![String::from("hello"), String::from("world"), String::from("!")];
    strings.extend_from_within(1..=2);
    assert_eq!(strings, ["hello", "world", "!", "world", "!"]);
}

#[testing(VEC)]
fn into_flattened() {
    let mut vec = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    assert_eq!(vec.pop(), Some([7, 8, 9]));

    let mut flattened = vec.into_flattened();
    assert_eq!(flattened.pop(), Some(6));
}
