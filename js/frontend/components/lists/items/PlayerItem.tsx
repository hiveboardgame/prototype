import { HTMLAttributes } from 'react';
import { HiveIcon } from '../../common/HiveIcon';
import { RowItem } from '../Row';
import { ColorKey } from 'hive-lib';

type PlayerItemMode = 'bar' | 'color';

interface PlayerItemProps {
  mode: PlayerItemMode;
  playerName: string | null;
  playerColor: ColorKey | null;
  isTurn: boolean;
}

const PlayerItem = (
  props: HTMLAttributes<HTMLDivElement> & PlayerItemProps
) => {
  const { mode, playerName, playerColor, isTurn, ...rest } = props;
  const borderSize = borderSizeClassName(mode);
  const barColor = barColorClassName(mode, isTurn);
  const textColor = textColorClassName(mode, isTurn);
  const fill = fillClassName(playerColor, isTurn);
  const stroke = strokeClassName(isTurn);
  return (
    <RowItem
      className={`${borderSize} ${barColor} ${textColor} group-hover:border-hive`}
      {...rest}
    >
      <HiveIcon
        width={18}
        height={18}
        className={`ml-1 mr-1.5 ${fill} ${stroke}`}
      />
      <div className={textColorClassName(mode, isTurn)}>{playerName}</div>
    </RowItem>
  );
};

function barColorClassName(mode: PlayerItemMode, isTurn: boolean): string {
  if (mode === 'color') return '';
  return isTurn ? 'border-[#f8a61c]' : 'border-transparent';
}

function borderSizeClassName(mode: PlayerItemMode): string {
  return mode === 'bar' ? 'border-l-8' : '';
}

function fillClassName(playerColor: ColorKey, isTurn: boolean): string {
  if (playerColor === 'w') {
    return 'fill-hivewhite';
  } else {
    return isTurn ? 'fill-hive' : 'fill-hiveblack';
  }
}

function strokeClassName(isTurn: boolean): string {
  return isTurn ? 'stroke-hive' : 'stroke-hiveblack';
}

function textColorClassName(mode: PlayerItemMode, isTurn: boolean): string {
  if (mode === 'bar') return '';
  return isTurn ? 'font-semibold text-[#f8a61c] group-hover:text-white' : '';
}

export { PlayerItem };
