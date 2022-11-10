/* This file is generated and managed by tsync */

/** Doc comments are preserved too! */
interface Book {
  /** Name of the book. */
  name: string
  /** Chapters of the book. */
  chapters: Array<Chapter>
  /**
   * Reviews of the book
   * by users.
   */
  user_reviews?: Array<string>
}

/**
 * Multiple line comments
 * are formatted on
 * separate lines
 */
interface Chapter {
  title: string
  pages: number
}

/** Time in UTC seconds */
type UTC = number

/** Generic struct test */
interface PaginationResult<T> {
  items: Array<T>
  total_items: number
}

/** Test integer */
const CONST_TEST_1 = 0;

/** Shouldn't compile but should convert */
const CONST_TEST_2 = 0.0;

/** Valid Rust but not valid typescript would be misleading if it made it into normal string ? */
const CONST_TEST_3 = b"Hello";

/** Test serde_json */
const SERDE_JSON_1 = { "a" : "b" };

const SERDE_JSON_2 = { "a" : "b" };

/**
 * Variants should to discriminated unions
 * The last serde/attribute combo matching the tag should be taken
 */
type Message =
  /** Per Enum case Docs One */
  | {
      last_precedent: "unit-case-left",
    }
  /** Per Enum case Docs Two */
  | {
      last_precedent: "request-long-take",
      id: string
      method: string
      params: number
    }
  | {
      last_precedent: "response",
      id: string
      result: Date
    };

/**
 * Integer enums should follow rust discrimination if literals (doesn't evaluate expression)
 * The case renaming defaults to SCREAMING_SNAKE_CASE
 */
enum Foo {
  BAR = 0,
  BAZ = 123,
  QUUX = 124,
}

enum Animal {
  DOG = 0,
  CAT = 1,
}
