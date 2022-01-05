import { useNotifications } from '../contexts/notifications/NotificationProvider';

function useTitle() {
  const { unread } = useNotifications();
  return unread === 0
    ? `lihive.org • Free Online Hive`
    : `(${unread}) lihive.org • Free Online Hive`;
}

export { useTitle };
