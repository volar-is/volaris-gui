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
  let step = 1;
  let actionTab = "encrypt";
  let newKeyFilePath = "";
  let isDragOver = false;
  let selectedHashAlgorithm = "Blake3Balloon";
  let selectedEncryptionAlgorithm = "XChaCha20Poly1305";
  let isDropdownOpen = null;
  let selectedHeaderVersion = "V5";
  let isProcessing = false;
  let startTime = 0;
  let elapsedTime = 0;
  let progress = 0;
  let interval = 0;

  function nextStep() {
    if (step < 5) step++;
  }

  function previousStep() {
    if (step > 1) step--;
  }

  listen("tauri://drag-drop", (event) => {
    const files = event.payload.paths;
    if (files.length > 0) {
      if (step === 2) {
        selectedFile = files[0];
        delay(500);
        step = 3;
      } else if (step === 3) {
        keyFile = files[0];
        
        step = 4;
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
    if (option === "file") {
      selectedFile = file && typeof file === "string" ? file : file?.path || "";
      delay(500);
      step = 3;
    } else if (option === "key") {
      keyFile = file && typeof file === "string" ? file : file?.path || "";
      delay(500);
      step = 4;
    }
  }

  async function saveKeyFile() {
    const file = await save({ directory: true });
    newKeyFilePath = file && typeof file === "string" ? file : file || "";
    if (newKeyFilePath) {
      showPasswordDialogHandler();
    }
  }

  async function delay(milliseconds) {
    return new Promise((resolve) => {
      setTimeout(resolve, milliseconds);
    });
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
          delay(1000);
          isProcessing = true;
          startTime = Date.now();
          progress = 0;
          const interval = setInterval(() => {
            elapsedTime = (Date.now() - startTime) / 1000;
            if (progress < 84) {
              progress += 2;
            }
          }, 500);

          const result = await invoke("encrypt_file_with_key", {
            input: selectedFile,
            output: `${selectedFile}.enc`,
            keyfile: keyFile,
            hash: selectedHashAlgorithm,
            ealgorithm: selectedEncryptionAlgorithm,
            headerver: selectedHeaderVersion,
          });

          clearInterval(interval);
          progress = 100;
          await delay(1000);
          elapsedTime = (Date.now() - startTime) / 1000;
          isProcessing = false;
          alert("File encrypted successfully: " + result);
        } else {
          alert("Encryption canceled.");
        }
      } catch (error) {
        clearInterval(interval);
        isProcessing = false;
        alert("Error during encryption: " + error);
      }
    } else if (!selectedFile) {
      alert("Please select a file first.");
    } else if (!keyFile) {
      alert("Please select or create a keyfile first.");
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
          // for now, this doesnt detect rust asking for prompt, so starts processing straight after, same with encrypt, so i just set a second long delay instead.
          delay(1000);
          isProcessing = true;
          startTime = Date.now();
          progress = 0;
          const interval = setInterval(() => {
            elapsedTime = (Date.now() - startTime) / 1000;
            if (progress < 84) {
              progress += 2;
            }
          }, 500);

          const result = await invoke("decrypt_file_with_key", {
            input: selectedFile,
            output: selectedFile.replace(".enc", ""),
            keyfile: keyFile,
          });

          clearInterval(interval);
          progress = 100;
          await delay(1000);
          elapsedTime = (Date.now() - startTime) / 1000;
          isProcessing = false;
          alert("File decrypted successfully: " + result);
        } else {
          alert("Decryption canceled.");
        }
      } catch (error) {
        clearInterval(interval);
        isProcessing = false;
        alert("Error during decryption: " + error);
      }
    } else if (!selectedFile) {
      alert("Please select a file first.");
    } else if (!keyFile) {
      alert("Please select or create a keyfile first.");
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

  function processFile() {
    performAction();
  }
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"
  />
</svelte:head>

<main class="flex flex-col min-h-screen bg-gray-900 text-gray-200">
  <div class="p-4 md:p-6">
    <div class="flex justify-between mb-4">
      <button
        on:click={previousStep}
        class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-500 disabled:opacity-50"
        disabled={step === 1}
      >
        Previous
      </button>
      <span class="text-lg font-semibold">Step {step} of 5</span>
      <button
        on:click={nextStep}
        class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-500 disabled:opacity-50"
        disabled={step === 1 || step === 5}
      >
        Next
      </button>
    </div>
  </div>

  <div class="flex-grow flex items-center justify-center">
    <div class="w-full max-w-lg bg-gray-700 rounded-lg shadow-lg p-6 md:p-8">
      {#if step === 1}
        <div class="flex flex-col items-center">
          <h2
            class="text-center text-xl md:text-2xl font-semibold text-gray-100 mb-4"
          >
            What would you like to do?
          </h2>
          <div class="flex flex-col space-y-4">
            <button
              on:click={() => {
                actionTab = "encrypt";
                nextStep();
              }}
              class="px-6 py-3 text-lg font-semibold bg-pink-500 text-white rounded-full flex items-center justify-center transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-green-300"
            >
              <span class="material-symbols-outlined text-lg">lock_open</span>
              <span class="ml-3">Encrypt a File</span>
            </button>
            <button
              on:click={() => {
                actionTab = "decrypt";
                nextStep();
              }}
              class="px-6 py-3 text-lg font-semibold bg-purple-600 text-white text-white rounded-full flex items-center justify-center transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-red-300"
            >
              <span class="material-symbols-outlined text-lg">lock</span>
              <span class="ml-3">Decrypt a File</span>
            </button>
          </div>
        </div>
      {/if}

      {#if step === 2}
        <div class="flex flex-col items-center">
          <h2
            class="text-center text-xl md:text-2xl font-semibold text-gray-100 mb-4"
          >
            File Management
          </h2>
          <div
            on:dragover={(event) => {
              event.preventDefault();
              isDragOver = true;
            }}
            on:dragleave={() => (isDragOver = false)}
            class={`w-full max-w-7xl border-4 ${isDragOver ? "border-purple-500 bg-gray-700 shadow-lg scale-105" : "border-dashed border-gray-600 bg-gray-800"} rounded-lg p-8 md:p-12 text-center flex flex-col items-center justify-center relative transition-all duration-300 ease-in-out`}
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
        </div>
      {/if}

      {#if step === 3}
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
                  <span class="px-4 text-gray-300 text-1xl font-semibold"
                    >OR</span
                  >
                  <div
                    class="flex-1 h-0.5 bg-gradient-to-l from-gray-700 to-gray-500"
                  ></div>
                </div>
              </div>

              <div
                id="keyDrag"
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
                } rounded-lg p-8 md:p-12 text-center flex flex-col items-center justify-center relative transition-all duration-300 ease-in-out transform-gpu hover:border-purple-500 hover:bg-gray-700 hover:shadow-lg hover:scale-105 cursor-pointer`}
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

      {#if step === 4}
        <section class="flex-1 p-4 md:p-8 flex flex-col items-center">
          <div
            class="w-full md:w-2/3 lg:w-1/2 bg-gray-700 p-4 rounded-lg shadow-md"
          >
            <h3 class="text-lg font-semibold text-gray-100 mb-4">Settings</h3>
            <div class="flex flex-col space-y-4">
              <div class="relative">
                <button
                  on:click={() => toggleDropdown("hashing")}
                  class="w-full py-2 px-4 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors duration-300"
                >
                  Hash Algorithm: {selectedHashAlgorithm}
                </button>
                {#if isDropdownOpen === "hashing"}
                  <div
                    class="absolute top-full mt-2 w-full bg-gray-800 text-gray-100 rounded-lg shadow-lg z-10"
                  >
                    <button
                      on:click={() => selectOption("Blake3Balloon", "hashing")}
                      class="w-full px-4 py-2 hover:bg-gray-700"
                    >
                      Blake3Balloon
                    </button>
                    <button
                      on:click={() => selectOption("Argon2ID", "hashing")}
                      class="w-full px-4 py-2 hover:bg-gray-700"
                    >
                      Argon2ID
                    </button>
                  </div>
                {/if}
              </div>

              <div class="relative">
                <button
                  on:click={() => toggleDropdown("encryption")}
                  class="w-full py-2 px-4 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors duration-300"
                >
                  Encryption Algorithm: {selectedEncryptionAlgorithm}
                </button>
                {#if isDropdownOpen === "encryption"}
                  <div
                    class="absolute top-full mt-2 w-full bg-gray-800 text-gray-100 rounded-lg shadow-lg z-10"
                  >
                    <button
                      on:click={() =>
                        selectOption("XChaCha20Poly1305", "encryption")}
                      class="w-full px-4 py-2 hover:bg-gray-700"
                    >
                      XChaCha20Poly1305
                    </button>
                    <button
                      on:click={() => selectOption("AES-256-GCM", "encryption")}
                      class="w-full px-4 py-2 hover:bg-gray-700"
                    >
                      AES-256-GCM
                    </button>
                  </div>
                {/if}
              </div>

              <div class="relative">
                <button
                  on:click={() => toggleDropdown("header")}
                  class="w-full py-2 px-4 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors duration-300"
                >
                  Header Version: {selectedHeaderVersion}
                </button>
                {#if isDropdownOpen === "header"}
                  <div
                    class="absolute top-full mt-2 w-full bg-gray-800 text-gray-100 rounded-lg shadow-lg z-10"
                  >
                    <button
                      on:click={() => selectOption("V5", "header")}
                      class="w-full px-4 py-2 hover:bg-gray-700"
                    >
                      V5
                    </button>
                    <button
                      on:click={() => selectOption("V3", "header")}
                      class="w-full px-4 py-2 hover:bg-gray-700"
                    >
                      V3 (For Argon2ID Hashing, don't use otherwise.)
                    </button>
                  </div>
                {/if}
              </div>
            </div>
          </div>
        </section>
      {/if}

      {#if step === 5}
        <div class="flex flex-col items-center space-y-4">
          <button
            on:click={processFile}
            class="px-6 py-3 text-lg font-semibold {actionTab === 'encrypt'
              ? 'bg-pink-500 text-white'
              : 'bg-purple-600 text-white'} rounded-full flex items-center justify-center transition-all duration-300 shadow-md hover:shadow-lg focus:outline-none focus:ring-4 focus:ring-blue-300"
          >
            {#if actionTab === "encrypt"}
              <span class="material-symbols-outlined text-lg">lock_open</span>
              <span class="ml-3 text-white">Encrypt</span>
            {/if}
            {#if actionTab === "decrypt"}
              <span class="material-symbols-outlined text-lg">lock</span>
              <span class="ml-3 text-white">Decrypt</span>
            {/if}
          </button>

          {#if isProcessing}
            <div
              class="mt-6 w-full max-w-md bg-gray-800 p-4 rounded-lg shadow-md"
            >
              <div class="mb-4">
                <p class="text-gray-300">
                  Time Elapsed: {Math.round(elapsedTime)} seconds
                </p>
              </div>
              <div class="relative h-4 bg-gray-600 rounded">
                <div
                  class="absolute top-0 left-0 h-full bg-green-500 rounded"
                  style="width: {progress}%; transition: width 0.5s;"
                ></div>
              </div>
              <p class="mt-2 text-gray-300">Progress: {progress}%</p>
            </div>
          {/if}
        </div>
      {/if}

      {#if showPasswordDialog}
        <PasswordPrompt
          on:submit={handlePasswordSubmit}
          on:cancel={handlePasswordCancel}
        />
      {/if}
    </div>
  </div>
</main>

<style>
  main {
    background-color: #1a202c;
    color: #f7fafc;
  }
  .highlight {
    color: #f6e05e;
  }
</style>
