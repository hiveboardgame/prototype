import grasshopper0 from './data/grasshopper/grasshopper0';
import grasshopper1 from './data/grasshopper/grasshopper1';
import { getValidGrasshopperMoveCoordinates } from './grasshopper';

describe('grasshopper moves', () => {
  const grasshoppers = [
    { index: 0, ...grasshopper0 },
    { index: 1, ...grasshopper1 }
  ];

  test.each(grasshoppers)(
    'test position $index',
    ({ board, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidGrasshopperMoveCoordinates(
        board,
        tileCoordinate
      );
      expect(validCoordinates).toHaveLength(expectedCoordinates.length);
      expect(validCoordinates).toEqual(
        expect.arrayContaining(expectedCoordinates)
      );
    }
  );
});
