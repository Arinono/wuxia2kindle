import { Option } from "./misc.ts";

export type Book = {
  id: number;
  name: string;
  chapter_count: Option<number>;
  author: Option<string>;
  translator: Option<string>;
  cover: Option<string>;
};
