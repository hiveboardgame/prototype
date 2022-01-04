import { SVGProps } from 'react';
import { HexGhost, HexGhostProps } from './HexGhost';

const HexGhostValidMove = (props: Omit<HexGhostProps, 'style'>) => {
  const style: SVGProps<SVGUseElement> = {
    fill: 'rgba(99, 255, 154, 0.2)',
    stroke: 'rgba(99,255,154,0.4)',
    strokeWidth: 2
  };
  return <HexGhost style={style} {...props} />;
};

export { HexGhostValidMove };
