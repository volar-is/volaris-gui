<script>
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import PasswordPrompt from "./PasswordPrompt.svelte";
  import { ask } from "@tauri-apps/plugin-dialog";
  import "./app.css";

  let selectedFile = "";
  let keyFile = "";
  let showPasswordDialog = false;
  let password = "";
  let selectedTab = "file";
  let actionTab = "encrypt";
  let newKeyFilePath = "";
  let isDragOver = false;

  listen("tauri://drag-drop", (event) => {
    const files = event.payload.paths;
    if (files.length > 0) {
      selectedFile = files[0];
    }
  });

  listen("tauri://drag-over", () => {
    isDragOver = true;
  });

  listen("tauri://drag-leave", () => {
    isDragOver = false;
  });

  function showPasswordDialogHandler() {
    showPasswordDialog = true;
  }

  function handlePasswordSubmit(event) {
    password = event.detail.password;
    showPasswordDialog = false;
    createKeyFile();
  }

  function handlePasswordCancel() {
    showPasswordDialog = false;
  }

  async function pickFile() {
    const file = await open({ multiple: false });
    selectedFile = file && typeof file === "string" ? file : file?.path || "";
  }

  async function pickKeyFile() {
    const file = await open({ multiple: false });
    keyFile = file && typeof file === "string" ? file : file?.path || "";
  }

  async function saveKeyFile() {
    const file = await open({ directory: true });
    newKeyFilePath = file && typeof file === "string" ? file : file || "";
    if (newKeyFilePath) {
      showPasswordDialogHandler();
    }
  }

  async function createKeyFile() {
    if (password && newKeyFilePath) {
      try {
        const result = await invoke("create_key_file", {
          name: password,
          path: newKeyFilePath,
        });
        keyFile = result.keyfilePath;
        alert("Key file created successfully.");
      } catch (error) {
        alert("Error creating key file: " + error);
      }
    } else if (!password) {
      alert("File name is required to create a key file.");
    } else if (!newKeyFilePath) {
      alert("File path is required to create a key file.");
    }
  }

  async function encryptFile() {
    if (selectedFile && keyFile) {
      try {
        const confirmation = await ask(
          "Are you sure you want to encrypt this file?",
          { title: "Confirmation", kind: "warning" },
        );

        if (confirmation) {
          const result = await invoke("encrypt_file_with_key", {
            input: selectedFile,
            output: `${selectedFile}.enc`,
            keyfile: keyFile,
          });
          alert("File encrypted successfully: " + result);
        } else {
          alert("Encryption canceled.");
        }
      } catch (error) {
        alert("Error during encryption: " + error);
      }
    } else if (keyFile == "" && selectedFile == "") {
      alert("Please select a file and keyfile.");
    } else if (keyFile == "") {
      alert("Please select or create a keyfile first.");
    } else if (selectedFile == "") {
      alert("Please select a file first.");
    }
  }

  async function decryptFile() {
    if (selectedFile && keyFile) {
      try {
        const confirmation = await ask(
          "Are you sure you want to decrypt this file?",
          { title: "Confirmation", kind: "warning" },
        );

        if (confirmation) {
          const result = await invoke("decrypt_file_with_key", {
            input: selectedFile,
            output: selectedFile.replace(".enc", ""),
            keyfile: keyFile,
          });
          alert("File decrypted successfully: " + result);
        } else {
          alert("Decryption canceled.");
        }
      } catch (error) {
        alert("Error during decryption: " + error);
      }
    } else {
      alert("Please select a file and a keyfile first.");
    }
  }

  function toggleAction() {
    actionTab = actionTab === "encrypt" ? "decrypt" : "encrypt";
  }

  function performAction() {
    if (actionTab === "encrypt") {
      encryptFile();
    } else {
      decryptFile();
    }
  }
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
  />
</svelte:head>

