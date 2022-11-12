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
export const CONST_TEST_1 = 0;

/** Shouldn't compile but should convert */
export const CONST_TEST_2 = 0.0;

/** Valid Rust but not valid typescript would be misleading if it made it into normal string ? */
export const CONST_TEST_3 = b"Hello";

/** Test serde_json */
export const SERDE_JSON_1 = { "a" : "b" };

export const SERDE_JSON_2 = { "a" : "b" };

/**
 * Variants should to discriminated unions
 * The last serde/attribute combo matching the tag should be taken
 */
type Message =
  /** Per Enum case Docs One */
  | {
      last_precedent: "UnitCaseLeft",
    }
  /** Per Enum case Docs Two */
  | {
      last_precedent: "RequestLongTake",
      id: string
      method: string
      params: number
    }
  | {
      last_precedent: "Response",
      id: string
      result: Date
    };

/** The default enum conversion uses external tagging */
type ExternalMessage =
  /** Per Enum case Docs One */
  | {
      "UnitCaseLeft": {}
    }
  /** Per Enum case Docs Two */
  | {
      "RequestLongTake": {
        id: string
        method: string
        params: number
      }
    }
  | {
      "Response": {
        id: string
        result: Date
      }
    };

/**
 * All Unit Enums go to union of constant strings
 * even if have explicit numeric annotations
 * There is no case renaming on default
 */
type Animal =
  | "Dog" | "Cat";

type AnimalTwo =
  | "dog_long_extra" | "cat";

/** Integer enums should follow rust discrimination if literals (doesn't evaluate expression) */
enum Foo {
  Bar = 0,
  Baz = 123,
  Quux = 124,
}
