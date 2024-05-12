/* This file is generated and managed by tsync */

/** Doc comments are preserved too! */
type Book = BookType & {
  /** Name of the book. */
  name: string;
  /** Chapters of the book. */
  chapters: Array<Chapter>;
  /**
   * Reviews of the book
   * by users.
   */
  user_reviews?: Array<string>;
}

/** Book struct with camelCase field names. */
interface BookCamel {
  /** Name of the book. */
  name: string;
  /** Chapters of the book. */
  chapters: Array<Chapter>;
  /**
   * Reviews of the book
   * by users.
   */
  userReviews?: Array<string>;
}

/**
 * Multiple line comments
 * are formatted on
 * separate lines
 */
interface Chapter {
  title: string;
  pages: number;
}

/** Generic struct test */
interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
}

/** Generic struct test with camelCase field names. */
interface PaginationResultCamel<T> {
  items: Array<T>;
  totalItems: number;
}

/** Struct with flattened field. */
type Author = AuthorName & {
  name: string;
}

interface AuthorName {
  alias?: string;
  first_name: string;
  last_name: string;
}

type BookType =
  | BookType__Fiction
  | BookType__NonFiction;

type BookType__Fiction = {
  type: "Fiction";
  genre: string;
};
type BookType__NonFiction = {
  type: "NonFiction";
  subject: string;
};
