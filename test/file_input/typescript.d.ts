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
  user_reviews: Array<string> | undefined
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
