// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/lib.rs
 *
 * Static assertions for Rust.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The HaruxOS Project Team
 */

#![no_std]

/// Statically asserts that an expression is true.
pub macro static_assert {
    ($($arguments:tt)*) => {
        const _: () = const { core::assert!($($arguments)*) };
    }
}

/// Statically asserts that two values are equal.
pub macro static_assert_eq {
    ($($arguments:tt)*) => {
        const _: () = const { core::assert_eq!($($arguments)*) };
    }
}

/// Statically asserts that two values are not equal.
pub macro static_assert_ne {
    ($($arguments:tt)*) => {
        const _: () = const { core::assert_ne!($($arguments)*) };
    }
}