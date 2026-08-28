/**
 * Core exceptions for the data structures library.
 */

export class EmptyStructureError extends Error {
  constructor(message: string = 'Structure is empty') {
    super(message);
    this.name = 'EmptyStructureError';
  }
}
