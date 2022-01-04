import { ColorKey, hexHeight, hexWidth } from 'hive-lib';

interface BugProps {
  bugLetter: string;
  bugNumber?: number;
  color: ColorKey;
  size: number;
}

const Bug = (props: BugProps) => {
  const { bugLetter, bugNumber, color, size } = props;
  const base = getBugColor(bugLetter, color);
  const accent = getAccentColor(color);
  const width = hexWidth(size);
  const height = hexHeight(size);
  const x = -width / 2;
  const y = -height / 2;
  return (
    <>
      <use
        href={`#${bugLetter}-base`}
        width={width}
        height={height}
        x={x}
        y={y}
        stroke='none'
        fill={base}
      />
      <use
        href={`#${bugLetter}-accent`}
        width={width}
        height={height}
        x={x}
        y={y}
        stroke='none'
        fill={accent}
      />
      {bugNumber && (
        <use
          href={`#circle-${bugNumber}`}
          width={width}
          height={height}
          x={x}
          y={y}
          stroke='none'
          fill={base}
        />
      )}
    </>
  );
};

export function getAccentColor(color: ColorKey): string {
  return color === 'b' ? '#222' : 'white';
}

export function getBaseColor(color: ColorKey): string {
  return color === 'b' ? 'white' : '#222';
}

export function getBugColor(bugLetter: string, color: ColorKey): string {
  switch (bugLetter) {
    case 'A':
      return '#0fa9f0';
    case 'B':
      return '#8779b9';
    case 'G':
      return '#2fbc3d';
    case 'L':
      return '#d72833';
    case 'M':
      return '#a6a6a6';
    case 'P':
      return '#49ad92';
    case 'Q':
      return '#fcb336';
    case 'S':
      return '#9f622d';
  }
  return getBaseColor(color);
}

export { Bug };
