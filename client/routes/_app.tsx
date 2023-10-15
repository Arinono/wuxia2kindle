import { AppProps } from '$fresh/server.ts';
import TopBar from '../islands/TopBar.tsx';

export default function App({ Component }: AppProps) {
  return (
    <>
      <div
        id='api'
        data-url={Deno.env.get('API_URL')}
        data-token={Deno.env.get('API_TOKEN')}
      />
      <div class='text-gray-200'>
        <TopBar />
        <div
          class='flex flex-col p-4 bg-gray-900 overflow-hidden'
          style='height: calc(100vh - 48px)'
        >
          <Component />
        </div>
      </div>
    </>
  );
}
