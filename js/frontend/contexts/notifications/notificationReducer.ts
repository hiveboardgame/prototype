import { Game, getTurnUid, UserData } from 'hive-db';
import { keyBy } from 'lodash';
import { turnNotification, Notification } from './notification';

interface State {
  user: UserData | null;
  notifications: Notification[];
  unread: number;
}

type ActionGames = { type: 'games'; data: { user: UserData; games: Game[] } };
type ActionRead = { type: 'mark-read'; notifications: Notification[] };
type Action = ActionGames | ActionRead;

function initialState(): State {
  return {
    user: null,
    notifications: [],
    unread: 0
  };
}

function notificationReducer(state: State, action: Action): State {
  switch (action.type) {
    case 'games':
      return handleGames(state, action);
    case 'mark-read':
      return handleMarkRead(state, action);
  }
  return state;
}

function handleGames(state: State, action: ActionGames): State {
  const { user, games } = action.data;

  // if the user is signed out then there cannot be any notifications
  if (user === null) {
    return initialState();
  }

  // if the user changes, start with any empty list of notifications, otherwise
  // use existing notifications
  const notifications = user !== state.user ? [] : state.notifications;

  // get the games where it's the user's turn and map games by their ids
  const turnGames = games.filter((g) => getTurnUid(g) === user.uid);
  const byId = keyBy(turnGames, (g) => g.gid);

  // get the ids of games we already have notifications for and of all current games
  const existingIds = new Set(notifications.map((n) => n.id));
  const allIds = new Set(turnGames.map((g) => g.gid));

  // create a list of notifications to add and of ones to remove
  const toAdd = setDifference(allIds, existingIds);
  const toRemove = setDifference(existingIds, allIds);

  // create the new list of notifications
  const newNotifications = [
    ...Array.from(toAdd).map((gid) => turnNotification(user.uid, byId[gid])),
    ...notifications.filter((n) => !toRemove.has(n.id))
  ];

  // count how many are unread
  const unread = newNotifications.filter((n) => !n.read).length;

  return {
    user,
    unread,
    notifications: newNotifications
  };
}

function handleMarkRead(state: State, action: ActionRead): State {
  const ids = new Set(action.notifications.map((n) => n.id));
  const notifications = state.notifications.map((n) => {
    return ids.has(n.id) ? { ...n, read: true } : n;
  });
  const unread = notifications.filter((n) => !n.read).length;
  return {
    ...state,
    notifications,
    unread
  };
}

/**
 * Compute the set difference {a} - {b}
 */
function setDifference<T>(a: Set<T>, b: Set<T>): Set<T> {
  const diff = new Set(a);
  b.forEach((el) => diff.delete(el));
  return diff;
}

export { notificationReducer, initialState };
