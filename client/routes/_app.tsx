import { AppProps } from '$fresh/server.ts';
import { TopBar } from '../components/Topbar.tsx';

export default function App({ Component }: AppProps) {
  return (
    <>
      <TopBar />
      <div
        class='text-gray-200 p-2 bg-gray-900 pt-4'
        style='height: calc(100vh - 48px)'
      >
        <Component />
      </div>
    </>
  );
}
