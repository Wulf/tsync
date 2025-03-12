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

type Flatten<T> = Array<T> & {
  name: string;
}

/** \n * Test enum represenations w/ generics\n */
type ExternalEnum<T, U> =
  | { "Bar": T }
  | { "Waz": U };

type AdjacentEnum<T, U> =
  | AdjacentEnum__Bar<T>
  | AdjacentEnum__Waz<U>;

type AdjacentEnum__Bar<T> = {
  "type": "Bar";
  "value": T;
};
type AdjacentEnum__Waz<U> = {
  "type": "Waz";
  "value": U;
};

type InternalEnum<T, U> =
  | InternalEnum__Bar<T>;

type InternalEnum__Bar<T> = {
  type: "Bar";
  value: T;
  alias: string;
};
