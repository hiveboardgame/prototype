import {
  ColorKey,
  getBugLetter,
  getBugNumber,
  getTileBug,
  getTileColor,
  TileId
} from 'hive-lib';
import { Bug } from './Bug';
import { Hex } from './Hex';

interface TileProps {
  tileId: TileId;
  hexSize: number;
  tilePadding: number;
  selected?: boolean;
}

const Tile = (props: TileProps) => {
  const { tileId, hexSize, tilePadding, selected } = props;
  const bug = getTileBug(tileId);
  const bugLetter = getBugLetter(bug);
  const bugNumber = getBugNumber(bug);
  const color = getTileColor(tileId);
  return (
    <g id={tileId} className='tile'>
      <Hex
        fill={fill(color)}
        size={hexSize - tilePadding}
        stroke={stroke(color, selected)}
        strokeWidth={strokeWidth(selected)}
      />
      <Bug
        bugLetter={bugLetter}
        bugNumber={bugNumber}
        color={color}
        size={hexSize - tilePadding}
      />
    </g>
  );
};

function fill(color: ColorKey): string {
  if (color === 'b') return '#222';
  if (color === 'w') return 'white';
  return 'none';
}

function stroke(color: ColorKey, selected?: boolean): string {
  if (selected) return '#f8a61c';
  if (color === 'b') return '#aaa';
  if (color === 'w') return '#888';
  return 'none';
}

function strokeWidth(selected?: boolean): number {
  if (selected) return 5;
  return 3;
}

export { Tile };
