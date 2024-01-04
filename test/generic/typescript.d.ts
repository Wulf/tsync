/* This file is generated and managed by tsync */

interface Folder {
  name: string;
  children: Paginated<Folder>;
}

interface Paginated<T> {
  data: Array<T>;
  page: number;
  total_pages: number;
}
