<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  async function openNewWindow() {
    try {
      // Call the create_window function using the Tauri invoke method
      await invoke("create_window");
    } catch (error) {
      console.error(error);
    }
  }
</script>

<div>
  <button on:click={openNewWindow}>Create Window</button>
</div>