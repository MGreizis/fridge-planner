<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  let appliances = [
    { name: "Oven", checked: false },
    { name: "Microwave", checked: false },
    { name: "Fridge", checked: false },
    { name: "Blender", checked: false },
  ];

  async function toggleAppliance(index: number) {
    appliances[index].checked = !appliances[index].checked;
  }

  async function savePreferences() {
    const selectedAppliances = appliances.filter((appliance) => appliance.checked).map((appliance) => appliance.name);
    console.log("Selected appliances:", selectedAppliances);

    try {
      await invoke("save_preferences", { appliances: JSON.stringify(selectedAppliances) });
    } catch (error) {
      console.error(error);
    }
  }
</script>

<main>
  <a href="/">
    <button class="border-1 border-black rounded-md hover:bg-green-600">Go to home page</button>
  </a>

  <form>
    <h2 class="text-2xl mb-4 text-red-600">Select your kitchen appliances</h2>
    <div class="grid grid-cols-2 gap-4 mx-4">
      {#each appliances as appliance, index (appliance.name)}
      <div
        class="bg-white p-4 border border-gray-200 rounded-md cursor-pointer hover:bg-blue-100"
        class:bg-blue-100={appliance.checked}
        on:click={() => toggleAppliance(index)}
      >
        <label for={appliance.name} class="text-lg font-semibold">
          <input type="checkbox" class="hidden" id={appliance.name} bind:checked={appliance.checked} />
          {appliance.name}
        </label>
      </div>
      {/each}
    </div>

    <button
      class="mt-6 bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md"
      on:click={savePreferences}
    >
      Save
    </button>
  </form>
</main>

<style>

</style>