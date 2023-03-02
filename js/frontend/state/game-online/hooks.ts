import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux';
import { GameDispatch, GameState } from './store';

export const useGameDispatch = () => useDispatch<GameDispatch>();
export const useGameSelector: TypedUseSelectorHook<GameState> = useSelector;
