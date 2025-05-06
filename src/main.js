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
        event.stopPropagation();
        if (file.name.endsWith(".html")) {
          const viewer = document.querySelector("#viewer");
          viewer.innerHTML = await invoke_rs("read_html_file", { path: file.path });
        }
      };
    }

    fileTree.appendChild(li);
  });
}



