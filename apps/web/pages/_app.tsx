import '@fontsource/inter/variable-full.css';
import 'focus-visible/dist/focus-visible';
import '../styles/app.css';
import { ChakraProvider } from '@chakra-ui/react';
import { Global, css } from '@emotion/react';
import { AppProps } from 'next/app';
import { PlayerProvider } from 'hive-db';
import { NotificationProvider } from '../contexts/notifications/NotificationProvider';
import { theme } from '../styles/theme';

const GlobalStyle = () => {
  return (
    <Global
      styles={css`
        #__next {
          display: flex;
          flex-direction: column;
          width: 100vw;
          height: 100vh;
          overflow-x: hidden;
        }
        .js-focus-visible :focus:not([data-focus-visible-added]) {
          outline: none;
          box-shadow: none;
        }
      `}
    />
  );
};

function App({ Component, pageProps }: AppProps) {
  return (
    <ChakraProvider theme={theme}>
      <PlayerProvider>
        <NotificationProvider>
          <GlobalStyle />
          <Component {...pageProps} />
        </NotificationProvider>
      </PlayerProvider>
    </ChakraProvider>
  );
}

export default App;
