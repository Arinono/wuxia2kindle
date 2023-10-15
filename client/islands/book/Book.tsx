import { Book } from '../../models/book.ts';
import { Chapter } from '../../models/chapter.ts';
import { signal } from '@preact/signals';
import { useState } from 'preact/hooks';
import BookCover from './BookCover.tsx';
import Export from './Export.tsx';

type Props = {
  book: Book;
  chapters: Array<Chapter>;
};

async function updateCover(
  url: string,
  token: string,
  id: number,
  cover: string,
): Promise<void> {
  await fetch(`${url}/book/${id}`, {
    method: 'PATCH',
    headers: {
      'content-type': 'application/json',
      authorization: `Basic ${token}`,
    },
    body: JSON.stringify({ cover }),
  });
}

export default function Book({ book, chapters }: Props) {
  const apiUrl =
    'Deno' in window && Deno !== undefined
      ? Deno.env.get('API_URL')
      : document.getElementById('api')?.getAttribute('data-url');
  const apiToken =
    'Deno' in window && Deno !== undefined
      ? Deno.env.get('API_TOKEN')
      : document.getElementById('api')?.getAttribute('data-token');

  const revChapters = Array.from(chapters);
  revChapters.reverse();

  const cover = signal(book.cover ?? null);
  const [asc, setAsc] = useState(false);

  cover.subscribe((val) => {
    if (val && val !== book.cover && typeof apiUrl === 'string') {
      updateCover(apiUrl, apiToken, book.id, val);
      // snackbar
    }
  });

  return (
    <>
      <div class='flex flex-row no-wrap'>
        <div
          class='h-76'
          style={'width: calc(300px / 1.45)'}
        >
          <BookCover
            readonly={false}
            cover={cover}
          />
        </div>
        <div class='flex flex-col justify-between ml-8'>
          <div>
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
          {chapters.length > 1 && (
            <Export
              name={book.name}
              book_id={book.id}
              chapters={chapters}
              apiUrl={apiUrl ?? null}
              apiToken={apiToken ?? null}
            />
          )}
        </div>
      </div>

      <div class='flex flex-row items-center'>
        <h2 class='text-5xl mt-8 mb-4 mr-8'>
          Chapters <span class='text-3xl'>({book.chapter_count})</span>
        </h2>
        <div
          class='text-4xl mt-6 cursor-pointer select-none'
          onClick={() => {
            setAsc(!asc);
          }}
        >
          {asc ? '⬇️ ' : '⬆️'}
        </div>
      </div>
      <ul class='overflow-y-auto grid grid-cols-2'>
        {(asc ? chapters : revChapters).map((c) => (
          <li>
            <a href={`/chapter/${c.id}`}>
              <strong>({c.number_in_book})</strong> {c.name}
            </a>
          </li>
        ))}
      </ul>
    </>
  );
}
