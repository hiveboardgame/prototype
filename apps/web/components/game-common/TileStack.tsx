import { TileId } from 'hive-lib';
import { MouseEvent, useState } from 'react';
import { Tile } from './Tile';

interface TileStackProps {
  stack: TileId[];
  hexSize: number;
  tilePadding: number;
  canReveal?: boolean;
  selectedTileId?: TileId;
  onClick?: (stack: TileId[]) => void;
}

const TileStack = (props: TileStackProps) => {
  const { stack, hexSize, tilePadding, canReveal, selectedTileId, onClick } =
    props;
  const [hoveredIndex, setHoveredIndex] = useState<number>();

  const handleClick = (event: MouseEvent<SVGGElement>) => {
    if (onClick !== undefined) {
      event.stopPropagation();
      onClick(stack);
    }
  };

  return (
    <g
      className={`tile-stack ${handleClick ? 'cursor-pointer' : ''}`}
      onClick={handleClick}
    >
      {stack.map((tileId, index) => {
        const doReveal =
          canReveal && hoveredIndex !== undefined && index > hoveredIndex;
        const transform = doReveal
          ? stackTileRevealTransform(index - hoveredIndex, hexSize)
          : stackTileTransform(index);
        return (
          <g
            key={tileId}
            className='transition-all duration-300 transform-gpu'
            style={{ transform: transform }}
            onMouseEnter={() => setHoveredIndex(index)}
            onMouseLeave={() => setHoveredIndex(undefined)}
            filter={doReveal ? stackTileRevealShadow(index) : undefined}
          >
            <Tile
              tileId={tileId}
              hexSize={hexSize}
              tilePadding={tilePadding}
              selected={tileId === selectedTileId}
            />
          </g>
        );
      })}
    </g>
  );
};

function stackTileTransform(index: number): string {
  const offsetX = 3;
  const offsetY = 2;
  return `translate(${offsetX * index}px, ${-offsetY * index}px)`;
}

function stackTileRevealTransform(index: number, hexSize: number): string {
  const offsetX = 1.25 * Math.cos((60 * Math.PI) / 180) * hexSize;
  const offsetY = hexSize - 1.25 * Math.sin((index * Math.PI) / 180) * hexSize;
  return `translate(${offsetX * index}px, ${-offsetY * index}px)`;
}

function stackTileRevealShadow(index: number): string {
  return `drop-shadow(-${index}px ${index}px ${index}px rgb(0 0 0 / 0.4))`;
}

export { TileStack };
