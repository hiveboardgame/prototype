import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { Game, getGameNotation, getMoveCount, playGameMove } from 'hive-db';
import {
  buildBoard,
  buildMove,
  getGameMoves,
  getTopTile,
  HexCoordinate,
  hexesEqual,
  StackCoordinate,
  TileId
} from 'hive-lib';
import { canProposeMove, initializeGame, notNull } from './game';

const initialState = initializeGame();

const slice = createSlice({
  name: 'game',
  initialState,
  reducers: {
    boardCentered(state) {
      state.boardCentered = new Date().toJSON();
    },
    gameChanged(state, action: PayloadAction<Game>) {
      const game = action.payload;
      if (state.game === null || game.gid !== state.game.gid) {
        // The game is being initialized so we're going to view the latest move.
        state.upTo = -1;
        state.newMovesToView = false;
      } else {
        // The game is being updated, so check if a move was played. If so and
        // the user isn't viewing the most recent move, set the game data
        // updated flag.
        const oldNotation = getGameNotation(state.game);
        const newNotation = getGameNotation(game);
        if (oldNotation !== newNotation && state.upTo !== -1) {
          state.newMovesToView = true;
        }
      }
      state.game = game;
      state.proposedMove = null;
      state.proposedMoveCoordinate = null;
    },
    ghostClicked(state, action: PayloadAction<HexCoordinate>) {
      const coordinate = action.payload;
      const { game, selectedTileId, validNextMoves } = state;

      if (canProposeMove(state) && notNull(game) && notNull(selectedTileId)) {
        const moves = getGameMoves(getGameNotation(game));
        const board = buildBoard(moves);
        state.proposedMove = buildMove(board, selectedTileId, coordinate);
        state.proposedMoveCoordinate = coordinate;
        state.upTo = -1;
      }
    },
    firstMoveClicked(state) {
      state.upTo = 0;
      state.proposedMove = null;
      state.proposedMoveCoordinate = null;
      state.selectedTileId = null;
    },
    handStackClicked(state, action: PayloadAction<TileId[]>) {
      const stack = action.payload;
      state.selectedTileId = stack[stack.length - 1];
      state.proposedMove = null;
      state.proposedMoveCoordinate = null;
    },
    lastMoveClicked(state) {
      state.upTo = -1;
      state.proposedMove = null;
      state.proposedMoveCoordinate = null;
      state.selectedTileId = null;
      state.newMovesToView = false;
    },
    nextMoveClicked(state) {
      if (notNull(state.game)) {
        const totalMoves = getMoveCount(state.game);
        if (0 <= state.upTo && state.upTo < totalMoves) {
          state.upTo += 1;
          if (state.upTo === totalMoves) {
            state.upTo = -1;
            state.newMovesToView = false;
          }
          state.proposedMove = null;
          state.proposedMoveCoordinate = null;
          state.selectedTileId = null;
        }
      }
    },
    previousMoveClicked(state) {
      if (notNull(state.game)) {
        const totalMoves = getMoveCount(state.game);
        if (state.upTo === -1) state.upTo = totalMoves;
        if (state.upTo > 0) state.upTo -= 1;
        state.proposedMove = null;
        state.proposedMoveCoordinate = null;
        state.selectedTileId = null;
      }
    },
    tableClicked(state) {
      state.selectedTileId = null;
      state.proposedMove = null;
      state.proposedMoveCoordinate = null;
    },
    tableStackClicked(state, action: PayloadAction<StackCoordinate>) {
      const { stack, coordinate } = action.payload;
      const { game, proposedMove, proposedMoveCoordinate } = state;

      if (
        notNull(game) &&
        notNull(proposedMove) &&
        notNull(proposedMoveCoordinate) &&
        hexesEqual(coordinate, proposedMoveCoordinate)
      ) {
        // The user has clicked their proposed move, so send it to the server
        state.selectedTileId = null;
        playGameMove(game, proposedMove)
          .then(({ game, validNextMoves }) => {
            state.game = game;
            state.validNextMoves = validNextMoves;
          })
          .catch((error) => {
            console.error(error);
          });
      } else {
        // The user has selected a new tile
        state.selectedTileId = getTopTile(stack);
        state.proposedMove = null;
        state.proposedMoveCoordinate = null;
      }
    },
    uidChanged(state, action: PayloadAction<string | null>) {
      state.uid = action.payload;
    },
    viewUpToPicked(state, action: PayloadAction<number>) {
      if (notNull(state.game)) {
        state.upTo = action.payload;
        state.proposedMove = null;
        state.proposedMoveCoordinate = null;
        state.selectedTileId = null;
      }
    }
  }
});

export const {
  boardCentered,
  firstMoveClicked,
  gameChanged,
  ghostClicked,
  lastMoveClicked,
  nextMoveClicked,
  previousMoveClicked,
  handStackClicked,
  tableClicked,
  tableStackClicked,
  uidChanged,
  viewUpToPicked
} = slice.actions;
export default slice.reducer;
