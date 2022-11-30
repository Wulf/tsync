/* This file is generated and managed by tsync */

/** Doc comments are preserved too! */
export interface Book {
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
export interface Chapter {
  title: string
  pages: number
}

/** Generic struct test */
export interface PaginationResult<T> {
  items: Array<T>
  total_items: number
}
