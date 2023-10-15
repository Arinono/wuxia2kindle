import { Signal, signal } from '@preact/signals';
import { createRef } from 'preact';
import { Chapter } from '../../models/chapter.ts';
import { useState } from 'preact/hooks';

type Props = {
  name: string;
  chapters: Array<Chapter>;
  apiUrl: string | null;
  book_id: number;
};

async function addToQueue(
  url: string,
  id: number,
  from: number,
  to: number,
): Promise<boolean> {
  return await fetch(`${url}/export`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      authorization: `Basic ${Deno.env.get('API_TOKEN')}`,
    },
    body: JSON.stringify({
      kind: {
        ChaptersRange: {
          book_id: id,
          chapters: [from, to],
        },
      },
    }),
  }).then(async (r) => {
    const text = await r.text();
    try {
      const res = JSON.parse(text);
      if ('AddToQueue' in res && 'success' in res.AddToQueue) {
        return res.AddToQueue.success;
      }
    } catch (_e) {
      console.error(text);
      throw new Error('unable to create export');
    }
  });
}

export default function Export({ name, book_id, chapters, apiUrl }: Props) {
  const dialog = createRef();
  const open = signal(false);
  const [success, setSuccess] = useState<boolean | null>(null);
  const [from, setFrom] = useState(chapters.at(0)!.number_in_book);
  const [to, setTo] = useState(
    chapters.at(chapters.length - 1)!.number_in_book,
  );

  const revChapters = Array.from(chapters);
  revChapters.reverse();

  open.subscribe((val) => {
    if (dialog && dialog.current) {
      if (val) {
        dialog.current.showModal();
        setTimeout(() => {
          dialog.current.addEventListener('close', onDialogClose);
        }, 0);
      } else {
        dialog.current.removeEventListener('close', onDialogClose);
      }
    }
  });

  function onDialogClose(e: Event) {
    e.preventDefault();
    if (e.target) {
      const [f_from, f_to] = (e.target as HTMLDialogElement).returnValue.split(
        ':',
      );
      if (f_from === 'cancel') {
        setSuccess(null);
        open.value = false;
        return;
      }
      if (typeof apiUrl === 'string') {
        addToQueue(apiUrl, book_id, parseInt(f_from), parseInt(f_to))
          .then((s) => {
            setSuccess(s);
          })
          .catch((e) => {
            console.log(e);
            setSuccess(false);
          });
      }
    }
  }

  const onInputHandler = (type: 'from' | 'to') => (e: Event) => {
    const target = e.target !== null ? (e.target as HTMLSelectElement) : null;

    if (target) {
      try {
        if (type === 'from') {
          setFrom(parseInt(target.value, 10));
        } else {
          setTo(parseInt(target.value, 10));
        }
      } catch (e) {
        // oof
        console.error(e);
      }
    }
  };

  function onSubmit(e: Event) {
    e.preventDefault();
    if (dialog && dialog.current) {
      dialog.current.close(`${from}:${to}`);
    }
  }

  return (
    <>
      <div class='flex items-center'>
        <div
          class='uppercase text-2xl rounded-md cursor-pointer
          w-fit-content px-4 py-2 select-none mr-8
          bg-indigo-500 hover:bg-indigo-600 active:bg-indigo-700
          '
          onClick={() => (open.value = !open.value)}
        >
          export
        </div>
        {typeof success === 'boolean' && success && (
          <span class='text-green-800'>Successfully created</span>
        )}
        {typeof success === 'boolean' && !success && (
          <span class='text-red-800'>Unable to create export</span>
        )}
      </div>
      <dialog ref={dialog}>
        <div class='flex flex-col'>
          <h2 class='text-2xl mb-8'>
            Creating an export for <strong>{name}</strong>
          </h2>
          <form class='flex flex-col'>
            <label class='flex justify-between'>
              <strong>From:</strong>
              <select
                name='from'
                class='ml-4'
                value={from}
                onInput={onInputHandler('from')}
              >
                {chapters.map((c) => (
                  <option value={c.number_in_book}>{c.name}</option>
                ))}
              </select>
            </label>
            <label class='flex justify-between mt-4'>
              <strong>To:</strong>
              <select
                name='to'
                class='ml-4'
                value={to}
              >
                {revChapters.map((c) => (
                  <option value={c.number_in_book}>{c.name}</option>
                ))}
              </select>
            </label>
            <div class='mt-8 flex justify-end w-full'>
              <button
                class='bg-red-400 hover:bg-red-500 active:bg-red-600
                  cursor-pointer text-lg px-4 py-2 rounded-md
                  focus:outline-none'
                value='cancel'
                formMethod='dialog'
              >
                Cancel
              </button>
              <button
                class='bg-indigo-400 hover:bg-indigo-500 active:bg-indigo-600
                  cursor-pointer text-lg px-4 py-2 rounded-md ml-4
                  focus:outline-none'
                value='export'
                onClick={onSubmit}
              >
                Export
              </button>
            </div>
          </form>
        </div>
      </dialog>
    </>
  );
}
