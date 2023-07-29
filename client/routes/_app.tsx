import { AppProps } from '$fresh/server.ts';
import TopBar from '../islands/TopBar.tsx';

export default function App({ Component }: AppProps) {
  return (
    <>
      <div class='text-gray-200'>
        <TopBar />
        <div
          class='p-2 bg-gray-900 pt-4'
          style='height: calc(100vh - 48px)'
        >
          <Component />
        </div>
      </div>
    </>
  );
}
