import { configureStore } from '@reduxjs/toolkit';
import gameReducer from './slice';

const store = configureStore({
  reducer: gameReducer
});

export type GameState = ReturnType<typeof store.getState>;
export type GameDispatch = typeof store.dispatch;

export default store;
