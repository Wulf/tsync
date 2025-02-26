/* This file is generated and managed by tsync */

interface AppleData {
  crunchy: boolean;
}

interface BananaData {
  size: number;
}

interface CarrotData {
  color: string;
}

type Fruit =
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
