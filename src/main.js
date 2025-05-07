const { invoke: invoke_rs } = window.__TAURI__.core;
const { listen: listen_rs } = window.__TAURI__.event;

window.addEventListener("DOMContentLoaded", async () => {
  Split(['.left-pane', '.right-pane'], {
    gutterSize: 4,
    sizes: [20, 80],
  });

  await loadTree();
});


listen_rs("log", (event) => {
  console.log(event.payload);
});


document.querySelector("#root-folder").addEventListener("click", async (e) => {
  e.preventDefault();
  await loadTree();
});

async function loadTree() {
  let fileTree = document.querySelector("#file-tree");
  fileTree.innerHTML = "";
  const folderName = await invoke_rs("get_root_path");

  await loadFiles(folderName);
}

async function loadFiles(path = ".", parentElement = null) {
  document.querySelector("#root-folder-name").textContent = await invoke_rs("get_root_path_name");

  const files = await invoke_rs("list_directory", { path });

  let fileTree;
  if (parentElement) {
    const icon = parentElement.querySelector('i');

    if (parentElement.dataset.expanded === "true") {
      parentElement.dataset.expanded = "false";
      parentElement.removeChild(parentElement.querySelector("ul"));

      icon.classList.remove('fa-folder-open');
      icon.classList.add('fa-folder');
      return;
    } else {
      parentElement.dataset.expanded = "true";
      fileTree = document.createElement("ul");
      parentElement.appendChild(fileTree);

      icon.classList.remove('fa-folder');
      icon.classList.add('fa-folder-open');

    }
  } else {
    fileTree = document.querySelector("#file-tree");
    fileTree.innerHTML = "";
  }

  files.forEach(file => {
    const li = document.createElement("li");
    li.style.cursor = "pointer";

    if (file.is_dir) {
      li.dataset.isFolder = "true";
      li.dataset.expanded = "false";
      li.innerHTML = `<i class="fa-solid fa-folder"></i> <span>${file.name}</span>`;
      li.onclick = (event) => {
        event.stopPropagation();
        loadFiles(file.path, li);
      };
    } else {
      li.innerHTML = `<i class="fa-solid fa-file-code"></i> <span>${file.name}</span>`;
      li.onclick = async (event) => {
        //try {
          event.stopPropagation();
          event.preventDefault();
          if (file.name.toUpperCase().endsWith(".HTML")) {
            // const iframe = document.querySelector('#iframe-viewer');
            // iframe.src = file.path;
            //const viewer = document.querySelector("#viewer");
            //viewer.innerHTML = await invoke_rs("read_html_file", { path: file.path });

            const iframe = document.querySelector('#iframe-viewer');
            iframe.src = "about:blank";
            const doc = iframe.contentWindow.document;
            doc.open();
            doc.write(await invoke_rs("read_html_file", {path: file.path}));
            doc.close();

            const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;
            // iframeDoc.addEventListener('click', (event) => {
            //   event.preventDefault();
            //   event.stopPropagation();
            //   console.log(iframe.src);
            // });
            /*
            const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;
            iframeDoc.addEventListener('click', (event) => {
              event.preventDefault();
              if (event.target.tagName === 'A') {
                window.open(event.target.href );
              } else if (event.target?.src?.startsWith("http")) {
                window.open(event.target.src);
              }
            });
            */
          }
        // }catch(e) {
        //   const iframe = document.querySelector('#iframe-viewer');
        //   iframe.src = "about:blank";
        //   const doc = iframe.contentWindow.document;
        //   doc.open();
        //   doc.write("");
        //   doc.close();
        //   console.log(e);
        // }
      };
    }

    fileTree.appendChild(li);
  });
}



