/**
 * Core exceptions for the data structures library.
 */

export class EmptyStructureError extends Error {
  constructor(message: string = 'Structure is empty') {
    super(message);
    this.name = 'EmptyStructureError';
  }
}

export class IndexOutOfRangeError extends Error {
  constructor(message: string = 'index out of range') {
    super(message);
    this.name = 'IndexOutOfRangeError';
  }
}

export class NotFoundError extends Error {
  constructor(message: string = 'not found') {
    super(message);
    this.name = 'NotFoundError';
  }
}

export class InvalidArgumentError extends Error {
  constructor(message: string = 'invalid argument') {
    super(message);
    this.name = 'InvalidArgumentError';
  }
}
