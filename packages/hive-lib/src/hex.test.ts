import {
  hexCoordinateKey,
  hexesAreNeighbors,
  hexesEqual,
  hexHeight,
  includesHex,
  relativeHexCoordinate,
  toHexDirection
} from './hex';
import { HexCoordinate } from './types';

describe('hex properties', () => {
  describe('hexCoordinateKey', () => {
    test('(0, 0)', () => expect(hexCoordinateKey({ q: 0, r: 0 })).toBe('00'));
    test('(1, 2)', () => expect(hexCoordinateKey({ q: 1, r: 2 })).toBe('12'));
    test('(-1, -2)', () =>
      expect(hexCoordinateKey({ q: -1, r: -2 })).toBe('-1-2'));
    test('(-10, 10)', () =>
      expect(hexCoordinateKey({ q: -10, r: 10 })).toBe('-1010'));
    test('(10, -10)', () =>
      expect(hexCoordinateKey({ q: 10, r: -10 })).toBe('10-10'));
    test('(-4, 58)', () =>
      expect(hexCoordinateKey({ q: -4, r: 58 })).toBe('-458'));
  });
  describe('hexesAreNeighbors', () => {
    const h0 = { q: 0, r: 0 };
    const h1 = { q: 1, r: -1 };
    const h2 = { q: 1, r: 0 };
    const h3 = { q: 0, r: 1 };
    const h4 = { q: -1, r: 1 };
    const h5 = { q: -1, r: 0 };
    const h6 = { q: 0, r: -1 };

    test.each([
      [1, h0, h1],
      [2, h0, h2],
      [3, h0, h3],
      [4, h0, h4],
      [5, h0, h5],
      [6, h0, h6]
    ])('origin, direction %d', (_, a, b) =>
      expect(hexesAreNeighbors(a, b)).toBe(true)
    );

    test.each([
      ['h1', 'h4', h1, h4],
      ['h2', 'h5', h2, h5],
      ['h3', 'h6', h3, h6]
    ])('%s, %s', (_, __, a, b) => expect(hexesAreNeighbors(a, b)).toBe(false));
  });
  describe('hexesEqual', () => {
    const h0 = { q: 0, r: 0 };
    const h1 = { q: 1, r: -1 };
    test('undef, undef', () => expect(hexesEqual()).toBe(false));
    test('undef, origin', () => expect(hexesEqual(undefined, h0)).toBe(false));
    test('origin, undef', () => expect(hexesEqual(h0)).toBe(false));
    test('origin, origin', () => expect(hexesEqual(h0, h0)).toBe(true));
    test('origin, neighbor', () => expect(hexesEqual(h0, h1)).toBe(false));
    test('neighbor, origin', () => expect(hexesEqual(h1, h0)).toBe(false));
  });
  describe('hexHeight', () => {
    test('height is 2*size', () => {
      expect(hexHeight(0)).toBe(0);
      expect(hexHeight(2)).toBe(4);
      expect(hexHeight(4)).toBe(8);
    });
  });
  describe('includesHex', () => {
    const hexes: HexCoordinate[] = [
      { q: 0, r: 0 },
      { q: 1, r: 0 },
      { q: 0, r: -1 }
    ];
    test('array does include hex', () =>
      expect(includesHex(hexes, { q: 0, r: 0 })).toBe(true));
    test('array does not include hex', () =>
      expect(includesHex(hexes, { q: 1, r: 1 })).toBe(false));
  });
  describe('relativeHexCoordinate', () => {
    const h0 = { q: 0, r: 0 };
    const h1 = { q: 1, r: -1 };
    const h2 = { q: 1, r: 0 };
    const h3 = { q: 0, r: 1 };
    const h4 = { q: -1, r: 1 };
    const h5 = { q: -1, r: 0 };
    const h6 = { q: 0, r: -1 };
    test.each([
      [0, h0, h0],
      [1, h0, h1],
      [2, h0, h2],
      [3, h0, h3],
      [4, h0, h4],
      [5, h0, h5],
      [6, h0, h6]
    ])('direction: %d', (direction, source, expected) =>
      expect(relativeHexCoordinate(source, direction)).toEqual(expected)
    );
    test('invalid direction', () =>
      expect(() => relativeHexCoordinate(h0, 7)).toThrow());
  });
  describe('toHexDirection', () => {
    test('0', () => expect(toHexDirection(0)).toBe(6));
    test('1', () => expect(toHexDirection(1)).toBe(1));
    test('2', () => expect(toHexDirection(2)).toBe(2));
    test('3', () => expect(toHexDirection(3)).toBe(3));
    test('4', () => expect(toHexDirection(4)).toBe(4));
    test('5', () => expect(toHexDirection(5)).toBe(5));
    test('6', () => expect(toHexDirection(6)).toBe(6));
    test('7', () => expect(toHexDirection(7)).toBe(1));
  });
});
