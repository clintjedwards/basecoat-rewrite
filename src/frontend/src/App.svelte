<script>
  import { globalHistory } from "svelte-routing/src/history";
  import Footer from "./components/Footer.svelte";

  let pathname = window.location.pathname;

  globalHistory.listen(({ location, _ }) => {
    pathname = location.pathname;
  });

  let routes = [{ path: "/formulas", text: "Formulas" }];
</script>

<main>
  <h1>Basecoat</h1>
  <div>
    <Footer />
  </div>
</main>

<style>
  :global(html) {
    overflow-y: scroll;
  }
</style>
