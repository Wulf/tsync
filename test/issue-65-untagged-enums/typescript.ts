/* This file is generated and managed by tsync */

export type Message =
  | [ number, number ]
  | number;

export type Message2<V, G> =
  | {
      a: V;
      b: G;
    }
  | {
      c: V;
    }
  | G
  | Array<G>;
