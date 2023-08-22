import { Handlers, PageProps } from '$fresh/server.ts';
import TBook from '../../islands/book/Book.tsx';
import { type Book, type Chapter } from '../../models/index.ts';

type GetBookResponse = {
  GetBook: {
    data: Book | null;
  };
};

type GetChaptersResponse = {
  GetChapters: {
    data: Array<Chapter>;
  };
};

type Data = {
  book: Book | null;
  chapters: Array<Chapter>;
};

export const handler: Handlers<Data | null> = {
  async GET(_req, ctx) {
    const { id } = ctx.params;
    const get_book = await fetch(`${Deno.env.get('API_URL')}/book/${id}`, {
      method: 'GET',
      headers: {
        'content-type': 'application/json',
      },
    });
    if (get_book.status !== 200) {
      return ctx.render(null);
    }

    const book: GetBookResponse = await get_book.json();

    if (book.GetBook.data === null) {
      return ctx.render({
        book: null,
        chapters: [],
      });
    }

    const get_chapters = await fetch(
      `${Deno.env.get('API_URL')}/book/${id}/chapters`,
      {
        method: 'GET',
        headers: {
          'content-type': 'application/json',
        },
      },
    );
    if (get_chapters.status !== 200) {
      return ctx.render(null);
    }

    const chapters: GetChaptersResponse = await get_chapters.json();

    return ctx.render({
      book: book.GetBook.data,
      chapters: chapters.GetChapters.data,
    });
  },
};

export default function Book({ data }: PageProps<Data | null>) {
  if (!data || !data.book) {
    return <div>Book not found 🥲</div>;
  }

  return (
    <TBook
      book={data.book}
      chapters={data.chapters}
    />
  );
}
