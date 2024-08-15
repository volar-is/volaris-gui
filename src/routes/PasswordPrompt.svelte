<script>
  import { createEventDispatcher } from "svelte";
  import "./app.css";

  let password = "";
  const dispatch = createEventDispatcher();

  function submit() {
    dispatch("submit", { password });
  }

  function cancel() {
    dispatch("cancel");
  }
</script>

<div
  class="fixed inset-0 bg-gray-900 bg-opacity-70 flex justify-center items-center z-50"
>
  <div class="bg-gray-800 rounded-lg p-6 w-full max-w-sm relative shadow-lg">
    <button
      on:click={cancel}
      class="absolute top-2 right-2 text-gray-400 hover:text-gray-300 transition"
    >
      &times;
    </button>
    <h3 class="text-2xl font-semibold text-pink-400 mb-4">Enter Filename</h3>
    <input
      bind:value={password}
      placeholder="Filename"
      class="w-full px-4 py-2 mb-4 border border-gray-600 rounded-lg bg-gray-700 text-gray-100 placeholder-gray-400 focus:border-pink-500 focus:ring focus:ring-pink-300"
    />
    <div class="flex justify-end space-x-4">
      <button
        on:click={submit}
        class="bg-purple-600 text-white py-2 px-4 rounded-lg shadow-md hover:bg-purple-500 transition duration-200"
      >
        Submit
      </button>
      <button
        on:click={cancel}
        class="bg-gray-600 text-white py-2 px-4 rounded-lg shadow-md hover:bg-gray-500 transition duration-200"
      >
        Cancel
      </button>
    </div>
  </div>
</div>
