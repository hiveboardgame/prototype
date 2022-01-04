import Document, { Head, Html, Main, NextScript } from 'next/document';

class HiveDocument extends Document {
  render() {
    return (
      <Html>
        <Head />
        <body className='antialiased'>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default HiveDocument;
