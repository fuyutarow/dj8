export const wasmImport = import('dj8-wasm');

export const fact = async (n: number) => {
  const { fact } = await wasmImport;
  return fact(n);
};

export const getName = async () => {
  const res = await fetch(
    'https://random-word-api.herokuapp.com/word?number=1'
  );
  const json = await res.json();
  return json[0];
};

export const beep = async () => {
  const { beep } = await wasmImport;
  return beep();
};

export const play_music = async () => {
  const { play_music } = await wasmImport;
  return play_music();
};
