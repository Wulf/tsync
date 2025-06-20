/* This file is generated and managed by tsync */

/** enum comment */
type EnumTest =
  | EnumTest__One
  | EnumTest__Two
  | EnumTest__Three;

/** enum property comment */
type EnumTest__One = {
  type: "ONE";
};
/** enum tuple comment */
type EnumTest__Two = {
  type: "TWO"} & StructTest
/** enum struct comment */
type EnumTest__Three = {
  type: "THREE";
  /** enum struct property comment */
  ID: string;
};

/** struct comment */
interface StructTest {
  /** struct field comment */
  name: string;
}

/** type comment */
type TypeTest = string
