<script>
import { Router, Route } from "svelte-navigator";
    import Login from "./Login.svelte";
    import Entry from "./Vault.svelte";
</script>

<Router>
  <Route path="/">
    <Login>
    </Login>
  </Route>

  <Route path="/Entry">
    <Entry>
    </Entry>
  </Route>
</Router>