/* This file is generated and managed by tsync */

export interface AppleData {
  crunchy: boolean;
}

export interface BananaData {
  size: number;
}

export interface CarrotData {
  color: string;
}

export type Fruit =
  | Fruit__Apple
  | Fruit__Banana
  | Fruit__Carrot;

type Fruit__Apple = {
  "kind": "Apple";
  "data": AppleData;
};
type Fruit__Banana = {
  "kind": "Banana";
  "data": BananaData;
};
type Fruit__Carrot = {
  "kind": "Carrot";
  "data": CarrotData;
};
