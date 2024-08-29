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

  function getRandomDecSentence() {
    var index = Math.floor(Math.random() * decsentences.length);
    return decsentences[index];
  }

  var decsentences = [
    "So encrypted, it’s like your data is hidden in a magical code",
    "So secure, it’s like having a secret key to unlock your files",
    "So cryptic, even the most skilled decryptors need a hint",
    "So well-guarded, it’s like a fortress for your information",
    "So concealed, it’s like your data is wrapped in a mystery",
    "So protected, it’s like having a spell that keeps prying eyes away",
    "So encoded, it’s like speaking in a language only you understand",
    "So locked down, even wizards would need a spell to break through",
    "So hidden, it’s like your secrets are tucked away in a magical vault",
    "So obfuscated, it’s like your files are wrapped in layers of intrigue",
  ];

  function getRandomEncSentence() {
    var index = Math.floor(Math.random() * encsentences.length);
    return encsentences[index];
  }

  var encsentences = [
    "Conceal your documents from sorcerers with Volaris",
    "Keep your files hidden from spellcasters using Volaris",
    "Shield your data from magical eyes with Volaris",
    "Protect your information from wizards with Volaris",
    "Guard your files from enchantments with Volaris",
    "So secretive, it’s like a magic cloak for your files",
    "So encrypted, even wizards can’t break the code",
    "So secure, it’s like hiding your data in a vault",
    "So protected, it’s like a spellbinding shield for your files",
    "So guarded, even the most powerful sorcerers can’t breach it",
  ];

  function nextStep() {
    if (step < 5) step++;
  }

  function previousStep() {
    if (actionTab === "key") {
      step = 1;
      actionTab = "encrypt";
    }
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
      if (file === null) {
        return;
      } else {
        selectedFile =
          file && typeof file === "string" ? file : file?.path || "";
        delay(500);
        step = 3;
      }
    } else if (option === "key") {
      if (file === null) {
        return;
      } else {
        keyFile = file && typeof file === "string" ? file : file?.path || "";
        delay(500);
        step = 4;
      }
    }
  }

  async function saveKeyFile() {
    const file = await open({ directory: true });
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
        step = 1;
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
          isProcessing = true;
          let progdelay = 500;
          startTime = Date.now();
          progress = 0;
          const interval = setInterval(() => {
            elapsedTime = (Date.now() - startTime) / 1000;
            if (progress < 84) {
              progress += Math.round(Math.random() * 2.5);
            } else if (progress > 83 || progress === 84) {
              progress += 0.5;
              progdelay = 1000;
            }
          }, progdelay);

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
          isProcessing = true;
          let progdelay = 500;
          startTime = Date.now();
          progress = 0;
          const interval = setInterval(() => {
            elapsedTime = (Date.now() - startTime) / 1000;
            if (progress < 84) {
              progress += Math.round(Math.random() * 4);
            } else if (progress > 83 || progress === 84) {
              progress += Math.round(Math.random() * 1);
              progdelay = 1000;
            }
          }, progdelay);

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

<!-- I'll comment this all one day  -->

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
      {#if actionTab === "key"}
        <span class="text-lg font-semibold">Step 2 of 2</span>
      {/if}
      {#if actionTab === "encrypt" || actionTab === "decrypt"}
        <span class="text-lg font-semibold">Step {step} of 5</span>
      {/if}
      <button
        on:click={nextStep}
        class="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-500 disabled:opacity-50"
        disabled={step === 1 || step === 5 || actionTab === "key"}
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
          <div
            class="w-full max-w-md mx-auto flex flex-wrap justify-center gap-4"
          >
            <div
              class="bg-gray-600 rounded-lg shadow-lg p-4 w-full cursor-pointer hover:bg-gray-500 transition-all duration-200 transform hover:scale-105 flex flex-col items-center"
              on:click={() => {
                actionTab = "encrypt";
                step = 2;
              }}
            >
              <span
                class="material-symbols-outlined text-pink-500 text-4xl mb-2"
              >
                lock
              </span>
              <p class="text-center text-lg font-medium">Encrypt a File</p>
            </div>

            <div
              class="bg-gray-600 rounded-lg shadow-lg p-4 w-full cursor-pointer hover:bg-gray-500 transition-all duration-200 transform hover:scale-105 flex flex-col items-center"
              on:click={() => {
                actionTab = "decrypt";
                step = 2;
              }}
            >
              <span
                class="material-symbols-outlined text-purple-600 text-4xl mb-2"
              >
                lock_open
              </span>
              <p class="text-center text-lg font-medium">Decrypt a File</p>
            </div>

            <div
              class="bg-gray-600 rounded-lg shadow-lg p-4 w-full cursor-pointer hover:bg-gray-500 transition-all duration-200 transform hover:scale-105 flex flex-col items-center"
              on:click={() => {
                actionTab = "key";
                step = 3;
              }}
            >
              <span
                class="material-symbols-outlined text-pink-400 text-4xl mb-2"
              >
                key
              </span>
              <p class="text-center text-lg font-medium">Generate a Key</p>
            </div>
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
            on:click={() => pickFile("file")}
            on:dragleave={() => (isDragOver = false)}
            class={`w-full max-w-7xl border-4 rounded-lg p-8 md:p-12 text-center flex flex-col items-center justify-center relative transition-all duration-300 ease-in-out transform-gpu cursor-pointer ${isDragOver ? "border-purple-500 bg-gray-700 shadow-lg scale-105" : "border-dashed border-gray-600 bg-gray-800"} hover:border-purple-500 hover:bg-gray-700 hover:shadow-lg hover:scale-105`}
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
              {#if actionTab === "encrypt" || actionTab === "decrypt"}
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
              {/if}
            </div>
          </div>
        </div>
      {/if}

      {#if step === 4}
        <section
          class="flex-1 p-6 md:p-8 flex flex-col items-center min-h-screen"
        >
          <div
            class="w-full bg-gray-800 p-8 rounded-lg border border-gray-700 shadow-lg"
          >
            <h3
              class="text-3xl font-semibold text-gray-100 mb-8 border-b border-gray-700 pb-4"
            >
              Settings
            </h3>
            <div class="flex flex-col space-y-6">
              <div class="flex flex-col space-y-2">
                <label class="text-lg font-semibold text-gray-200"
                  >Hash Algorithm:</label
                >
                <div class="relative">
                  <button
                    on:click={() => toggleDropdown("hashing")}
                    class="w-full py-4 px-6 bg-gray-700 text-gray-200 rounded-lg flex items-center justify-between border border-gray-600 hover:bg-gray-600 transition-colors duration-300 text-lg"
                  >
                    <span class="flex-1">{selectedHashAlgorithm}</span>
                    <span class="material-symbols-outlined text-xl">
                      {isDropdownOpen === "hashing"
                        ? "arrow_upward"
                        : "arrow_downward"}
                    </span>
                  </button>
                  {#if isDropdownOpen === "hashing"}
                    <div
                      class="absolute top-full mt-2 w-full bg-gray-700 text-gray-200 rounded-lg border border-gray-600 shadow-lg z-10"
                    >
                      <button
                        on:click={() =>
                          selectOption("Blake3Balloon", "hashing")}
                        class="w-full px-6 py-3 hover:bg-gray-600 transition-colors duration-200 text-lg"
                      >
                        Blake3Balloon
                      </button>
                      <button
                        on:click={() => selectOption("Argon2ID", "hashing")}
                        class="w-full px-6 py-3 hover:bg-gray-600 transition-colors duration-200 text-lg"
                      >
                        Argon2ID
                      </button>
                    </div>
                  {/if}
                </div>
              </div>

              <div class="flex flex-col space-y-2">
                <label class="text-lg font-semibold text-gray-200"
                  >Encryption Algorithm:</label
                >
                <div class="relative">
                  <button
                    on:click={() => toggleDropdown("encryption")}
                    class="w-full py-4 px-6 bg-gray-700 text-gray-200 rounded-lg flex items-center justify-between border border-gray-600 hover:bg-gray-600 transition-colors duration-300 text-lg"
                  >
                    <span class="flex-1">{selectedEncryptionAlgorithm}</span>
                    <span class="material-symbols-outlined text-xl">
                      {isDropdownOpen === "encryption"
                        ? "arrow_upward"
                        : "arrow_downward"}
                    </span>
                  </button>
                  {#if isDropdownOpen === "encryption"}
                    <div
                      class="absolute top-full mt-2 w-full bg-gray-700 text-gray-200 rounded-lg border border-gray-600 shadow-lg z-10"
                    >
                      <button
                        on:click={() =>
                          selectOption("XChaCha20Poly1305", "encryption")}
                        class="w-full px-6 py-3 hover:bg-gray-600 transition-colors duration-200 text-lg"
                      >
                        XChaCha20Poly1305
                      </button>
                      <button
                        on:click={() =>
                          selectOption("AES-256-GCM", "encryption")}
                        class="w-full px-6 py-3 hover:bg-gray-600 transition-colors duration-200 text-lg"
                      >
                        AES-256-GCM
                      </button>
                    </div>
                  {/if}
                </div>
              </div>

              <div class="flex flex-col space-y-2">
                <label class="text-lg font-semibold text-gray-200"
                  >Header Version:</label
                >
                <div class="relative">
                  <button
                    on:click={() => toggleDropdown("header")}
                    class="w-full py-4 px-6 bg-gray-700 text-gray-200 rounded-lg flex items-center justify-between border border-gray-600 hover:bg-gray-600 transition-colors duration-300 text-lg"
                  >
                    <span class="flex-1">{selectedHeaderVersion}</span>
                    <span class="material-symbols-outlined text-xl">
                      {isDropdownOpen === "header"
                        ? "arrow_upward"
                        : "arrow_downward"}
                    </span>
                  </button>
                  {#if isDropdownOpen === "header"}
                    <div
                      class="absolute top-full mt-2 w-full bg-gray-700 text-gray-200 rounded-lg border border-gray-600 shadow-lg z-10"
                    >
                      <button
                        on:click={() => selectOption("V5", "header")}
                        class="w-full px-6 py-3 hover:bg-gray-600 transition-colors duration-200 text-lg"
                      >
                        V5
                      </button>
                      <button
                        on:click={() => selectOption("V3", "header")}
                        class="w-full px-6 py-3 hover:bg-gray-600 transition-colors duration-200 text-lg"
                      >
                        V3 (For Argon2ID Hashing, don't use otherwise.)
                      </button>
                    </div>
                  {/if}
                </div>
              </div>
            </div>
          </div>
        </section>
      {/if}

      {#if step === 5}
        <div class="flex flex-col items-center space-y-6">
          <div class="text-center">
            <h3
              class="text-4xl font-bold text-gray-100 border-b border-gray-700 pb-4 mb-4 flex items-center justify-center space-x-4"
            >
              <span class="material-symbols-outlined text-4xl">
                {actionTab === "encrypt" ? "lock" : "lock_open"}
              </span>
              <span>
                {actionTab === "encrypt" ? "Encryption" : "Decryption"}
              </span>
            </h3>

            <p class="text-lg text-gray-300 max-w-lg mx-auto">
              {actionTab === "encrypt"
                ? getRandomEncSentence()
                : getRandomDecSentence()}
            </p>
          </div>
          <button
            on:click={processFile}
            class="px-8 py-4 text-lg font-semibold rounded-full flex items-center justify-center transition-all duration-300 shadow-lg hover:shadow-xl focus:outline-none focus:ring-4 {actionTab ===
            'encrypt'
              ? 'bg-pink-500 text-white'
              : 'bg-purple-600 text-white'}"
          >
            <span class="material-symbols-outlined text-2xl"> play_arrow </span>
            <span class="ml-4 text-white font-medium">
              {actionTab === "encrypt"
                ? "Start Encryption"
                : "Start Decryption"}
            </span>
          </button>

          {#if isProcessing}
            <div class="w-full max-w-md bg-gray-800 p-6 rounded-lg shadow-lg">
              <h3 class="text-xl font-semibold text-gray-100 mb-4">
                Processing Your {actionTab === "encrypt"
                  ? "Encryption"
                  : "Decryption"}
              </h3>
              <p class="text-gray-300 mb-2">
                Your file is currently being {actionTab === "encrypt"
                  ? "encrypted"
                  : "decrypted"}. Please wait.
              </p>
              <p class="text-gray-300 mb-4">
                Time Elapsed: {Math.round(elapsedTime)} seconds
              </p>
              <div class="relative h-4 bg-gray-600 rounded overflow-hidden">
                <div
                  class="absolute top-0 left-0 h-full bg-green-500 rounded transition-all duration-500"
                  style="width: {progress}%;"
                ></div>
              </div>
              <p class="mt-2 text-gray-300">
                Progress: {progress}%
              </p>
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
