<script>
  import { invoke } from "@tauri-apps/api/tauri"
    import { navigate } from "svelte-navigator";

let name = "";
let passMsg = ""

async function unlock_msg(){
  passMsg = await invoke("unlock_msg", { name })
  navigate("/Entry")
}
</script>

<main class="container">
  <h1>Spartan Key</h1>

  <div class="row">
    <a href="https://github.com/EdArdolino/spartan-key" target="_blank">
      <img src="/spartankey.png" class="logo vite" alt="Spartan Key" />
    </a>
  </div>

  <p>
    Enter Master Password:
  </p>

  <div class="row">
      <form class="row" on:submit|preventDefault={unlock_msg}>
        <input id="password-input" type="password" placeholder="Password:" bind:value={name} />
        <button type="submit">Unlock</button>
      </form>
      <p>{passMsg}</p>
    </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  .row {
    display: flex; 
    flex-direction: column;
    justify-content: center;
  }
</style>
