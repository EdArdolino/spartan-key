<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let passMsg = ""

  async function password(){
    passMsg = await invoke("password", { name })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={password}>
    <input id="password-input" type="password" placeholder="Password:" bind:value={name} />
    <button type="submit">Unlock</button>
  </form>
  <p>{passMsg}</p>
</div>

