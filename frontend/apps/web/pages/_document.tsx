import Document, { Head, Html, Main, NextScript } from 'next/document';
import { Favicon } from '../components/common/Favicon';

class HiveDocument extends Document {
  render() {
    return (
      <Html>
        <Head>
          <Favicon />
        </Head>
        <body className='antialiased'>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default HiveDocument;
