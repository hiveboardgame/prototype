import {
  forwardRef,
  Menu,
  MenuButton,
  MenuGroup,
  MenuGroupProps,
  MenuItem,
  MenuItemProps,
  MenuList
} from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { useCallback, useMemo } from 'react';
import { BsBell } from 'react-icons/bs';
import {
  isTurnNotification,
  TurnNotification
} from '../../contexts/notifications/notification';
import { useNotifications } from '../../contexts/notifications/NotificationProvider';
import { useHasMounted } from '../../hooks/useHasMounted';
import { HiveIcon } from '../common/HiveIcon';
import { NavText } from '../common/NavText';

const BellIdle = () => {
  return (
    <NavText className='relative'>
      <BsBell className='fill-slate-300' />
    </NavText>
  );
};

const BellActive = () => {
  return (
    <NavText className='relative'>
      <BsBell className='fill-hive' />
    </NavText>
  );
};

const BellNew = () => {
  return (
    <NavText className='relative'>
      <BsBell className='fill-hive' />
      <span className='animate-pulse absolute top-0 right-0 inline-block w-2 h-2 transform -translate-x-1/2 -translate-y-1/2 bg-hive rounded-full' />
    </NavText>
  );
};

const Bell = () => {
  const { notifications, unread } = useNotifications();
  if (!notifications.length) return <BellIdle />;
  return unread > 0 ? <BellNew /> : <BellActive />;
};

interface TurnItemProps extends MenuItemProps {
  notification: TurnNotification;
}

const TurnItem = forwardRef<TurnItemProps, any>((props, ref) => {
  const { notification, ...rest } = props;
  const { gid, opponent, opponentColor } = notification;
  const router = useRouter();

  const onClick = () => router.push(`/game/${gid}`);

  return (
    <MenuItem
      ref={ref}
      onClick={onClick}
      className='prose text-sm'
      icon={<HiveIcon hexColor={opponentColor} width={14} height={14} />}
      {...rest}
    >
      {opponent}
    </MenuItem>
  );
});

interface TurnGroupProps extends MenuGroupProps {
  notifications: TurnNotification[];
}

const TurnGroup = forwardRef<TurnGroupProps, any>((props, ref) => {
  const { notifications, ...rest } = props;
  return (
    <MenuGroup ref={ref} title={`It's your turn against:`} {...rest}>
      {notifications.map((n) => (
        <TurnItem key={n.gid} notification={n} />
      ))}
    </MenuGroup>
  );
});

const NotificationsBell = () => {
  const { notifications, markRead } = useNotifications();
  const mounted = useHasMounted();

  // Separate the notifications out into their types
  const turnNotifications = useMemo(() => {
    return notifications.filter(isTurnNotification);
  }, [notifications]);

  // When the user opens the menu, they'll have seen all of the notifications
  // so create a callback to mark them all as read
  const onOpen = useCallback(
    () => markRead(notifications),
    [markRead, notifications]
  );

  // For SSG, just render the idle bell
  if (!mounted) {
    return <BellIdle />;
  }

  // Otherwise render the fully functional bell
  return (
    <Menu onOpen={onOpen}>
      <MenuButton disabled={notifications.length === 0}>
        <Bell />
      </MenuButton>
      <MenuList>
        <TurnGroup notifications={turnNotifications} />
      </MenuList>
    </Menu>
  );
};

export { NotificationsBell };
