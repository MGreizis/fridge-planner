<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";

  let appliances = [
    { name: "Oven", checked: false },
    { name: "Microwave", checked: false },
    { name: "Fridge", checked: false },
    { name: "Blender", checked: false },
  ];

  async function toggleAppliance(index: number) {
    appliances[index].checked = !appliances[index].checked;
  }

  async function saveAppliances() {
    const selectedAppliances = appliances.map((appliance) => ({
      name: appliance.name,
      checked: appliance.checked,
    }));
    console.log("Selected appliances:", selectedAppliances);

    try {
      await invoke("save_appliances", {
        appliances: JSON.stringify(selectedAppliances),
      });

      goto("/preferences");
    } catch (error) {
      console.error(error);
    }
  }
</script>

<main>
  <a href="/">
    <button class="border-1 border-black rounded-md hover:bg-slate-500"
      >Go to home page</button
    >
  </a>

  <form>
    <h2 class="text-2xl mb-4 text-red-600">Select your kitchen appliances</h2>
    <section class="grid grid-cols-2 gap-4 mx-4">
      {#each appliances as appliance, index (appliance.name)}
        <div
          class="bg-white p-4 border-2 border-black rounded-md cursor-pointer hover:bg-blue-100"
          class:bg-blue-100={appliance.checked}
          on:click={() => toggleAppliance(index)}
          on:keyup={(e) => {
            if (e.key === "Enter") {
              toggleAppliance(index);
            }
          }}
          role="button"
          tabindex="0"
        >
          <label for={appliance.name} class="text-lg font-semibold">
            <input
              type="checkbox"
              class="hidden"
              id={appliance.name}
              bind:checked={appliance.checked}
            />
            {appliance.name}
          </label>
        </div>
      {/each}
    </section>

    <button
      class="mt-6 bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md"
      on:click={saveAppliances}
    >
      Save
    </button>
  </form>
</main>

<style>
</style>
