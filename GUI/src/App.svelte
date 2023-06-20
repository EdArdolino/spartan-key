<script>
  import Greet from './lib/Greet.svelte'
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
    <Greet />
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
