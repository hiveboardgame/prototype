import { Game, getOpponentColor, getOpponentUsername } from 'hive-db';
import { ColorKey } from 'hive-lib';

export interface InviteNotification {
  type: 'invite';
  id: string;
  read: boolean;
  gid: string;
  opponent: string;
}

export interface TurnNotification {
  type: 'turn';
  id: string;
  read: boolean;
  gid: string;
  opponent: string;
  opponentColor: ColorKey;
}

export type Notification = TurnNotification | InviteNotification;

function inviteNotification(uid: string, game: Game): InviteNotification {
  const { gid } = game;
  const opponent = getOpponentUsername(game, uid);
  return {
    type: 'invite',
    id: gid,
    read: false,
    gid,
    opponent
  };
}

function isInviteNotification(
  notification: Notification
): notification is InviteNotification {
  return notification.type === 'invite';
}

function turnNotification(uid: string, game: Game): TurnNotification {
  const { gid } = game;
  const opponent = getOpponentUsername(game, uid);
  const opponentColor = getOpponentColor(game, uid);
  return {
    type: 'turn',
    id: gid,
    read: false,
    gid,
    opponent,
    opponentColor
  };
}

function isTurnNotification(
  notification: Notification
): notification is TurnNotification {
  return notification.type === 'turn';
}

export { turnNotification, isTurnNotification };