<main class="flex flex-col h-screen bg-gray-800 text-gray-100">
  <nav class="w-full bg-gray-700 shadow-md">
    <div class="flex items-center justify-between px-8 py-4">
      <h2 class="text-3xl font-bold text-pink-400">Volaris</h2>
      <div class="flex items-center space-x-4">
        <button
          on:click={() => (selectedTab = "file")}
          class={`px-6 py-2 rounded-lg text-lg font-semibold flex items-center space-x-3 ${selectedTab === "file" ? "bg-purple-600 text-white" : "bg-gray-800 text-gray-300 hover:bg-gray-600"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined w-6 h-6">save</span>
          <span>File</span>
        </button>
        <button
          on:click={() => (selectedTab = "key")}
          class={`px-6 py-2 rounded-lg text-lg font-semibold flex items-center space-x-3 ${selectedTab === "key" ? "bg-pink-500 text-white" : "bg-gray-800 text-gray-300 hover:bg-gray-600"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined w-6 h-6">key</span>
          <span>Key File</span>
        </button>
        <button
          on:click={toggleAction}
          class={`px-6 py-2 rounded-lg text-lg font-semibold flex items-center space-x-3 ${actionTab === "encrypt" ? "bg-purple-600 text-white" : "bg-pink-500 text-white"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined w-6 h-6"
            >{actionTab === "encrypt" ? "lock" : "no_encryption"}</span
          >
          <span>{actionTab === "encrypt" ? "Encrypt" : "Decrypt"}</span>
        </button>
        <button
          on:click={performAction}
          class="px-6 py-2 font-semibold bg-pink-500 hover:bg-pink-400 text-white text-lg rounded-lg flex items-center space-x-3 transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-pink-300"
        >
          <span class="material-symbols-outlined w-6 h-6">play_arrow</span>
          <span>Run</span>
        </button>
      </div>
    </div>
  </nav>

  <div class="flex-1 p-6 overflow-y-auto">
    {#if selectedTab === "file"}
      <div class="flex flex-col items-center">
        <div
          on:drop={(event) => {
            event.preventDefault();
            selectedFile = event.dataTransfer.files[0].path;
            isDragOver = false;
          }}
          on:dragover={(event) => {
            event.preventDefault();
            isDragOver = true;
          }}
          on:dragleave={() => (isDragOver = false)}
          class={`w-full max-w-lg border-4 ${isDragOver ? "border-purple-600 bg-gray-700" : "border-dashed border-gray-600 bg-gray-800"} rounded-lg p-12 text-center flex items-center justify-center relative transition-colors duration-300`}
          style="min-height: 400px;"
        >
          <p class="text-gray-300 text-lg">
            Drag & drop a file here or
            <a
              on:click={pickFile}
              class="text-purple-600 hover:text-purple-400 text-lg font-medium cursor-pointer transition-colors duration-200"
            >
              Select a file
            </a>
          </p>
          {#if selectedFile}
            <p class="mt-4 text-sm text-gray-400">
              Selected file:
              <span class="text-gray-300">{selectedFile.split("/").pop()}</span>
              <span
                class="absolute left-1/2 transform -translate-x-1/2 top-full mt-2 bg-gray-700 text-gray-200 p-3 rounded-lg opacity-0 hover:opacity-100 transition-opacity duration-300"
              >
                {selectedFile}
              </span>
            </p>
          {/if}
        </div>

        <div class="mt-8 w-full max-w-lg bg-gray-700 p-6 rounded-lg shadow-lg">
          <h3 class="text-xl font-semibold text-gray-200 mb-2">
            About Volaris
          </h3>
          <p class="text-gray-300">
            Volaris is an encryption tool designed to prioritize privacy and
            security. Built using Rust, it offers a modern and efficient
            solution for securing your data across multiple platforms, including
            desktops, command-line interfaces (CLI), and mobile devices. You can
            find documentation <a
              class="text-pink-500"
              href="https://docs.volaris.leohanney.com">here</a
            > if you'd like to learn more.
          </p>
        </div>
      </div>
    {/if}

    {#if selectedTab === "key"}
      <div class="flex flex-col items-center space-y-6">
        <div class="w-full max-w-lg bg-gray-700 p-8 rounded-lg shadow-lg">
          <h2 class="text-2xl font-semibold text-gray-100 mb-4">
            Key File Management
          </h2>
          <div class="flex flex-col gap-4">
            <button
              on:click={saveKeyFile}
              class="px-6 py-3 font-semibold bg-purple-600 hover:bg-purple-500 text-white text-lg rounded-lg transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-purple-300"
              style="height: 56px;"
            >
              Create New Key File
            </button>
            <button
              on:click={pickKeyFile}
              class="px-6 py-3 font-semibold bg-pink-500 hover:bg-pink-400 text-white text-lg rounded-lg transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-pink-300"
              style="height: 56px;"
            >
              Select Existing Key File
            </button>
          </div>
        </div>
      </div>
    {/if}
  </div>

  {#if showPasswordDialog}
    <PasswordPrompt
      on:submit={handlePasswordSubmit}
      on:cancel={handlePasswordCancel}
    />
  {/if}
</main>
