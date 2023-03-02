import { usePlayer } from 'hive-db';
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useReducer
} from 'react';
import { initialState, notificationReducer } from './notificationReducer';
import { Notification } from './notification';

export interface NotificationsContextProps {
  notifications: Notification[];
  unread: number;
  markRead: (notifications: Notification[]) => void;
}

const notificationsContext = createContext<NotificationsContextProps>(
  defaultNotificationsContext()
);

const NotificationProvider = ({ children }: { children?: ReactNode }) => {
  const notificationsState = useNotificationsState();
  return (
    <notificationsContext.Provider value={notificationsState}>
      {children}
    </notificationsContext.Provider>
  );
};

const useNotifications = () => {
  return useContext(notificationsContext);
};

function useNotificationsState(): NotificationsContextProps {
  const { user, activeGames } = usePlayer();
  const [state, dispatch] = useReducer(notificationReducer, initialState());

  useEffect(() => {
    dispatch({ type: 'games', data: { user: user, games: activeGames } });
  }, [user, activeGames]);

  return {
    notifications: state.notifications,
    unread: state.unread,
    markRead: (notifications: Notification[]) => {
      dispatch({ type: 'mark-read', notifications });
    }
  };
}

function defaultNotificationsContext(): NotificationsContextProps {
  return {
    notifications: [],
    unread: 0,
    markRead: (_) => {}
  };
}

export { NotificationProvider, useNotifications };
