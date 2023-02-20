import { hexHeight, hexWidth } from 'hive-lib';
import { SVGProps } from 'react';

type HexProps = SVGProps<SVGUseElement> & {
  size: number;
};

const Hex = (props: HexProps) => {
  const { size, ...rest } = props;
  const width = hexWidth(size);
  const height = hexHeight(size);
  return (
    <use
      href='#hex'
      width={width}
      height={height}
      x={-width / 2}
      y={-height / 2}
      {...rest}
    />
  );
};

export { Hex };
