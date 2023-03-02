import { HexCoordinate, hexToTransform } from 'hive-lib';
import { MouseEvent, SVGProps } from 'react';
import { Hex } from './Hex';

export interface HexGhostProps {
  coordinate: HexCoordinate;
  hexSize: number;
  onClick?: (coordinate: HexCoordinate) => void;
  style?: SVGProps<SVGUseElement>;
}

const HexGhost = (props: HexGhostProps) => {
  const { coordinate, hexSize, onClick, style } = props;

  const transform = hexToTransform(coordinate, hexSize);
  const handleClick = (event: MouseEvent<SVGGElement>) => {
    if (onClick !== undefined) {
      event.stopPropagation();
      onClick(coordinate);
    }
  };

  return (
    <g
      className={`tile-ghost ${handleClick ? 'cursor-pointer' : ''}`}
      transform={transform}
      onClick={handleClick}
    >
      <Hex size={hexSize - 2} {...style} />
    </g>
  );
};

export { HexGhost };
