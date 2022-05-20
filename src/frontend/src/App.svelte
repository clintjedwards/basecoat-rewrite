<script>
  import { onMount } from "svelte";
  import { Route, Router } from "svelte-routing";
  import { globalHistory } from "svelte-routing/src/history";
  import { client } from "./client.js";
  import Footer from "./components/Footer.svelte";
  import Login from "./components/Login.svelte";
  import { AppStore } from "./store.js";

  let pathname = window.location.pathname;

  globalHistory.listen(({ location, _ }) => {
    pathname = location.pathname;
  });

  onMount(async () => {
    client.get_system_info().then((info) => {
      AppStore.backend_info.set(info);
    });
  });
</script>

<main class="mx-auto w-3/4 flex flex-col min-h-screen">
  <div class="flex-1">
    <h1 class="text-8xl">Basecoat</h1>
    <Router>
      <Route path="login" component={Login} />
      <Route path="/" component={Login} />
    </Router>
  </div>
  <div>
    <Footer />
  </div>
</main>

<style>
  :global(html) {
    overflow-y: scroll;
  }
</style>
