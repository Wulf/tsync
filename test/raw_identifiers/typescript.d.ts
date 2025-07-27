/* This file is generated and managed by tsync */

interface RawIdentifierStruct {
  type: string;
  async: number;
  loop: boolean;
  normal_field: string;
}

interface RawIdentifierCamelCase {
  type: string;
  const: number;
  regularField: string;
}

type RawIdentifierEnum =
  | "type" | "async" | "match" | "NormalVariant";

type RawIdentifierEnumUppercase =
  | "TYPE" | "CONST" | "NORMAL VARIANT";

type RawIdentifierNumericEnum =
  | "type" | "async" | "NormalVariant";

type RawIdentifierTaggedEnum =
  | RawIdentifierTaggedEnum__type
  | RawIdentifierTaggedEnum__async
  | RawIdentifierTaggedEnum__NormalVariant;

type RawIdentifierTaggedEnum__type = {
  kind: "type";
  value: string;
};
type RawIdentifierTaggedEnum__async = {
  kind: "async";
  count: number;
};
type RawIdentifierTaggedEnum__NormalVariant = {
  kind: "NormalVariant";
  data: boolean;
};

type type = string

type async = Array<number>
