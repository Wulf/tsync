/* This file is generated and managed by tsync */

/** enum comment */
type EnumTest =
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
  id: string;
};

/** struct comment */
interface StructTest {
  /** struct field comment */
  name: string;
}

/** type comment */
type TypeTest = string
