import classNames from 'classnames';
import { ColorKey } from 'hive-lib';
import { SVGProps } from 'react';
import { HexPath } from './HexPath';

interface HiveIconProps extends SVGProps<SVGSVGElement> {
  hexColor?: ColorKey;
}

const HiveIcon = (props: HiveIconProps) => {
  const { className, hexColor, ...rest } = props;
  const cn = classNames(className, {
    'stroke-hiveblack fill-hivewhite': hexColor === 'w',
    'stroke-hiveblack fill-hiveblack': hexColor === 'b'
  });
  return (
    <svg className={cn} viewBox='-12 -12 24 24' {...rest}>
      <HexPath />
    </svg>
  );
};

export { HiveIcon };
