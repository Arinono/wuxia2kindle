import { useSignal } from '@preact/signals';
import { Book } from '../models/book.ts';
import { BookCover } from '../components/BookCover.tsx';

type Props = {
  book: Book;
};

export default function BookCard({ book }: Props) {
  const cover = `data:image/png;base64,${book.cover}`;
  const maxNameLen = 40;
  const name =
    book.name.length > maxNameLen
      ? book.name.substring(0, maxNameLen).concat('...')
      : book.name;

  const open = useSignal(false);

  function toggle() {
    open.value = !open.value;
  }

  function goToBook() {
    location.href = `book/${book.id}`;
  }

  return (
    <>
      <div class='w-full h-full'>
        <div class='w-full h-full relative group'>
          {book.cover && (
            <img
              src={cover}
              alt={`${name} cover`}
              class='h-full object-cover'
            />
          )}
          {!book.cover && <BookCover />}
          <div
            class={`w-full flex flex-col items-end absolute inset-0
            transition-all ease-in-out duration-200 bg-gray-800 bg-opacity-80 ${
              open.value ? 'max-h-full' : 'max-h-0'
            }`}
          >
            <div
              class='mx-4 mt-2 cursor-pointer text-xl'
              onClick={toggle}
            >
              {!open.value ? 'ℹ' : '╳'}
            </div>
            {open.value && (
              <div class='pt-8 px-4'>
                <h3 class='mb-2 text-sm'>
                  <strong>Author</strong>: {book.author}
                </h3>
                <h3 class='mb-4 text-sm'>
                  <strong>Translator</strong>: {book.translator}
                </h3>
                <h4 class='text-sm'>
                  <strong>Chapters</strong>: {book.chapter_count}
                </h4>
              </div>
            )}
          </div>
          <div
            class='w-full h-0 group-hover:h-16 flex justify-center items-center
            text-lg inset-x-0 bottom bg-gray-800 bg-opacity-80 absolute
            transition-all ease-in-out duration-200 group-hover:-translate-y-16
            relative'
          >
            <div class='opacity-0 group-hover:opacity-100'>
              <h2>{name}</h2>
              <div class='absolute right-0 bottom-0 pb-2 pr-2 pt-4 pl-4 text-lg cursor-pointer' onClick={goToBook}>⤴ </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
