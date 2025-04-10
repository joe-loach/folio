const toast = (id) => {
  const toasts = document.getElementById("toasts");

  const child = toasts.querySelector(`#${id}`);
  child.classList.remove("hidden");

  setTimeout(() => child.classList.add("hidden"), 3000);
};
