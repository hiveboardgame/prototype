import { HTMLAttributes, useEffect, useState } from 'react';
import { RowItem } from '../Row';
import {
  differenceInSeconds,
  formatDistanceToNow,
  formatDistanceToNowStrict
} from 'date-fns';

interface ElapsedTimeItemProps {
  since: Date | null;
  placeholder: string;
}

const ElapsedTimeItem = (
  props: HTMLAttributes<HTMLDivElement> & ElapsedTimeItemProps
) => {
  const { since, placeholder, ...rest } = props;
  const [elapsed, setElapsed] = useState<string>(
    formatDistance(since, placeholder)
  );

  useEffect(() => {
    const id = setInterval(() => {
      setElapsed(formatDistance(since, placeholder));
    }, 1000);
    return () => clearInterval(id);
  }, [since, placeholder]);

  return <RowItem {...rest}>{elapsed}</RowItem>;
};

function formatDistance(since: Date | null, placeholder: string): string {
  if (!since) return placeholder;
  return differenceInSeconds(Date.now(), since) < 59
    ? formatDistanceToNowStrict(since, { addSuffix: true })
    : formatDistanceToNow(since, { addSuffix: true });
}

export { ElapsedTimeItem };
