import { Handlers, PageProps } from '$fresh/server.ts';
import { Book, Chapter } from '../../models/index.ts';

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

  const { book, chapters } = data;
  const cover = `data:image/png;base64,${book.cover}`;

  return (
    <div class='ml-4'>
      <div class="flex flex-row no-wrap h-76">
        <img
          src={cover}
          alt={`${book.name} cover`}
        />
        <div class="ml-8">
          <h1 class='text-6xl'>{book.name}</h1>
          <div class='mt-4'>
            <h3>
              <strong>Written by</strong>: {book.author}
            </h3>
            <h3>
              <strong>Translated by</strong>: {book.translator}
            </h3>
          </div>
        </div>
      </div>

      <h2 class='text-5xl mt-8 mb-4'>Chapters <span class="text-3xl">({ book.chapter_count })</span></h2>
      <ul>
        {chapters.map((c) => (
          <li>{c.name}</li>
        ))}
      </ul>
    </div>
  );
}
