import { Head } from '$fresh/runtime.ts';
import { Handlers, PageProps } from '$fresh/server.ts';
import { BookCard } from '../components/BookCard.tsx';
import { Book } from '../models/book.ts';

type Response = {
  GetBooks: {
    data: Array<Book>;
  };
};

export const handler: Handlers<Array<Book> | null> = {
  async GET(_req, ctx) {
    const resp = await fetch(`${Deno.env.get('API_URL')}/books`, {
      method: 'GET',
      headers: {
        'content-type': 'application/json',
      },
    });
    if (resp.status !== 200) {
      return ctx.render(null);
    }

    const parsed: Response = await resp.json();
    return ctx.render(parsed.GetBooks.data);
  },
};

export default function Home({ data }: PageProps<Array<Book> | null>) {
  if (!data) {
    return <h2>no books 🥲</h2>;
  }

  return (
    <>
      <Head>
        <title>Wuxia 2 Kindle</title>
      </Head>
      <div>
        <ul class='flex flex-wrap justify-between'>
          {data.map((b) => (
            <li class='mb-4'>
              <BookCard book={b} />
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}
