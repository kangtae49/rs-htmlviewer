const { invoke: invoke_rs } = window.__TAURI__.core;
const { listen: listen_rs } = window.__TAURI__.event;

listen_rs("log", (event) => {
  console.log(event.payload);
});





