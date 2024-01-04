/* This file is generated and managed by tsync */

export interface Folder {
  name: string;
  children: Paginated<Folder>;
}

export interface Paginated<T> {
  data: Array<T>;
  page: number;
  total_pages: number;
}
