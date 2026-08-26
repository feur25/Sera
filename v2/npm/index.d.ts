export * from './pkg/seraplot';

export declare class Chart {
  html: string;
  constructor(html: string);
  call(name: string, argsJson?: string): Chart;
  toSvg(): string;
  save(path: string): Chart;
  toString(): string;
}

export declare function call(name: string, json: string): Chart;
