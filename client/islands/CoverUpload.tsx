import { type Signal } from '@preact/signals';
import { createRef } from 'preact';

type Props = {
  cover: Signal<string | null>;
};

export default function CoverUpload(props: Props) {
  const input = createRef();

  function file2B64(file: File): Promise<string> {
    return new Promise((resolve, reject) => {
      const fr = new FileReader();
      fr.readAsDataURL(file);

      fr.onload = () => {
        if (typeof fr.result === 'string') {
          resolve(fr.result);
        } else {
          reject('not a valid image ?');
        }
      };

      fr.onerror = reject;
    });
  }

  async function onFilePicked(evt: Event): Promise<void> {
    const target = evt.target as HTMLInputElement;
    if (target === null || target.files === null) {
      return;
    }

    const file = target.files[0];
    const b64file = await file2B64(file);
    props.cover.value = b64file;

    input.current.removeEventListener('change', onFilePicked);
  }

  function onClickHandler() {
    input.current.addEventListener('change', onFilePicked);
    input.current.click();
  }

  return (
    <div class='w-4 h-4'>
      <button
        class='cursor-pointer focus:outline-none select-none'
        onClick={onClickHandler}
      >
        🖼️
      </button>
      <input
        ref={input}
        type='file'
        class='hidden'
        accept='image/png'
      />
    </div>
  );
}
