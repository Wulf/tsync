/* This file is generated and managed by tsync */

/** enum comment */
export type EnumTest =
  | EnumTest__One
  | EnumTest__Three;

/** enum property comment */
type EnumTest__One = {
  type: "ONE";
};
/** enum struct comment */
type EnumTest__Three = {
  type: "THREE";
  /** enum struct property comment */
  ID: string;
};

/** struct comment */
export interface StructTest {
  /** struct field comment */
  name: string;
}

/** type comment */
export type TypeTest = string

/** const comment */
export const CONST_TEST = "test";
