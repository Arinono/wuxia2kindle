import { Book } from '../models/book.ts';

type Props = {
  book: Book;
};
export function BookCard({ book }: Props) {
  console.log(book);
  const cover = `"data:image/png;base64,${book.cover}"`;
  const maxNameLen = 40;
  const name =
    book.name.length > maxNameLen
      ? book.name.substring(0, maxNameLen).concat('...')
      : book.name;

  return (
    <>
      <div class='w-48'>
        <div
          style={`background-image: url(${cover})`}
          class='h-64 bg-center bg-no-repeat bg-contain border'
        >
          <span class="">ℹ</span>
        </div>
        <div class='text-lg'>{name}</div>
      </div>
    </>
  );
}
