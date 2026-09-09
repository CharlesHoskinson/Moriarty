import { parseSuccessorSource } from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/frontend.ts';
for (const point of [0x301,0x903]) {
 const source=`profile "moriarty-successor-syntax/0"; agreement P { unit ${String.fromCodePoint(point)}; }`;
 try {parseSuccessorSource(source);console.log('unexpected acceptance');} catch(error){console.log(JSON.stringify({source,codepoint:'U+'+point.toString(16).toUpperCase().padStart(4,'0'),unicodeLetterDigitConnector:/[\p{L}\p{N}\p{Pc}]/u.test(String.fromCodePoint(point)),code:error.code,start:error.start,end:error.end}));}
}
