/* This file is generated and managed by tsync */

type Message =
  | Message__UnitCaseLeft
  | Message__RequestLongTake;

/** Per Enum case Docs One */
type Message__UnitCaseLeft = {
  last_precedent: "UnitCaseLeft";
};
/** Per Enum case Docs Two */
type Message__RequestLongTake = {
  last_precedent: "RequestLongTake";
  id: string;
  method: string;
  params: number;
};

/** The default enum conversion uses external tagging */
type ExternalMessage =
  /** Per Enum case Docs One */
  | {
      "UnitCaseLeft": {}
    }
  /** Per Enum case Docs Two */
  | {
      "RequestLongTake": {
        id: string;
        method: string;
        params: number;
      }
    }
  /** Newtype variant with exactly one variable */
  | { "Response": Response };

interface Response {
  id: string;
  result: Date;
}

/**
 * All Unit Enums go to union of constant strings
 * even if have explicit numeric annotations
 * There is no case renaming on default
 */
type Animal =
  | "Dog" | "Cat";

type AnimalTwo =
  | "dog_long_extra" | "cat";

/** Integer enums should follow rust discrimination if literals (doesn't evaluate expression) */
declare enum Foo {
  Bar = 0,
  Baz = 123,
  Quux = 124,
}
