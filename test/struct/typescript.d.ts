/* This file is generated and managed by tsync */

/** Doc comments are preserved too! */
interface Book {
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
