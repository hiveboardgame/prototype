import { ForwardedRef, forwardRef, HTMLAttributes } from 'react';
import { HiveIcon } from '../common/HiveIcon';

const HiveLogoLink = forwardRef(
  (
    props: HTMLAttributes<HTMLAnchorElement>,
    ref: ForwardedRef<HTMLAnchorElement>
  ) => {
    const { className, ...rest } = props;
    return (
      <a
        className={`group flex items-center flex-shrink-0 mr-6 cursor-pointer ${
          className || ''
        }`}
        ref={ref}
        {...rest}
      >
        <HiveIcon
          className='fill-hive transition-all motion-reduce:transition-none group-hover:rotate-[-30deg]'
          width={34}
          height={34}
        />
        <span className='relative ml-2'>
          <span className='relative prose font-semibold text-xl tracking-tight'>
            lihive
          </span>
          <span className='prose font-semibold text-xl tracking-tight text-hive'>
            .org
          </span>
          <span className='transition-all absolute bottom-[1px] left-0 w-0 h-[2px] bg-hive group-hover:w-[70px]' />
        </span>
      </a>
    );
  }
);
HiveLogoLink.displayName = 'HiveLogoLink';

export { HiveLogoLink };
