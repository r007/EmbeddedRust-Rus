window.addEventListener('DOMContentLoaded', () => {
  const rightButtons = document.querySelector(".right-buttons");
  if (!rightButtons) return;

  const epubLink = document.createElement("a");
  // path_to_root задаёт mdbook для каждой страницы: ссылка работает и локально, и на GitHub Pages
  const root = typeof path_to_root === "string" ? path_to_root : "";
  epubLink.href = root + "%D0%A3%D1%87%D0%B5%D0%B1%D0%BD%D0%B8%D0%BA%20Discovery%20%D0%BF%D0%BE%20Rust%20%D0%BD%D0%B0%20BBC%20micro-bit%20v2.epub";
  epubLink.textContent = "📘ePUB";
  epubLink.download = "Учебник Discovery по Rust на BBC micro-bit v2.epub"; // force download
  epubLink.classList.add("epub-download");

  rightButtons.appendChild(epubLink);
});
