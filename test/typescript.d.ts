/* This file is generated and managed by tsync */

// Doc comments are preserved too!
interface Book {
  name: string
  chapters: Array<Chapter>
  user_reviews: Array<string> | undefined
}

interface Chapter {
  title: string
  pages: number
}

// Time in UTC seconds
type UTC = number
