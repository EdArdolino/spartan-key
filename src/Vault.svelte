<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import { navigate } from "svelte-navigator";
  
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
  
  <main class="container">
    <h1>Vault</h1>
  
    <p>
      Add A New Entry
    </p>
  
    <div class="row">
  
        <form class="row" on:submit|preventDefault={enter_name}>
          <input id="name-input" placeholder="Name of Entry:" bind:value={name_entry} />
        </form>
        <p>{nameMsg}</p>
      
        <form class="row" on:submit|preventDefault={enter_password}>
          <input id="password-input" type="password" placeholder="Password:" bind:value={password_entry} />
          <button type="submit">Save</button>
        </form>
        <p>{passMsg}</p>
    </div>
  
    <button on:click={()=>navigate("/")}>Lock</button>
  </main>
  
  <style>
    .logo.vite:hover {
      filter: drop-shadow(0 0 2em #ff3e00);
    }
  </style>
  