import { Handlers, PageProps } from '$fresh/server.ts';
import TBook from '../../islands/book/Book.tsx';
import { type Book, type Chapter } from '../../models/index.ts';
import { Option } from '../../models/misc.ts';

type GetChapterResponse = {
  GetChapter: {
    data: Option<Chapter>;
  };
};

type Data = {
  chapter: Option<Chapter>;
};

export const handler: Handlers<Data | null> = {
  async GET(_req, ctx) {
    const { id } = ctx.params;
    const get_chapter = await fetch(
      `${Deno.env.get('API_URL')}/chapter/${id}`,
      {
        method: 'GET',
        headers: {
          'content-type': 'application/json',
	  'authorization': `Basic ${Deno.env.get('API_TOKEN')}`,
        },
      },
    );
    if (get_chapter.status !== 200) {
      return ctx.render(null);
    }

    const chapter: GetChapterResponse = await get_chapter.json();

    if (chapter.GetChapter.data === null) {
      return ctx.render({
        chapter: null,
      });
    }

    return ctx.render({
      chapter: chapter.GetChapter.data,
    });
  },
};

export default function Book({ data }: PageProps<Data | null>) {
  if (!data || !data.chapter) {
    return <div>Book not found 🥲</div>;
  }

  const { name, content } = data.chapter;

  return (
    <>
      <h1>{name}</h1>

      <div
        class='mt-8 flex flex-col overflow-y-auto'
        dangerouslySetInnerHTML={{ __html: content }}
      />
    </>
  );
}
