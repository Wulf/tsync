/* This file is generated and managed by tsync */

type Message =
  | [ number, number ]
  | number;

type Message2<V, G> =
  | {
      a: V;
      b: G;
    }
  | {
      c: V;
    }
  | G
  | Array<G>;
