import { Book } from '../../models/book.ts';
import { Chapter } from '../../models/chapter.ts';
import { signal } from '@preact/signals';
import BookCover from './BookCover.tsx';

type Props = {
  book: Book;
  chapters: Array<Chapter>;
};

async function updateCover(
  url: string,
  id: number,
  cover: string,
): Promise<void> {
  await fetch(`${url}/book/${id}`, {
    method: 'PATCH',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify({ cover }),
  });
}

export default function Book({ book, chapters }: Props) {
  const apiUrl =
    'Deno' in window && Deno !== undefined
      ? Deno.env.get('API_URL')
      : document.getElementById('api_url')?.getAttribute('data-url');

  const revChapters = Array.from(chapters);
  revChapters.reverse();

  const cover = signal(book.cover ?? null);

  cover.subscribe((val) => {
    if (val && val !== book.cover && typeof apiUrl === 'string') {
      updateCover(apiUrl, book.id, val);
      // snackbar
    }
  });

  return (
    <div class='ml-4'>
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
        <div class='ml-8'>
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

      <h2 class='text-5xl mt-8 mb-4'>
        Chapters <span class='text-3xl'>({book.chapter_count})</span>
      </h2>
      <ul class='h-64 overflow-y-scroll'>
        {revChapters.map((c) => (
          <li>
            {c.id} - {c.name}
          </li>
        ))}
      </ul>
    </div>
  );
}
