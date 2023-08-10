/* This file is generated and managed by tsync */

/**
 * Internally tagged enums have a key-value pair
 * that discrimate which variant it belongs to
 */
export type InternalTopping =
  | InternalTopping__Pepperoni
  | InternalTopping__ExtraCheese;

/**
 * Tasty!
 * Not vegetarian
 */
type InternalTopping__Pepperoni = {
  type: "PEPPERONI";
};
/** For cheese lovers */
type InternalTopping__ExtraCheese = {
  type: "EXTRA CHEESE";
  kind: string;
};

/**
 * Externally tagged enums ascribe the value to a key
 * that is the same as the variant name
 */
export type ExternalTopping =
  /**
   * Tasty!
   * Not vegetarian
   */
  | {
      "Pepperoni": {}
    }
  /** For cheese lovers */
  | {
      "ExtraCheese": {
        kind: string;
      }
    }
  /**
   * Custom toppings
   * May expire soon
   * Note: this test case is specifically for specifying a single type in the tuple
   */
  | { "Custom": CustomTopping };

export interface CustomTopping {
  name: string;
  expires_in: Date;
}

/**
 * All Unit Enums go to union of constant strings
 * even if have explicit numeric annotations
 * There is no case renaming on default
 */
export type Animal =
  | "Dog" | "Cat";

export type AnimalTwo =
  | "dog_long_extra" | "cat";

/** Integer enums should follow rust discrimination if literals (doesn't evaluate expression) */
export type Foo =
  | "Bar" | "Baz" | "Quux";
