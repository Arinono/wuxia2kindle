import { Handlers, PageProps } from '$fresh/server.ts';
import { Book } from '../../models/book.ts';

type Response = {
  GetBook: {
    data: Book | null;
  };
};

export const handler: Handlers<Book | null> = {
  async GET(_req, ctx) {
    const { id } = ctx.params;
    const resp = await fetch(`${Deno.env.get('API_URL')}/book/${id}`, {
      method: 'GET',
      headers: {
        'content-type': 'application/json',
      },
    });
    if (resp.status !== 200) {
      return ctx.render(null);
    }

    const parsed: Response = await resp.json();
    return ctx.render(parsed.GetBook.data);
  },
};

export default function Book({ data }: PageProps<Book | null>) {
  if (!data) {
    return <div>Book not found 🥲 </div>;
  }
  return <div>{data.name}</div>;
}
