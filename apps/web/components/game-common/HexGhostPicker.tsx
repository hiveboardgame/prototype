import { SVGProps } from 'react';
import { HexGhost, HexGhostProps } from './HexGhost';

const HexGhostPicker = (props: Omit<HexGhostProps, 'style'>) => {
  const style: SVGProps<SVGUseElement> = {
    fill: '#31979510',
    stroke: '#319795',
    strokeWidth: 1
  };
  return <HexGhost style={style} {...props} />;
};

export { HexGhostPicker };
