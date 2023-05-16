import { invoke } from '@tauri-apps/api/tauri';

let currentState: HTMLElement | null;

async function con() {
  try {
    await invoke('connect_to_wifi', { ssid: "WIFI" });
    if (currentState) {
      currentState.textContent = "Service is ON"
    }
  } catch (error) {
    alert(`Failed to connect to Wi-Fi network: ${error}`);
  }
}

async function dis() {
  try {
    await invoke('disconnect_from_wifi');
    if (currentState) {
      currentState.textContent = "Service is OFF"
    }
  } catch (error) {
    alert(`Failed to disconnect from Wi-Fi network: ${error}`);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  currentState = document.querySelector("#current-state");
  document.querySelector('#connect-button')?.addEventListener('click', async () => con());
  document.querySelector('#disconnect-button')?.addEventListener('click', async () => dis());
});
