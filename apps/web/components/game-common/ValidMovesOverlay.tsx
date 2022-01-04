import { HexCoordinate } from 'hive-lib';
import { HexGhostValidMove } from './HexGhostValidMove';

interface GhostOverlayProps {
  coordinates: HexCoordinate[];
  hexSize: number;
  onClick?: (coordinate: HexCoordinate) => void;
}

const ValidMovesOverlay = (props: GhostOverlayProps) => {
  const { coordinates, hexSize, onClick } = props;
  return (
    <g className='ghost-overlay'>
      {coordinates.map((coordinate, index) => (
        <HexGhostValidMove
          key={index}
          coordinate={coordinate}
          hexSize={hexSize}
          onClick={onClick}
        />
      ))}
    </g>
  );
};

export { ValidMovesOverlay };
