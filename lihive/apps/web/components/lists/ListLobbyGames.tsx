import { HTMLAttributes } from 'react';

interface ListLobbyGamesProps extends HTMLAttributes<HTMLDivElement> {}

const ListLobbyGames = (props: ListLobbyGamesProps) => {
  const { className, ...rest } = props;
  return (
    <div
      className={`w-full h-32 bg-white flex items-center justify-center prose max-w-none ${
        className || ''
      }`}
      {...rest}
    >
      Coming Soon!
    </div>
  );
};

export { ListLobbyGames };
