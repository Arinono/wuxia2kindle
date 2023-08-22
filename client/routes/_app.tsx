import { AppProps } from '$fresh/server.ts';
import TopBar from '../islands/TopBar.tsx';

export default function App({ Component }: AppProps) {
  return (
    <>
      <div
        id='api_url'
        data-url={Deno.env.get('API_URL')}
      />
      <div class='text-gray-200'>
        <TopBar />
        <div
          class='p-2 bg-gray-900 pt-4 overflow-hidden'
          style='height: calc(100vh - 48px)'
        >
          <Component />
        </div>
      </div>
    </>
  );
}
