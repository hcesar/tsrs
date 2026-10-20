type LetterMap = { A: string, B: number }
type LetterCaller<K extends keyof LetterMap> = { [P in K]: { letter: Record<P, LetterMap[P]>, caller: (x: Record<P, LetterMap[P]>) => void } }[K];

function call<K extends keyof LetterMap>({ letter, caller }: LetterCaller<K>): void {
  caller(letter);
}

type A = { A: string };
type B = { B: number };
type ACaller = (a: A) => void;
type BCaller = (b: B) => void;

declare const xx: { letter: A, caller: ACaller } | { letter: B, caller: BCaller };

call(xx);
