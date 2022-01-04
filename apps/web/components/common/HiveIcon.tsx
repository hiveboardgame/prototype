import { SVGProps } from 'react';
import { HexPath } from './HexPath';

const HiveIcon = (props: SVGProps<SVGSVGElement>) => {
  return (
    <svg viewBox='-12 -12 24 24' {...props}>
      <HexPath />
    </svg>
  );
};

export { HiveIcon };
