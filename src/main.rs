
use core::assert_eq;
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};
use std::mem::transmute;
use std::thread;

let atomic = AtomicU16::new(0);

let vec2 = Vec::from([1, 2, 3, 4]);
let atomic_ptr = AtomicPtr::new(vec2);

assert_eq!(atomic_ptr, [1,2,3,4]);
