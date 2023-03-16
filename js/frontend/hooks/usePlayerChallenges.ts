import { getUserChallenges, usePlayer } from 'hive-db';
import useSWR from 'swr';

const fetcher = ([user, authToken]) => getUserChallenges(user, authToken);

export function usePlayerChallenges() {
    const { user, authToken } = usePlayer();
    const { data, error, mutate, isLoading } = useSWR([user, authToken], fetcher);

    return {
        challenges: data,
        error,
        mutate,
        isLoading,
    };
}
