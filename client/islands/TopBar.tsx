export default function TopBar() {
  function goToHome() {
    const url = new URL(location.href);
    location.href = url.origin;
  }

  return (
    <>
      <div class='h-12 sticky shadow z-10 top bg-gray-800 flex items-center'>
        <div
          class='text-lg ml-4 cursor-pointer'
          onClick={goToHome}
        >
          <strong>Home</strong>
        </div>
      </div>
    </>
  );
}
