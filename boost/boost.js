const API = "http://localhost:3000/chapter"
const AUTH = "Basic ******"
const METADATA_ATTR = "data-amplitude-params"
// https://www.wuxiaworld.com/novel/rmjiir/rmjiir-chapter-20
const TO = null // 'dr-chapter-293'

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
  contents.map(el => el.querySelector('button[type="button"]')?.remove())

  const content = contents.map(el => el.innerText).join('<p>')

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

async function onClick() {
  if (TO !== null) {
    await sleep(3)
  }
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
    author: metadata.novelWriter,
    translator: metadata.novelTranslator,
    content,
  }

  send(chapter)
    .then(green)
    .catch(red)
    .finally(async() => {
      await sleep(.5)
      base()
    })

  if (TO !== null) {
    const url = location.href.split('/')
    const chap = url.splice(-1)[0]
    const base = url.join('/')
    if (chap !== TO) {
      const nb = parseInt(chap.replace(/\D/g, ''))
      location.href = `${base}/${chap.replace(/\d+/g, nb + 1)}`
    }
  }
}

function send(chapter) {
  return new Promise(async (resolve, reject) => {
    try {
      const response = await fetch(API, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": AUTH,
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
  if (TO !== null) {
    onClick()
  }
}

document.addEventListener("DOMContentLoaded", loaded)
