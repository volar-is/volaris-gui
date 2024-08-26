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
  let selectedHashAlgorithm = "Blake3Balloon";
  let selectedEncryptionAlgorithm = "XChaCha20Poly1305";
  let isDropdownOpen = null;
  let selectedHeaderVersion = "V5";

  listen("tauri://drag-drop", (event) => {
    const files = event.payload.paths;
    if (files.length > 0) {
      if (selectedTab === "file") {
        selectedFile = files[0];
      } else {
        keyFile = files[0];
      }
    }
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

  async function pickFile(option) {
    const file = await open({ multiple: false });
    if (option == "file") {
      selectedFile = file && typeof file === "string" ? file : file?.path || "";
    } else {
      keyFile = file && typeof file === "string" ? file : file?.path || "";
    }
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
          hash: selectedHashAlgorithm,
          header: selectedHeaderVersion,
        });
        if (password.endsWith(".key")) {
          keyFile = newKeyFilePath + "/" + password;
        } else {
          keyFile = newKeyFilePath + "/" + password + ".key";
        }

        alert("Key file created successfully." + "\n" + keyFile);
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
            hash: selectedHashAlgorithm,
            ealgorithm: selectedEncryptionAlgorithm,
            headerver: selectedHeaderVersion,
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
    if (selectedFile && keyFile !== undefined) {
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
    } else if (
      (keyFile === "" || keyFile === undefined) &&
      selectedFile === ""
    ) {
      alert("Please select a file and keyfile.");
    } else if (keyFile === "" || keyFile === undefined) {
      alert("Please select or create a keyfile first.");
    } else if (selectedFile == "") {
      alert("Please select a file first.");
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

  function toggleDropdown(type) {
    isDropdownOpen = isDropdownOpen === type ? null : type;
  }

  function selectOption(option, type) {
    if (type === "hashing") {
      selectedHashAlgorithm = option;
    } else if (type === "encryption") {
      selectedEncryptionAlgorithm = option;
    } else if (type === "header") {
      selectedHeaderVersion = option;
    }
    isDropdownOpen = null;
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
    <div class="flex items-center justify-between px-4 py-2 md:px-8 md:py-4">
      <h2 class="text-2xl font-bold text-pink-400 md:text-3xl">Volaris</h2>
      <div class="flex items-center space-x-2 md:space-x-4">
        <button
          on:click={() => (selectedTab = "file")}
          class={`px-4 py-2 md:px-6 md:py-3 text-sm md:text-lg font-semibold flex items-center justify-center rounded-full ${selectedTab === "file" ? "bg-purple-600 text-white" : "bg-gray-800 text-gray-300 hover:bg-gray-600"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined text-base md:text-xl"
            >save</span
          >
        </button>
        <button
          on:click={() => (selectedTab = "key")}
          class={`px-4 py-2 md:px-6 md:py-3 text-sm md:text-lg font-semibold flex items-center justify-center rounded-full ${selectedTab === "key" ? "bg-pink-500 text-white" : "bg-gray-800 text-gray-300 hover:bg-gray-600"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined text-base md:text-xl">key</span
          >
        </button>
        <button
          on:click={() => (selectedTab = "advanced")}
          class={`px-4 py-2 md:px-6 md:py-3 text-sm md:text-lg font-semibold flex items-center justify-center rounded-full ${selectedTab === "advanced" ? "bg-teal-500 text-white" : "bg-gray-800 text-gray-300 hover:bg-gray-600"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined text-base md:text-xl"
            >settings</span
          >
        </button>
        <button
          on:click={toggleAction}
          class={`px-4 py-2 md:px-6 md:py-3 text-sm md:text-lg font-semibold flex items-center justify-center rounded-full ${actionTab === "encrypt" ? "bg-purple-600 text-white" : "bg-pink-500 text-white"} transition-colors duration-300`}
        >
          <span class="material-symbols-outlined text-base md:text-xl"
            >{actionTab === "encrypt" ? "lock" : "no_encryption"}</span
          >
        </button>
        <button
          on:click={performAction}
          class="px-4 py-2 md:px-6 md:py-3 text-sm md:text-lg font-semibold bg-pink-500 hover:bg-pink-400 text-white rounded-full flex items-center justify-center transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-pink-300"
        >
          <span class="material-symbols-outlined text-base md:text-xl"
            >play_arrow</span
          >
        </button>
      </div>
    </div>
  </nav>

  <div class="flex-1 p-4 md:p-6 overflow-y-auto">
    {#if selectedTab === "file"}
      <div class="flex flex-col items-center">
        <div
          on:drop={(event) => {
            event.preventDefault();
            // @ts-ignore
            selectedFile = event.dataTransfer?.files[0];
            isDragOver = false;
          }}
          on:dragover={(event) => {
            event.preventDefault();
            isDragOver = true;
          }}
          on:dragleave={() => (isDragOver = false)}
          class={`w-full max-w-7xl border-4 ${isDragOver ? "border-purple-500 bg-gray-700 shadow-lg scale-105" : "border-dashed border-gray-600 bg-gray-800"} rounded-lg p-8 md:p-12 text-center flex flex-col items-center justify-center relative transition-all duration-300 ease-in-out transform-gpu`}
          style="min-height: 300px;"
        >
          <p class="text-gray-300 text-base md:text-lg">
            Drag & drop a file here or
            <a
              on:click={() => pickFile("file")}
              class="text-purple-600 hover:text-purple-400 cursor-pointer"
            >
              Select a file
            </a>
          </p>
          {#if selectedFile}
            <div class="mt-4 text-xs md:text-sm text-gray-400">
              <p title={selectedFile}>Selected file:</p>
              <p title={selectedFile} class="text-gray-300">
                {selectedFile.split("/").pop()}
              </p>
              <span
                class="absolute left-1/2 transform -translate-x-1/2 top-full mt-2 bg-gray-700 text-gray-200 p-2 md:p-3 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-300"
              >
                {selectedFile}
              </span>
            </div>
          {/if}
        </div>

        <div
          class="mt-6 w-full max-w-7xl bg-gray-700 p-4 md:p-6 rounded-lg shadow-lg"
        >
          <h3 class="text-lg md:text-xl font-semibold text-gray-200 mb-2">
            How to Use Volaris
          </h3>
          <p class="text-gray-300 text-sm md:text-base mb-4">
            Follow these steps to encrypt or decrypt files using Volaris:
          </p>
          <ol
            class="list-decimal list-inside text-gray-300 text-sm md:text-base space-y-2"
          >
            <li>
              Select the <span class="font-semibold">File</span> tab and choose the
              file you want to encrypt or decrypt by dragging and dropping it into
              the designated area or by clicking on the "Select a file" link.
            </li>
            <li>
              Go to the <span class="font-semibold">Key File</span> tab to either
              create a new key file or select an existing one. A key file is essential
              for both encryption and decryption.
            </li>
            <li>
              Toggle the action tab to choose between <span
                class="font-semibold">Encrypt</span
              >
              or <span class="font-semibold">Decrypt</span> depending on what you
              want to do with the selected file.
            </li>
            <li>
              Click the <span class="font-semibold">Run</span> button to initiate
              the encryption or decryption process. You will be prompted for confirmation
              before the action is performed.
            </li>
            <li>
              Once the process is complete, you will receive a notification
              indicating whether it was successful.
            </li>
          </ol>
        </div>
      </div>
    {/if}

    {#if selectedTab === "key"}
      <div class="flex flex-col items-center space-y-4 md:space-y-6">
        <div
          class="w-full max-w-7xl bg-gray-700 p-4 md:p-6 rounded-lg shadow-lg"
        >
          <h2
            class="text-center text-xl md:text-2xl font-semibold text-gray-100 mb-4"
          >
            Key File Management
          </h2>
          <div class="flex flex-col gap-4">
            <button
              on:click={saveKeyFile}
              class="px-4 py-2 md:px-6 md:py-3 text-base md:text-lg font-semibold bg-purple-600 hover:bg-purple-500 text-white rounded-full flex items-center justify-center transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-purple-300"
              style="height: 48px;"
            >
              <span class="material-symbols-outlined text-base md:text-lg"
                >create</span
              >
              <span class="md:inline md:ml-3">Create New Key File</span>
            </button>
            <div class="relative text-center mt-4">
              <div class="flex items-center">
                <div
                  class="flex-1 h-0.5 bg-gradient-to-r from-gray-700 to-gray-500"
                ></div>
                <span class="px-4 text-gray-300 text-1xl font-semibold">OR</span
                >
                <div
                  class="flex-1 h-0.5 bg-gradient-to-l from-gray-700 to-gray-500"
                ></div>
              </div>
            </div>

            <div
              id="keyDrag"
              on:drop={(event) => {
                event.preventDefault();
                // @ts-ignore
                keyFile = event.dataTransfer?.files[0];
                isDragOver = false;
              }}
              on:dragover={(event) => {
                event.preventDefault();
                isDragOver = true;
              }}
              on:dragleave={() => (isDragOver = false)}
              on:click={() => pickFile("key")}
              class={`w-full max-w-7xl border-4 ${
                isDragOver
                  ? "border-purple-500 bg-gray-700 shadow-lg scale-105"
                  : "border-dashed border-gray-600 bg-gray-800"
              } rounded-lg p-8 md:p-12 text-center flex flex-col items-center justify-center relative transition-all duration-300 ease-in-out transform-gpu 
  hover:border-purple-500 hover:bg-gray-700 hover:shadow-lg hover:scale-105 cursor-pointer`}
              style="min-height: 300px;"
            >
              <p class="text-gray-300 text-base md:text-lg">
                Drag & drop your key file here
              </p>
              {#if keyFile}
                <div class="mt-4 text-xs md:text-sm text-gray-400">
                  <p title={keyFile}>Selected file:</p>
                  <p title={keyFile} class="text-gray-300">
                    {keyFile.split("/").pop()}
                  </p>
                  <span
                    class="absolute left-1/2 transform -translate-x-1/2 top-full mt-2 bg-gray-700 text-gray-200 p-2 md:p-3 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                  >
                    {keyFile}
                  </span>
                </div>
              {/if}
            </div>
          </div>
        </div>
      </div>
    {/if}

    {#if selectedTab === "advanced"}
      <div
        class="flex flex-col items-center justify-center flex-grow w-full max-w-3xl space-y-4 text-center"
      >
        <h2 class="text-3xl font-semibold text-gray-100 md:text-4xl">
          Advanced Options
        </h2>

        <div class="flex flex-col w-full max-w-3xl space-y-4 text-center">
          <label class="text-lg font-semibold text-gray-200"
            >Hashing Algorithm:</label
          >
          <div class="relative inline-block w-full">
            <button
              on:click={() => toggleDropdown("hashing")}
              class="flex items-center justify-between w-full px-4 py-2 text-lg bg-gray-800 text-white rounded-lg border-2 border-gray-500 focus:outline-none focus:ring-2 focus:ring-pink-500 transition-all duration-300"
            >
              {selectedHashAlgorithm || "Select Hashing Algorithm"}
              <span class="material-symbols-outlined text-base md:text-lg"
                >arrow_drop_down</span
              >
            </button>
            {#if isDropdownOpen === "hashing"}
              <div
                class="absolute left-0 z-10 mt-2 w-full bg-gray-800 text-white border border-gray-500 rounded-lg shadow-lg"
              >
                <ul class="py-1">
                  <li
                    on:click={() => selectOption("Blake3Balloon", "hashing")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    Blake3Balloon
                  </li>
                  <li
                    on:click={() => selectOption("Argon2ID", "hashing")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    Argon2ID
                  </li>
                </ul>
              </div>
            {/if}
          </div>
        </div>

        <!-- Encryption Algorithm Dropdown -->
        <div class="flex flex-col w-full max-w-3xl space-y-4 text-center">
          <label class="text-lg font-semibold text-gray-200"
            >Encryption Algorithm:</label
          >
          <div class="relative inline-block w-full">
            <button
              on:click={() => toggleDropdown("encryption")}
              class="flex items-center justify-between w-full px-4 py-2 text-lg bg-gray-800 text-white rounded-lg border-2 border-gray-500 focus:outline-none focus:ring-2 focus:ring-pink-500 transition-all duration-300"
            >
              {selectedEncryptionAlgorithm || "Select Encryption Algorithm"}
              <span class="material-symbols-outlined text-base md:text-lg"
                >arrow_drop_down</span
              >
            </button>
            {#if isDropdownOpen === "encryption"}
              <div
                class="absolute left-0 z-10 mt-2 w-full bg-gray-800 text-white border border-gray-500 rounded-lg shadow-lg"
              >
                <ul class="py-1">
                  <li
                    on:click={() => selectOption("AES-256-GCM", "encryption")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    AES-256-GCM
                  </li>
                  <li
                    on:click={() =>
                      selectOption("XChaCha20Poly1305", "encryption")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    XChaCha20Poly1305
                  </li>
                </ul>
              </div>
            {/if}
          </div>
        </div>

        <!-- Header Version Dropdown -->
        <div class="flex flex-col w-full max-w-3xl space-y-4 text-center">
          <label class="text-lg font-semibold text-gray-200"
            >Header Version:</label
          >
          <div class="relative inline-block w-full">
            <button
              on:click={() => toggleDropdown("header")}
              class="flex items-center justify-between w-full px-4 py-2 text-lg bg-gray-800 text-white rounded-lg border-2 border-gray-500 focus:outline-none focus:ring-2 focus:ring-pink-500 transition-all duration-300"
            >
              {selectedHeaderVersion || "Select Header Version"}
              <span class="material-symbols-outlined text-base md:text-lg"
                >arrow_drop_down</span
              >
            </button>
            {#if isDropdownOpen === "header"}
              <div
                class="absolute left-0 z-10 mt-2 w-full bg-gray-800 text-white border border-gray-500 rounded-lg shadow-lg"
              >
                <ul class="py-1">
                  <li
                    on:click={() => selectOption("v3", "header")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    v3
                  </li>
                  <li
                    on:click={() => selectOption("v4", "header")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    v4
                  </li>
                  <li
                    on:click={() => selectOption("v5", "header")}
                    class="px-4 py-2 hover:bg-gray-700 cursor-pointer"
                  >
                    v5
                  </li>
                </ul>
              </div>
            {/if}
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
