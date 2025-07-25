/* This file is generated and managed by tsync */

export interface RawIdentifierStruct {
  type: string;
  async: number;
  loop: boolean;
  normal_field: string;
}

export interface RawIdentifierCamelCase {
  type: string;
  const: number;
  regularField: string;
}

export type RawIdentifierEnum =
  | "type" | "async" | "match" | "NormalVariant";

export type RawIdentifierEnumUppercase =
  | "TYPE" | "CONST" | "NORMAL VARIANT";

export type RawIdentifierNumericEnum =
  | "type" | "async" | "NormalVariant";

export type RawIdentifierTaggedEnum =
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

export type type = string

export type async = Array<number>
