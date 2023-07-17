import { Head } from "$fresh/runtime.ts";
import { useSignal } from "@preact/signals";
import { Handlers, PageProps } from "$fresh/server.ts";

type Option<T> = T | null;
type Book = {
  id: number,
  name: string,
  chapter_count: Option<number>,
  author: Option<string>,
  translator: Option<string>,
  cover: Option<string>,
}

type Response = {
  GetBooks: {
    data: Array<Book>;
  }
}

export const handler: Handlers<Array<Book> | null> = {
  async GET(_req, ctx) {
    const resp = await fetch(`${Deno.env.get("API_URL")}/books`, {
      method: "GET",
      headers: {
        "content-type": "application/json",
      },
    })
    if (resp.status !== 200) {
      return ctx.render(null);
    }

    const parsed: Response = await resp.json();
    return ctx.render(parsed.GetBooks.data);
  },
}

export default function Home({ data }: PageProps<Array<Book> | null>) {
  if (!data) {
    return <h2>no books 🥲 </h2>;
  }

  // dumb display, will do a cmpt later
  return (
    <>
      <Head>
        <title>Wuxia 2 Kindle</title>
      </Head>
      <div>
        <ul>
          { data.map((b) => (
            <li>
              <h2>{ b.name } ({ b.chapter_count })</h2>
              <h4>{ b.author }</h4>
              <h4>{ b.translator }</h4>
              <img width={200} src={`data:image/pmg;base64,${b.cover}`} />
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}
