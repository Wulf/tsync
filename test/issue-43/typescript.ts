/* This file is generated and managed by tsync */

export interface HasTuple {
  foo: number;
  bar?: [string, number];
  baz: [string, number];
  zip: [number, string, [string, [number, number]]];
  qux: [string | undefined, [number, string]];
  ping: [number, string, [string, [number, number]] | undefined];
  pong?: [number, string, [string, [number, number] | undefined] | undefined];
}

export type IsTuple = [ number, string ]

export type IsTupleComplex = [ number, string, [string, [number, number]] ]
