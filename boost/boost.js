const API = "http://localhost:3000/chapter"
const METADATA_ATTR = "data-amplitude-params"

const btn = document.createElement("div")
btn.classList.add("wuxia2kindle-btn")
btn.addEventListener('click', onClick)

function safeParse(obj) {
  try {
    return JSON.parse(obj)
  } catch (e) {
    console.warn(e)
    return null
  }
}

function sleep(t) {
  return new Promise(resolve => setTimeout(resolve, t * 1000))
}

function getContent() {
  const contents = Array.from(document.querySelectorAll('.chapter-content p'))
  const content = contents.map(el => el.innerText).join('\n\n')

  return content
}

function green() {
  base()
  btn.classList.add("green")
}

function red(err) {
  base()
  btn.classList.add("red")
  console.warn(err)
}

function base() {
  btn.classList.remove("green")
  btn.classList.remove("red")
}

function onClick() {
  const dataContainer = document.querySelector(`[${METADATA_ATTR}]`)
  const metadata = safeParse(dataContainer.getAttribute(METADATA_ATTR))
  if (!safeParse) {
    return
  }

  const content = getContent();

  const chapter = {
    book: metadata.novelName,
    name: metadata.chapterTitle,
    number_in_book: metadata.chapterNo,
    content,
  }

  send(chapter)
    .then(green)
    .catch(red)
    .finally(async () => {
      await sleep(0.5)
      base()
    })
}

function send(chapter) {
  return new Promise(async (resolve, reject) => {
    try {
      const response = await fetch(API, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(chapter),
      });

      if (response.status >= 300) {
        reject("failed to send")
      } else {
        resolve()
      }
    } catch (e) {
      reject(e);
    }
  })
}

function loaded() {
  document.body.appendChild(btn)
}

document.addEventListener("DOMContentLoaded", loaded)
