export class Declaration {
  constructor() {
    this.onlyUndefined = undefined;
  }
}

export const Expression = class {
  constructor() {
    this.onlyNull = null;
  }
};

export class Annotated {
  constructor() {
    /** @type {number | undefined} */
    this.value = undefined;
  }
}
