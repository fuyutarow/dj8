import { NextPage } from 'next';
import dynamic from 'next/dynamic';
import React, { useState, useEffect, useContext, useRef, useCallback } from 'react';

import { OGP } from 'models/ogp';
import { MetaHead } from 'components/Header';
import { fact, getName } from 'workers/wasm.worker';
import * as worker from 'workers/wasm.worker';

const View: React.FC = () => {
  const [number, setNumber] = useState(0);
  const [name, setName] = useState("");
  const [handle, setHandle] = useState<any>(null);
  const [handle2, setHandle2] = useState<any>(null);

  const Beep = () => {
    return (
      <button onClick={async () => {
        // handleRef.current = await worker.beep();
        const h = await worker.beep();
        setHandle(h);
      }}>beep</button>
    )
  }

  const Stop = () => {
    return (
      <button onClick={async () => {
        handle.free();
        setHandle(null);
      }}>stop</button>
    )
  }

  return (
    <div>
      <div>
        <button onClick={() => alert('hello')}>greet</button>
      </div>
      <div>
        {
          handle
            ? <Stop />
            : <Beep />
        }
      </div>
      <div>
        {
          handle2
            ? (
              <button onClick={async () => {
                handle2.free();
                setHandle2(null);
              }}>stop music</button>
            )
            : (
              <button onClick={async () => {
                const h = await worker.play_music();
                setHandle2(h);
              }}>play music</button>
            )
        }
      </div>
      <div>
        <button onClick={async () => {
          const res = await getName();
          setName(res);
        }}>getName</button>
        <div>{name}</div>
      </div>
      <div>
        <button onClick={async () => {
          const res = await fact(10);
          setNumber(res);
        }}>calc</button>
        <div>{number}</div>
      </div>
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
