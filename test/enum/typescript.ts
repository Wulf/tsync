/* This file is generated and managed by tsync */

/**
 * Internally tagged enums have a key-value pair
 * that discrimate which variant it belongs to
 */
export type InternalTopping =
  | InternalTopping__Pepperoni
  | InternalTopping__ExtraCheese
  | InternalTopping__Custom;

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
  KIND: string;
};
/**
 * Custom toppings
 * May expire soon
 * Note: because this is a newtype variant, it should be included in the typescript
 */
type InternalTopping__Custom = {
  type: "CUSTOM"} & CustomTopping

/**
 * Adjacently tagged enums have a key-value pair
 * that discrimate which variant it belongs to, and
 * can support tuple variants
 */
export type AdjacentTopping =
  | AdjacentTopping__Pepperoni
  | AdjacentTopping__ExtraCheese
  | AdjacentTopping__Custom
  | AdjacentTopping__CustomTwo;

/**
 * Tasty!
 * Not vegetarian
 */
type AdjacentTopping__Pepperoni = {
  type: "Pepperoni";
};
/** For cheese lovers */
type AdjacentTopping__ExtraCheese = {
  type: "ExtraCheese";
  kind: string;
};
/**
 * Custom toppings
 * May expire soon
 */
type AdjacentTopping__Custom = {
  "type": "Custom";
  "value": CustomTopping;
};
/**
 * two custom toppings
 * Note: this test case is specifically for specifying a tuple of types
 */
type AdjacentTopping__CustomTwo = {
  "type": "CustomTwo";
  "value": [ CustomTopping, CustomTopping ];
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
  | { "Custom": CustomTopping }
  /**
   * two custom toppings
   * Note: this test case is specifically for specifying a tuple of types
   */
  | { "CustomTwo": [ CustomTopping, CustomTopping ] };

export interface CustomTopping {
  name: string;
  expires_in: Date;
}

export interface CustomToppingCamel {
  name: string;
  expiresIn: Date;
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

export type Tagged =
  | Tagged__Test;

type Tagged__Test = {
  type: "Test";
};
