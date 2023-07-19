<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name_entry = "";
  let nameMsg = ""

  async function enter_name(){
    nameMsg = await invoke("name", { name_entry })
  }

  let password_entry = "";
  let passMsg = ""

  async function enter_password(){
    passMsg = await invoke("password", { password_entry })
  }
</script>


<div>
  <form class="row" on:submit|preventDefault={enter_name}>
    <input id="name-input" placeholder="Name of Entry:" bind:value={name_entry} />
  </form>
  <p>{nameMsg}</p>
</div>

<div>
  <form class="row" on:submit|preventDefault={enter_password}>
    <input id="password-input" type="password" placeholder="Password:" bind:value={password_entry} />
    <button type="submit">Save</button>
  </form>
  <p>{passMsg}</p>
</div>

