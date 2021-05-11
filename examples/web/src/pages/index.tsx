import { NextPage } from 'next';
import dynamic from 'next/dynamic';
import React, { useState, useEffect, useContext, useRef, useCallback } from 'react';

import { OGP } from 'models/ogp';
import { MetaHead } from 'components/Header';
import { fact, getName } from 'workers/wasm.worker';

const View: React.FC = () => {
  const [number, setNumber] = useState(0);
  const [name, setName] = useState("");

  return (
    <div>
      <div>ok</div>
      <button onClick={() => alert('hello')}>greet</button>
      <button onClick={async () => {
        const res = await getName();
        setName(res);
      }}>getName</button>
      <div>{name}</div>
      <button onClick={async () => {
        const res = await fact(10);
        setNumber(res);
      }}>calc</button>
      <div>{number}</div>
    </div>
  )
}

const Page: NextPage<{
  ogp: OGP
}> = ({ ogp }) => {

  return (
    <>
      <MetaHead {...{ ogp }} />
      <View />
    </>
  );
};

Page.getInitialProps = async ({ req }) => {
  const ogp: OGP = {
    title: 'dj8',
    url: 'https://dj8.vercel.app',
    description: 'https://github.com/fuyutarow/dj8',
    // imageUrl: req ? `//${req.headers.host}${icons[512]}` : icons[512],
  };

  return { ogp };
};

export default Page;
