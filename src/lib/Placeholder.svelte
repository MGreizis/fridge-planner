<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let text = ""
  let finalText = ""

  // invoke("placeholder", { text: "funny message"})
  async function placeholder() {
    finalText = await invoke("placeholder", { text })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={placeholder}>
    <input id="placeholder-input" placeholder="Enter a placeholder..." bind:value={text} />
    <button type="submit">Placeholder</button>
  </form>
  <p>{finalText}</p>
</div>