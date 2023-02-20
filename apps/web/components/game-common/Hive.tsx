import {
  HexCoordinate,
  hexToTransform,
  StackCoordinate,
  TileId
} from 'hive-lib';
import { useMemo } from 'react';
import { TileStack } from './TileStack';

interface HiveProps {
  stacks: StackCoordinate[];
  hexSize: number;
  tilePadding: number;
  selectedTileId?: TileId;
  onClick?: (coordinate: HexCoordinate, stack: TileId[]) => void;
}

const Hive = (props: HiveProps) => {
  const { stacks, hexSize, tilePadding, selectedTileId, onClick } = props;
  const sorted = useMemo(() => renderSort(stacks), [stacks]);
  return (
    <g id='hive'>
      {sorted.map(({ coordinate, stack }, index) => {
        const transform = hexToTransform(coordinate, hexSize);
        const onClickStack = onClick
          ? (stack: TileId[]) => onClick(coordinate, stack)
          : undefined;
        return (
          <g key={index} transform={transform}>
            <TileStack
              stack={stack}
              hexSize={hexSize}
              tilePadding={tilePadding}
              canReveal={true}
              selectedTileId={selectedTileId}
              onClick={onClickStack}
            />
          </g>
        );
      })}
    </g>
  );
};

/**
 * Sort stacks of tiles so that they are ordered from back to front, shortest
 * to tallest.
 *
 * @param stacks An array of stacks.
 * @return A sorted copy of the array of stacks.
 */
function renderSort(stacks: StackCoordinate[]): StackCoordinate[] {
  return stacks.slice().sort((a, b) => {
    const dr = b.coordinate.r - a.coordinate.r;
    if (dr < 0) return 1;
    if (dr === 0) return a.stack.length - b.stack.length;
    return -1;
  });
}

export { Hive };
