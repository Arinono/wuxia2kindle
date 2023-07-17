import { Head } from "$fresh/runtime.ts";
import { useSignal } from "@preact/signals";

export default function Home() {
  return (
    <>
      <Head>
        <title>Fresh App</title>
      </Head>
      <style jsx>{`
        body {
          background: rgba(0, 0, 0, .86);
        }
      `}</style>
      <div>
      </div>
    </>
  );
}
