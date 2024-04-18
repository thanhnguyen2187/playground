import { describe, it, expect } from "vitest"
import { isValidDate } from './machine';

describe('isValidDate', () => {
  it('true', () => {
    expect(isValidDate('01.01.2022')).toBe(true);
    expect(isValidDate('27.03.2014')).toBe(true);
  });
  it('false', () => {
    expect(isValidDate('')).toBe(false);
    expect(isValidDate('27.03')).toBe(true);
    expect(isValidDate('27-03-2014')).toBe(false);
  });
});
