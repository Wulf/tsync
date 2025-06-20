/* This file is generated and managed by tsync */

interface HasTuple {
  foo: number;
  bar?: [string, number];
  baz: [string, number];
  zip: [number, string, [string, [number, number]]];
  qux: [string | undefined, [number, string]];
  ping: [number, string, [string, [number, number]] | undefined];
  pong?: [number, string, [string, [number, number] | undefined] | undefined];
}

type IsTuple = [ number, string ]

type IsTupleComplex = [ number, string, [string, [number, number]] ]
