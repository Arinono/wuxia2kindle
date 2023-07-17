import { AppProps } from "$fresh/server.ts";

export default function App({ Component }: AppProps) {
  return (
    <>
      <style jsx>{`
        .wrapper {
          background: rgba(0, 0, 0, .86);
          width: 100vw;
          height: 100vh;
          color: #e9e9e9;
          padding: .5rem;
        }
      `}</style>
      <div class="wrapper">
        <Component />
      </div>
    </>
  );
}
