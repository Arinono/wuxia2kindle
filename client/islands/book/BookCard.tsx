import { useSignal } from '@preact/signals';
import { Book } from '../../models/book.ts';
import BookCover from './BookCover.tsx';

type Props = {
  book: Book;
};

export default function BookCard({ book }: Props) {
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
          <BookCover
            readonly
            cover={book.cover}
            alt={`${name} cover`}
          />
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
            relative cursor-pointer'
            onClick={goToBook}
          >
            <div class='opacity-0 group-hover:opacity-100'>
              <h2>{name}</h2>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
