/* This file is generated and managed by tsync */

/** Test integer */
export const CONST_TEST_1 = 0;

/** Shouldn't compile but should convert */
export const CONST_TEST_2 = 0.0;

/** Valid Rust but not valid typescript would be misleading if it made it into normal string ? */
export const CONST_TEST_3 = b"Hello";

/** Test serde_json */
export const SERDE_JSON_1 = { "a" : "b" };

export const SERDE_JSON_2 = { "a" : "b" };
