<script>
  import { Route, Router } from "svelte-routing";
  import { globalHistory } from "svelte-routing/src/history";
  import Footer from "./components/Footer.svelte";
  import Login from "./components/Login.svelte";

  let pathname = window.location.pathname;

  globalHistory.listen(({ location, _ }) => {
    pathname = location.pathname;
  });
</script>

<main class="mx-auto w-3/4 flex flex-col min-h-screen">
  <div class="flex-1">
    <h1>Basecoat</h1>
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
