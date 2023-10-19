<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let dietaryPreferences = [
    { name: 'Vegetarian', selected: false },
    { name: 'Vegan', selected: false },
    { name: 'Gluten-Free', selected: false },
    { name: 'Dairy-Free', selected: false },
    { name: 'Keto', selected: false },
  ];

  async function togglePreference(index: number) {
    dietaryPreferences[index].selected = !dietaryPreferences[index].selected;
  }

  async function savePreferences() {
    const selectedPreferences = dietaryPreferences.map((preference) => ({
      name: preference.name,
      selected: preference.selected,
    }));
    console.log("Selected preferences:", selectedPreferences);

    try {
      await invoke("save_dietary_preferences", { preferences: JSON.stringify(selectedPreferences) });
    } catch (error) {
      console.error(error);
    }
  }
</script>

<main>
  <a href="/">
    <button class="border-1 border-black rounded-md hover:bg-slate-500">Go to home page</button>
  </a>

  <form>
    <h2 class="text-2xl mb-4 text-green-600">Select Your Dietary Preferences</h2>
    <div class="grid grid-cols-2 gap-4 mx-4">
      {#each dietaryPreferences as preference, index (preference.name)}
      <div
        class="bg-white p-4 border-2 border-black rounded-md cursor-pointer hover:bg-blue-100"
        class:bg-blue-100={preference.selected}
        on:click={() => togglePreference(index)}
      >
        <label for={preference.name} class="text-lg font-semibold">
          <input type="checkbox" class=hidden id={preference.name} bind:checked={preference.selected} />
          {preference.name}
        </label>
      </div>
      {/each}
    </div>

    <button
      class="mt-6 bg-blue-500 hover-bg-blue-600 text-white py-2 px-4 rounded-md"
      on:click={savePreferences}
    >
      Save
    </button>
  </form>
</main>